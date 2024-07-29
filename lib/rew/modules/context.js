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
const { Usage, usingFunction } = require("../const/usage");
const runtime = require("./runtime");
const { straceLog } = require("../misc/strace");
const reval = require("../functions/reval");
const STDNS = require("../const/stdns");

let mainFile = "";
const isMainFile = (filepath) => filepath == mainFile;
module.exports.prepareContext = function (
  custom_context,
  options,
  filepath = "",
  runPath = () => {},
) {
  straceLog('PREPARE() NEW CONTEXT');
  if (mainFile == "") mainFile = filepath;
  /** @type {Record<string, any>} */
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
    ...reval(filepath),
    __using__: {}
  };
  if (options.useContext) {
    context = {
      ...custom_context,
      ...context,
    };
  } else {
    const std = {...defaultContext};
    context = {
      ...context,
      ...std,
      ...pathLib(filepath),
      ...execLib(filepath),
      ...custom_context,
      Usage
    };
    std.prototype = { ns: (cb) => {
      return new (class extends STDNS {
        constructor(){
          super();
          for(let i in std){
            this[i] = std[i];
          }
          this.define = std.prototype.define;
          this.Main = std.prototype.Main;
          this['@cb'] = cb;
        }
      });
    }, define: (name, object) => {
      if(Array.isArray(name) && name.length == 2 && typeof name[0] == 'string'){
        object = name[1];
        name = name[0];
      }
      if(!context.module.exports) context.module.exports = {};
      context.module.exports[name] = object;
      context[name] = object;
    }, Main: (cb) => {
      if(cb?.main){
        cb.main._class = cb;
        if(cb.prepare){
          cb.prepare((object) => {
            for(let i in object){
              cb[i] = object[i];
            }
          });
        }
      }
      return (['main', cb?.main ?? cb]);
    }, attach: (object) => {
      for(let i in object){
        if(!context[i]) context[i] = object[i];
      }
    } }
    Object.defineProperty(std.prototype, 'void', {
      get: () => void 0
    })
    context.std = std;
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
      chdir: (dir) => process.chdir(dir),
      disconnect: () => process.disconnect(),
      arch: process.arch,
      pid: process.pid,
      platform: process.platform,
      channel: process.channel,
      uptime: () => process.uptime(),
      nextTick: (callback, ...args) => process.nextTick(callback, ...args),
      permission: process.permission,
      transmit: {
        send: (...data) => process.send(...data),
        recieve: (cb) => process.on('message', cb)
      }
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
      if(e.message.match('Module') && e.message.match('not found')){
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
      } else {
        throw e;
      }
    }
  };
  context.pub = pubFunction(context);
  context.exports = exportsFunction(context);

  context.using = usingFunction(context, runtime);

  if(context.app?.config?.exec?.['auto import']){
    const autoipath = path.join(context.app.path, context.app.config?.exec?.['auto import']);
    if(autoipath !== filepath){
      straceLog('==> AUTOIMPORT()', autoipath);
      const all = context.imp(path.relative(path.dirname(filepath), autoipath));
      for(let i in all) context[i] = all[i];
    }
  }

  if(!context.app){
    straceLog('==> APP NOT FOUND');
    context.appPackage = (packageName) => context.app = { config: { manifest: { package: packageName } } }
  }

  Object.defineProperty(context, 'packageName', {
    get: () => context.app?.config?.manifest?.package,
    enumerable: true,
    configurable: true
  });

  if (
    context.module.main ||
    (options.fromMain == true && options.as == "main")
  ) {
    straceLog('==> RUNTIME() as MAIN for', filepath);
    context.opt = {
      set: (key, value) => (execOptions[key] = value),
      get: (key) => execOptions[key],
      push: (key, value) => execOptions[key]?.push(value),
      pop: (key) => execOptions[key]?.pop(),
    };
  } else delete context.opt;
  return context;
};
