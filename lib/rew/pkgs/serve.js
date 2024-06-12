const http = require('http');
const { IttyRouter, AutoRouter, Router, createResponse, cors, error, StatusError, html } = require('itty-router');
const { json } = require('../const/default');
const { Readable } = require('stream');

module.exports = (context) => {

  // http.createServer((req, res) => {
  //   res.end();
  // }).listen(1400);

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

    port(port){
      this.listen = port;
      return this;
    }

    log(string){
      console.log(string.replace(/\$([A-Za-z0-9_]+)/g, (_, name) => this.options[name] || _));
    }
  }

  class SvrRouter {
    static new(Class, options, props){
      const router = AutoRouter(options);
      for(let i in props) router[i] = props[i];
      router.to = (server) => {
        if(server instanceof Server){
          server.routers[this.id] = this;
        }
      };
      return router;
    }
  }
  
  class Svr {
    create(options){
      return new Server(options);
    }

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

    html(string, options){
      return html(string, options);
    }

    cors(options = {}){
      return cors(options);
    }

    error(status, body){
      return error(status, body);
    }
  }

  class SvrResponse extends Response {}
  class SvrRequest extends Request {}

  Svr.prototype.Response = SvrResponse;
  Svr.prototype.Request = SvrRequest;
  Svr.prototype.StatusError = StatusError;

  IttyRouter
  
  return Svr;
}