rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/exec.coffee", function(globalThis){
with (globalThis) {
  var f;
rew.prototype.mod.prototype.find(module, "#std.shell");
rew.prototype.mod.prototype.find(module, "#std.encoding");
using(namespace(rew.prototype.ns()));


// channel = rew::channel::new 1, -> 
f = async () => {
  var child;
  child = await rew.prototype.shell.prototype.exec('echo hii')
  return print(rew.prototype.encoding.prototype.bytesToString(await child.output))
}
f()

}
return globalThis.module.exports;
}, ["app://test.app/exec"]);(function(module){
"no-compile"
class ExecPipedError extends Error {
  constructor(message) {
    super(message);
    this.name = "ExecPipedError";
  }
}
if(!rew.extensions.has('shell')) rew.extensions.add('shell', (Deno) => rew.extensions.createClass({
  _namespace(){
    return 'shell';
  },
  kill(pid, signal = "SIGTERM") {
    Deno.kill(pid, signal);
  },
  spawn(command, options = {}) {
    return Deno.run({
      cmd: Array.isArray(command) ? command : command.split(" "),
      ...options,
    });
  },
  async wait(process) {
    const status = await process.status();
    process.close();
    return status;
  },
  fexec(command, options = {}) {
    const process = this.spawn(command, { ...options, stdout: "piped", stderr: "piped" });
    return this.wait(process).then((status) => {
      return {
        status,
        output: process.output(),
        error: process.stderrOutput(),
      };
    });
  },
  exec(command, options = {}) {
    const process = this.spawn(command, { ...options, stdout: "piped", stderr: "piped" });
    return this.wait(process).then((status) => {
      if (status.success) {
        return process.output();
      }
      return process.stderrOutput().then((error) => {
        throw new ExecPipedError(`Command failed ${status.code}: ${error}`);
      });
    });
  }
}));
})({filename: "#std.shell"});(function(module){
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
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/exec.coffee');