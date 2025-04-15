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
const emitter = require("../functions/emitter");

let mainFile = "";
const isMainFile = (filepath) => filepath == mainFile;
const globalContext = {};
const globalEmitter = emitter();
module.exports.prepareContext = function (
  custom_context,
  options,
  filepath = "",
  runPath = () => {},
) {
  straceLog('!-NEW context()');
  if (mainFile == "") mainFile = filepath;
  /** @type {Record<string, any>} */
  let context = {
    TextDecoder,
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
    __using__: {},
    get _(){
      return void 0;
    },
    ...globalContext
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
          this.attach = std.prototype.attach;
          this.named = std.prototype.named;
          this.detach = std.prototype.detach;
          this.out = std.prototype.out;
          this.inp = std.prototype.in;
          this.signal = std.prototype.signal;
          this['@cb'] = cb;
        }
      });
    }, out: {...process.stdout, cols: process.stdout.columns, rows: process.stdout.rows, put: (...logs) => context.print(...logs), strace: (...logs) => straceLog('==> !+OUT straceOut(): ', ...logs  ), write: (logs) => context.printf(logs+'\n') }, in: {...process.stdin, read: (...args) => context.input(...args)}, define: (name, object) => {
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
    }, named: (name) => {
      return (item) => [name, item];
    }, attach: (name, object) => {
      if(Array.isArray(name) && name.length == 2 && typeof name[0] == 'string'){
        object = name[1];
        name = name[0];
      }
      if(typeof name == "object" && typeof object == "string"){
        const tname = object;
        object = name;
        name = tname;
      }
      if(typeof name == "object" && !object){
        object = name;
        name = object.name;
      }
      if(typeof name == "function"){
        object = name;
        name = object.name;
      }
      if(!name) return false;
      globalContext[name] = object;
      globalEmitter.emit('attach_name', name, object);
      return true;
    }, detach: (nameOrObject) => {
      const name = Object.keys(globalContext).find((key) => key == nameOrObject || globalContext[key] == nameOrObject);
      if(name) {
        delete globalContext[name];
        globalEmitter.emit('detach_name', name);
        return true;
      }
      return false;
    }, signal: globalEmitter }
    Object.defineProperty(std.prototype, 'void', {
      get: () => void 0
    })
    context.std = std;
  }
  globalEmitter.on('attach_name', (name, object) => {
    context[name] = object;
  });
  globalEmitter.on('detach_name', (name) => delete context[name]);
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

  context.std.prototype.__ = {
    get prototype(){
      const globals = context.app?.config?.assets?.globals ?? '_';
      const p = context.pjoin(context.app ? context.app.path : context.dirname(context.module.filename), globals);
      if(!context.exists(p)) return {};
      const files = context.ls(p);
      const pr = {};
      files.forEach(file => {
        if(!file.endsWith('.coffee')) return;
        const e = runPath(context.pjoin(p, file));
        let name = e.context.module.modset || file.split('.').shift();
        pr[name] = e.context.module.exports || {};
        if(typeof pr[name] == "object" && pr[name].default && Object.keys(pr[name]).length == 1){
          pr[name] = pr[name].default;
        }
      });
      return pr;
    }
  }

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
      return context.imp(package, asserts, true);
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
      straceLog('==> !ACTION autoImport()', autoipath);
      const all = context.imp(path.relative(path.dirname(filepath), autoipath));
      for(let i in all) context[i] = all[i];
    }
  }

  if(!context.app){
    straceLog('==> WARN: App not found');
    context.appPackage = (packageName) => context.app = { config: { manifest: { package: packageName } } }
  } else {
    context.appPackage = context.mod = (packageName) => context.module.modset = packageName;
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
    straceLog('==> !MAIN', filepath);
    context.opt = {
      set: (key, value) => (execOptions[key] = value),
      get: (key) => execOptions[key],
      push: (key, value) => execOptions[key]?.push(value),
      pop: (key) => execOptions[key]?.pop(),
    };
  } else delete context.opt;
  return context;
};
