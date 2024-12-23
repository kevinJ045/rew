const { CONFIG_PATH } = require("../const/config_path");
const path = require('path');
const fs = require('fs');
const conf = require("../pkgs/conf");
const { log } = require("./log");

const binpath = path.join(conf({}).create('').root, '.bin');
const logspath = path.join(conf({}).create('').root, '.logs');
const cachepath = path.join(conf({}).create('').root, '.cache');
const localBinPath = path.join(binpath, '../../../', 'bin');

const FILE_DL_EXTRACT_REGEX = /^file\+sha\(([^)]+)\)\+([a-zA-Z0-9_-]+)(\([^\)]*\))?:(.*)$|^file\+([a-zA-Z0-9_-]+)(\([^\)]*\))?:(.*)$/;
const HTTP_REGEX = /^https?:\/\/(www\.)?[a-zA-Z0-9\-._~:\/?#@!$&'()*+,;=%]+$/

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

/**
 * 
 * @param {string} string 
 * @param {Record<string,(replacedString: string, originalString: string) => any>} order 
 * @returns { { string: string, [key: string]: any } }
 */
function hashTags(string, order){
  const hashes = Object.keys(order);
  const h = {};
  let s = string;
  for(let i of hashes){
    if(string.includes(`#${i}`)){
      const str = s.replace(`#${i}`, '');
      h[i] = order[i](str, string);
      if(h[i]?.$set) {s = h[i].$set; string = s }
      else s = str;
    }
  }
  return {
    string: s,
    ...h
  };
}

module.exports = {
  binpath,
  logspath,
  cachepath,
  localBinPath,
  npm_package_name,
  getAllPipeInput,
  hashTags,
  FILE_DL_EXTRACT_REGEX,
  HTTP_REGEX
}