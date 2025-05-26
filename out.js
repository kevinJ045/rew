rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/s.coffee", function(globalThis){
with (globalThis) {
  const s = rew.prototype.mod.prototype.find(module,  "./d.coffee")

using(namespace(rew.prototype.ns()))
rew.prototype.io.prototype.out.print(typeof s.default, new s.default)
rew.prototype.io.prototype.out.print(typeof s.GGG, new s.GGG)
rew.prototype.io.prototype.out.print(rew.prototype.process.prototype.args)
rew.prototype.io.prototype.out.print(s)
}
return globalThis.module.exports;
}, ["app://test.app/s"]);rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/d.coffee", function(globalThis){
with (globalThis) {
  var sayhello, g, hello, name;
rew.prototype.mod.prototype.find(module, "./e.coffee")
using(namespace(rew.prototype.ns()))

print(module.options)
print("Imported Script from")


sayhello = function(...a) {
  return print("hello", ...a)
}

sayhello(g = "")

module.exports.default =  class Gangarmada {
  mmm = "mmm"
}
module.exports.GGG =  class GGG {
  name = "sss"
}
module.exports.hello =  hello = "shhshsh"
module.exports.name =  name = "jjj"
}
return globalThis.module.exports;
}, ["app://test.app/d"]);rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/e.coffee", function(globalThis){
with (globalThis) {
  
var x;
x = 1

rew.prototype.io.prototype.out.print("eeeeeeeeeeeeeeeeeeeeeee")




}
return globalThis.module.exports;
}, ["app://test.app/e"]);
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/s.coffee');