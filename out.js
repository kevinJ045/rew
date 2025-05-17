rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/ffi.coffee", function(globalThis){
            with (globalThis) {
              var ins, init_socket, recv_message, send_message, loopm;
rew.prototype.mod.prototype.find(module, "#std.ffi!")
using(namespace(rew.prototype.ns()))

ins = instantiate(class {
  init_socket = rew.prototype.ffi.prototype.typed( function() { return 'i32' })
  recv_message = rew.prototype.ffi.prototype.typed(ffi.prototype.ptr, 'usize', function() { return 'i32' })
  send_message = rew.prototype.ffi.prototype.typed(ffi.prototype.ptr, ffi.prototype.pre('i32', Number))
});

({ init_socket, recv_message, send_message } = ffi.prototype.open('/home/makano/workspace/testing/rustyscript/test_ffi/target/release/libmy_add_lib.so', ins))

init_socket()

loopm = function() {
  var buf;
  buf = new Uint8Array(40960)
  if (recv_message(rew.prototype.ptr.prototype.of(buf), 40960) > 0) {
    print(rew.prototype.encoding.prototype.bytesToString(buf))
  }
  return setTimeout(loopm, 1)
}

loopm()



setTimeout(() => send_message(rew.prototype.ptr.prototype.of(rew.prototype.encoding.prototype.stringToBytes("Hello from JS"))), 1000)


            }
            return globalThis.module.exports;
          }, );(function(module){

//declare* "=ffi_type" = rew::ffi::typed;

})({filename: "#std.ffi"});
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/ffi.coffee');