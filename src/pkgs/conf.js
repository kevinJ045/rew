const fs = require('fs');
const jsYaml = require('js-yaml');
const path = require('path');

const CONFIG_PATH = path.resolve(process.env.HOME, '.config/rew');

const createPackageRoot = (packageName) => {
	const rootPath = path.join(CONFIG_PATH, packageName);
	fs.mkdirSync(rootPath, { recursive: true });
	return rootPath;
};

module.exports = (context) => ({
	create: (packageName) => {
		const rootPath = createPackageRoot(packageName);

		const conf = {};

		const setData = (optionCenter, key, value) => {
			conf[optionCenter.name][key] = value;
			fs.writeFileSync(optionCenter.root, jsYaml.dump(conf[optionCenter.name]));
			return true;
		};

		const removeData = (optionCenter, key) => {
			delete conf[optionCenter.name][key];
			fs.writeFileSync(optionCenter.root, jsYaml.dump(conf[optionCenter.name]));
			return true;
		};

		const getData = (optionCenter, key) => {
			return conf[optionCenter.name][key];
		};

		const createOptionCenter = (name, defaults = {}) => {
			const optionRoot = path.join(rootPath, name + '.yaml');
			if (!fs.existsSync(path.dirname(optionRoot))) fs.mkdirSync(path.dirname(optionRoot), { recursive: true });
			if (!fs.existsSync(optionRoot)) {
				fs.writeFileSync(optionRoot, jsYaml.dump(defaults));
				conf[name] = defaults;
			} else {
				conf[name] = jsYaml.load(fs.readFileSync(optionRoot, { encoding: 'utf-8' }));
			}

			const optionCenter = {
				root: optionRoot,
				name,
				package: packageName,
			};

			return {
				get: (key) => getData(optionCenter, key),
				set: (key) => setData(optionCenter, key),
				remove: (key) => removeData(optionCenter, key),
				reset: () => fs.writeFileSync(optionCenter.root, jsYaml.dump(defaults)) && (conf[name] = defaults),
				...optionCenter,
			};
		};

		const defaultCenter = createOptionCenter('_default');

		return {
			optionCenter: createOptionCenter,
			set: (key, value) => defaultCenter.set(key, value),
			get: (key, value) => defaultCenter.get(key, value),
			remove: (key, value) => defaultCenter.remove(key, value),
			root: rootPath,
			package: packageName,
		};
	},
});
