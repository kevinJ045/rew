import "#std.ffi!";
import "#std.threads!";
using namespace rew::ns;


lib = rew::ffi::threaded "/home/makano/workspace/rew.smn/demo.so", {
  add: {
    result: 'i32',
    parameters: ['i32', 'i32']
  }
  ssmmtt: {
    result: 'buffer',
    parameters: ['buffer']
  }
}

lib.ssmmtt("ss").then(print)
lib.add(12, 12).then(print)