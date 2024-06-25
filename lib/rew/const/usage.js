


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
  trigger = () => {};
  save = true;

  constructor(name, trigger, save){
    this.name = name;
    this.trigger = trigger;
    this.save = save;
  }

  create(name, trigger, save = true){
    return new Usage(name, trigger, save);
  }
}

class Namespace extends module.exports.Usage {
  namespace = {};
  constructor(ns, cb){
    super('namespace');
    this.save = false;
    this.trigger = cb;
    this.namespace = ns;
  }
}
module.exports.Namespace = Namespace;

module.exports.namespace = (namespace, cb) => {
  return new Namespace(namespace, cb);
}