import "#std.shell";
using namespace rew::ns();


# channel = rew::channel::new 1, -> 
f = =>
  child = await rew::shell::spawn 'echo', ['hello']
  print child
  print await child.read()
f()
