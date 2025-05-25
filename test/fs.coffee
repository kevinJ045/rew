import "#std.fs"

something = null
f = ->
  something = await rew::fs::read "./ffi.coffee"
  rew::io::out.print something

f()

channel = rew::channel::new 1000, ->
  if something then channel.stop()