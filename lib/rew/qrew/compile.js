const { deserializeData, serializeData, gen_key } = require("../misc/bin");




module.exports.to_qrew = (fileContent, secret) => {
  return serializeData(Buffer.from(fileContent), gen_key(secret));
}

module.exports.from_qrew = (fileBuffer, secret) => {
  return deserializeData(Buffer.from(fileBuffer), gen_key(secret));
}