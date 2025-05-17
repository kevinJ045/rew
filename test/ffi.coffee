import "#std.ffi!"
using namespace rew::ns()

ins = instantiate class
  ffi_type() init_socket = -> 'i32'
  ffi_type(ffi::ptr, 'usize') recv_message = -> 'i32'
  ffi_type(ffi::ptr) send_message = ffi::pre 'i32', Number

{ init_socket, recv_message, send_message } = ffi::open '/home/makano/workspace/testing/rustyscript/test_ffi/target/release/libmy_add_lib.so', ins

init_socket()

loopm = ->
  buf = new Uint8Array(40960)
  if recv_message(rew::ptr::of(buf), 40960) > 0
    print rew::encoding::bytesToString(buf)
  setTimeout(loopm, 1)

loopm()



setTimeout(() => send_message(rew::ptr::of rew::encoding::stringToBytes("Hello from JS")), 1000)

