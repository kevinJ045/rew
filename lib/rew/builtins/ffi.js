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
}));