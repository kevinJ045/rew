const fs = require('fs');
const jsYaml = require('js-yaml');
const path = require('path');
const { CONFIG_PATH } = require('../const/config_path');
const { seededID } = require('../misc/seededid');

const createPackageRoot = (packageName) => {
	const rootPath = path.join(CONFIG_PATH, packageName);
	fs.mkdirSync(rootPath, { recursive: true });
	return rootPath;
};

module.exports = (context) => ({
	CONFIG_PATH,
	_onImport() {
		if (context.app) {
			return this.create(context.app.config.manifest.package);
		} else {
			return this.create(seededID(path.basename(context.module.filepath).replace(/[-_/\.]/g, '')));
		}
	},
	create: (packageName) => {
		const rootPath = createPackageRoot(packageName);

		const conf = {};

		const dumpYaml = (val) => {
			if (JSON.stringify(val) == '{}') return '';
			else return jsYaml.dump(val);
		};

		const setData = (optionCenter, key, value) => {
			conf[optionCenter.name][key] = value;
			fs.writeFileSync(optionCenter.root, dumpYaml(conf[optionCenter.name]));
			return true;
		};

		const removeData = (optionCenter, key) => {
			delete conf[optionCenter.name][key];
			fs.writeFileSync(optionCenter.root, dumpYaml(conf[optionCenter.name]));
			return true;
		};

		const getData = (optionCenter, key) => {
			return conf[optionCenter.name][key];
		};

		const staticFile = (name, defaultValue = '') => {
			const fileRoot = path.join(rootPath, name);
			const exists = fs.existsSync(fileRoot);
			return {
				write(value, ifExists) {
					if (!fs.existsSync(path.dirname(fileRoot))) fs.mkdirSync(path.dirname(fileRoot), { recursive: true });
					if(ifExists && fs.existsSync(fileRoot)) return this;
					fs.writeFileSync(fileRoot, value || defaultValue);
					return this;
				},
				read(s){
					let file = fs.readFileSync(fileRoot);
					return typeof s == "string" ? file.toString() : 
					typeof s == "object" ? file.toJSON() : file;
				},
				fileRoot,
				exists,
			};
		};

		const createOptionCenter = (name, defaults = {}) => {
			const optionRoot = path.join(rootPath, name + '.yaml');
			if (!fs.existsSync(path.dirname(optionRoot))) fs.mkdirSync(path.dirname(optionRoot), { recursive: true });
			if (!fs.existsSync(optionRoot)) {
				conf[name] = defaults;
				fs.writeFileSync(optionRoot, dumpYaml(defaults));
			} else {
				conf[name] = jsYaml.load(fs.readFileSync(optionRoot, { encoding: 'utf-8' }));
			}

			const optionCenter = {
				root: optionRoot,
				name,
				package: packageName,
			};

			return {
				get: (key, defaultValue) => getData(optionCenter, key) ?? defaultValue,
				set: (key, value) => setData(optionCenter, key, value),
				remove: (key) => removeData(optionCenter, key),
				reset: () => fs.writeFileSync(optionCenter.root, dumpYaml(defaults)) && (conf[name] = defaults),
				getAll: (str = false) => (str ? dumpYaml(conf[name]) : conf[name]),
				...optionCenter,
			};
		};

		const defaultCenter = createOptionCenter('_default', { default: true });

		return {
			optionCenter: createOptionCenter,
			staticFile: staticFile,
			set: (key, value) => defaultCenter.set(key, value),
			get: (key, defaultValue) => defaultCenter.get(key, defaultValue),
			remove: (key) => defaultCenter.remove(key),
			root: rootPath,
			package: packageName,
			loadYaml: (file) => jsYaml.load(fs.readFileSync(file, { encoding: 'utf-8' }))
		};
	},
});
