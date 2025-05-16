
#declare* "=sayhello" = sayhello;
import * as imported from "./d.coffee"

import "#std.ghost"


using namespace rew::ns()
print imported
# print magnificento

sayhello = (...a) ->
  print "hello", ...a

sayhello g 
# means g = sayhello()
sayhello("a)", 1 + 2) h
# means h = sayhello("a", "v")
sayhello i = "j"
# means i = sayhello("j")
sayhello("a", "b") j = "c"
# means j = sayhello("a", "b", "c")

f = ->
  print await rew::fs::read './d.coffee', { binary: true }

f()
# print rew::encoding::toBase64 await rew::fs::read './d.coffee', { binary: true }
