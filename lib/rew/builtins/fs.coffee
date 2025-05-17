
unless then rew.extensions.add 'fs', rew.extensions.createClass({
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
})
