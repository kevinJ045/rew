import "#std.http";
import "#std.net";
import "#std.encoding";

rew::http::withOptions(port: 3000) (req) ->
  rew::http::Response::new("Hello, World!")

rew::net::listen(hostname: "0.0.0.0", port: 4444) (conn) ->
  try
    buffer = new Uint8Array(1024);
    n = await conn.read(buffer)
    rew::io::out.print "Data received from connection", rew::encoding::bytesToString(buffer.subarray(0, n))
  catch err
    rew::io::out.print "Error reading from connection:", err

  await conn.write(rew::encoding::stringToBytes("Hello from the server!"))

rew::channel::timeout 1000, ->
  rew::net::fetch("http://localhost:3000")
  .then (res) ->
    rew::io::out.print "Response received:", await res.text()
  .catch (err) ->
    rew::io::out.print "Error fetching:", err
  
  rew::net::connect({ hostname: "127.0.0.1", port: 4444 }) (conn) ->
    message = "Hello from client!";
    await conn.write(rew::encoding::stringToBytes(message));
    buffer = new Uint8Array(1024);
    n = await conn.read(buffer)
    rew::io::out.print "Data received from connection", rew::encoding::bytesToString(buffer.subarray(0, n))
    conn.close()
