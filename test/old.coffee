
import "#std.ffi!";
import imported, { hello } from "./d.coffee"
import * as smn from "agamada.domago"
import * as tst from "agamada.domago/test"



using namespace rew::ns

# print 'ffi: ', rew::ffi

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

# print rew::encoding::toBase64 await rew::fs::read './d.coffee', { binary: true }
