import "#std.ffi!"
import "#std.types!";
import "#std.conf"
import "#std.encoding"
import "#std.fs"
import "#std.os"
import "#std.path"
import "#std.shell"
import "#std.threads"
import "#std.http"

#declare* "@print" = rew::io::out.print;
#declare* "@printf" = rew::io::out.printf;
#declare* "@in" = rew::io::out.input;
#declare* "@exec" = rew::shell::exec;
#declare* "@spawn" = rew::shell::spawn;
