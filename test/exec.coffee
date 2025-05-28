import "#std.shell";
using namespace rew::ns;

l = rew::shell::spawn 'sleep 40'
l.status()
print l.pid
rew::shell::kill l.pid
print 'output:', rew::shell::sync 'echo hii'
