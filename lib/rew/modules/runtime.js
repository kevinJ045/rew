const vm = require("vm");
const { compileFile } = require("./compiler");
const { prepareContext } = require("./context");

const exec = (module.exports.exec = function (code, context) {
  return vm.runInNewContext(code, vm.createContext(context), {
    filename: context.module.filepath,
    lineOffset: 0,
    displayErrors: true,
  });
});

module.exports.runPath = function runPath(
  filepath,
  options = {},
  custom_context = {},
) {
  const { compiled_code, file } = compileFile(filepath, options);
  const context = prepareContext(custom_context, options, file.path, runPath);

  context.module.compiled = compiled_code;
  context.process.exit = (int) => process.exit(int);

  return {
    context,
    returns: exec(compiled_code, context),
  };
};
