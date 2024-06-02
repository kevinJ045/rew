const { compileFile } = require('./modules/compiler');
const { exec, runPath } = require('./modules/runtime');
const fs = require('fs');
const { imp } = require('./functions/import');
const { FILES } = require('./const/files');

module.exports.run = function (filepath, options = {}, custom_context = {}) {
	FILES.forEach((file) => {
		if (fs.existsSync(file.path)) return;
		if (file.content) {
			fs.writeFileSync(file.path, file.content);
		} else {
			fs.mkdirSync(file.path, { recursive: true });
		}
	});
	return runPath(filepath, options, custom_context);
};
