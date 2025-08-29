import ddd from "./test.txt"
import jjj from "./some.json"
import yyy from "./app.yaml"

rew::io::out.print ddd, jjj, yyy
f = ->
  rew::io::out.print await imp "./d.coffee"

f()