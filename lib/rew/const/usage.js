


const JSX_FRAGMENT_SYMBOL = Symbol('fragment');
module.exports.USING_DEFAULT = {
  JSX: {
    param: (param, fragment) => param ? ({ createElement: param, Fragment: fragment || param(JSX_FRAGMENT_SYMBOL, { isFragment: true }), fragmentSymbol: JSX_FRAGMENT_SYMBOL }) : {},
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
    this.save = name == '' ? false : save;
  }

  create(name, trigger, save = true){
    return new Usage(name, trigger, save);
  }

  group(...group){
    return new Usage.Group(group, {});
  }

  /**
   * 
   * @param  {...Usage} usages 
   * @returns {any[]}
   */
  merge(...usages){
    return new Usage(usages.map(u => u.name).join('-'), (...args) => {
      return usages.map(u => {
        return u.trigger(...args)
      });
    })
  }

  static Group = class UsageGroup {
    g = []
    constructor(g, props = {}){
      this.g = g;
      if(props) for(let i in props){
        this[i] = props[i];
      }
    }
    
    with(props){
      return new UsageGroup(this.g, props);
    }
  }
}

class Namespace extends module.exports.Usage {
  namespace = {};
  constructor(ns, cb, parent){
    super('namespace');
    if(ns instanceof Namespace.Group){
      if(ns.onUse) this.onUse = ns.onUse;
      if(ns.parent) parent = ns.parent;
      if(ns.onAfterUse) this.onAfterUse = ns.onAfterUse;
      cb = ns.g[1]
      ns = ns.g[0]
    }
    this.save = false;
    this.trigger = cb;
    this.namespace = ns;
    this.parent = parent;
  }

  static Group = class NamespaceGroup extends module.exports.Usage.Group {}
}
module.exports.Namespace = Namespace;

module.exports.namespace = (namespace, cb, parent) => {
  return new Namespace(namespace, cb, parent);
}

module.exports.namespace.group = (group, props) => new Namespace.Group(group, props);