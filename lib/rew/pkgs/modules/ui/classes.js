const emitter = require("../../../functions/emitter");
const { struct } = require("../../../models/struct");
const { generateRandomID } = require("../../../functions/id");

module.exports.uiClasses = (context, options, send, recieve, hook, rmHook) => {
  const _sanitizeOptions = (options) => {
    return {
      ...options,
      children: options.children.map(i => i.options)
    }
  }
  const RemWidgetOptions = struct({
    element: 'div',
    class: '',
    attr: {},
    id: '',
    data: {
      text: ''
    },
    children: [],
    uuid: '',
    parent: '!any',
    style: {}
  });

  const CreatedElements = [];

  class RewWidget {
    _emitter = emitter();
    on(event, callback){
      const hookID = this.uuid+'_'+generateRandomID(4);
      this._emitter.on(event, callback, { hookID });
      hook(hookID, 'event_'+event, (data) => {
        this.emit(event, data);
      }, false);
      send({ action: 'eventListen', data: { uuid: this.uuid, event, hookID } })
      return this;
    }
    off(event, callback){
      this._emitter.off(event, callback, (e) => rmHook(e.hookID));
      return this;
    }
    emit(event, callback){
      this._emitter.emit(event, callback);
      return this;
    }

    options = RemWidgetOptions();
    constructor(options = RemWidgetOptions()){
      const config = RemWidgetOptions(options);
      config.uuid = generateRandomID();
      this.options = config;
      this.options.children.forEach(child => child.parent = this);
      this.init();
      CreatedElements.push(this);
    }

    init(){
      send({ action: 'createElement', data: _sanitizeOptions(this.options) })
    }

    parent = null;

    get uuid(){
      return this.options.uuid;
    }

    get id(){
      return this.options.id;
    }

    get children(){
      return this.options.children;
    }

    find(id, recursive = true){
      let childFound = this.children.find(e => e.id == id) || this.children.find(e => e.uuid == id);
      if(childFound) return childFound;
      else if(!recursive) return null;
      for(let child of this.children){
        let subchild = child.find(id);
        if(subchild) {
          return subchild;
        }
      }
    }

    update(){
      send({ action: 'updateElement', data: _sanitizeOptions(this.options) });
      return this;
    }

    text(text){
      this.options.data.text = text;
      return this.update();
    }

    data(key, value){
      if(!value) return this.options.data[key];
      this.options.data[key] = value;
      return this.update(); 
    }

    attr(attr, reset = false){
      if(reset) this.options.attr = attr;
      else this.options.attr = { ...this.options.attr, ...attr };
      return this.update();
    }

    style(style, reset = false){
      if(reset) this.options.style = style;
      else this.options.style = { ...this.options.style, ...style };
      return this.update();
    }

    add(child){
      this.options.children.push(child);
      return this.update();
    }

    remove(childId, recursive = true){
      const child = typeof childId == "string" ? this.find(childId, recursive) : childId;
      if(!child) return this;
      if(recursive && child.parent !== this){
        child.parent.remove(child);
      } else {
        this.options.children.splice(this.options.children.indexOf(child), 1);
        this.update();
      }
      return this;
    }

  }

  class RewTextWidget extends RewWidget {
    constructor(text = '', options = RemWidgetOptions({})){
      super({
        ...options,
        data: { ...(options.data), text }
      });
    }
  }

  class StyleSheet {
    constructor(css = ''){
      send({ action: 'addStyleSheet', data: css });
    }
  }

  function findElement(id){
    return new Promise((r) => {
      const rid = generateRandomID();
      hook(rid, 'findElement', (data) => {
        r(CreatedElements.find(e => e.uuid == data.uuid) || data);
      });
      send({ action: 'findElement', data: { id, rid } });
    });
  }

  // hook('event_trigger', 'eventTrigger', (data) => {
  //   const el = CreatedElements.find(e => e.uuid = data.uuid);
  //   if(el){
  //     el.emit(data.event, data.data);
  //   }
  // }, false);

  const Transmitter = {
    send: (data) => send({ action: 'message', data }),
    recieve: (cb) => recieve((data) => cb(data.data))
  }

  return {
    Widget: RewWidget,
    Text: RewTextWidget,
    WidgetOptions: RemWidgetOptions,
    findElement,
    StyleSheet: StyleSheet,
    Transmitter
  }
}