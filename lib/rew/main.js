const { runPath } = require('./modules/runtime');

module.exports.run = function (filepath, options = {}, custom_context = {}) {
	return runPath(filepath, options, custom_context);
};
