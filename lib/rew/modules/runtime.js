const vm = require('vm');
const { compileFile, compile } = require('./compiler');
const { prepareContext } = require('./context');
const { existsSync, readFileSync } = require('fs');
const { CONFIG_PATH } = require('../const/config_path');
const path = require('path');

const preScript = readFileSync(path.join(__dirname, '../const/pre-exec.js'), { encoding: 'utf-8' });

const exec = (module.exports.exec = function (code, context, original = '', filepath) {
	return vm.runInNewContext(code, context.do ? null : vm.createContext(context), {
		filename: filepath || context.module.filepath,
		lineOffset: (original.split('\n').length + preScript.split('\n').length) - code.split('\n').length,
		displayErrors: true,
	});
});

module.exports.runPath = function runPath(filepath, options = {}, custom_context = {}) {
	if(filepath.endsWith('.js')) options.type = 'js';
	if(filepath.endsWith('.coffee')) options.type = 'coffee';
	if(filepath.endsWith('.qrew')) options.type = 'qrew';

	if(options.import?.async) options.async = true;
	let { compiled_code, file } = compileFile(options.code ? { content: options.code, path: filepath } : filepath, options);
	// context.module.compiled = compiled_code;
	// context.process.exit = (int) => process.exit(int);

	const doCode = () => {
		const context = options.import?.takeThisContext ? custom_context : prepareContext(custom_context, options, file.path, runPath);
	
		if(context.app){
			const p = path.join(CONFIG_PATH, context.app.config.manifest.package, 'app');
			if(existsSync(p) && context.app.path !== p){
				console.log("App with the same package name has been installed. Conflicts happened. \nTo fix this, change your app's package name or remove the app making the conflict.");
				return {
					context: { module: { exports: null } },
					returns: null
				}
			}
		}
	
		compiled_code = preScript+'\n'+compiled_code;
	
		return {
			context,
			returns: exec(compiled_code, context, file.content),
		};
	}

	if(options.async){
		return new Promise(async (r, re) => {
			compiled_code.then((e) => {
				compiled_code = e;
				r(doCode());
			});
		});
	}

	return doCode();
};
