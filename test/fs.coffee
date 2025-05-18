import "#std.fs"

f = -> rew::io::out.print await rew::fs::read "./ffi.coffee"

f()