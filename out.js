rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/ffi.autoload.coffee", {
"/home/makano/workspace/rew-rust/test/ffi.autoload.coffee"(globalThis){
with (globalThis) {
  rew.prototype.mod.prototype.find(module, "#std.ffi!")
using(namespace(rew.prototype.ns))

let symbols = ffi.prototype.autoload('/home/makano/.rew/apps/rew_bindgen_test/target/release/librew_bindgen_test.so')

symbols.say_hello()



print(symbols.add(100, 10))

let cb = function() { return print("hi") }

symbols.call_every_second(
  rew.prototype.ptr.prototype.fn([], 'void', cb).pointer
)

let s = function() { return setTimeout(s, 1) }
s()
}
return globalThis.module.exports;
}          
}, ["app://test.app/ffi.autoload"]);(function(module){
"no-compile"
//declare* "=ffi_type" = rew::ffi::typed;
if(!rew.extensions.has('ffi')) rew.extensions.add('ffi', (Deno) => rew.extensions.createClass({
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
  buffer: "buffer",
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
  autoload(libPath){
    const { symbols: meta } = Deno.dlopen(libPath, {
      __rew_symbols: { parameters: [], result: "pointer" },
    });
    
    const view = new Deno.UnsafePointerView(meta.__rew_symbols());
    const json = view.getCString();
    const def = JSON.parse(json);
    
    const ffiDef = this._translateFFIData(def);
    // rew.prototype.io.prototype.out.print(ffiDef);
    
    const lib = Deno.dlopen(libPath, ffiDef);

    return this._buildFFI(def, lib);
  },
  _translateFFIData(meta) {
    const result = {};
  
    for (const [symbolName, symbol] of Object.entries(meta)) {
      if (symbol.kind !== "Function") continue;
  
      const sig = symbol.signature;
      const parts = sig.match(/fn\s+\w+\((.*?)\)(?:\s*->\s*(\S+))?/);
  
      const paramList = parts?.[1]?.split(",").filter(Boolean) ?? [];
      const returnType = parts?.[2]?.trim() ?? "void";
  
      const parameters = paramList.map(param => {
        const typeStr = param.split(/\s*:\s*/)[1]?.trim();
        return this._mapTypeRust(typeStr || "pointer");
      });

      result[symbol.name] = {
        parameters,
        result: this._mapTypeRust(returnType),
      };
    }
  
    return result;
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
  },
  _buildFFI(meta, lib) {
    const result = {};
    const structs = {};

    for (const [symbolName, symbol] of Object.entries(meta)) {
      if (symbol.kind === "Function") {
        const { name, signature } = symbol;

        const isMethod = name.includes("::");
        const parts = signature.match(/fn\s+\w+\((.*?)\)(?:\s*->\s*(\S+))?/);
        const paramList = parts?.[1]?.split(",").filter(Boolean) ?? [];
        const returnType = parts?.[2]?.trim() ?? null;

        const params = paramList.map(param => {
          const [_name, typeStr] = param.trim().split(/\s*:\s*/);
          return this._mapTypeRust(typeStr);
        });

        const fn = lib.symbols[name];
        if (!fn) {
          continue;
        }

        const jsWrapper = (...args) => fn(...args);

        if (isMethod) {
          const [structName, methodName] = name.split("::");
          if (!structs[structName]) structs[structName] = {};
          structs[structName][methodName] = jsWrapper;
        } else {
          result[name] = jsWrapper;
        }
      }

      if (symbol.kind === "Struct") {
        const { name, fields } = symbol;
        if (!structs[name]) structs[name] = {};
        structs[name]._fields = fields;
      }
    }

    for (const [structName, methods] of Object.entries(structs)) {
      result[structName] = class {
        constructor(ptr) {
          this.ptr = ptr;
        }

        static _fields = methods._fields ?? [];

        static from(ptr) {
          return new result[structName](ptr);
        }

        static registerMethods() {
          for (const [key, fn] of Object.entries(methods)) {
            if (key === "_fields") continue;
            this.prototype[key] = function (...args) {
              return fn(this.ptr, ...args);
            };
          }
        }
      };

      result[structName].registerMethods();
    }

    return result;
  },
  _mapTypeRust(type) {
    if(!type) return "pointer";
    const base = type.replace(/\.ty$/, "").trim();
  
    switch (base) {
      case "i32": return "i32";
      case "i64": return "i64";
      case "f32": return "f32";
      case "f64": return "f64";
      case "bool": return "u8";
      case "void": return "void";
      case "Callback": return "function";
      case "* const std :: os :: raw :: c_char":
      case "* const c_char":
      case "* mut c_char":
      case "char_ptr":
        return "pointer";
      default:
        if (base.startsWith("*")) return "pointer";
        return "pointer";
    }
  }
}));
})({filename: "#std.ffi"});
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/ffi.autoload.coffee');