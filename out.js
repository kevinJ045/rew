rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/fs.coffee", {
"/home/makano/workspace/rew-rust/test/fs.coffee"(globalThis){
with (globalThis) {
  rew.prototype.mod.prototype.find(module, "#std.fs")

let something = null

using(namespace(rew.prototype.ns))
// print rew::fs::read "./exec.coffee!"

print(rew.prototype.fs.prototype.sha('./ffi.coffee'))
print(rew.prototype.fs.prototype.read("./ffi.coffee"))

}
return globalThis.module.exports;
}          
}, ["app://test.app/fs"]);(function(module){
"no-compile"

function trackPromise(promise){
  return Promise.resolve(promise);
}

if(!rew.extensions.has('fs')) rew.extensions.add('fs', (Deno, module) => rew.extensions.createClass({
  _namespace(){
    return this;
  },

  ...Deno.fs,

  read(path, options = { binary: false }) {
    const result = rew.ops.op_fs_read(module.filename, path, options);
    if (options.binary) {
      return new Uint8Array(result);
    }
    return result;
  },
  
  async write(path, content, options = { binary: false, create_dirs: false }) {
    if (options.binary && content instanceof Uint8Array) {
      content = Array.from(content);
    }
    return await rew.ops.op_fs_write(module.filename, path, content, options);
  },
  
  async readBinary(path) {
    return await this.read(path, { binary: true });
  },
  
  async writeBinary(path, data) {
    return await this.write(path, data, { binary: true, create_dirs: true });
  },
  
  stringToBytes(str) {
    const encoder = new TextEncoder();
    return encoder.encode(str);
  },
  
  bytesToString(bytes) {
    const decoder = new TextDecoder();
    return decoder.decode(bytes);
  },
 
  sha(path) {
    return rew.ops.op_fs_sha(module.filename, path);
  },

  exists(path) {
    return rew.ops.op_fs_exists(module.filename, path);
  },
  
  async rm(path, recursive = false) {
    return trackPromise(rew.ops.op_fs_rm(module.filename, path, {recursive}));
  },
  
  stats(path) {
    const statsJson = rew.ops.op_fs_stats(module.filename, path);
    return JSON.parse(statsJson);
  },
  
  async mkdir(path, recursive = false) {
    return trackPromise(rew.ops.op_fs_mkdir(module.filename, path, {recursive}));
  },
  
  readdir(path, options = { include_hidden: true, filter_type: null, sort_by: null }) {
    const entriesJson = rew.ops.op_fs_readdir(module.filename, path, options);
    return JSON.parse(entriesJson);
  },
  
  async copy(src, dest, options = {
    recursive: true,
    create_dirs: true,
    overwrite: false,
  }) {
    return trackPromise(rew.ops.op_fs_copy(module.filename, src, dest, options));
  },
  
  async rename(src, dest) {
    return trackPromise(rew.ops.op_fs_rename(module.filename, src, dest));
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
}));

})({filename: "#std.fs"});
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/fs.coffee');