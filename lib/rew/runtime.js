(function () {
  const _log_out = console.log;
  const _err_out = console.error;


  // _log_out(Object.keys(globalThis.Deno.core));

  const MODULES = {};
  const GLOBALS = {};
  const USES = [];
  const ops = globalThis.Deno.core.ops;
  const Deno = globalThis.Deno;

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
    constructor(filename){
      this.filename = filename;
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
      this.rew = {...globalThis.rew};
      this.globalThis = {};
      delete this.rew.prototype.mod.prototype.define;
      delete this.rew.prototype.mod.prototype.defineNew;
      delete this.rew.prototype.mod.prototype.new;
      delete this.rew.prototype.ops;

      for(let usage of USES){
        usage.system(this, ...(usage.args || []))
      }

      this.rew.prototype.ns = () => {
        const namespace = {
          print: this.rew.prototype.io.prototype.out.print
        };
        const iterateGroup = (group) => {
          for(let i in group){
            if(i == "__class__") continue;
            if(group[i].__class__ && group[i].prototype){
              iterateGroup(group[i].prototype);
            } else namespace[i] = group[i];
          }
        }
        iterateGroup(this.rew.prototype.fs.prototype);
        return namespace;
      };

      this.rew.prototype.fs = _createClass({
        // Read file (returns string for text, Uint8Array for binary)
        async read(path, options = { binary: false }) {
          const result = await ops.op_fs_read(module.filename, path, options);
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
          return await ops.op_fs_write(module.filename, path, content, options);
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
          return ops.op_fs_exists(module.filename, path);
        },
        
        async rm(path, options = {}) {
          return trackPromise(ops.op_fs_rm(module.filename, path, options));
        },
        
        stats(path) {
          const statsJson = ops.op_fs_stats(module.filename, path);
          return JSON.parse(statsJson);
        },
        
        async mkdir(path, options = {}) {
          return trackPromise(ops.op_fs_mkdir(module.filename, path, options));
        },
        
        readdir(path, options = {}) {
          const entriesJson = ops.op_fs_readdir(module.filename, path, options);
          return JSON.parse(entriesJson);
        },
        
        async copy(src, dest, options = {}) {
          return trackPromise(ops.op_fs_copy(module.filename, src, dest, options));
        },
        
        async rename(src, dest) {
          return trackPromise(ops.op_fs_rename(module.filename, src, dest));
        },
        
        cwd() {
          return ops.op_fs_cwd();
        },
        
        resolve(path) {
          if (!path) return ".";
          return rew.prototype.path.prototype.resolve(module.filename, path);
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
      })

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
            _execVM(namespace.child, fn)
          } else {
            if(namespace instanceof Public) {
              namespace = namespace.child;
              const u = new Usage('namespace', (ctx) => {
                for(let i in namespace){
                  ctx[i] = namespace[i];
                }
              });
              USES.push(u);
            }
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
  globalThis.rew = _createClass({
    mod: _createClass({
      define(mod, fn, require = [], options = {}){
        const name = mod instanceof Mod ? mod.name : mod;
        MODULES[name] = {
          _call: fn,
          require,
          options,
          _extract: {}
        };
        return MODULES[name];
      },
      defineNew(name, fn){
        const mod = globalThis.rew.prototype.mod.prototype.new(name);
        return globalThis.rew.prototype.mod.prototype.define(mod, (options = {}) => {
          const internalMod = new InternalMod(mod.name);
          internalMod.options = options;
          const context = new RewExecutionContext(internalMod);
          return fn(context);
        });
      },
      get(name, options = {}){
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
        // _log_out(name)
        const fromPath = from instanceof Mod ? from.name : from?.filename || from;
        if(options.main){
          options.main = MODULES[from?.filename].main || false;
        }
        const path = globalThis.rew.prototype.path.prototype.resolve(fromPath, name);
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
      out: {
        ...globalThis.Deno.stdout,
        print(...a) {
          return _log_out(...a);
        },
        err(...a) {
          return _err_out(...a);
        }
      },
      "in": {
        ...globalThis.Deno.stdin
      }
    }),
    ops: {
      ...globalThis.Deno.core.ops
    }
  });
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
