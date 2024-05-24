const ui = require('./ui');

const packages = {};

packages.ui = ui;

module.exports = {
  findPackage(pkg){
    if(pkg in packages) return packages[pkg];
    return null;
  }
}