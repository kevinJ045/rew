const path = require('path');
const fs = require('fs');
const conf = require('../pkgs/conf');
const jsYaml = require('js-yaml');

module.exports = {
  conf(command, fullPath, key, value){
    const con = conf({});
    if(command == 'get'){
      if(!fullPath || fullPath == 'list'){
        return fs.readdirSync(con.CONFIG_PATH).join('\n');
      } else {
        const name = fullPath.indexOf('/') ? fullPath.split('/')[0] : fullPath;
        const dpath = fullPath.indexOf('/') ? fullPath.split('/')[1] : '';
        const root = con.create(name);
        if(dpath){
          const fp = path.join(root.root, dpath);
          if(fs.existsSync(fp) && fs.statSync(fp).isDirectory()){
            return fs.readdirSync(fp).join('\n');
          } else {
            const o = con.create(name).optionCenter(dpath);
            return key ? o.get(key) : o.getAll(true);
          }
        } else {
          return fs.readdirSync(root.root).join('\n');
        }
      }
    } else {
      const name = fullPath.indexOf('/') ? fullPath.split('/')[0] : fullPath;
      const dpath = fullPath.indexOf('/') ? fullPath.split('/')[1] : '';
      if(name && key){
        const root = con.create(name);
        const o = dpath ? root.optionCenter(dpath) : root;
        if(command == 'set') {
          if(value){
            o.set(key, value);
          } else {
            console.error('Value not specified');
          }
        } else {
          o.remove(key);
        }
      } else {
        console.error(
          !name ? 'Path not specified' : 'Key not specified'
        );
      }
    }
  }
}