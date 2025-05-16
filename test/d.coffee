import "./e.coffee"
using namespace rew::ns()

print module.options
print "Imported Script from"


sayhello = (...a) ->
  print "hello", ...a

sayhello g = ""

module.exports.hello = "shhshsh"
