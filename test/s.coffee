
import "#std.ffi!";
using namespace rew::ns()

ffi_type('i32', 'i32') add_ffi = -> 'i32'

{ add } = ffi::open '/home/makano/workspace/testing/rustyscript/test_ffi/target/release/libmy_add_lib.so', add: add_ffi

print add 1, 2