const path = require('path'); // Import the 'path' module
const fs = require('fs'); // Import the 'path' module
const { straceLog } = require('./strace');

module.exports.findAppPath = (currentDir = __dirname) => {
	straceLog('findApp() for', '"' + currentDir + '"');
	const appYamlPath = path.join(currentDir, 'app.yaml');
	if (fs.existsSync(appYamlPath)) {

		straceLog('==> INFO found path', `"${appYamlPath}"`);
		return currentDir;
	}

	const parentDir = path.dirname(currentDir);

	if (parentDir === currentDir) {
		return null;
	}

	return module.exports.findAppPath(parentDir);
};
