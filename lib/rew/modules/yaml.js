const yaml = require('js-yaml');
const path = require('path');
const { getFile } = require('./fs');

function yamlFile(file) {
	const schema = new yaml.Schema([
		new yaml.Type('!import', {
			kind: 'scalar',
			construct: (p) => importYaml(path.resolve(path.dirname(file.path), p)),
		}),
		new yaml.Type('!int', {
			kind: 'scalar',
			construct: (data) => parseInt(data),
		}),
		new yaml.Type('!float', {
			kind: 'scalar',
			construct: (data) => parseFloat(data),
		}),
		new yaml.Type('!bool', {
			kind: 'scalar',
			construct: (data) => (data == 'true' ? true : false),
		}),
	]);

	return file.content.startsWith('---') ? yaml.loadAll(file.content, { schema })[0] : yaml.load(file.content, { schema });
}

const importYaml = (module.exports.importYaml = function importYaml(filepath, file) {
	if (!file) {
		file = getFile(filepath);
	}
	return yamlFile(file);
});
