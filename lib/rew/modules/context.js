const defaultContext = require("../const/default");
const { execOptions } = require("../const/opt");
const { exportsFunction, pubFunction } = require("../functions/export");
const { imp } = require("../functions/import");
const { customRequire } = require("../functions/require");
const fsLib = require("../functions/fs");
const pathLib = require("../functions/path");
const path = require("path");
const execLib = require("../functions/exec");
const { findAppInfo } = require("../misc/findAppInfo");
const { USING_DEFAULT, Usage, Namespace } = require("../const/usage");
const runtime = require("./runtime");
const { permission } = require("process");

let mainFile = "";
const isMainFile = (filepath) => filepath == mainFile;
module.exports.prepareContext = function (
  custom_context,
  options,
  filepath = "",
  runPath = () => {},
) {
  if (mainFile == "") mainFile = filepath;
  let context = {
    module: {
      exports: null,
      filepath,
      main: isMainFile(filepath),
      imports: [],
    },
    imports: {
      meta: { url: new URL('file://'+filepath), main: isMainFile(filepath) },
      assert: options.import ?? {},
    },
    app: findAppInfo(filepath),
    ...fsLib(filepath),
    __using__: {}
  };
  if (options.useContext) {
    context = {
      ...custom_context,
      ...context,
    };
  } else {
    context = {
      ...context,
      ...defaultContext,
      ...pathLib(filepath),
      ...execLib(filepath),
      ...custom_context,
      Usage
    };
  }
  if (!context.process)
    context.process = {
      argv: options.argv || process.argv,
      target: {
        on: (event, listener) => process.on(event, listener),
        off: (event, listener) => process.off(event, listener),
        emit: (event, code) => process.emit(event, code),
      },
      __execFile: global.fileName,
      env: process.env,
      cwd: () => process.cwd(),
      abort: () => process.abort(),
      kill: () => process.kill(),
      exit: (code) => process.exit(code),
      arch: process.arch,
      pid: process.pid,
      platform: process.platform,
      permission: process.permission
    };

  context.global = context;
  context.imports.assert = options.import ?? {};
  context.imp = imp(runPath, context);
  context.import = context.imp;
  context.require = (package, esm) => {
    try {
      const search = execOptions.nativeRequire || package.startsWith("node:")
        ? require(
            package.startsWith("node:")
              ? package.split("node:")[1]
              : package,
          )
        : customRequire(package, filepath, esm);
        if(!search) throw new Error("Module " + package + " not found");
        return search;
    } catch (e) {
      throw e;
    }
  };
  context.inc = (package, asserts) => {
    try {
      if (package.startsWith("node:") || package.startsWith("pkg:"))
        throw new Error("");
      return context.imp(package, asserts);
    } catch (e) {
      let pname = package.startsWith("pkg:") ? package.split("pkg:")[1] : package;
      if(pname.endsWith('#esm')){
        pname = pname.slice(0, -4);
        if(!asserts) asserts = { esm: true };
        else asserts.esm = true;
      }
      return context.require(
        pname,
        asserts?.esm
      );
    }
  };
  context.pub = pubFunction(context);
  context.exports = exportsFunction(context);

  context.using = (name, ...params) => {
    if(name instanceof Usage.Group){
      params.unshift(...name.g.slice(1));
      name = name.g[0];
    }
    if(USING_DEFAULT[name]){
      if(USING_DEFAULT[name].param) {
        context.__using__[name] = USING_DEFAULT[name].param(...params);
      }
    } else if(name instanceof Namespace) {
      const trigger = name.trigger;
      const childContext = {...context, ...name.namespace, trigger};
      childContext.currentNamespace = name.namespace;
      childContext.parentNamespace = context;
      const code = `(${trigger.toString()})()`;
      if(name.onUse) name.onUse();
      runtime.exec(code, childContext, code, context.module.filepath);
    } else if(name instanceof Usage) {
      const v = name.trigger(...params) || true;
      if(name.save !== false) context.__using__[name.name] = v;
    } else {
      context.__using__[name] = params.length ? params.length > 1 ? [...params] : params : true;
    }
  };

  if(context.app?.config?.exec?.['auto import']){
    const autoipath = path.join(context.app.path, context.app.config?.exec?.['auto import']);
    if(autoipath !== filepath){
      const all = context.imp(path.relative(path.dirname(filepath), autoipath));
      for(let i in all) context[i] = all[i];
    }
  }

  if (
    context.module.main ||
    (options.fromMain == true && options.as == "main")
  ) {
    context.opt = {
      set: (key, value) => (execOptions[key] = value),
      get: (key) => execOptions[key],
      push: (key, value) => execOptions[key]?.push(value),
      pop: (key) => execOptions[key]?.pop(),
    };
  } else delete context.opt;
  return context;
};
