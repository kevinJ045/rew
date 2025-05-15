# import * as imported from "./d.coffee"
using pvt namespace rew::ns(), ->
  print rew::encoding::toBase64 await rew::fs::read './d.coffee', { binary: true }