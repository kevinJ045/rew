const axios = require("axios")


module.exports.req = async (url, options = {}) => {
  return await axios
    .get(url, { ...options });
}