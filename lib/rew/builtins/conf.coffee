
unless rew.extensions.has('conf') then rew.extensions.add('conf', rew.extensions.createClass({
  _namespace(){
    return "conf";
  },

  read(key) {
    return rew.ops.op_data_read(this.current_app.config.manifest.package, key);
  },
  
  async write(key, content) {
    if (typeof content !== 'string') {
      content = JSON.stringify(content);
    }
    return await rew.ops.op_data_write(this.current_app.config.manifest.package, key, content);
  },
  
  async delete(key) {
    return await rew.ops.op_data_delete(this.current_app.config.manifest.package, key);
  },
  
  exists(key) {
    return rew.ops.op_data_exists(this.current_app.config.manifest.package, key);
  },
  
  list(prefix = '') {
    const result = rew.ops.op_data_list(this.current_app.config.manifest.package, prefix);
    return JSON.parse(result);
  },
  
  readJSON(key) {
    const content = this.read(key);
    return JSON.parse(content);
  },
  
  async writeJSON(key, data) {
    return await this.write(key, JSON.stringify(data, null, 2));
  },

  readYAML(key) {
    return rew.ops.op_data_read_yaml(this.current_app.config.manifest.package, key);
  },
  
  async writeYAML(key, data) {
    return await rew.ops.op_data_write_yaml(this.current_app.config.manifest.package, key, data);
  },
  
  readBinary(key) {
    const data = rew.ops.op_data_read_binary(this.current_app.config.manifest.package, key);
    return new Uint8Array(data);
  },
  
  async writeBinary(key, data) {
    const arrayData = if data instanceof Uint8Array then Array.from(data) else data;
    return await rew.ops.op_data_write_binary(this.current_app.config.manifest.package, key, arrayData);
  },
  
  readAuto(key) {
    const [exists, format] = rew.ops.op_data_get_info(this.current_app.config.manifest.package, key);
    
    if (!exists) {
      throw new Error("File not found: #{key}");
    }
    
    switch (format) {
      case 'json':
        return this.readJSON(key);
      case 'yaml':
        return this.readYAML(key);
      case 'binary':
        return this.readBinary(key);
      case 'text':
      default:
        return this.read(key);
    }
  },
  
  async writeAuto(key, data) {
    // Determine format based on file extension
    const ext = key.split('.').pop().toLowerCase();
    
    if (data instanceof Uint8Array || Array.isArray(data) && data.every(item => typeof item === 'number' && item >= 0 && item <= 255)) {
      return await this.writeBinary(key, data);
    } else if (typeof data === 'object') {
      if (ext === 'yaml' || ext === 'yml') {
        return await this.writeYAML(key, data);
      } else {
        return await this.writeJSON(key, data);
      }
    } else {
      return await this.write(key, String(data));
    }
  },
  
  getInfo(key) {
    const [exists, format] = rew.ops.op_data_get_info(this.current_app.config.manifest.package, key);
    return { exists, format };
  },

  current_app: null,
}), ({ module, rew }) => {
  rew.prototype.conf.prototype.current_app = module.app || { config: {}, path: "" };
})
