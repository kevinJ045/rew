const { cenum } = require("../models/enum");
const { struct } = require("../models/struct");
const emitter = require("../functions/emitter");
const future = require("../functions/future");
const sleep = require("../functions/sleep");
const { match } = require("../functions/match");
const { map } = require("../functions/map");
const { typex, typeis, typedef, typei } = require("../functions/types");
const { isEmpty, clone, deepClone, merge, uniqueId, compose, curry } = require("../functions/core");
const { print } = require("../functions/stdout");

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

  isEmpty,
  clone,
  deepClone,
  merge,
  uniqueId,
  compose,
  curry,

  print
};
