


module.exports.USING_DEFAULT = {
  JSX: {
    param: (param) => ({ createElement: param }),
    use:  (options) => options.jsx = true
  },
  TYPES: {
    use:  (options) => options.typescript = true
  }
}