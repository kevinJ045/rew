"no-compile"
if(!rew.extensions.has('net')) rew.extensions.add('net', (Deno, module) => rew.extensions.createClass({
  _namespace() {
    return "net";
  },
  _connect: Deno.net.connect,
  _listen: Deno.net.listen,
  connectTls: Deno.connectTls,
  createUdpSocket: Deno.net.createUdpSocket,
  createUnixSocket: Deno.net.createUnixSocket,
  createTcpListener: Deno.net.createTcpListener,
  createUnixListener: Deno.net.createUnixListener,
  createWebSocketStream: Deno.net.createWebSocketStream,
  createHttpStream: Deno.net.createHttpStream,

  connect: (opts) => {
    let onConnect = () => {};
    let conn = Deno.net.connect(opts)
    .then((socket) => {
      onConnect(socket);
    }).catch((err) => {
      if (onConnect) {
        onConnect(null, err);
      } else {
        throw err;
      }
    });
    return (cb) => { onConnect = cb; return conn; };
  },

  listen: (opts) => {
    let onListen = () => {};
    let listener = Deno.net.listen(opts);
    (async function(){
      for await (conn of listener) {
        onListen(conn, listener);
      }
    })();
    return (cb) => { onListen = cb; return listener; };
  },

  fetch: Deno.fetch.fetch,
}));