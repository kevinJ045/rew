const emitter = require("../../../functions/emitter");
const { struct } = require("../../../models/struct");
const WebSocket = require('ws');
const { generateRandomID } = require("./id");
/**
 * 
 * @param {*} context 
 * @param {*} options 
 * @param {*} svr 
 */
module.exports.uiClasses = (context, options, svr, send) => {
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
    parent: '!any'
  });
  class RewWidget {
    _emitter = emitter();
    on(event, callback){
      this._emitter.on(event, callback);
      return this;
    }
    off(event, callback){
      this._emitter.off(event, callback);
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
      this.init();
    }

    init(){
      send(JSON.stringify({ action: 'createElement', data: _sanitizeOptions(this.options) }))
    }

    get uuid(){
      return this.options.uuid;
    }

    get children(){
      return this.options.children;
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

  return {
    Widget: RewWidget,
    Text: RewTextWidget,
    WidgetOptions: RemWidgetOptions
  }
}