
const utils = require("./lib/rew/cli/utils");
const { run } = require("./lib/rew/main");
const { compileFile, compile } = require("./lib/rew/modules/compiler");
const pkgs = require("./lib/rew/pkgs/pkgs");

module.exports = { 
  compile,
  compileFile,
  run,
  pkgs,
  utils
};