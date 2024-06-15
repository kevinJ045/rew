
const emitter = require("../functions/emitter");
const { compile } = require("../modules/compiler");
const { wait } = require("../functions/wait");
const { generateRandomID } = require("../functions/id");
const { REW_FILE_TYPE } = require("../const/ext");


const selfClosingElements = new Set([
  'area', 'base', 'br', 'col', 'embed', 'hr', 'img', 'input', 'link', 'meta', 'source', 'track', 'wbr'
]);

class Node { }

class State {
  _target = emitter();
  id = generateRandomID();
  constructor(value) {
    this._value = value;
  }
  get value() {
    return this._value;
  }
  set value(value) {
    const oldValue = this._value;
    this._value = value;
    this._target.emit('change', { old: oldValue, new: this._value })
  }

  set(value){
    this.value = value;
    return this;
  }

  subscribe(renderCallback) {
    this._target.on('change', renderCallback);
  }
}

const nodeType = (name) => class extends Node {
  [name] = "";
  constructor(value) {
    super();
    this[name] = value;
  }
}

const node = (typeNode) => class extends Node {
  constructor(value) {
    super();
    if (!value.type instanceof typeNode) throw new TypeError('Node does not match it\'s type');
    for (let i in value) this[i] = value[i];

    this.props.children.forEach(child => parentify(child, this));
  }
  parent = null;
  _target = emitter();
  find(prop, value = null) {
    return findChild(this, prop, value, true);
  }
  add(...children) {
    for (let child of children) addChildren(this, child);
    return this;
  }
  remove() {
    this.parent?.children.splice(this.parent.children.indexOf(this), 1);
    return this;
  }
  setProp(key, val) {
    if (key == 'children') return;
    this.props[key] = val;
    return this;
  }
  render() {
    renderToString(this, true);
    this._target.emit('render', () => { });
    return this;
  }
}


class TextTypeNode extends nodeType('text') { };
class TextNode extends node(TextTypeNode) { };
function createTextNode(text) {
  let t = text instanceof State ? text.value : text;
  const node = new TextNode({
    type: new TextTypeNode(t),
    props: {
      children: []
    }
  });
  if (text instanceof State) {
    node.props.children.push(text);
    text.subscribe(() => node.parent?.render());
    node.states = {
      ':text': text
    };
  }
  return node;
}

class ElementTypeNode extends nodeType('element') { };
class ElementNode extends node(ElementTypeNode) { };
function createElement(type, props, ...children) {
  const flattenChildren = (childrenArray) =>
    childrenArray.flatMap(child => Array.isArray(child) ? flattenChildren(child) : child).map(child => {
      if (typeof child !== 'object') return createTextNode(child.toString());

      if (child instanceof State) {
        const textNode = createTextNode(child);
        child.subscribe(() => {
          textNode.parent?.render();
        });
        return textNode;
      }

      return child;
    });

  if (type instanceof State) {
    return createTextNode(child);
  }

  if (typeof type === 'function') {
    return type({ ...props, children: flattenChildren(children) })
  }

  const newChildren = flattenChildren(children);


  const resolvedProps = {};
  const attributeStates = {};
  for (const key in props) {
    if (props[key] instanceof State) {
      resolvedProps[key] = props[key].value;
      props[key].subscribe(() => {
        if (props[key].value !== resolvedProps[key]) {
          resolvedProps[key] = props[key].value;
          if (elementInstance) elementInstance.render();
        }
      });
      attributeStates[key] = props[key]; 
    } else {
      resolvedProps[key] = props[key];
    }
  }


  return new ElementNode({
    type: new ElementTypeNode(type),
    props: {
      ...resolvedProps,
      children: newChildren
    },
    states: attributeStates
  });
}

function parentify(child, parent) {
  child.parent = parent;
}

function addChildren(nest, child) {
  nest.props.children.push(child);
  parentify(child, nest);
}

function findChild(nest, prop, value, recurse = false) {
  let child = nest.props.children.find(element => element instanceof State ? false : value ? element.props[prop] == value : element.type.element == prop);
  if (recurse && !child) return nest.props.children.find(element => element instanceof State ? false : findChild(element, prop, value, recurse));
  return child;
}

function cloneNest(node) {
  const clonedNode = new ElementNode({
    type: node.type,
    props: { ...node.props },
  });

  clonedNode.props.children = node.props.children.map(child => {
    if (child instanceof ElementNode) {
      return cloneNest(child);
    } else if (child instanceof TextNode) {
      return new TextNode({ type: child.type, props: { ...child.props } });
    } else {
      return child;
    }
  });

  return clonedNode;
}

