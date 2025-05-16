(function () {
  const _log_out = console.log;
  const _err_out = console.error;

  const MODULES = {};
  const GLOBALS = {};
  const USES = [];
  const ops = globalThis.Deno.core.ops;
  const Deno = globalThis.Deno;
  


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
      this.rew = _createRew();
      this.globalThis = {};
      delete this.rew.prototype.mod.prototype.define;
      delete this.rew.prototype.mod.prototype.defineNew;
      delete this.rew.prototype.mod.prototype.new;
      delete this.rew.prototype.ops;

      for(let usage of USES){
        usage.system(this, ...(usage.args || []))
      }

      this.rew.prototype.ns = () => {
        return Namespace.group(this.rew.prototype.io, this.rew.prototype.fs, this.rew.prototype.ffi);
      };

      this.rew.prototype.fs.prototype.cwd = () => module.filename;

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


  const _createRew = () =>  _createClass({
    mod: _createClass({
      define(mod, fn, require = [], options = {}){
        const name = mod instanceof Mod ? mod.name : mod;
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
        
        // Check if this is an app module path
        const isAppModule = name.startsWith('app://');
        
        // For app modules, we want to store them with their app:// prefix
        // but pass the actual file path to the InternalMod constructor
        const moduleName = isAppModule ? name : mod.name;
        const filename = isAppModule ? name.replace('app://', '') : mod.name;
        
        return globalThis.rew.prototype.mod.prototype.define(moduleName, (options = {}) => {
          const internalMod = new InternalMod(filename);
          internalMod.options = options;
          
          // For app modules, add additional metadata
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
        
        // Check if this is an app import
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
    encoding: _createClass({
      toBase64(data) {
        if (data instanceof Uint8Array) {
          return ops.op_to_base64(Array.from(data));
        }
        return ops.op_to_base64(data);
      },
      
      fromBase64(encoded, options = { asString: false }) {
        const result = ops.op_from_base64(encoded, { as_string: options.asString });
        if (!options.asString) {
          return new Uint8Array(result);
        }
        return result;
      },
      
      stringToBytes(str) {
        return Deno.core.encode(str);
      },
      
      bytesToString(bytes) {
        return Deno.core.decode(bytes);
      },
      
      encodeURIComponent(str) {
        return encodeURIComponent(str);
      },
      
      decodeURIComponent(str) {
        return decodeURIComponent(str);
      },
      
      bytesToHex(bytes) {
        if (!(bytes instanceof Uint8Array)) {
          throw new Error("Expected Uint8Array");
        }
        return Array.from(bytes)
          .map(b => b.toString(16).padStart(2, '0'))
          .join('');
      },
      
      hexToBytes(hex) {
        if (typeof hex !== 'string') {
          throw new Error("Expected string");
        }
        
        hex = hex.startsWith('0x') ? hex.slice(2) : hex;
        
        if (hex.length % 2 !== 0) {
          hex = '0' + hex;
        }
        
        const bytes = new Uint8Array(hex.length / 2);
        for (let i = 0; i < hex.length; i += 2) {
          bytes[i / 2] = parseInt(hex.substr(i, 2), 16);
        }
        
        return bytes;
      }
    }),
    path: _createClass({
      resolve(base, relative) {
        const baseParts = base.split('/');
        baseParts.pop();
        const relativeParts = relative.split('/');
      
        for (const part of relativeParts) {
          if (part === '.' || part === '') continue;
          if (part === '..') baseParts.pop();
          else baseParts.push(part);
        }
      
        return baseParts.join('/');
      }
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
    ffi: _createClass({
      _namespace(){
        return "ffi";
      },
      cwd(){},
      typed: (...types) => {
        if(!types.length) return;
        const fn = types.pop();
        if(typeof fn != "function") return;
        return {
          parameters: types,
          result: fn()
        };
      },
      void: "void",
      ptr: "pointer",
      open(libPath, ClassDef) {
        const instance = typeof ClassDef == "function" ? new ClassDef() : ClassDef;
        const className = instance.name;
        const entries = Object.entries(instance).filter(([k]) => k !== 'name');
    
        // Build symbol definitions for dlopen
        const symbols = {};
        for (const [funcName, def] of entries) {
          const symbolName = className ? `__${className}_${funcName}` : funcName;
          symbols[symbolName] = def;
        }
    
        const { symbols: nativeSymbols } = Deno.dlopen(libPath, symbols);
    
        // Generate wrapper functions for each symbol
        const wrappers = {};
        for (const [funcName, def] of entries) {
          const symbolName = className ? `__${className}_${funcName}` : funcName;
          wrappers[funcName] = (...args) => {
            const ret = nativeSymbols[symbolName](...args);
            if (funcName === 'init' && def.result === 'pointer') {
              return generatedClass.__constructor(ret);
            }
            return ret;
          };
        }
    
        // Generate the class
        class generatedClass {
          ptr;
    
          static __constructor(ptr) {
            const self = Object.create(generatedClass.prototype);
            self.ptr = ptr;
            return self;
          }
    
          constructor() {
            return wrappers['init']?.();
          }
    
          [Symbol.dispose]() {
            if (this.ptr) {
              if (wrappers['dealloc']) {
                wrappers['dealloc'](this.ptr);
              }
              this.ptr = null;
            }
          }
        }
    
        for (const [funcName, def] of entries) {
          if (funcName === 'init') continue;
    
          Object.defineProperty(generatedClass.prototype, funcName, {
            value: function (...args) {
              return wrappers[funcName](...(typeof ClassDef == "function" ? [this.ptr] : []), ...args);
            },
            enumerable: true,
          });
        }
    
        return typeof ClassDef == "function" ? generatedClass : new generatedClass;
      }
    }),
    fs: _createClass({
      cwd(){
        return "";
      },
      
      async read(path, options = { binary: false }) {
        const result = await ops.op_fs_read(this.cwd(), path, options);
        if (options.binary) {
          // Convert array of numbers to Uint8Array
          return new Uint8Array(result);
        }
        return result;
      },
      
      // Write file (accepts string for text, Uint8Array/Array for binary)
      async write(path, content, options = { binary: false, create_dirs: false }) {
        if (options.binary && content instanceof Uint8Array) {
          // Convert Uint8Array to regular array for serialization
          content = Array.from(content);
        }
        return await ops.op_fs_write(this.cwd(), path, content, options);
      },
      
      // Utility methods for binary data
      async readBinary(path) {
        return await this.read(path, { binary: true });
      },
      
      async writeBinary(path, data) {
        return await this.write(path, data, { binary: true, create_dirs: true });
      },
      
      // Convert string to binary data
      stringToBytes(str) {
        const encoder = new TextEncoder();
        return encoder.encode(str);
      },
      
      // Convert binary data to string
      bytesToString(bytes) {
        const decoder = new TextDecoder();
        return decoder.decode(bytes);
      },
      
      // Other methods remain the same
      exists(path) {
        return ops.op_fs_exists(this.cwd(), path);
      },
      
      async rm(path, options = {}) {
        return trackPromise(ops.op_fs_rm(this.cwd(), path, options));
      },
      
      stats(path) {
        const statsJson = ops.op_fs_stats(this.cwd(), path);
        return JSON.parse(statsJson);
      },
      
      async mkdir(path, options = {}) {
        return trackPromise(ops.op_fs_mkdir(this.cwd(), path, options));
      },
      
      readdir(path, options = {}) {
        const entriesJson = ops.op_fs_readdir(this.cwd(), path, options);
        return JSON.parse(entriesJson);
      },
      
      async copy(src, dest, options = {}) {
        return trackPromise(ops.op_fs_copy(this.cwd(), src, dest, options));
      },
      
      async rename(src, dest) {
        return trackPromise(ops.op_fs_rename(this.cwd(), src, dest));
      },
      
      cwd() {
        return ops.op_fs_cwd();
      },
      
      resolve(path) {
        if (!path) return ".";
        return rew.prototype.path.prototype.resolve(this.cwd(), path);
      },
      
      async ensureDir(path) {
        return await this.mkdir(path, { recursive: true });
      },
      
      async rmrf(path) {
        return await this.rm(path, { recursive: true });
      },
      
      isDirectory(path) {
        try {
          const stats = this.stats(path);
          return stats.isDirectory;
        } catch (e) {
          return false;
        }
      },
      
      isFile(path) {
        try {
          const stats = this.stats(path);
          return stats.isFile;
        } catch (e) {
          return false;
        }
      }
    }),
  });

  globalThis.rew = {
    ..._createRew(),
    ops: {
      ...globalThis.Deno.core.ops
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
