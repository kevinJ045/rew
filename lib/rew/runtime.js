(function () {
  "use strict";
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
          this.rew.prototype.fs, 
          this.rew.prototype.ffi,
          ...Object.keys(_rew_extensions)
          .map((key) => {
            let ext = _rew_extensions[key];
            if(ext[0]?.prototype?._namespace){
              return ext[0]
            } else return null;
          }).filter(Boolean)
        );
      };

      // Set the module for data operations
      for(let ext in _rew_extensions){
        const [_, fn] = _rew_extensions[ext];
        if(typeof fn == "function"){
          fn(this);
        }
      }
      
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
  const _createRew = () =>  _createClass({
    ptr: _createClass({
      of(val) {
        return Deno.UnsafePointer.of(val);
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
      pre(...types){
        return () => types;
      },
      typed: (...types) => {
        if(!types.length) return;
        const fn = types.pop();
        if(typeof fn != "function") return;
        let returnType = fn();
        let pre;
        if(Array.isArray(returnType)){
          pre = returnType.pop();
          returnType = returnType[0];
        }
        return {
          pre: pre,
          parameters: types,
          result: returnType
        };
      },
      void: "void",
      ptr: "pointer",
      buffer: "buffer", // e.g. Uint8Array
      u8: "u8",
      u16: "u16",
      u32: "u32",
      u64: "u64",
      i8: "i8",
      i16: "i16",
      i32: "i32",
      i64: "i64",
      f32: "f32",
      f64: "f64",
      struct: (def) => ({ struct: def }),
      open_raw: (libPath, symbols) => {
        try {
          return Deno.dlopen(libPath, symbols);
        } catch (e) {
          throw new Error(`Failed to load dynamic library "${libPath}": ${e.message}`);
        }
      },
      open(libPath, instance) {
        const entries = Object.entries(instance);
        const symbols = {};

        for (const [funcName, def] of entries) {
          if (!def || typeof def !== "object") {
            throw new Error(`Invalid FFI definition for "${funcName}".`);
          }

          const symbolName = funcName;
          try {
            symbols[symbolName] = {
              parameters: def.parameters.map(p => this._mapType(p)),
              result: this._mapType(def.result)
            };
          } catch (err) {
            throw new Error(`Error mapping FFI types for "${funcName}": ${err.message}`);
          }
        }

        let nativeSymbols;
        try {
          ({ symbols: nativeSymbols } = Deno.dlopen(libPath, symbols));
        } catch (e) {
          throw new Error(`Failed to load dynamic library "${libPath}": ${e.message}`);
        }

        const wrappers = {};
        for (const [funcName, def] of entries) {
          const symbolName = funcName;
          wrappers[funcName] = (...args) => {
            try {
              const result = nativeSymbols[symbolName](...args);
              return def.pre ? def.pre(result) : result;
            } catch (e) {
              throw new Error(`FFI call "${funcName}" failed: ${e.message}`);
            }
          };
        }

        const generated = {};
        for (const funcName of Object.keys(wrappers)) {
          Object.defineProperty(generated, funcName, {
            value: (...args) => wrappers[funcName](...args),
            enumerable: true
          });
        }

        return generated;
      },

      _mapType(type) {
        if (typeof type === "string") return type;
        if (type === this.ptr) return "pointer";
        if (type === this.buffer) return "buffer";
        if (typeof type === "object" && type.struct) {
          return {
            struct: type.struct
          };
        }
        throw new Error(`Unsupported FFI type: ${JSON.stringify(type)}`);
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
    ...Object.fromEntries(
      Object.keys(_rew_extensions).map((key) => {
        return [key, _rew_extensions[key][0]];
      })
    )
  });

  globalThis.rew = {
    ..._createRew(),
    ops: {
      ...globalThis.Deno.core.ops
    },
    extensions: {
      add(name, _class, fn){
        _rew_extensions[name] = [_class, fn || (() => {})];
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
