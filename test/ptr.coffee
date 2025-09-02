using namespace rew::io::out;

name = "ff"
name_ptr = &name
print &112
print rew::ptr::deref &11
print *name_ptr
print rew::ptr::deref &true
print rew::ptr::deref &"hello"
# name_value = *name

ptr_int = &112

g = 1 & 3

h = 1 * 4

print 1 & g & *ptr_int
print 1 * g * *ptr_int