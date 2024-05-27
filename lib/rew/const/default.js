const { cenum } = require("../models/enum");
const { struct } = require("../models/struct");
const emitter = require("../functions/emitter");
const future = require("../functions/future");
const sleep = require("../functions/sleep");
const { match } = require("../functions/match");
const { map } = require("../functions/map");
const { typex, typeis, typedef, typei, int, float, num, str, bool } = require("../functions/types");
const { isEmpty, clone, deepClone, merge, uniqueId, compose, curry } = require("../functions/core");
const { print, input } = require("../functions/stdout");

module.exports = {
  cenum,
  struct,
  future,
  emitter,
  sleep,
  match,
  map,

  typex,
  typei,
  typeis,
  typedef,

  int,
  float,
  num,
  str,
  bool,

  isEmpty,
  clone,
  deepClone,
  merge,
  uniqueId,
  compose,
  curry,

  print,
  input
};
