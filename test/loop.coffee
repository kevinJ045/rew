import "#std.ffi";

# closure = rew::ptr::fn [], 'void', -> loop rew::io::out.print 'hi'

# rew::channel::loopC rew::ptr::val closure

# rew::channel::new()