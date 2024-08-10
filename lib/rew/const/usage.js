const STDNS = require("./stdns");



const JSX_FRAGMENT_SYMBOL = Symbol('fragment');
const USING_DEFAULT = module.exports.USING_DEFAULT = {
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

class Usage {
  name = "null";
  trigger = () => {};
  save = true;

  constructor(name, trigger, save){
    this.name = name || '';
    this.trigger = trigger;
    this.save = name == '' || name == null ? false : save;
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
module.exports.Usage = Usage;

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
  if(namespace instanceof STDNS) {
    if(namespace['@cb'] && !cb) {
      cb = namespace['@cb'];
      delete namespace['@cb'];
    }
  }
  return new Namespace(namespace, cb, parent);
}

module.exports.usingFunction = (context, runtime) => {
  return function using(name, ...params) {
    if(name instanceof Usage.Group){
      params.unshift(...name.g.slice(1));
      name = name.g[0];
    }
    if(USING_DEFAULT[name]){
      if(USING_DEFAULT[name].param) {
        context.__using__[name] = USING_DEFAULT[name].param(...params);
      }
    } else if(name instanceof Namespace) {
      const trigger = name.trigger;
      const parentContext = name.parent || context;
      const childContext = {...parentContext, ...name.namespace, trigger};
      childContext.$self = name.namespace;
      childContext.$parent = parentContext;
      const code = `(${trigger.toString()})()`;
      if(name.onUse) name.onUse();    
      const r = runtime.exec(code, childContext, code, context.module.filepath);
      if(name.onAfterUse) name.onAfterUse();
      return r;
    } else if(name instanceof Usage) {
      const v = name.trigger(...params);
      if(name.save !== false) context.__using__[name.name] = v ?? true;
      return v || (typeof name.result === "function" ? name.result(v) : name.result || true);
    } else {
      context.__using__[name] = params.length ? params.length > 1 ? [...params] : params : true;
    }
  }
}

module.exports.namespace.group = (group, props) => new Namespace.Group(group, props);