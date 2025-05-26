(function () {
  "use strict";
  const _log_out = console.log;
  const _err_out = console.error;

  globalThis.dispatchEvent = () => {}

  const MODULES = {};
  const GLOBALS = {};
  const USES = [];
  const ops = globalThis.Deno.core.ops;
  const Deno = globalThis.Deno;

  const _envdata = JSON.parse(ops.op_fetch_env());

  function resolve_namespace(ns){
    if(ns.__class__){
      if(ns.prototype._namespace){
        const result = ns.prototype._namespace();
        return typeof result == "string" ? { [result]: ns } : result;
      }
      return ns.prototype;
    }
    return ns;
  }

  const _createClass = (items) => {
    return {
      prototype: items,
      __class__: true
    }
  }

  let main = true;

  class Mod {
    exports = {};
    name = "";

    constructor(name){
      this.name = name;
    }
  }

  class InternalMod {
    filename = "";
    exports = {};
    options = {};
    app = {};  

    constructor(filename){
      this.filename = filename;
      
      // Initialize app property
      const appPath = ops.op_find_app(filename);
      if (appPath) {
        this.app = {
          path: appPath,
          config: appPath ? ops.op_app_loadconfig(appPath) : {}
        };
      } else {
        this.app = {
          path: "",
          config: {}
        };
      }
    }

    clone(newdata = {}){
      const new_mod = new InternalMod(this.filename);
      for(let i in (newdata || {})){
        new_mod[i] = newdata[i];
      }
      return new_mod;
    }
  }

  class ModuleNotFoundError extends Error {
    constructor(name){
      super(`Module "${name}" not found`);
    }
  }

  class Usage {
    system = (...args) => {}
    name = ""
    
    constructor(name, system){
      this.system = system;
      this.name = name;
    }
  }
  class Namespace extends Usage {
    namespace = {};
    static group = (...namespaces) => {
      let ns = {};
      
      namespaces.forEach(cns => {
        const rns = resolve_namespace(cns);
        ns = {
          ...ns,
          ...rns
        }
      });

      return ns;
    }
  }
  class INode {
    child = {}
    constructor(child){
      this.child = child;
    }
  }
  class Private extends INode {}
  class Public extends INode {}

  class RewExecutionContext {
    constructor(module){
      this.module = module;
      this.rew = _createRew(module, this);
      delete this.rew.prototype.mod.prototype.define;
      delete this.rew.prototype.mod.prototype.defineNew;
      delete this.rew.prototype.mod.prototype.new;
      delete this.rew.prototype.ops;
      

      for(let usage of USES){
        usage.system(this, ...(usage.args || []))
      }

      this.rew.prototype.ns = () => {
        return Namespace.group(
          this.rew.prototype.io,
          ...Object.keys(this.rew.prototype)
          .map((key) => {
            let ext = this.rew.prototype[key];
            if(ext?.prototype?._namespace){
              return ext
            } else return null;
          }).filter(Boolean)
        );
      };

      this.pvt = (child) => {
        if(child instanceof Namespace){
          child.namespace = new Private(child.namespace);
          return child;
        } else return new Private(child);
      }
      
      this.pub = (child) => {
        if(child instanceof Namespace){
          child.namespace = new Public(child.namespace);
          return child;
        } else return new Public(child);
      }

      this.instantiate = (...c) => new (c.pop())(...c)

      this.namespace = (namespace, fn) => {
        if(typeof namespace !== "object") {
          throw new TypeError("Namespace is not an object");
        }
        const system = () => {
          let namespace = ns.namespace;
          if(namespace instanceof Private && fn){
            _execVM(resolve_namespace(namespace.child), fn)
          } else {
            if(namespace instanceof Public) {
              namespace = resolve_namespace(namespace.child);
              const u = new Usage('namespace', (ctx) => {
                for(let i in namespace){
                  ctx[i] = namespace[i];
                }
              });
              USES.push(u);
            } else namespace = resolve_namespace(namespace);
            // _log_out(Object.keys(namespace));
            for(let i in namespace){
              this[i] = namespace[i];
            }
          }
        }
        const ns = new Namespace('namespace', system);
        ns.namespace = namespace;
        return ns;
      }

      // for(let usage of USES){
      //   if(usage.definitions())
      // }

      this.using = (usage, ...args) => {
        let pub = false, pvt = false;
        if(usage instanceof Public) {
          usage = usage.child;
          pub = true;
        }
        if(usage instanceof Private) {
          usage = usage.child;
          pvt = true;
        }
        if(usage instanceof Namespace){
          usage.system(this, ...args)
        } else if(usage instanceof Usage) {
          if(pvt){
            usage.system(this, ...args)
          } else {
            usage.system(...args)
          }
          if(pub){
            usage.args = args;
            USES.push(usage)
          }
        } else if(typeof usage == "string"){
          USES[usage] = args.length ? args : true;
        }
      }
    }
  }

  delete globalThis.console;

  const _rew_extensions = {};
  const _createRew = (...args) =>  _createClass({
    ptr: _createClass({
      _namespace(){
        return 'ptr';
      },
      of(val) {
        return Deno.UnsafePointer.of(val);
      },

      cb(params, result, callback){
        return new Deno.UnsafeCallback({
          parameters: params,
          result,
        }, callback);
      },
    
      view(ptr) {
        if (!(ptr instanceof Deno.UnsafePointer)) {
          throw new Error("Expected UnsafePointer");
        }
        return new Deno.UnsafePointerView(ptr);
      },
    
      read(ptr, type = "u8") {
        const view = this.view(ptr);
        switch (type) {
          case "u8": return view.getUint8();
          case "u16": return view.getUint16();
          case "u32": return view.getUint32();
          case "i8": return view.getInt8();
          case "i16": return view.getInt16();
          case "i32": return view.getInt32();
          case "f32": return view.getFloat32();
          case "f64": return view.getFloat64();
          default: throw new Error("Unsupported type: " + type);
        }
      },
    
      write(ptr, value, type = "u8") {
        const view = this.view(ptr);
        switch (type) {
          case "u8": return view.setUint8(value);
          case "u16": return view.setUint16(value);
          case "u32": return view.setUint32(value);
          case "i8": return view.setInt8(value);
          case "i16": return view.setInt16(value);
          case "i32": return view.setInt32(value);
          case "f32": return view.setFloat32(value);
          case "f64": return view.setFloat64(value);
          default: throw new Error("Unsupported type: " + type);
        }
      },
    
      deref(ptr, length = 1) {
        const view = this.view(ptr);
        return view.getArrayBuffer(length);
      },
    
      toBytes(ptr, length = 1) {
        const buf = this.deref(ptr, length);
        return new Uint8Array(buf);
      },
    
      string(ptr, length) {
        const view = this.view(ptr);
        return view.getCString(length);
      }
    }),
    mod: _createClass({
      define(mod, fn, require = [], options = {}){
        const name = mod instanceof Mod ? mod.name : mod;
        if(options.aliases.includes('::pvt')) return fn();
        MODULES[name] = {
          _call: fn,
          require,
          options,
          _extract: {}
        };
        if(options.aliases){
          for(let i of options.aliases){
            MODULES[i] = MODULES[name];
          }
        }
        return MODULES[name];
      },
      defineNew(name, fn, aliases){
        const mod = globalThis.rew.prototype.mod.prototype.new(name);
        
        const isAppModule = name.startsWith('app://');
        
        // For app modules, we want to store them with their app:// prefix
        // but pass the actual file path to the InternalMod constructor
        const moduleName = isAppModule ? name : mod.name;
        const filename = isAppModule ? name.replace('app://', '') : mod.name;
        
        return globalThis.rew.prototype.mod.prototype.define(moduleName, (options = {}) => {
          const internalMod = new InternalMod(filename);
          internalMod.options = options;
          
          if (isAppModule) {
            const parts = filename.split('/');
            internalMod.isApp = true;
            internalMod.packageName = parts[0];
            internalMod.entryName = parts[1] || 'main';
          }
          
          const context = new RewExecutionContext(internalMod);
          return fn(context);
        }, [], { aliases });
      },
      get(name, options = {}){
        if(name.startsWith('#')){
          return
        }
        if(!MODULES[name]){
          throw new ModuleNotFoundError(name);
        }
        let _extract = MODULES[name]._extract;
        let _extract_name = options && Object.keys(options) < 1 ? JSON.stringify(options) : '_defaults';
        if(!_extract[_extract_name]) {
          _extract[_extract_name] = MODULES[name]._call(options)
        }
        MODULES[name]._extract = _extract;
        if(main){
          MODULES[name].main = true;
          main = false;
        }
        return _extract[_extract_name];
      },
      find(from, name, options = {}){
        const fromPath = from instanceof Mod ? from.name : from?.filename || from;
        if(options.main){
          options.main = MODULES[from?.filename].main || false;
        }
        if(name.endsWith('!')){
          name = name.slice(0, -1);
        }
        
        const isAppImport = !name.startsWith('#') && 
                            !name.startsWith('.') && 
                            !name.includes('\\') && 
                            (name.includes('/') ? name.split('/')[0] : name);
        
        if (isAppImport) {
          const packageName = name.includes('/') ? name.split('/')[0] : name;
          const entryName = name.includes('/') ? name.split('/')[1] : 'main';
          
          const appModuleName = `app://${packageName}/${entryName}`;
          
          if (MODULES[appModuleName]) {
            return globalThis.rew.prototype.mod.prototype.get(appModuleName, options);
          }

          options.isApp = true;
          options.packageName = packageName;
          options.entryName = entryName;
        }
        
        const path = name.startsWith('#') ? name : 
                     name.startsWith('app://') ? name :
                     globalThis.rew.prototype.path.prototype.resolve(fromPath, name);
        return globalThis.rew.prototype.mod.prototype.get(path, options);
      },
      new(name){
        return new Mod(name);
      }
    }),
    channel: _createClass({
      new(interval = 1, cb){
        if(typeof interval == "function"){
          cb = interval;
          interval = 1;
        }
        if(interval < 1) interval = 1;
        let stop = 0;
        let lastTimeout = 0;
        const keepAlive = () => {
          if(typeof cb == "function"){
            cb();
          }
          if(!stop){
            lastTimeout = setTimeout(keepAlive, interval);
          }
        }
        keepAlive();
        return {
          stop(){
            stop = 1;
            clearTimeout(lastTimeout);
            return this;
          },
          start(){
            stop = 0;
            keepAlive();
            return this;
          },
          setpoll(_cb){
            cb = _cb;
            return this;
          }
        };
      },
      interval(interval = 1, cb){
        return setInterval(cb, interval);
      },
      timeout(interval = 1, cb){
        return setTimeout(cb, interval);
      },
      timeoutClear(c){
        return clearTimeout(c);
      },
      intervalClear(c){
        return clearInterval(c);
      }
    }),
    env: _createClass({
      env: _envdata.env,
      get(key) {
        return this.env[key];
      },
      set(key, value) {
        this.env[key] = value;
        return this;
      },
      delete(key) {
        delete this.env[key];
        return this;
      },
      has(key) {
        return !!this.env[key];
      },
      keys() {
        return Object.keys(this.env);
      }
    }),
    process: _createClass({
      pid: Deno.pid,
      ppid: Deno.ppid,
      cwd: _envdata.cwd,
      execPath: _envdata.execPath,
      args: Deno.args,
      onExit: (cb) => {
        Deno.os.setExitHandler(cb);
      },
      exit(code = 0) {
        Deno.core.ops.op_set_exit_code(code);
        Deno.core.ops.op_exit();
      },
    }),
    io: _createClass({
      _namespace(){
        return {
          print: this.out.print,
          printerr: this.out.err
        }
      },
      out: {
        ...Deno.stdout,
        print(...a) {
          return _log_out(...a);
        },
        err(...a) {
          return _err_out(...a);
        }
      },
      "in": {
        ...Deno.stdin
      }
    }),
    ...Object.fromEntries(
      Object.keys(_rew_extensions).map((key) => {
        return [key, _rew_extensions[key](Deno, ...args)];
      })
    )
  });

  globalThis.rew = {
    ..._createRew(),
    ops: {
      ...globalThis.Deno.core.ops
    },
    extensions: {
      add(name, _class, _require){
        if(_require){
          _require.find((item) => !_rew_extensions[item] ? (() => {throw new ReferenceError(`Rew extension ${name} requires extension ${item}.`)})() : '')
        }
        _rew_extensions[name] = _class;
        return _class;
      },
      createClass: _createClass,
      has: (name) => !!_rew_extensions[name]
    }
  };
  delete globalThis.Deno;
  // _log_out(__filename);
  
  // globalThis.inc = function (path) {
  //   function resolvePath(base, relative) {
  //     const baseParts = base.split('/');
  //     baseParts.pop(); // remove filename
  //     const relativeParts = relative.split('/');
    
  //     for (const part of relativeParts) {
  //       if (part === '.' || part === '') continue;
  //       if (part === '..') baseParts.pop();
  //       else baseParts.push(part);
  //     }
    
  //     return baseParts.join('/');
  //   }    
  // };
})();
