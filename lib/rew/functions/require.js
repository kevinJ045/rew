const fs = require('fs');
const path = require('path');
const { execOptions } = require('../const/opt');
const { wait } = require('./wait');

const cahcedRequires = {};

const doImp = (path) => wait(async () => await import(path));

module.exports.customRequire = function customRequire(modulePath, filePath, esm) {
	let pathname = modulePath;
	if (modulePath.startsWith('./') || modulePath.startsWith('../') || path.isAbsolute(modulePath)) {
		pathname = path.resolve(modulePath);
	}
	if(cahcedRequires[pathname]) {
		return cahcedRequires[pathname];
	}
	const resolvedPath = resolveModulePath(modulePath, filePath);
	if(!resolvedPath) throw new Error('Module '+modulePath+' not found');

	let isEsm = esm || false;
	const get_file = () => cahcedRequires[resolvedPath] ? cahcedRequires[resolvedPath] : execOptions.useImport || isEsm ? doImp(resolvedPath) : require(resolvedPath);

	let r;

	try{
		r = get_file();
	} catch(e){
		if(e.code === 'ERR_REQUIRE_ESM') {
			isEsm = true;
			console.log('Trying with esm');
			r = get_file();
		} else {
			throw e;
		}
	}
	
	if(!cahcedRequires[resolvedPath]) cahcedRequires[resolvedPath] = r;
	if(!cahcedRequires[pathname]) cahcedRequires[pathname] = r;
	return r;
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
		} else if (fs.existsSync(fullPath) && fs.statSync(fullPath).isFile()) {
			return fullPath;
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
			if(Array.isArray(main)) main = execOptions.useImport ? main[0].import || main[0].require : main[0].require || main[0].import;
			else main = execOptions.useImport ? main.import || main.require : main.require || main.import;
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