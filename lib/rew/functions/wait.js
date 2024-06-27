const deasync = require("deasync");


module.exports.wait = (...args) => {
  const fn = args.shift();
  if(typeof fn !== "function") throw new TypeError("The first argument must be a function to use wait.");
  const df = deasync(async (cb) => {
    fn(...args)
      .then(d => cb(null, d))
      .catch(d => cb(d));
  });
  return df();
}