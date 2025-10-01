import "#std.ffi!";
import "#std.types!";
import "#std.threads!";
using namespace rew::ns;

struct myStruct = {
  dd: int.of('i32')
}

lib = rew::ffi::threaded "/home/makano/workspace/rew.smn/demo.so", {
  add: {
    result: 'i32',
    parameters: ['i32', 'i32']
  }
  ssmmtt: {
    result: 'buffer',
    parameters: ['buffer']
  }
  dofn: {
    result: 'i32',
    parameters: ['function']
  }
  doStruct: {
    result: myStruct::type,
    parameters: [myStruct::type]
  }
}

fn = ptr_fn ['i32'], 'i32', (e) ->
  print e
  return 28

st = myStruct::new dd: 10

lib.ssmmtt("ss").then((s) => print ptr_string s)
lib.add(12, 12).then(print)
lib.dofn(fn.pointer).then(print)
lib.doStruct(st.toBuff()).then((b) => print(myStruct::fromBuff(b)))