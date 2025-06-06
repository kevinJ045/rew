"no-compile"
if(!rew.extensions.has('encoding')) rew.extensions.add('encoding', (Deno, module) => rew.extensions.createClass({
  _namespace(){
    return {
      ...this,
      toBytes: (string) => {
        return this.stringToBytes(string);
      },
      strBytes: (bytes) => {
        return this.bytesToString(bytes);
      }
    };
  },
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