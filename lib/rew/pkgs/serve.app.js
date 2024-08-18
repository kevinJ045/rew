const { typex } = require("../const/default");
const emitter = require("../functions/emitter");
const path = require('path');

module.exports = (context) => {
  const svr = context.imp('serve');

  let appOptions = {};

  const mkRouter = (options = {}) => svr.prototype.router({ type: 'auto', ...appOptions, ...options });

  class Events {
    target = emitter();
    on(...args){
      this.target.on(...args);
      return this;
    }
    off(...args){
      this.target.off(...args);
      return this;
    }
    emit(...args){
      this.target.emit(...args);
      return this;
    }
  }

  const Context = (name, inherit = []) => {
    const ctxC = class Context {

      constructor(item){
        this[name] = item;
      }

      on(evt, cb){
        this[name].on(evt, cb);
        return this;
      }
      
      off(evt, cb){
        this[name].off(evt, cb);
        return this;
      }
      
      emit(evt, ...data){
        this[name].emit(evt, ...data);
        return this;
      }
    };

    inherit?.forEach(fname => {
      ctxC.prototype[fname] = function(...args){
        this[name][fname](...args);
        return this;
      }
    });

    return ctxC;
  }

  class AppContext extends Context('app', ['start', 'redirect']) {
    inject(item){
      this.app.inject(item);
      return this;
    }

    module(...items){
      return this.inject(...items);
    }
  }

  class ServeApp extends Events {
    #injectables = [];
    server;
    router;

    constructor(options){
      super();
      appOptions = options;
      this.router = mkRouter(options);
      this.server = svr.prototype.create({
        fetch: this.router.fetch
      });
    }

    /** @param {Injectable[]} injectables */
    inject(...injectables){
      injectables.filter(i => i instanceof Injectable)
      .forEach(injectable => {
        this.#injectables.push(injectable);
        injectable.emit('activate', this);
        this.emit('activate', this);
      });
      return this;
    }

    redirect(at = '*', router){
      this.router.all(at, typeof router == 'function' ? router : router.fetch);
      return this;
    }

    new(options = {}){
      return (cb) => new context.Usage('', () => {
        const app = new ServeApp(options);
        const appContext = new AppContext(app);
        cb.call(appContext);
        return app;
      });
    }

    raw(){
      return context.using(this.new(() => {}));
    }

    #setup(){
      this.#injectables
      .filter(injectable => injectable instanceof SModule)
      .forEach(module => {
        /** @type {SController[]} */
        const controllers = module.getControllers();
        controllers.forEach(controller => {
          // console.log(path.join('/', controller.root, '*'), controller);
          this.redirect(path.join('/', controller.root, '*'), controller.router);
        })
      });
    }

    start(port){
      this.#setup();
      this.server.port(port).listen;
      this.emit('start', port);
      return this;
    }
  }

  class Injectable extends Events {
    #injected = [];
    forEachInjectable(cb, event, ...injectables){
      injectables.filter(i => i instanceof Injectable)
      .forEach(injectable => {
        cb(injectable);
        injectable.emit(event, this);
        this.emit(event, this);
      });
      return this;
    }
    inject(...injectables){
      return this.forEachInjectable(
        (injectable) => this.#injected.push(injectable),
        'activate',
        ...injectables
      );
    }
  }


  class SModuleContext extends Context('module') {

    provider(...provider){
      this.module.addProvider(...provider);
      return this;
    }

    controller(...controller){
      this.module.addController(...controller);
      return this;
    }
  }

  class SModule extends Injectable {
    #providers = [];
    #controllers = [];
    #app = {};

    constructor(){
      super();
      this.on('activate', (app) => {
        this.#app = app;
        this.#controllers.concat(
          this.#providers
        ).forEach(injectable => injectable.emit('activate:app', app));
      });
    }

    addProvider(...providers){
      return this.forEachInjectable(
        (injectable) => this.#providers.push(injectable),
        'activate',
        ...providers
      );
    }

    hasProvider(provider){
      return this.#providers.find(p => p == provider);
    }

    addController(...controllers){
      return this.forEachInjectable(
        (injectable) => this.#controllers.push(injectable),
        'activate',
        ...controllers
      );
    }

    getControllers(){
      return this.#controllers;
    }

    new(cb){
      return new context.Usage('', () => {
        const module = new SModule();
        const moduleContext = new SModuleContext(module);
        cb.call(moduleContext);
        return module;
      });
    }
  }

  class SServiceContext extends Context('service') {
    function(name){
      return (cb) => {
        this[name] = (...args) => cb.call(this, ...args);
        this.service[name] = this[name];
        return this[name];
      }
    }
  }
  class SProvider {
    constructor(name, injectable){
      this.name = name;
      this.injectable = injectable;
    }
  }
  class SService extends Injectable {
    new(cb){
      return new context.Usage('', () => {
        const service = new SService();
        const serviceContext = new SServiceContext(service);
        cb.call(serviceContext);
        return service;
      });
    }

    as(name){
      return new SProvider(name, this);
    }
  }

  const ggreq = (that, method, _path, middleWare) => (cb) => {
    cb.parent = that;
    that.controller.register(method, _path, cb, middleWare);
    return cb;
  }
  const mkReq = (usage) => (req, ctx) => usage.call(req, req, ctx);
  class SControllerContext extends Context('controller') {

    middleWares = [];

    getMiddleWares(md){
      return (...args) => {
        this.middleWares.forEach(middleWare => {
          mkReq(middleWare)(...args);
        });
        if(md){
          mkReq(md)(...args);
        }
      }
    }

    withCookies(){
      return svr.prototype.withCookies;
    }
    withContent(){
      return svr.prototype.withContent;
    }

    get(_path, middleWare){
      return ggreq(this, 'get', _path, this.getMiddleWares(middleWare));
    } 
    post(_path, middleWare){
      return ggreq(this, 'post', _path, this.getMiddleWares(middleWare));
    } 
    delete(_path, middleWare){
      return ggreq(this, 'delete', _path, this.getMiddleWares(middleWare));
    } 
    patch(_path, middleWare){
      return ggreq(this, 'patch', _path, this.getMiddleWares(middleWare));
    } 
    put(_path, middleWare){
      return ggreq(this, 'put', _path, this.getMiddleWares(middleWare));
    }
    all(_path, middleWare){
      return ggreq(this, 'all', _path, this.getMiddleWares(middleWare));
    }

    use(usage){
      this.controller.router.all('*', mkReq(usage));
      return this;
    }

    useEach(usage){
      this.middleWares.push(usage);
      return this;
    }

    usem(...middleWares){
      return function(...args){
        middleWares.forEach(middleWare => middleWare.call(this, ...args));
      }
    }

    normal(cb){
      return function(){
        return cb(this.request, this);
      }
    }
  }

  class SController extends Injectable {

    constructor(root){
      super();
      this.root = root;
      this.router = mkRouter({ base: path.join('/', root) });
    }

    mkCallback(cb){
      return async (req, ctx) => {
        let context = {...{
          request: req,
          body: svr.prototype.withContent({...req, body: req.data.buffer() })
        },...ctx, ...(cb.parent || {}), ...{
          text: svr.prototype.text,
          html: svr.prototype.html,
          status: svr.prototype.status,
          json: svr.prototype.json,
          error: svr.prototype.error
        }};
        return await cb.call(context, ...Object.keys(context).map(i => context[i]));
      }
    }

    register(method, url, cb, middleWare){
      this.router[method](url || '/', middleWare || this.mkCallback(cb), middleWare ? this.mkCallback(cb) : undefined);
      return this;
    }

    new(root = '/'){
      const controller = new SController(root || '/');
      const controllerContext = new SControllerContext(controller);
      const cb = (cb) => new context.Usage('', () => {
        cb.call(controllerContext);
        return controller;
      });
      cb.with = (...args) => {
        controller.on('activate', (parentModule) => {
          args
          .filter(arg => arg instanceof SProvider)
          .filter(arg => parentModule.hasProvider(arg.injectable))
          .forEach(e => {
            controllerContext[e.name] = e.injectable;
          });
        });
        return cb;
      }
      return cb;
    }
  }

  class SControllerMiddleware extends Injectable {
    constructor(root, router){
      super();
      this.root = root;
      this.router = router;
    }
  }

  function createSControllerMiddleware(root = '') {
    return (cb) => new SControllerMiddleware(root, cb);
  }

  return {
    ServeApp,
    SModule,
    SService,
    SController,
    SControllerMiddleware: createSControllerMiddleware
  };
}