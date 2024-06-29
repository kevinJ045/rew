const { CONFIG_PATH } = require("../const/config_path");
const path = require('path');
const fs = require('fs');
const conf = require("../pkgs/conf");
const { log } = require("./log");

const binpath = path.join(conf({}).create('').root, '.bin');
const logspath = path.join(conf({}).create('').root, '.logs');
const cachepath = path.join(conf({}).create('').root, '.cache');
const localBinPath = path.join(binpath, '../../../', 'bin');

if (!fs.existsSync(CONFIG_PATH) || !fs.existsSync(CONFIG_PATH + '/repos.yaml')) {
	fs.mkdirSync(CONFIG_PATH, { recursive: true });
	log('First time init');
  conf({}).create('').optionCenter('repos').set('rewpkgs', '//raw.githubusercontent.com/kevinJ045/rewpkgs/main/main.yaml');
  fs.mkdirSync(binpath, { recursive: true });
  fs.mkdirSync(cachepath, { recursive: true });
  fs.mkdirSync(logspath, { recursive: true });
}

const npm_package_name = '@makano/rew';

function getAllPipeInput(){
  return new Promise((resolve) => {
    let data = '';
    process.stdin.setEncoding('utf8');
    
    process.stdin.on('data', (chunk) => {
      data += chunk;
    });
    
    process.stdin.on('end', () => {
      resolve(data);
    });
  });
}

module.exports = {
  binpath,
  logspath,
  cachepath,
  localBinPath,
  npm_package_name,
  getAllPipeInput
}