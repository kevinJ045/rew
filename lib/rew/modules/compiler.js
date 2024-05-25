const { compile } = require("../../coffeescript/coffeescript");
const { getFile } = require("./fs");

const cpl = (module.exports.compile = function (file, options = {}) {
  return compile(file.content, options);
});

module.exports.compileFile = function (filepath, options = {}) {
  const f = getFile(filepath);
  const compiled_code =
    options.compile == false ? f.content : cpl(f, { ...options });

  return {
    compiled_code,
    file: f,
  };
};
