import "#std.fs"

something = null

using namespace rew::ns
# print rew::fs::read "./exec.coffee!"
export main = ->
  print rew::fs::open "./ffi.coffee", write: true
  print rew::fs::sha './ffi.coffee'
  print rew::fs::read "./ffi.coffee"



