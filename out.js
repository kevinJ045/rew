rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/s.coffee", function(context){
            with (context) {
              
var sayhello, g, h, i, j, f;
const imported = rew.prototype.mod.prototype.find(module,  "./d.coffee")

rew.prototype.mod.prototype.find(module, "#std.ghost")


using(namespace(rew.prototype.ns()))
print(imported)
// print magnificento

sayhello = function(...a) {
  return print("hello", ...a)
}

g = sayhello() 
// means g = sayhello()
h = sayhello("a)", 1 + 2)
// means h = sayhello("a", "v")
i = sayhello( "j")
// means i = sayhello("j")
j = sayhello("a", "b", "c")
// means j = sayhello("a", "b", "c")

f = async function() {
  return print(await rew.prototype.fs.prototype.read('./d.coffee', { binary: true }))
}

f()
// print rew::encoding::toBase64 await rew::fs::read './d.coffee', { binary: true }

            }
            return context.module.exports;
          });rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/d.coffee", function(context){
            with (context) {
              var sayhello, g;
rew.prototype.mod.prototype.find(module, "./e.coffee")
using(namespace(rew.prototype.ns()))

print(module.options)
print("Imported Script from")


sayhello = function(...a) {
  return print("hello", ...a)
}

g = sayhello( "")

module.exports.hello = "shhshsh"

            }
            return context.module.exports;
          });rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/e.coffee", function(context){
            with (context) {
              
var x;
x = 1

rew.prototype.io.prototype.out.print("eeeeeeeeeeeeeeeeeeeeeee")




            }
            return context.module.exports;
          });
rew.prototype.io.prototype.out.print('hi')
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/s.coffee');