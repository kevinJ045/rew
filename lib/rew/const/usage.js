


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

  group(group, props){
    return new Usage.Group(group, props);
  }

  static Group = class UsageGroup {
    g = [{}, () => {}]
    constructor(g, props){
      this.g = g;
      for(let i in props){
        this[i] = props[i];
      }
    }
  }
}

class Namespace extends module.exports.Usage {
  namespace = {};
  constructor(ns, cb){
    super('namespace');
    if(ns instanceof Namespace.Group){
      cb = ns.g[1]
      ns = ns.g[0]
    }
    this.save = false;
    this.trigger = cb;
    this.namespace = ns;
  }

  static Group = class NamespaceGroup extends module.exports.Usage.Group {}
}
module.exports.Namespace = Namespace;

module.exports.namespace = (namespace, cb) => {
  return new Namespace(namespace, cb);
}

module.exports.namespace.group = (group, props) => new Namespace.Group(group, props);