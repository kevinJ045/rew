const future = require("./future");

module.exports = async function sleep(time) {
  return new Promise((r) => setTimeout(r, time));
};
