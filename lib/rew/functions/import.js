const path = require("path");
const { getFile, file } = require("../modules/fs");
const { importYaml } = require("../modules/yaml");
const { findPackage, getPackage } = require("../pkgs/pkgs");

module.exports.imp = function (runPath, context) {
  return function (filename, options = {}) {
    let type = options.type || "coffee";
    let exports,
      ispkg = findPackage(filename);

    const filepath = path.resolve(path.dirname(context.module.filepath), filename);

    // console.log(typeof runPath);

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
