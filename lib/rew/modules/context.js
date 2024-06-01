const defaultContext = require("../const/default");
const { execOptions } = require("../const/opt");
const emitter = require("../functions/emitter");
const { exportsFunction, pubFunction } = require("../functions/export");
const { imp } = require("../functions/import");
const { customRequire } = require("../functions/require");
const fsLib = require('../functions/fs');
const pathLib = require('../functions/path');
const execLib = require('../functions/exec');

module.exports.prepareContext = function (
  custom_context,
  options,
  filepath = "",
  runPath = () => {},
) {
  let context = {
    module: {
      exports: null,
      filepath,
      main: options.main ?? true,
      imports: []
    },
    imports: {
      meta: {},
      assert: options.import ?? {}
    },
    ...fsLib(filepath),
  };
  if (options.useContext) {
    context = {
      ...context,
      ...custom_context,
    };
  } else {
    context = {
      ...context,
      ...defaultContext,
      ...pathLib(filepath),
      ...execLib(filepath),
      require: (package) => {
        try {
          return execOptions.nativeRequire || package.startsWith('node:') ? require(package.startsWith('node:') ? package.split('node:')[1] : package) : customRequire(package, filepath);
        } catch (e) {
          throw new Error("Module "+package+" not found");
        }
      },
      opt: {
        set: (key, value) => execOptions[key] = value,
        get: (key) => execOptions[key],
        push: (key, value) => execOptions[key]?.push(value),
        pop: (key) => execOptions[key]?.pop()
      },
      ...custom_context,
    };
    context.imp = imp(runPath, context);
    context.inc = (package, asserts) => {
      try{
        return context.imp(package, asserts);
      } catch(e) {
        return context.require(package);
      }
    };
    context.pub = pubFunction(context);
    context.exports = exportsFunction(context);
  }
  if (!context.global) context.global = context;
  if (!context.process)
    context.process = {
      argv: process.argv,
      target: emitter(),
      env: process.env,
      cwd: () => process.cwd(),
      arch: process.arch
    };
  context.imports.assert = options.import ?? {};
  return context;
};
