rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/s.coffee", function(context){
            with (context) {
              
var add_ffi, add;
rew.prototype.mod.prototype.find(module, "#std.ffi!");
using(namespace(rew.prototype.ns()))

add_ffi = rew.prototype.ffi.prototype.typed('i32', 'i32', function() { return 'i32' });

({ add } = ffi.prototype.open('/home/makano/workspace/testing/rustyscript/test_ffi/target/release/libmy_add_lib.so', {add: add_ffi}))

print(add(1, 2))
            }
            return context.module.exports;
          }, );

//declare* "=ffi_type" = rew::ffi::typed;


rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/s.coffee');