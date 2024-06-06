// run.js
const { run } = require('../main');

function exec(filePath, argv) {
	return run(filePath, { argv })?.context?.module?.imports || [];
}

module.exports = { execRewFile: exec };
