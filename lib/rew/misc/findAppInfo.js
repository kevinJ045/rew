const jsYaml = require('js-yaml');
const { findAppPath } = require('./findAppPath');
const path = require('path');
const { readFileSync } = require('fs');
const { straceLog } = require('./strace');

module.exports.findAppInfo = function (filepath) {
	const appPath = findAppPath(path.dirname(filepath));
	straceLog('FINDAPPINFO() for', filepath);
	if (appPath) {
		const config = jsYaml.load(readFileSync(path.join(appPath, 'app.yaml')));
		straceLog('==> FOUND CONFIG AT', appPath);
		straceLog('==> APP PACKAGE', config?.manifest?.package);
		return {
			path: appPath,
			config,
		};
	}
	return null;
};
