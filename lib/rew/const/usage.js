


module.exports.USING_DEFAULT = {
  JSX: {
    param: (param) => ({ createElement: param }),
    use:  (options) => options.jsx = true
  },
  DECORATORS: {
    use:  (options) => options.typescript = true
  },
  TYPES: {
    use:  (options) => options.typescript = true
  }
}