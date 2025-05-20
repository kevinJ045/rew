rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/ffi.coffee", function(globalThis){
with (globalThis) {
  var ins, init_socket, recv_message, send_message, loopm;
rew.prototype.mod.prototype.find(module, "#std.ffi!")
rew.prototype.mod.prototype.find(module, "#std.encoding")
using(namespace(rew.prototype.ns()))

ins = instantiate(class {
  init_socket = rew.prototype.ffi.prototype.typed( function() { return 'i32' })
  recv_message = rew.prototype.ffi.prototype.typed(ffi.prototype.ptr, 'usize', function() { return 'i32' })
  send_message = rew.prototype.ffi.prototype.typed(ffi.prototype.ptr, ffi.prototype.pre('i32', Number))
});

({ init_socket, recv_message, send_message } = ffi.prototype.open('/home/makano/workspace/testing/rustyscript/test_ffi/target/release/libmy_add_lib.so', ins))

init_socket()

loopm = function() {
  var buf;
  buf = new Uint8Array(40960)
  if (recv_message(rew.prototype.ptr.prototype.of(buf), 40960) > 0) {
    rew.prototype.io.prototype.out.print(rew.prototype.encoding.prototype.bytesToString(buf))
  }
  return setTimeout(loopm, 1)
}

loopm()



setTimeout(() => send_message(rew.prototype.ptr.prototype.of(rew.prototype.encoding.prototype.stringToBytes("Hello from JS"))), 1000)


}
return globalThis.module.exports;
}, ["app://test.app/ffi"]);(function(module){
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
})({filename: "#std.ffi"});(function(module){
"no-compile"
if(!rew.extensions.has('encoding')) rew.extensions.add('encoding', (Deno, module) => rew.extensions.createClass({

  toBase64(data) {
    if (data instanceof Uint8Array) {
      return rew.ops.op_to_base64(Array.from(data));
    }
    return rew.ops.op_to_base64(data);
  },
  
  fromBase64(encoded, options = { asString: false }) {
    const result = rew.ops.op_from_base64(encoded, { as_string: options.asString });
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
}));
})({filename: "#std.encoding"});
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/ffi.coffee');