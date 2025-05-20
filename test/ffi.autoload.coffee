import "#std.ffi!"
using namespace rew::ns()

symbols = ffi::autoload '/home/makano/.rew/apps/rew_bindgen_test/target/release/librew_bindgen_test.so'

symbols.say_hello()



print symbols.add(100, 10)

cb = -> print "hi"

symbols.call_every_second(
  rew::ptr::cb(cb, [], 'void').pointer
)

s = -> setTimeout s, 1
s()