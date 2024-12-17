const jsYaml = require('js-yaml');
const { findAppPath } = require('./findAppPath');
const path = require('path');
const { readFileSync } = require('fs');
const { straceLog } = require('./strace');

module.exports.findAppInfo = function (filepath) {
	const appPath = findAppPath(path.dirname(filepath));
	straceLog('findAppInfo() for', filepath);
	if (appPath) {
		const config = jsYaml.load(readFileSync(path.join(appPath, 'app.yaml')));
		straceLog('==> INFO Found config at:', '"' + appPath + '"');
		straceLog('==> INFO App Package:', '"' + config?.manifest?.package + '"');
		return {
			path: appPath,
			config,
		};
	}
	return null;
};
