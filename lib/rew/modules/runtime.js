const vm = require('vm');
const { compileFile, compile } = require('./compiler');
const { prepareContext } = require('./context');
const { existsSync, readFileSync } = require('fs');
const { CONFIG_PATH } = require('../const/config_path');
const path = require('path');
const { straceLog } = require('../misc/strace');

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

	straceLog('RUN() CURRENTFILE = ', filepath, 'as', options.type || 'UNKNOWN');

	if(options.import?.async) options.async = true;
	if(options.async) straceLog('ASYNCMODE() for CURRENTFILE');
	let { compiled_code, file } = compileFile(options.code ? { content: options.code, path: filepath } : filepath, options);
	straceLog('COMPILE_DONE() with COMPILEDATA');
	// context.module.compiled = compiled_code;
	// context.process.exit = (int) => process.exit(int);

	const doCode = () => {
		straceLog('RUNCODE() COMPILEDATA');
		const context = options.import?.takeThisContext ? custom_context : prepareContext(custom_context, options, file.path, runPath);
	
		if(context.app){
			const p = path.join(CONFIG_PATH, context.app.config.manifest.package, 'app');
			const p2 = path.join(CONFIG_PATH, context.app.config.manifest.package, 'app/.allow');
			if(existsSync(p) && context.app.path !== p && !existsSync(p2)){
				console.log("App with the same package name has been installed. Conflicts happened. \nTo fix this, change your app's package name or remove the app making the conflict.");
				return {
					context: { module: { exports: null } },
					returns: null
				}
			}
		}
	
		compiled_code = preScript+'\n'+compiled_code;

		let execd = exec(compiled_code, context, file.content);

		if(context.module.main && (context.module.exports?.main || (typeof context.module.exports == "function" && context.module.exports.name == 'main'))){
			const mainFn = context.module.exports.main ?? context.module.exports;
			let ctx = context;
			if(mainFn._class){
				ctx = mainFn._class;
				for(let i in context){
					ctx[i] = context[i];
				}
			}
			return {
				context,
				returns: mainFn.call(ctx, context.process.argv)
			}
		} else {
			return {
				context,
				returns: execd,
			};
		}
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