function assessKey(key){
  if(key.startsWith('on')) return key.toLowerCase();
  return key;
} 

function assessValue(value, key){
  if(key.startsWith('on')) return value = `(${value})()`;
  return value;
} 

function renderToString(element, js = false) {
  if (typeof element === 'string') {
    return element
  }

  const { type, props = {} } = element

  if (typeof type === 'function') {
    return renderToString(type(props))
  }

  const children = element instanceof TextNode ? [] : props?.children || []
  const childrenParsed = Array.isArray(children) ? children.map((c) => renderToString(c, js ? 'raw' : false)) : renderToString(children, js ? 'raw' : false)
  const childrenHTML = Array.isArray(childrenParsed) ? childrenParsed.join('') : childrenParsed;

  const propsString = Object.entries(props)
    .filter(([key]) => key !== 'children')
    .map(([key, value]) => ` ${assessKey(key)}="${assessValue(value, key)}"`)
    .join('')

  const eltJSON = {
    ...element,
    nodeType: element instanceof TextNode ? 'text' : 'element',
    props: {
      ...props,
      children: childrenParsed,
    },
    states: element.states || {}
  };
  for(let i in eltJSON.props) {
    if(typeof eltJSON.props[i] == "function"){
      eltJSON.props[assessKey(i)] = assessValue(eltJSON.props[i].toString(), i);
    }
  }
  delete eltJSON.parent;
  delete eltJSON._target;

  return js ? js === 'raw' ? eltJSON : JSON.stringify(eltJSON) : element instanceof TextNode ? `${type.text}` : `<${type.element}${propsString}>${selfClosingElements.has(type.element) ? '' : childrenHTML}${selfClosingElements.has(type.element) ? '' : `</${type.element}>`}`;
}

class Page extends Node {
  constructor() {
    super();
  }
  /** @type{ElementNode} */
  root = null;
  /** @type{ElementNode} */
  head = null;
  /** @type{ElementNode} */
  body = null;
  find(key, value = null) {
    return this.root.find(key, value);
  }
  add(...children) {
    return this.body.add(...children);
  }
  script(scriptString) {
    if (typeof scriptString == "object" && scriptString.src)
      return this.add(createElement('script', { src: scriptString.src }));
    else
      return this.add(createElement('script', null, createTextNode(scriptString)));
  }
  style(styleString) {
    if (typeof styleString == "object" && styleString.href)
      return this.head.add(createElement('link', { href: styleString.href, rel: 'stylesheet' }));
    else
      return this.head.add(createElement('style', null, createTextNode(styleString)));
  }
  link(rel, href){
    return this.head.add(createElement('link', { href, rel }));
  }
  serializeState() {
    const states = {};
    function extractStates(node) {
      if (node instanceof ElementNode) {
        if (node.props && node.props.children) {
          node.props.children.forEach(child => extractStates(child));
        }
      }
      if(node.states){
        for(let i in node.states) states[node.states[i].id] = node.states[i];
      }
    }
    extractStates(this.root);
    this.initialState = states;
    return JSON.stringify(states);
  }
  render(staticRender = false) {
    return staticRender ? renderToString(this.root) : `<script>
    const __INITIAL_STATE__ = ${this.serializeState()};
    const DOMObject = ${renderToString(this.root, true)};
    const generateRandomID = ${generateRandomID}
    const emitter = ${emitter}
    ${State}
    const states = [];

    function setAttribute(el, key, value){
      let defVal = value;
      if(key == 'style'){
        for(let i in value){
          const v = value[i];
          el.style.setProperty(i, value[i]);
        }
        return;
      } else if(key.startsWith('on')){
        el.addEventListener(key.replace('on', '').toLowerCase(), (event) => {
          new Function('event', value).call(event.target, event);
        });
        return;
      }
      el.setAttribute(key, defVal);
    }

    function rehydrate() {
      const initialState = __INITIAL_STATE__;
      
      function updateDOM(node, state) {
        const elt = node.DOMELEMENT ? node.DOMELEMENT : node.nodeType == 'text' ? document.createTextNode(node.type.text) : document.createElement(node.type.element);
        node.DOMELEMENT = elt;

        if (node.nodeType == 'text') {
          if(node.states?.[':text']){
            const state = node.states[':text'];
            if(state) elt.textContent = getState(state.id)?.value || state._value;
          }
        } else if (node.nodeType !== 'text' && node.props.children) {
          const nodeState = node.states || {};
          node.props.children.forEach(child => {
            child.parent = node;
            updateDOM(child, state);
          });
          Object.keys(node.props).forEach(key => {
            if (key !== 'children') {
              if(key in nodeState){
                setAttribute(elt, key, getState(nodeState[key].id)?.value ?? nodeState[key]._value);
              } else setAttribute(elt, key, node.props[key]);
            }
          });
          if('data-only-if' in node.props){
            if(elt.getAttribute('data-only-if') == 'true'){
              elt.hidden = false;
            } else {
              elt.hidden = true;
            }
          }
        }
        if(node.parent && !elt.parentNode){
          node.parent.DOMELEMENT.appendChild(elt);
        }
        return node;
      }

      function createState(inState, val, key){
        const state = new State(inState._value);
        state.id = inState.id;
        states.push(state);
        state.subscribe(() => updateDOM(DOMObject, { [key]: Array.isArray(val) ? [...val.filter(i => i.id !== state.id), state] : state }));
      }
      
      Object.keys(initialState).forEach(key => {
        if(Array.isArray(initialState[key])) initialState[key].forEach((i) => createState(i, initialState[key], key));
        else createState(initialState[key], initialState[key], key);
      });

      document.body.parentNode.remove();
      document.appendChild(updateDOM(DOMObject, initialState).DOMELEMENT);
    }
    window.getState = (id) => states.find(s => s.id == id);
    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', rehydrate);
    } else {
      rehydrate();
    }
    </script>`;
  }

