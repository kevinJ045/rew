import "#std.encoding";
import "#std.types";
using namespace rew::types;
using namespace rew::io::out;

print &112
print rew::ptr::deref &11, 'i32'
print rew::ptr::deref &true, 'bool'
print rew::ptr::deref &"hello", 'string'


name = "MyName"
name_ptr = &name
print *name_ptr!
*name_ptr = "SomeDude"
print *name_ptr as str

ptr_int = &112

g = 1 & 3

h = 1 * 4

print 1 & g & *ptr_int
*ptr_int = 39
print 1 * g * *ptr_int
