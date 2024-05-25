const defaultContext = require("../const/default");
const emitter = require("../functions/emitter");
const { exportsFunction } = require("../functions/export");
const { imp } = require("../functions/import");

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
          return require(package);
        } catch (e) {
          throw new Error("Module not found");
        }
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
    };
  // console.log(custom_context);
  return context;
};
