const { compile } = require("../modules/compiler")

const _compilePart = (code, filepath) => compile(
  {
    path: filepath,
    content: code
  },
  {}
);

module.exports = (filepath) => ({
  _compilePart: (code) => {
    return _compilePart(code, filepath);
  },
  _call: (fn, ...args) => args.length ? fn.call(...args) : fn()
})