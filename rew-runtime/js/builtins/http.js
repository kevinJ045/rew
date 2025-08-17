"no-compile"
if(!rew.extensions.has('http')) rew.extensions.add('http', (Deno, module) => rew.extensions.createClass({
  serveSimple(options, handler) {
    return Deno.serve(options, handler);
  },
  withOptions(options) {
    return (handler) => Deno.serve(options, handler);
  },
  serveHttp: Deno.serveHttp,
  Response: class extends Deno.Response {
    new(body, init) {
      return new Deno.Response(body, init);
    }
  },
  Request: class extends Deno.Request {
    new(input, init) {
      return new Deno.Request(input, init);
    }
  },
  upgradeWebSocket: Deno.upgradeWebSocket,
}));