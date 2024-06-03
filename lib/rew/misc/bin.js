

const msgpack = require('tiny-msgpack');
const crypto = require('crypto');

const algorithm = 'aes-256-ctr';

const encrypt = (data, encryptionKey) => {
  const iv = crypto.randomBytes(16);
  const cipher = crypto.createCipheriv(algorithm, Buffer.from(encryptionKey, 'hex'), iv);
  const encrypted = Buffer.concat([cipher.update(data), cipher.final()]);
  return Buffer.concat([iv, encrypted]);
};

const decrypt = (data, encryptionKey) => {
  const iv = data.slice(0, 16);
  const encryptedData = data.slice(16);
  const decipher = crypto.createDecipheriv(algorithm, Buffer.from(encryptionKey, 'hex'), iv);
  const decrypted = Buffer.concat([decipher.update(encryptedData), decipher.final()]);
  return decrypted;
};

module.exports.serializeData = (data, encryptionKey) => {
  return encrypt(msgpack.encode(data), encryptionKey);
};

module.exports.deserializeData = (buffer, encryptionKey) => {
  return msgpack.decode(decrypt(buffer, encryptionKey));
};

module.exports.gen_key = (secret) => {
  if (secret) {
    return crypto.createHash('sha256').update(secret).digest('hex');
  } else {
    return crypto.randomBytes(32).toString('hex');
  }
}