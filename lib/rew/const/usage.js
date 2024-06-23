


module.exports.USING_DEFAULT = {
  JSX: {
    param: (param) => ({ createElement: param }),
    use:  (options) => options.jsx = true
  },
  TYPES: {
    use:  (options) => options.typescript = true
  },
  DECORATORS: {
    use:  (options) => options.decorators = true
  }
}

module.exports.Usage = class Usage {
  name = "null";
  trigger = () => {}

  constructor(name, trigger){
    this.name = name;
    this.trigger = trigger;
  }

  create(name, trigger){
    return new Usage(name, trigger);
  }
}