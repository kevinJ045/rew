
import imported, { hello } from "./d.coffee"
import * as smn from "agamada.domago"
import * as tst from "agamada.domago/test"
using namespace rew::ns()

my_linked_fn = ->->->->->->->->-> '=============>> linked fn result'

print my_linked_fn()()()()()()()()()

#d=eclare "=default" = ONLYIF(prev="export") rew::mod::export; 
#de=clare "=export" = rew::mod::export;
#dec=lare "export" = ONLYIF(next="default"); 
f = 1
export { f }
# export default f = 1


export default f

#ifdef Garmenanarnarnaruman
print 'Chaugemagangemaug', 'Gaugemachangemaug', 'Garmanarnar'
#endif

print smn
print tst
print imported, hello
# print magnificento

print module.app

print ffi

class MyClass
  name = "Context"
  
  ffi_type init = -> rew::ffi::ptr
  ffi_type(ffi::ptr) lsusb = -> rew::ffi::void
  ffi_type(ffi::ptr, 'u16', 'u16') open = -> rew::ffi::ptr
  ffi_type(ffi::ptr) dealloc = -> rew::ffi::void

Context = ffi::open '/home/makano/workspace/testing/rustyscript/deno_bindgen/example/target/debug/libdeno_usb.so', MyClass

ctx = new Context
ctx.lsusb();

f = ->
  print await read './d.coffee', { binary: true }

f()
# print rew::encoding::toBase64 await rew::fs::read './d.coffee', { binary: true }
