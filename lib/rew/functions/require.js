const fs = require('fs');
const path = require('path');

module.exports.customRequire = function customRequire(modulePath, filePath) {
	const resolvedPath = resolveModulePath(modulePath, filePath);
	if(!resolvedPath) throw new Error('Module '+modulePath+' not found');
	return require(resolvedPath);
};

function resolveModulePath(modulePath, filePath) {
	if (modulePath.startsWith('./') || modulePath.startsWith('../') || path.isAbsolute(modulePath)) {
		return path.resolve(modulePath);
	}

	const paths = module.constructor._nodeModulePaths(path.dirname(filePath));
	for (const basePath of paths) {
		const fullPath = path.join(basePath, modulePath);
		if (fs.existsSync(fullPath + '.js')) {
			return fullPath + '.js';
		}
		if (fs.existsSync(fullPath + '.json')) {
			return fullPath + '.json';
		}
		
		if (fs.existsSync(fullPath) && fs.statSync(fullPath).isDirectory()) {
			return searchInPath(fullPath);
		}

		const rootPath = modulePath.split('/').shift();
		const halfFullPath = path.join(basePath, rootPath);
		if (fs.existsSync(halfFullPath) && fs.statSync(halfFullPath).isDirectory()) {
			return searchInPath(halfFullPath, ['.'].concat(fullPath.split('/').slice(1)).join('/'));
		}
	}
}

function searchInPath(fullPath, exportses){
	const packageJsonPath = path.join(fullPath, 'package.json');
	if (fs.existsSync(packageJsonPath)) {
		const packageJson = require(packageJsonPath);
		let main = packageJson.main || 'index.js';
		if(exportses){
			if(packageJson.exports){
				if(exportses in packageJson.exports) main = packageJson.exports[exportses];
			}
		}
		if(typeof main == "object"){
			if(Array.isArray(main)) main = main[0].require;
			else main = main.require;
		}
		const mainPath = path.join(fullPath, main);
		if (fs.existsSync(mainPath)) {
			return mainPath;
		}
	}
	const indexPath = path.join(fullPath, 'index.js');
	if (fs.existsSync(indexPath)) {
		return indexPath;
	}
}