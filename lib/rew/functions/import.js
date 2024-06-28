const path = require('path');
const { getFile, file } = require('../modules/fs');
const { importYaml } = require('../modules/yaml');
const { findPackage, getPackage } = require('../pkgs/pkgs');
const { existsSync, readFileSync } = require('fs');
const conf = require('../pkgs/conf');
const jsYaml = require('js-yaml');
const { execOptions } = require('../const/opt');
const { REW_FILE_TYPE } = require('../const/ext');

const cachedFiles = [];
module.exports.cleanCache = () => {
	while(cachedFiles.length) cachedFiles.pop();
};
const lookUpInOtherApps = (fullPath) => {
	const con = conf({});
	const name = fullPath.indexOf('/') ? fullPath.split('/')[0] : fullPath;
	let dpath = fullPath.indexOf('/') ? fullPath.split('/')[1] : '';
	let ppath = path.join(con.CONFIG_PATH, name, 'app');
	if (!existsSync(ppath)) return null;
	const config = jsYaml.load(readFileSync(path.join(ppath, 'app.yaml'), 'utf-8'));
	if (!dpath) {
		dpath = config.exec.entry;
	}
	if(config.private == true) return null;
	if(dpath in config.exec) dpath = config.exec[dpath];
	const pepath = path.join(ppath, dpath);
	if(Array.isArray(config.manifest.private)){
		if(config.private.find(f => pepath == path.join(ppath, f))) return null;
	}
	if (existsSync(pepath)) return pepath;
	else return null;
};

module.exports.imp = function (runPath, context) {
	return function (filename, options = {}) {
		if (!options) options = {};
		let type = options.type || filename.endsWith('.coffee') ? 'coffee' : REW_FILE_TYPE.TYPE;
		let exports,
			ispkg = findPackage(filename);

		if (filename.startsWith('@') && context.app) {
			filename = filename.replace('@', context.app.path);
		}

		let filepath = path.resolve(path.dirname(context.module.filepath), filename);
		if(path.extname(filepath) == '.qrew') options.qrew = true;

		const lookUp = () => {
			const otherPath = lookUpInOtherApps(filename);
			if (!otherPath) throw new Error('Module "' + filename + '" not found');
			else filepath = otherPath;
		};

		const foundCache = cachedFiles.find((f) => f.filepath == filepath);

		if (!ispkg && foundCache) {
			exports = foundCache.exports;
		}

		if (!ispkg && !existsSync(filepath) && !foundCache) {
			if (Array.isArray(execOptions.resolveExtensions) && execOptions.resolveExtensions.length) {
				const resolve = execOptions.resolveExtensions.find((ext) =>
					typeof ext == 'string' ? existsSync(filepath + ext) : existsSync(filepath + (ext.ext || '')),
				);
				if (resolve) {
					filepath += typeof resolve == 'string' ? resolve : resolve.ext;
					if (typeof resolve == 'object' && resolve.options) {
						if (resolve.options.type) type = resolve.options.type;
						for (let i in resolve.options) options[i] = resolve.options[i];
					}
				} else lookUp();
			} else lookUp();
		}

		const exec = (coptions = {}) => {
			const r = runPath(
				filepath,
				{
					import: options,
					main: false,
					useContext: execOptions.sharedContext == false ? false : !(options.context && options.context == 'new'),
					...coptions,
					as: options.as == 'main' ? (context.module.main ? 'main' : 'parent') : options.as == 'parent' ? 'parent' : 'child',
					fromMain: context.module.main,
					qrew: options.qrew,
					package: context.app ? context.app.config.package : path.basename(filepath)
				},
				execOptions.sharedContext == false ? {} : options.context && options.context == 'new' ? {} : context,
			);
			if(r instanceof Promise){
				return new Promise((resolve) => r.then(c => resolve(c.context.module.exports)));
			}
			return r.context.module.exports;
		}

		if (ispkg) {
			const pkg = getPackage(filename)(context, options);
			exports = pkg._onImport ? pkg._onImport() : pkg;
			if(options.useDefaultForPackages) exports = { default: exports };
		} else if (foundCache) {
		} else if (type == REW_FILE_TYPE.TYPE || type == "coffee") {
			exports = exec({});
		} else if (type == 'js') {
			exports = exec({ compile: false });
		} else if (type == 'yaml' || type == 'json' || type == 'text') {
			const f = getFile(filepath);
			if (type == 'yaml') {
				exports = importYaml(f.path, f);
			} else if (type == 'json') {
				try {
					exports = JSON.parse(f.content);
				} catch (e) {
					exports = {};
				}
			} else {
				exports = f.content;
			}
		}

		if (options.save && (type == 'js' || type == REW_FILE_TYPE.TYPE || type == "coffee")) {
			if (typeof options.save == 'string') context[options.save] = exports[i];
			else
				for (let i in exports) {
					context[i] = exports[i];
				}
		}

		// Hehe, i just had an idea for a
		// descriptive code
		// you put them in comment blocks
		// and name it something
		// then you can simple see 
		// which part of a code contains a certain
		// task. cool right?
	
		//**  If is not package, post exec section
		/**/ if (!ispkg) context.module.imports.push(filepath);
		/**/ if (!ispkg) cachedFiles.push({ filepath, exports });
		//** 

		//**  Mock imports section	
		/**/ if(!exports) exports = options.mock;
		/**/ if(options.mock === null) return null;
		//** 

		return exports;
	};
};
