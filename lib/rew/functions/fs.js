const fs = require('fs');
const path = require('path');
const { execOptions } = require('../const/opt');


module.exports = (currentFile) => {

  function gp(filepath){
    return path.resolve(filepath.startsWith(execOptions.cwdAlias) ? process.cwd() : path.dirname(currentFile), filepath.replaceAll(execOptions.cwdAlias+'/', ''));
  }

  function read(filepath, options = { encoding: 'utf-8' }){
    return fs.readFileSync(gp(filepath), options);
  }

  function write(filepath, content, options){
    return fs.writeFileSync(gp(filepath), content, options);
  }

  function exists(filepath, options){
    return fs.existsSync(filepath);
  }

  function fstat(filepath, options){
    return fs.statSync(gp(filepath), options);
  }

  function rm(filepath, options){
    return fs.unlinkSync(filepath);
  }

  function chmod(filepath, mode, options){
    return fs.chmodSync(gp(filepath), mode);
  }

  function mkdir(filepath, options){
    return fs.mkdirSync(gp(filepath), options);
  }

  function ls(filepath, options){
    return fs.readdirSync(gp(filepath), options);
  }

  return {
    ls,
    mkdir,
    chmod,
    rm,
    fstat,
    exists,
    write,
    read
  };
} 