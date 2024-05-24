


module.exports.map = function map(...args) {
  if (args.length % 2 !== 0) {
    throw new Error('Arguments must be in key-value pairs');
  }

  const result = new Map();
  for (let i = 0; i < args.length; i += 2) {
    const key = args[i];
    const value = args[i + 1];
    result.set(key, value);
  }

  return result;
};