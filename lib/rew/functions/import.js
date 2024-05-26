const path = require("path");
const { getFile, file } = require("../modules/fs");
const { importYaml } = require("../modules/yaml");
const { findPackage, getPackage } = require("../pkgs/pkgs");
const { existsSync, readFileSync } = require("fs");
const conf = require("../pkgs/conf");
const jsYaml = require("js-yaml");

const lookUpInOtherApps = (fullPath) => {
  const con = conf({});
  const name = fullPath.indexOf('/') ? fullPath.split('/')[0] : fullPath;
  let dpath = fullPath.indexOf('/') ? fullPath.split('/')[1] : '';
  let ppath = path.join(con.CONFIG_PATH, name, 'app');
  if(!existsSync(ppath)) return null;
  if(!dpath){
    dpath = jsYaml.load(readFileSync(path.join(ppath, 'app.yaml'), 'utf-8')).entry;
  }
  ppath = path.join(ppath, dpath);
  if(existsSync(ppath)) return ppath;
  else return null;
}

module.exports.imp = function (runPath, context) {
  return function (filename, options = {}) {
    let type = options.type || "coffee";
    let exports,
      ispkg = findPackage(filename);

    let filepath = path.resolve(path.dirname(context.module.filepath), filename);

    // console.log(typeof runPath);

    if(!ispkg && !existsSync(filepath)){
      const otherPath = lookUpInOtherApps(filename);
      if(!otherPath) throw new Error('Module "'+filename+'" not found');
      else filepath = otherPath;
    }

    if (ispkg) {
      exports = getPackage(filename)(context);
    } else if (type == "coffee") {
      exports = runPath(
        filepath,
        { ...options, useContext: true },
        context,
      ).context.module.exports;
    } else if (type == "js") {
      exports = runPath(
        filepath,
        { ...options, useContext: true, compile: false },
        context,
      ).context.module.exports;
    } else if (type == "yaml" || type == "json" || type == "text") {
      const f = getFile(filepath);
      if (type == "yaml") {
        exports = importYaml(f.path, f);
      } else if (type == "json") {
        try {
          exports = JSON.parse(f.content);
        } catch (e) {
          exports = {};
        }
      } else {
        exports = f.content;
      }
    }

    if (options.save && (type == "js" || type == "coffee")) {
      if (typeof options.save == "string") context[options.save] = exports[i];
      else
        for (let i in exports) {
          context[i] = exports[i];
        }
    }

    if(!ispkg) context.module.imports.push(filepath);

    return exports;
  };
};
