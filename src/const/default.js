const { cenum } = require("../models/enum");
const { struct } = require("../models/struct");
const emitter = require("../functions/emitter");
const future = require("../functions/future");
const sleep = require("../functions/sleep");
const { match } = require("../functions/match");
const { map } = require("../functions/map");
const { typex, typeis, typedef, typei } = require("../functions/types");

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
  print: function (...arguments) {
    return console.log(...arguments);
  },
};
