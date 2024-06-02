const jsYaml = require("js-yaml");
const { findAppPath } = require("./findAppPath");
const path = require('path');
const { readFileSync } = require("fs");

module.exports.findAppInfo = function(filepath){
  const appPath = findAppPath(path.dirname(filepath));
  if(appPath){
    const config = jsYaml.load(readFileSync(path.join(appPath, 'app.yaml')));
    return {
      path: appPath,
      config
    }
  }
  return null;
}