const path = require('path'); // Import the 'path' module
const fs = require('fs'); // Import the 'path' module

module.exports.findAppPath = (currentDir = __dirname) => {
	// Check if app.yaml exists in the current directory
	const appYamlPath = path.join(currentDir, 'app.yaml');
	if (fs.existsSync(appYamlPath)) {
		return currentDir;
	}

	// If not found, move up a directory level
	const parentDir = path.dirname(currentDir);

	// Check if we reached the root directory
	if (parentDir === currentDir) {
		return null; // Not found
	}

	// Recursively call the function on the parent directory
	return module.exports.findAppPath(parentDir);
};
