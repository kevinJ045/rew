const defaultContext = require("../const/default");
const { execOptions } = require("../const/opt");
const emitter = require("../functions/emitter");
const { exportsFunction } = require("../functions/export");
const { imp } = require("../functions/import");
const { customRequire } = require("../functions/require");
const fsLib = require('../functions/fs');

module.exports.prepareContext = function (
  custom_context,
  options,
  filepath = "",
  runPath = () => {},
) {
  let context = {
    module: {
      exports: null,
      options,
      filepath,
      imports: []
    },
    ...fsLib(filepath)
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
  // console.log(custom_context);
  return context;
};
