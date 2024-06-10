// run.js
const { run } = require('../main');

function exec(filePath, argv, options = {}) {
	return run(filePath, { argv, ...options })?.context?.module?.imports || [];
}

module.exports = { execRewFile: exec };
