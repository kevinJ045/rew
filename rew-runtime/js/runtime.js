(function () {
  "use strict";
  const _log_out = console.log;
  const _err_out = console.error;
  const compile = globalThis.compile;
  delete globalThis.compile;

  globalThis.dispatchEvent = () => { };

  const MODULES = {};
  const PREPROCESSORS = [
    {
      checker: /\.json$/,
      fn: (_, code) => JSON.parse(code)
    },
    {
      checker: /\.yaml$/,
      fn: (_, code) => ops.op_string_to_yaml(code)
    }
  ];
  const USES = [];
  const ops = globalThis.Deno.core.ops;
  const Deno = globalThis.Deno;

  const _envdata = JSON.parse(ops.op_fetch_env());

  // Cross-platform path handling
  const isWindows = ops.op_os_info_os() === 'windows';
  const pathSep = isWindows ? '\\' : '/';
  const pathSepRegex = isWindows ? /[/\\]/g : /\//g;

  function normalizePath(path) {
    return path.replace(pathSepRegex, pathSep);
  }

  function splitPath(path) {
    return path.split(pathSepRegex);
  }

  function joinPath(...parts) {
    return parts.join(pathSep);
  }

  function isAbsolutePath(path) {
    return isWindows ? /^[A-Za-z]:[/\\]/.test(path) : path.startsWith('/');
  }

  function resolve_namespace(ns) {
    if (ns.__class__) {
      if (ns.prototype._namespace) {
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
      __class__: true,
    };
  };

  let main = true;

  class Mod {
    exports = {};
    name = "";

    constructor(name) {
      this.name = name;
    }
  }

  const stdin = Deno.stdin;
  const stdout = Deno.stdout;
  stdout.print = (...a) => {
    return _log_out(...a);
  };
  stdout.printf = (string) => {
    return stdout.writeSync(Deno.core.encode(string));
  };
  stdout.err = (...a) => {
    return _err_out(...a);
  };
  stdout.size = () => {
    return ops.op_terminal_size();
  };
  stdin.input = (print) => {
    const buf = new Uint8Array(1024);
    if (print) stdout.printf(print);
    const n = stdin.readSync(buf);
    return Deno.core.decode(buf.subarray(0, n)).trim();
  };

  class InternalMod {
    filename = "";
    exports = {};
    options = {};
    app = {};

    constructor(filename) {
      this.filename = (() => {
        let parts = splitPath(filename);
        let app_name = parts.shift();
        let file_path = joinPath(...parts);
        if (globalThis['__app__' + app_name]) {
          return normalizePath(joinPath(globalThis['__app__' + app_name], file_path));
        } else return filename;
      })();
      this.id = filename;

      // Initialize app property
      const appPath = ops.op_find_app(this.filename);
      if (appPath) {
        this.app = {
          path: appPath,
          config: appPath ? ops.op_app_loadconfig(appPath) : {},
        };
      } else {
        this.app = {
          path: "",
          config: {},
        };
      }
    }

    clone(newdata = {}) {
      const new_mod = new InternalMod(this.filename);
      for (let i in newdata || {}) {
        new_mod[i] = newdata[i];
      }
      return new_mod;
    }
  }

  class ModuleNotFoundError extends Error {
    constructor(name) {
      super(`Module "${name}" not found`);
    }
  }

  class Usage {
    system = (...args) => { };
    name = "";

    constructor(name, system) {
      this.system = system;
      this.name = name;
    }
  }
  class Namespace extends Usage {
    namespace = {};
    static group = (...namespaces) => {
      let ns = {};

      namespaces.forEach((cns) => {
        const rns = resolve_namespace(cns);
        ns = {
          ...ns,
          ...rns,
        };
      });

      return ns;
    };
  }
  class INode {
    child = {};
    args = [];
    constructor(child, args) {
      this.child = child;
      this.args = args || [];
    }
  }
  class Private extends INode { }
  class Public extends INode { }

  class JSXFragment { }

  class SubPackage {
    // init(module){
    //   let that = this;
    //   return new Proxy(that, {
    //     get(target, p, reciever){
    //       if(module.app.config.manifest.package !== that.packageName){
    //         throw new ReferenceError('Sub package unavailable');
    //       }
    //       return Reflect.get(target, p, reciever);
    //     }
    //   });
    // }
    prototype = {};
    __class__ = true;
    define(name, value) {
      this.prototype[name] = value;
      return value;
    }
  }
  const SUB_PACKAGES = [];

  class PackageList {
    _default = null;
  }

  class RewExecutionContext {
    constructor(module) {
      this.module = module;
      this.rew = _createRew(module, this);
      //this.this = this;
      delete this.rew.prototype.mod.prototype.define;
      delete this.rew.prototype.mod.prototype.defineNew;
      delete this.rew.prototype.mod.prototype.new;
      delete this.rew.prototype.ops;

      this.rew.prototype.mod.prototype.package = (packageName) => {
        packageName = packageName.replace(/\:\:/g, ".");
        let p = new SubPackage();
        let name = packageName;
        let path = null;
        if (packageName.match(".")) {
          let parts = packageName.split(".");
          name = parts.pop();
          path = parts;
        }
        Object.defineProperty(p, "packageName", {
          value: module.app.config.manifest.package,
          writable: false,
          configurable: false,
          enumerable: true,
        });
        Object.defineProperty(p, "name", {
          value: name,
          writable: false,
          configurable: false,
          enumerable: true,
        });
        p.define("_namespace", () => {
          return p.prototype?.namespace || name;
        });
        if (path && path.length) {
          let main = path.shift();
          let parent =
            SUB_PACKAGES.find((p) => p.name == main) ||
            this.rew.prototype.mod.prototype.package(main);
          path.forEach((p) => {
            parent =
              parent.prototype[p] ||
              this.rew.prototype.mod.prototype.package(p);
          });
          parent.define(name, p);
          this[name] = p;
        } else {
          SUB_PACKAGES.push(p);
          this[name] = p;
        }
        return p;
      };

      SUB_PACKAGES.filter(
        (p) => p.packageName == module.app.config.manifest.package,
      ).forEach((item) => {
        Object.defineProperty(this, item.name, {
          value: item,
          writable: false,
          configurable: false,
          enumerable: true,
        });
      });

      for (let usage of USES) {
        usage.system(this, ...(usage.args || []));
      }

      this.imp = async (filename) => {
        let [filepath, to_exec] = await ops.op_dyn_imp(
          isAbsolutePath(filename) ? (isWindows ? "C:\\" : "/") : module.filename,
          filename,
        );
        _evalVM(to_exec);
        return this.rew.prototype.mod.prototype.find(module, filepath);
      };
      this.Usage = _createClass({
        create: (fn) => {
          return new Usage(fn.name, fn);
        },
      });

      this.declare = (name, item) => {
        if (name && !item) {
          item = name;
          name = item.name;
        }

        if (item instanceof SubPackage) {
          if (!(this.module.exports instanceof PackageList)) {
            this.module.exports = new PackageList();
          }
          if (this.module.exports._default) {
            this.module.exports._default = null;
          } else this.module.exports._default = name;
          this.module.exports[name] = item;
        } else if (name) {
          this.module.exports[name] = item;
        }
        return item;
      }

      this.rew.prototype.__defineGetter__("ns", () => {
        return Namespace.group(
          this.rew.prototype.io,
          ...Object.keys(this.rew.prototype)
            .map((key) => {
              if (key == "ns" || key == "io") return null;
              let ext = this.rew.prototype[key];
              if (ext?.prototype?._namespace) {
                return ext;
              } else return null;
            })
            .filter(Boolean),
        );
      });

      this.genUid = function (length = 12, seed) {
        return ops.op_gen_uid(length, seed);
      }
      this.randFrom = function (min, max, seed) {
        return ops.op_rand_from(min, max, seed);
      }
      this.pickRandom = (...picks) => {
        return this.pickRandomWithSeed(undefined, ...picks);
      }
      this.pickRandomWithSeed = function (seed, ...picks) {
        if (picks.length < 2) return picks[0] || picks;
        return picks[ops.op_rand_from(0, picks.length - 1, seed)];
      }

      this.pvt = (child, ...args) => {
        if (child instanceof Namespace) {
          child.namespace = new Private(child.namespace);
          return child;
        } else return new Private(child, args);
      };
      this.pvt.is = (item) => item instanceof Private;

      this.pub = (child, ...args) => {
        if (child instanceof Namespace) {
          child.namespace = new Public(child.namespace, args);
          return child;
        } else if (child instanceof SubPackage) {
          this.declare(child)
        } else return new Public(child, args);
      };
      this.pub.is = (item) => item instanceof Public;

      this.instantiate = (...c) => new (c.pop())(...c);

      this.namespace = (namespace, fn) => {
        if (typeof namespace !== "object") {
          throw new TypeError("Namespace is not an object");
        }
        const system = () => {
          let namespace = ns.namespace;
          if (namespace instanceof Private && fn) {
            _execVM(resolve_namespace(namespace.child), fn);
          } else {
            if (namespace instanceof Public) {
              namespace = resolve_namespace(namespace.child);
              const u = new Usage("namespace", (ctx) => {
                for (let i in namespace) {
                  ctx[i] = namespace[i];
                }
              });
              USES.push(u);
            } else namespace = resolve_namespace(namespace);
            // _log_out(Object.keys(namespace));
            for (let i in namespace) {
              this[i] = namespace[i];
            }
          }
        };
        const ns = new Namespace("namespace", system);
        ns.namespace = namespace;
        return ns;
      };

      // for(let usage of USES){
      //   if(usage.definitions())
      // }

      let jsx = this.JSX = new Usage("jsx", (ctx, fn) => {
        // _log_out(ctx.module.filename, fn);
        ctx.__jsx__prefix = fn;
        jsx.prototype.__prefix = fn;
      });
      this.JSX.prototype = {
        __prefix(name, props = {}, ...children) {
          return {
            name,
            props,
            children
          }
        },
        Fragment: new JSXFragment,
        isFragment(item) {
          return item instanceof JSXFragment;
        },
        new(...items) {
          return jsx.prototype.__prefix(...items);
        }
      };

      this.using = (usage, ...args) => {
        let pub = false,
          pvt = false;
        if (usage instanceof Public) {
          args = usage.args;
          usage = usage.child;
          pub = true;
        }
        if (usage instanceof Private) {
          args = usage.args;
          usage = usage.child;
          pvt = true;
        }
        if (usage instanceof Namespace) {
          usage.system(this, ...args);
        } else if (usage instanceof Usage) {
          if (pvt) {
            usage.system(this, ...args);
          } else {
            usage.system(this, ...args);
          }
          if (pub) {
            usage.args = args;
            USES.push(usage);
          }
        } else if (typeof usage == "string") {
          USES[usage] = args.length ? args : true;
        }
      };
    }
  }

  delete globalThis.console;

  const _rew_extensions = {};
  const _createRew = (...args) =>
    _createClass({
      ptr: _createClass({
        _namespace() {
          return "ptr";
        },
        of(val) {
          return Deno.UnsafePointer.of(val);
        },

        fn(params, result, callback) {
          return new Deno.UnsafeCallback(
            {
              parameters: params,
              result,
            },
            callback,
          );
        },

        view(ptr) {
          return new Deno.UnsafePointerView(ptr);
        },

        read(ptr, type = "u8") {
          const view = this.view(ptr);
          switch (type) {
            case "u8":
              return view.getUint8();
            case "u16":
              return view.getUint16();
            case "u32":
              return view.getUint32();
            case "i8":
              return view.getInt8();
            case "i16":
              return view.getInt16();
            case "i32":
              return view.getInt32();
            case "f32":
              return view.getFloat32();
            case "f64":
              return view.getFloat64();
            default:
              throw new Error("Unsupported type: " + type);
          }
        },

        write(ptr, value, type = "u8") {
          const view = ptr instanceof Deno.UnsafePointerView ? ptr : this.view(ptr);
          switch (type) {
            case "u8":
              return view.setUint8(value);
            case "u16":
              return view.setUint16(value);
            case "u32":
              return view.setUint32(value);
            case "i8":
              return view.setInt8(value);
            case "i16":
              return view.setInt16(value);
            case "i32":
              return view.setInt32(value);
            case "f32":
              return view.setFloat32(value);
            case "f64":
              return view.setFloat64(value);
            default:
              throw new Error("Unsupported type: " + type);
          }
        },

        readArray(ptr, length, type = "u8") {
          const view = this.view(ptr);
          const arr = [];
          for (let i = 0; i < length; i++) {
            arr.push(this.read(Deno.UnsafePointer.create(ptr.valueOf() + i), type));
          }
          return arr;
        },

        writeArray(ptr, array, type = "u8") {
          const view = this.view(ptr);
          const typeSize = this.sizeOf(type);
          for (let i = 0; i < array.length; i++) {
            this.write(Deno.UnsafePointer.create(i * typeSize), array[i], type);
          }
        },

        readBool(ptr) {
          return !!this.read(ptr, "u8");
        },

        writeBool(ptr, val) {
          this.write(ptr, val ? 1 : 0, "u8");
        },

        readStruct(ptr, structDef) {
          const result = {};
          let offset = 0;
          for (const [field, type] of Object.entries(structDef)) {
            result[field] = this.read(Deno.UnsafePointer.create(ptr.valueOf() + offset), type);
            offset += this.sizeOf(type);
          }
          return result;
        },

        sizeOf(type) {
          switch (type) {
            case "u8": case "i8": return 1;
            case "u16": case "i16": return 2;
            case "u32": case "i32": case "f32": return 4;
            case "f64": return 8;
            default: throw new Error("Unknown type " + type);
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
        },
      }),
      mod: _createClass({
        define(mod, fn, require = [], options = {}) {
          const name = mod instanceof Mod ? mod.name : mod;
          if (options.aliases?.includes("::pvt")) return fn();
          MODULES[name] = {
            _call: fn,
            require,
            options,
            main,
            _extract: {},
          };
          if (main) main = false;
          if (options.aliases) {
            for (let i of options.aliases) {
              MODULES[i] = MODULES[name];
            }
          }
          return MODULES[name];
        },
        defineNew(name, fn, aliases) {
          const mod = globalThis.rew.prototype.mod.prototype.new(name);
          const isAppModule = name.startsWith("app://");
          fn = fn[name] || fn;

          // For app modules, we want to store them with their app:// prefix
          // but pass the actual file path to the InternalMod constructor
          const moduleName = isAppModule ? name : mod.name;
          const filename = isAppModule ? name.replace("app://", "") : mod.name;

          return globalThis.rew.prototype.mod.prototype.define(
            moduleName,
            (options = {}) => {
              const internalMod = new InternalMod(filename);
              internalMod.options = options;

              if (isAppModule) {
                const parts = filename.split("/");
                internalMod.isApp = true;
                internalMod.packageName = parts[0];
                internalMod.entryName = parts[1] || "main";
              }

              const context = new RewExecutionContext(internalMod, name);
              return fn?.call(context, context);
            },
            [],
            { aliases },
          );
        },
        get(name, options = {}) {
          if (name.startsWith("#")) {
            return;
          }
          if (!MODULES[name]) {
            throw new ModuleNotFoundError(name);
          }
          let _extract = MODULES[name]._extract;
          let _extract_name =
            options && Object.keys(options) > 1
              ? JSON.stringify(options)
              : "_defaults";
          if (!_extract[_extract_name]) {
            try {
              _extract[_extract_name] = MODULES[name]._call(options);
            } catch (e) {
              e.stack = e.stack.split('at Object.globalThis.rew.mod.define.aliases.aliases [as _call]')?.[0]?.trim() || e.stack;
              throw e;
            }
          }
          MODULES[name]._extract = _extract;
          if (MODULES[name].main) {
            //_log_out(name, _extract, _extract_name);
            if (_extract[_extract_name]?.main) {
              _extract[_extract_name].main();
            }
          }
          return _extract[_extract_name];
        },
        find(from, name, options = {}) {
          const fromPath =
            from instanceof Mod ? from.name : from?.id || from;
          if (options.main) {
            options.main = MODULES[from?.id].main || false;
          }
          if (name.endsWith("!")) {
            name = name.slice(0, -1);
          }

          const isAppImport =
            !name.startsWith("#") &&
            !name.startsWith(".") &&
            !name.includes("\\") &&
            !(isWindows ? /^[A-Za-z]:/.test(name) : name.startsWith("/")) &&
            (name.includes("/") ? name.split("/")[0] : name);

          if (isAppImport) {
            const packageName = name.includes("/") ? name.split("/")[0] : name;
            const entryName = name.includes("/") ? name.split("/")[1] : "main";

            const appModuleName = `app://${packageName}/${entryName}`;

            if (MODULES[appModuleName]) {
              return globalThis.rew.prototype.mod.prototype.get(
                appModuleName,
                options,
              );
            }

            options.isApp = true;
            options.packageName = packageName;
            options.entryName = entryName;
          }

          const path =
            name.startsWith("#") || isAbsolutePath(name)
              ? name
              : name.startsWith("app://")
                ? name
                : globalThis.rew.prototype._path.prototype.resolveFrom(
                  normalizePath(fromPath),
                  name,
                );

          let exported = globalThis.rew.prototype.mod.prototype.get(path, options);

          if (exported instanceof PackageList) {

            for (let i in exported) {
              if (i == "_default") continue;
              if (args[1]) {
                args[1][i] = exported[i];
              }
            }

            if (exported._default) {
              return exported[exported._default];
            }
          }

          return exported;
        },
        new(name) {
          return new Mod(name);
        },
        preprocess(name, code) {
          for (let prerocessor of PREPROCESSORS) {
            if (prerocessor.checker.test(name)) {
              return prerocessor.fn(name, code);
            }
          }
          return code;
        },
        registerPreprocessor(checker, fn) {
          PREPROCESSORS.push({
            checker,
            fn
          });
        }
      }),
      _path: _createClass({
        resolveFrom(base, relative) {
          const baseParts = splitPath(base);
          baseParts.pop();
          const relativeParts = splitPath(relative);

          for (const part of relativeParts) {
            if (part === "." || part === "") continue;
            if (part === "..") baseParts.pop();
            else baseParts.push(part);
          }

          return joinPath(...baseParts);
        },
      }),
      channel: _createClass({
        new(interval = 1, cb) {
          if (typeof interval == "function") {
            cb = interval;
            interval = 1;
          }
          if (interval < 1) interval = 1;
          let stop = 0;
          let lastTimeout = 0;
          const keepAlive = () => {
            if (typeof cb == "function") {
              cb.call(ctx);
            }
            if (!stop) {
              lastTimeout = setTimeout(keepAlive, interval);
            }
          };
          const ctx = {
            stop() {
              stop = 1;
              clearTimeout(lastTimeout);
              return this;
            },
            start() {
              stop = 0;
              keepAlive();
              return this;
            },
            setpoll(_cb) {
              cb = _cb;
              return this;
            },
          };
          keepAlive();
          return ctx;
        },
        interval(interval = 1, cb) {
          return setInterval(cb, interval);
        },
        timeout(interval = 1, cb) {
          return setTimeout(cb, interval);
        },
        timeoutClear(c) {
          return clearTimeout(c);
        },
        intervalClear(c) {
          return clearInterval(c);
        },
        emitter() {
          return {
            _listeners: [],
            on(event, cb) {
              if (Array.isArray(event)) {
                this._listeners.push(
                  event.map(event => ({ event, cb }))
                );
              } else {
                this._listeners.push({ event, cb });
              }
              return this;
            },
            off(event, cb) {
              this._listeners.filter((item) => {
                if (Array.isArray(item)) {
                  if (event.includes(item.event) && (cb ? item.cb == cb : true)) {
                    return false;
                  }
                } else {
                  if (item.event == event && (cb ? item.cb == cb : true)) {
                    return false;
                  }
                }
                return true;
              });
              return false;
            },
            emit(event, ...data) {
              return this._listeners.map(item => {
                if (item.event == event || (Array.isArray(event) ? event.includes(item.event) : true)) {
                  return item.cb.call(this, ...data);
                }
              }).filter(Boolean);
            }
          }
        }
      }),
      env: _createClass({
        env: {
          REW_ROOT: _envdata.rewPath,
          REW_TEMP_DIR: _envdata.tempDir,
          ..._envdata.env,
        },
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
        },
      }),
      process: _createClass({
        pid: Deno.pid,
        ppid: Deno.ppid,
        cwd: _envdata.cwd,
        execPath: _envdata.execPath,
        args: ops.op_get_args(),
        onExit: (cb) => {
          Deno.os.setExitHandler(cb);
        },
        exit(code = 0) {
          Deno.core.ops.op_set_exit_code(code);
          Deno.core.ops.op_exit();
        },
      }),
      bootstrap: _createClass({
        compile: compile,
      }),
      vfile: _createClass({
        find(path) {
          return ops.op_vfile_get(path)
        },
        add(path, content) {
          return ops.op_vfile_set(path, content);
        }
      }),
      io: _createClass({
        _namespace() {
          return {
            print: this.out.print,
            printerr: this.out.err,
            printf: this.out.printf,
            input: this.in.input,
          };
        },
        out: stdout,
        in: stdin,
        err: Deno.stderr,
      }),
      ...Object.fromEntries(
        Object.keys(_rew_extensions).map((key) => {
          return [key, _rew_extensions[key](Deno, ...args)];
        }),
      ),
    });

  globalThis.rew = {
    ..._createRew(),
    ops: {
      ...globalThis.Deno.core.ops,
    },
    extensions: {
      add(name, _class, _require) {
        if (_require) {
          _require.find((item) =>
            !_rew_extensions[item]
              ? (() => {
                throw new ReferenceError(
                  `Rew extension ${name} requires extension ${item}.`,
                );
              })()
              : "",
          );
        }
        _rew_extensions[name] = _class;
        return _class;
      },
      createClass: _createClass,
      has: (name) => !!_rew_extensions[name],
    },
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
