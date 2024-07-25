const { findAppInfo } = require("../misc/findAppInfo");
const { log } = require("./log");
const colors = require('colors');
const { input } = require('../functions/stdout');
const path = require('path');
const fs = require('fs');
const { CONFIG_PATH } = require("../const/config_path");
const rune = require("../pkgs/rune");

module.exports = {
	types(projectPath){
    if(!projectPath) projectPath = process.cwd();
    else projectPath = path.resolve(process.cwd(), projectPath);
    const projectinfo = findAppInfo(projectPath+'/app.yaml');
    if(!projectinfo){
      log('Path not a rew app'.red.bold, ':end')
      return;
    }
    let typesToLoad = ['rew'];
    if(projectinfo.config?.types){
      typesToLoad = projectinfo.config?.types;
    }
    fs.mkdirSync(path.join(projectPath, 'node_modules/@types/rew'), { recursive: true });


    typesToLoad.forEach(name => {
      let filename = name+'.d.ts';
      if(name == 'rew'){
				fs.copyFileSync(path.join(__dirname, '../../../runtime.d.ts'), path.join(projectPath, 'node_modules/@types/rew/index.d.ts'));
        return;
      }
      let p = path.resolve(CONFIG_PATH, name, 'app', 'types.d.ts');
      
      if(name.indexOf('/') > -1) {
        const fn = name.split('/').slice(1).join('/')+'.d.ts';
        p = path.resolve(CONFIG_PATH, name.split('/')[0], 'app', fn);
        filename = name.split('/')[0]+'-'+path.basename(fn);
      }
      if(fs.existsSync(p)) fs.copyFileSync(p, path.join(projectPath, 'node_modules/@types/rew/'+filename));
    });
	},
  'keygen': () => console.log('Encryption Key:', rune({}).genKey(input('Secret Value: ') || null))
}