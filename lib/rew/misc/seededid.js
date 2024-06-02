module.exports.seededID = function (seed) {
  const charCodes = seed.split("").map((char) => char.charCodeAt(0));

  let result = "";
  let sum = 0;

  for (let i = 0; i < charCodes.length; i++) {
    sum += charCodes[i];
    result += String.fromCharCode(((charCodes[i] + sum) % 26) + 97);
  }

  return result.slice(0, 12);
};
