const path = require('path');

const CONFIG_PATH = path.resolve(process.env.HOME || process.env.USERPROFILE, '.local/share/rew');
module.exports.CONFIG_PATH = CONFIG_PATH;
