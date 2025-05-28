"no-compile"
if(!rew.extensions.has('fs')) rew.extensions.add('fs', (Deno, module) => rew.extensions.createClass({
  _namespace(){
    return this;
  },

  ...Deno.fs,

  async read(path, options = { binary: false }) {
    const result = await rew.ops.op_fs_read(module.filename, path, options);
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
  
  exists(path) {
    return rew.ops.op_fs_exists(module.filename, path);
  },
  
  async rm(path, options = {}) {
    return trackPromise(rew.ops.op_fs_rm(module.filename, path, options));
  },
  
  stats(path) {
    const statsJson = rew.ops.op_fs_stats(module.filename, path);
    return JSON.parse(statsJson);
  },
  
  async mkdir(path, options = {}) {
    return trackPromise(rew.ops.op_fs_mkdir(module.filename, path, options));
  },
  
  readdir(path, options = {}) {
    const entriesJson = rew.ops.op_fs_readdir(module.filename, path, options);
    return JSON.parse(entriesJson);
  },
  
  async copy(src, dest, options = {}) {
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