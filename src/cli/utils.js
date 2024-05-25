const path = require('path');
const fs = require('fs');
const conf = require('../pkgs/conf');
const jsYaml = require('js-yaml');
const readline = require('readline');
const { log, logget } = require('./log');
const { execSync } = require('child_process');

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
            log('Value not specified', ':end');
          }
        } else {
          o.remove(key);
        }
      } else {
        log(
          !name ? 'Path not specified' : 'Key not specified', ':end'
        );
      }
    }
  },
  createProject: (ppath) => {
    const projectPath = path.join(process.cwd(), ppath);
    log('Crating at', ppath);
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });    
    const project = {};
    const create = () => {
      fs.mkdirSync(projectPath, { recursive: true });
      const confPath = path.join(projectPath, 'app.yaml');
      const entryFile = path.join(projectPath, 'main.coffee');
      fs.writeFileSync(confPath, jsYaml.dump({ package: project.package, entry: 'app.yaml' }));
      fs.writeFileSync(entryFile, `print("Hello World!")`);
      if(project.git) {
        execSync('cd '+projectPath+' && git init .');
      }
      if(project.node) {
        execSync('cd '+projectPath+' && npm init -y');
      }
      r1.close();
    }
    if (!fs.existsSync(projectPath)) {
      rl.question(logget('Package Name: '), (pkg) => {
        if(pkg.trim()) {
          project.package = pkg.trim();
          rl.question(logget('Use git(y/N): '), (use_git) => {
            project.git = use_git.toLowerCase() == 'y' || use_git.toLowerCase() == 'yes';
            rl.question(logget('Use Node(y/N): '), (use_node) => {
              project.node = use_node.toLowerCase() == 'y' || use_node.toLowerCase() == 'yes';
              create();
            });
          });
        } else {
          r1.close();
        }
      });
    } else {
      log(`Project ${ppath} already exists at ${projectPath}`, ':end');
      rl.close();
    }
  }
}