  toString() {
    return this.render();
  }

  clone(){
    const page = new Page();
    page.root = cloneNest(this.root);
    page.body = page.root.find('body');
    page.head = page.root.find('head');
    page.body.parent = page.root;
    page.head.parent = page.root;
    return page;
  }
}
function createPage(options) {
  const page = new Page;
  const root = createElement('html');
  page.root = root;

  const head = createElement('head');
  page.head = head;

  if (options.viewportMeta) {
    head
      .add(createElement('meta', { charset: 'UTF-8' }))
      .add(createElement('meta', { name: 'viewport', content: typeof options.viewportMeta == 'string' ? options.basicMeta : 'width=device-width, initial-scale=1.0' }));
  }

  const title = createElement('title', null, 'Document');

  if (options.title) title.props.children = [createTextNode(options.title)];

  if (options.title !== false) {
    head.add(title);
    page.title = title;
  }

  const body = createElement('body');
  page.body = body;

  root.add(head);
  root.add(body);

  return page;
}

module.exports = (context, importOptions) => {

  const { build } = wait(async () => await import('vite'));

  class Web {
    create(options) {
      return createPage(options);
    }
    isNode(node) {
      return node instanceof Node;
    }
    isTextNode(node) {
      return node instanceof TextNode;
    }
    isElementNode(node) {
      return node instanceof ElementNode;
    }
    createText(text) {
      return createTextNode(text);
    }
    createElement(...args) {
      return createElement(...args);
    }
    state(value) {
      return new State(value);
    }
    invokeState(states, callback){
      const statesMapped = states.map(i => i instanceof State ? `getState('${i.id}')` : JSON.stringify(i));
      return `((${callback})(event, ...[${statesMapped}]))`;
    }
    async bundle(filepath, options = {}) {
      const virtualModuleId = `virtual:${filepath}`;
      const result = await build({
        build: {
          rollupOptions: {
            input: options.code ? virtualModuleId : filepath,
            output: {
              format: 'iife', // Immediately Invoked Function Expression for the browser
              entryFileNames: '[name].js',
            },
          },
          write: false, // Do not write to file system, get the output as string
        },
        logLevel: 'silent',
        plugins: [
          (function rew() {
            return {
              name: 'rew',
              resolveId(id) {
                if (options.code && id === virtualModuleId) {
                  return virtualModuleId;
                }
                return null;
              },
              load(id) {
                if (options.code && id === virtualModuleId) {
                  return options.code;
                }
                return null;
              },
              async transform(code, id) {
                if (id.endsWith(REW_FILE_TYPE.EXTENSION) || id.endsWith('.coffee')) {
                  const result = compile({ content: code, path: filepath }, { jsx: true, async: true, keepImports: true });
                  return {
                    code: await result,
                    map: null,
                  };
                }
              },
            };
          })(),
          ...(options.plugins ?? [])
        ],
      });
      return result.output[0].code;
    }
  }

  return importOptions.instance ? new Web : Web;
}