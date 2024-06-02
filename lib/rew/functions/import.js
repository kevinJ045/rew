const path = require("path");
const { getFile, file } = require("../modules/fs");
const { importYaml } = require("../modules/yaml");
const { findPackage, getPackage } = require("../pkgs/pkgs");
const { existsSync, readFileSync } = require("fs");
const conf = require("../pkgs/conf");
const jsYaml = require("js-yaml");
const { execOptions } = require("../const/opt");

const cachedFiles = [];

const lookUpInOtherApps = (fullPath) => {
  const con = conf({});
  const name = fullPath.indexOf("/") ? fullPath.split("/")[0] : fullPath;
  let dpath = fullPath.indexOf("/") ? fullPath.split("/")[1] : "";
  let ppath = path.join(con.CONFIG_PATH, name, "app");
  if (!existsSync(ppath)) return null;
  if (!dpath) {
    dpath = jsYaml.load(
      readFileSync(path.join(ppath, "app.yaml"), "utf-8"),
    ).entry;
  }
  ppath = path.join(ppath, dpath);
  if (existsSync(ppath)) return ppath;
  else return null;
};

module.exports.imp = function (runPath, context) {
  return function (filename, options = {}) {
    if (!options) options = {};
    let type = options.type || "coffee";
    let exports,
      ispkg = findPackage(filename);

    if (filename.startsWith("@") && context.app) {
      filename = filename.replace("@", context.app.path);
    }

    let filepath = path.resolve(
      path.dirname(context.module.filepath),
      filename,
    );

    // console.log(typeof runPath);

    const lookUp = () => {
      const otherPath = lookUpInOtherApps(filename);
      if (!otherPath) throw new Error('Module "' + filename + '" not found');
      else filepath = otherPath;
    };

    const foundCache = cachedFiles.find((f) => f.filepath == filepath);

    if (!ispkg && foundCache) {
      exports = foundCache.exports;
    }

    if (!ispkg && !existsSync(filepath)) {
      if (
        Array.isArray(execOptions.resolveExtensions) &&
        execOptions.resolveExtensions.length
      ) {
        const resolve = execOptions.resolveExtensions.find((ext) =>
          typeof ext == "string"
            ? existsSync(filepath + ext)
            : existsSync(filepath + (ext.ext || "")),
        );
        if (resolve) {
          filepath += typeof resolve == "string" ? resolve : resolve.ext;
          if (typeof resolve == "object" && resolve.options) {
            if (resolve.options.type) type = resolve.options.type;
            for (let i in resolve.options) options[i] = resolve.options[i];
          }
        } else lookUp();
      } else lookUp();
    }

    const exec = (coptions = {}) =>
      runPath(
        filepath,
        {
          import: options,
          main: false,
          useContext:
            execOptions.sharedContext == false
              ? false
              : !(options.context && options.context == "new"),
          ...coptions,
          as:
            options.as == "main"
              ? context.module.main
                ? "main"
                : "parent"
              : options.as == "parent"
                ? "parent"
                : "child",
          fromMain: context.module.main,
        },
        execOptions.sharedContext == false
          ? {}
          : options.context && options.context == "new"
            ? {}
            : context,
      ).context.module.exports;

    if (ispkg) {
      const pkg = getPackage(filename)(context);
      exports = pkg._onImport ? pkg._onImport() : pkg;
    } else if (foundCache) {
    } else if (type == "coffee") {
      exports = exec({});
    } else if (type == "js") {
      exports = exec({ compile: false });
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

    if (!ispkg) context.module.imports.push(filepath);
    if (!ispkg) cachedFiles.push({ filepath, exports });

    return exports;
  };
};
