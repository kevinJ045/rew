import "#std.shell";
import "#std.encoding";
using namespace rew::ns();


# channel = rew::channel::new 1, -> 
f = =>
  child = await rew::shell::exec 'echo hii'
  print rew::encoding::bytesToString await child.output
f()
