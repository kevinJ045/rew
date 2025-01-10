const yaml = require('js-yaml');
const path = require('path');
const { getFile } = require('./fs');
const { withOut } = require('../functions/core');

function yamlFile(file, schemaProps) {
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
	].concat(
		(schemaProps || []).map((item) => new yaml.Type(item.key, {
			...withOut(item, 'key')
		}))
	));

	return file.content.startsWith('---') ? yaml.loadAll(file.content, { schema })[0] : yaml.load(file.content, { schema });
}

module.exports.yamlFile = yamlFile;

const importYaml = (module.exports.importYaml = function importYaml(filepath, file) {
	if (!file) {
		file = getFile(filepath);
	}
	return yamlFile(file);
});
