# import * as imported from "./d.coffee"
using namespace rew::ns()
f = -> print rew::encoding::toBase64 await rew::fs::read './d.coffee', { binary: true }
f()