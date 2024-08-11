const deasync = require("deasync");


module.exports.wait = (...args) => {
  const fn = args.shift();
  if(typeof fn !== "function" && typeof fn?.then != "function" && !fn?._isfuture) throw new TypeError("The first argument must be a function, future or a Promise to use wait.");
  const df = deasync(async (cb) => {
    let result = typeof fn == "function" ? fn(...args) : fn;

    if(result._isfuture){
      result = result.wait();
    }

    if(typeof result.then == "function"){
      result
        .then(d => cb(null, d))
        .catch(d => cb(d));
    } else {
      cb(null, result);
    }
  });
  return df();
}