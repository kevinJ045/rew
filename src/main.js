const { compileFile } = require("./modules/compiler");
const { exec, runPath } = require("./modules/runtime");
const path = require("path");
const { imp } = require("./functions/import");

module.exports.run = function (filepath, options = {}, custom_context = {}) {
  return runPath(filepath, options, custom_context);
};
