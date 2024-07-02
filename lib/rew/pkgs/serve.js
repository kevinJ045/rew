const { readdirSync, existsSync } = require('fs');
const http = require('http');
const { IttyRouter, AutoRouter, Router, createResponse, cors, error, StatusError, html, json, withCookies, withContent, withParams, jpeg, png, webp, text, status } = require('itty-router');
const path = require('path');
const { run } = require('../main');
const { runPath } = require('../modules/runtime');
const { cleanCache } = require('../functions/import');
const { REW_FILE_TYPE } = require('../const/ext');
const { Usage } = require('../const/usage');

const lookUpFiles = ['route', 'page', 'page.s'];

module.exports = (context) => {

  // http.createServer((req, res) => {
  //   res.end();
  // }).listen(1400);

  const imp = (file) => context.imp(file);
  const Web = imp('web');

  function mkReq(req) {
    const url = `http://${req.headers.host}${req.url}`;
    const options = {
      method: req.method,
      headers: req.headers,
      body: req.body
    };
  
    return new Request(url, options);
  }

  class Server {
    _server = {};
    routers = {};

    constructor(options){
      this.options = options;
      this._server = http.createServer((req, res) => {
        options.handler ? options.handler(req, res) : this.handleRequest(req, res);
      });
      if(options.routers){
        options.routers.forEach(router => router.to(this));
      }
    }
  
    async handleRequest(req, res){ 
      try {
        let response = new Response();
        const request = mkReq(req);
        if(this.options.fetch == 'router'){
          if(!Object.keys(this.options.routers).length) throw new Error('No fetch function nor routers found');
          response = await this.options.routers[Object.keys(this.options.routers)[0]].fetch(request);
        } else {
          response = await this.options.fetch(request);
        }

        if(!response){
          res.end('Cannot '+req.method+' '+req.url);
          return;
        }

        response.headers.forEach((value, name) => {
          res.setHeader(name, value);
        });
        
        res.writeHead(response.status);
  
        const buffer = await response.arrayBuffer();
        res.end(Buffer.from(buffer));
      } catch (error) {
        // Handle errors
        console.error("Error:", error);
        res.writeHead(500, {'Content-Type': 'text/plain'});
        res.end("Internal Server Error");
      }
    }
    
    get listen(){
      this._server.listen(this.options.port);
      return this;
    }

    set listen(port){
      this.options.port = port;
      return this;
    }

    fetcher(cb){
      this.options.fetch = cb;
      return this;
    }

    port(port){
      this.listen = port;
      return this;
    }

    log(string){
      console.log(string.replace(/\$([A-Za-z0-9_]+)/g, (_, name) => this.options[name] || _));
      return this;
    }
  }

  class SvrRouter {
    static new(Class, options, props){
      const router = Class(options);
      for(let i in props) router[i] = props[i];
      router.to = (server) => {
        if(server instanceof Server){
          server.routers[this.id] = this;
        }
      };
      return router;
    }
  }

  function findLayoutFiles(filePath, root, isClientSide = true, resolveExtensions = [REW_FILE_TYPE.EXTENSION,  '.coffee', '.js', '.jsx']) {
    const layouts = [];
    const rootDir = root;
    let currentDir = path.dirname(filePath);
  
    while (currentDir !== rootDir) {
      for (const ext of resolveExtensions) {
        const layoutFile = path.join(currentDir, `layout${isClientSide ? '' : '.s'}${ext}`);
        if (existsSync(layoutFile)) {
          layouts.push(layoutFile);
        }
      }
      currentDir = path.dirname(currentDir);
    }
    
    for (const ext of resolveExtensions) {
      const layoutFile = path.join(currentDir, `layout${isClientSide ? '' : '.s'}${ext}`);
      if (existsSync(layoutFile)) {
        layouts.push(layoutFile);
      }
    }
  
    return layouts.reverse();
  }  

  const defaultBundlerEntry = (file, layouts, data) => `
  import * as target from "${file}";
  ${layouts.map((layout, ind) => `import * as layout${ind} from "${layout}";`).join('\n')}
  let page = target.render ? target.render(${JSON.stringify(data)}) : target.default ? target.default(${JSON.stringify(data)}) : null;
  ${layouts.reverse().map((_, ind) => `if (layout${ind}.render) page = layout${ind}.render(${JSON.stringify(data)}, page);`).join('\n')}
  `;
  
  const defaultSsrBundlerEntry = (file, layouts, data) => `
  files = "${layouts.length ? layouts.join(',')+',' : ''}${file}".split(',')

  renderers = []
  staticRendering = false

  for key, file of files
    renderers.push imp file
  
  staticRendering = true if renderers[renderers.length-1].staticRendering

  render = (req, data) -> 
    target = renderers.pop()
    page = target.render req, data
    for renderer in renderers
      page = renderer.render req, data, page
    page

  exports { render, staticRendering }
  `;

  function createFileRouter({
    onError = () => error(404, 'Path not found'),
    root = '',
    basePath = '',
    resolveExtensions = [REW_FILE_TYPE.EXTENSION, '.coffee', '.js', '.jsx'],
    bundlerOptions = {},
    bundlerEntry = defaultBundlerEntry,
    ssrBundlerEntry = defaultSsrBundlerEntry,
  }) {

    const params = {};
  
    function lookUp(pathname) {
      const routeParts = pathname.split('/').filter(Boolean);
      let routePath = root;


      Object.keys(params).forEach(key => delete params[key]);
    
      for (const part of routeParts) {
        const dir = readdirSync(routePath);
    
        const match = dir.find(d => d === part || d.match(/^\[.*\]$/));
        if (!match) return null;

        if (match.match(/^\[.*\]$/)) {
          const paramName = match.slice(1, -1);
          params[paramName] = part;
        }
    
        routePath = path.join(routePath, match);
      }
      
      for (const base of lookUpFiles) {
        for (const ext of resolveExtensions) {
          const filePath = path.join(routePath, `${base}${ext}`);
          if (existsSync(filePath)) {
            return filePath;
          }
        }
      }
      
      return null;
    }

    function getReqProps(req) {
      return {
        params: {
          ...params,
          ...(req.params || {})
        },
        query: req.query,
        url: req.url,
        method: req.method
      }
    }

    const w = new Web();

    async function renderPage(file, basename, req){
      const page = w.create({ viewportMeta: true });
      let staticRendering = false;
      if(basename.endsWith('.s')){
        // SSR is enabled, do only ssr
        const layouts = findLayoutFiles(file, root, false);
        const fileContext = runPath(file, { code: ssrBundlerEntry(file, layouts) }).context.module.exports || {};
        if(typeof fileContext.render !== "function") throw new ReferenceError("Route does not export function render");
        let pageContent = fileContext.render(req, { page, ...getReqProps(req) });
        if(fileContext.staticRendering) staticRendering = true;
        if(!w.isNode(pageContent)) throw new TypeError("Route.render does not return an element");
        if(pageContent?.type?.element == 'head'){
          page.root.props.children.splice(page.root.props.children.indexOf(page.head), 1);
          page.head = pageContent;
          page.root.add(pageContent);
        } else if(pageContent?.type?.element == 'body'){
          page.root.props.children.splice(page.root.props.children.indexOf(page.body), 1);
          page.body = pageContent;
          page.root.add(pageContent);
        } else if(pageContent?.type?.element == 'html'){
          page.root = pageContent;
          page.body = pageContent.find('body');
          page.head = pageContent.find('head');
        } else {
          page.add(pageContent);
        }
      } else {
        const layouts = findLayoutFiles(file, root, true);
        const scriptString = await w.bundle(path.join(root, 'bundle.js'), {
          ...bundlerOptions,
          code: bundlerEntry(file, layouts, getReqProps(req))
        });
        page.script(scriptString);
        staticRendering = true;
      }
      return html(page.render(staticRendering));
    }
  
    async function handleRequest(req, file) {
      const ext = path.extname(file);
      const basename = path.basename(file, ext);
  
      if (basename.startsWith('route')) {
        const fileContext = run(file).context;
        const handlers = fileContext.module.exports;
        const method = req.method.toUpperCase();
        if (handlers[method]) {
          return await handlers[method](req, getReqProps(req));
        } else {
          return error(405, `Method ${method} not allowed`);
        }
      } else if (basename.startsWith('page')) {
        return await renderPage(file, basename, req);
      }
    }
  
  
    return async (req) => {
      const url = new URL(req.url);
      const pathname = basePath ? url.pathname.replace(new RegExp('^'+basePath), '') : url.pathname;
      const file = lookUp(pathname);
      cleanCache();
  
      if (file) {
        const response = handleRequest(req, file);
        response.catch(() => onError());
        return await response;
      } else {
        return onError();
      }
    };
  }
  

  
  class Svr {
    create(options){
      return new Server(options);
    }

    service = Usage.prototype.create('svr.service', (cb) => {
      const server = Svr.prototype.create();
      cb(server);
    });

    router({ id = '/', type = 'normal', ...options }){
      let router;
      if(type == 'default') router = SvrRouter.new(IttyRouter, {...options}, { id });
      if(type == 'auto') router = SvrRouter.new(AutoRouter, {...options}, { id });
      if(type == 'normal') router = SvrRouter.new(Router, {...options}, { id });

      return router;
    }

    createResponse(format, transform, type = 'normal'){
      return type == 'json' ? json(format, transform) : createResponse(format, transform);
    }

    html(string, options = {}){
      return html(string, options);
    }
    
    json(object, options = {}){
      return json(object, options);
    }

    jpeg(image, options = {}){
      return jpeg(image, options);
    }

    png(image, options = {}){
      return png(image, options);
    }

    webp(image, options = {}){
      return webp(image, options);
    }
    
    text(string, options = {}){
      return text(string, options);
    }
    
    status(code, options = {}){
      return status(code, options);
    }

    cors(options = {}){
      return cors(options);
    }

    error(status, body){
      return error(status, body);
    }

    createFileRouter(o){
      return createFileRouter(o);
    }
  }

  class SvrResponse extends Response {}
  class SvrRequest extends Request {}

  Svr.prototype.Response = SvrResponse;
  Svr.prototype.Request = SvrRequest;
  Svr.prototype.URL = URL;
  Svr.prototype.StatusError = StatusError;

  Svr.prototype.withContent = withContent;
  Svr.prototype.withCookies = withCookies;
  Svr.prototype.withParams = withParams;

  IttyRouter
  
  return Svr;
}