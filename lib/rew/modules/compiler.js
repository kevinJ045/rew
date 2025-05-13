const { execOptions } = require('../const/opt');
const { findAppInfo } = require('../misc/findAppInfo');
const { from_qrew } = require('../qrew/compile');
const { getFile, file } = require('./fs');
const babel = require('@babel/core');
const path = require('path');
const babelReact = require('@babel/preset-react');
const { readFileSync, existsSync } = require('fs');
const { REW_FILE_TYPE } = require('../const/ext');
const { straceLog } = require('../misc/strace');
const { compileCivetStuff } = require('./compiler.raw');


const cpl = (module.exports.compile = function (file, options = {}) {
	let compiledCode;
	const result = compileCivetStuff(file, {
		...options,
		parseOptions: { 
			coffeeCompat: options.type == "coffee",
		}
	});
	
	options = result.options;
	compiledCode = result.compiled;

	const babelify = (code, options) => {
		straceLog('!COMPILER babel()');
		if(doJSX) straceLog('==> INFO !-WITH JSX');
		if(doTypes) straceLog('==> INFO !-WITH Types');
		if(doDecorators) straceLog('==> INFO !-WITH DECORATORS');
		return babel.transformSync(code, {
			presets: [
				...(doJSX ? [[babelReact, { throwIfNamespace: false, pragmaFrag: options.jsxPragmaFrag || execOptions.jsxPragmaFrag, pragma: options.jsxPragma || execOptions.jsxPragma }]] : [])
			],
			plugins: [
				...(doDecorators ? [[require('@babel/plugin-proposal-decorators'), { version: '2023-05' }], [require('@babel/plugin-proposal-class-properties'), { loose: true }], [require('@babel/plugin-transform-class-static-block'), {}]] : []),
				// doJSX ? require('./jsx') : null
			].filter(Boolean),
		}).code;
	}

	const doJSX = execOptions.jsx || options.jsx;
	const doTypes = execOptions.typescript || options.typescript;
	const doDecorators = execOptions.decorators || options.decorators;
	const doBabel = doJSX || doTypes || doDecorators;

	if(compiledCode instanceof Promise){
		return compiledCode.then((compiledCode) => {
			if (doBabel) {
				compiledCode = babelify(compiledCode, options);
			}
			return compiledCode;
		});
	}

	// console.log(c);
	if (doBabel) {
		compiledCode = babelify(compiledCode, options);
	}
	return compiledCode;
});

module.exports.compileFile = function (filepath, options = {}) {
	straceLog('compile(currentFile)');
	const f = typeof filepath == "object" ? filepath : getFile(filepath);
	if(typeof filepath == "object") filepath = filepath.path;
	let qrew = false;

	if(options.qrew || path.extname(filepath) == '.qrew') {
		qrew = true
		f.content = from_qrew(readFileSync(f.path), options.package || findAppInfo(filepath)?.config.manifest.package || path.basename(filepath).split('.').slice(0, -1).join('.')).toString();
		options.type = f.content.split('\n')[0]?.match(/"initFile (.+)"/)?.[1]?.split('.').pop();
		straceLog('decodeCrew(currentFile).as(', `"${options.type}"`,')');
	}

	let compiled_code =  cpl(f, { ...options });

	if(options.onlyCompile && !qrew){
		straceLog('writeAndQuit(compileData)');
		if(compiled_code instanceof Promise){
			compiled_code.then((r) => {
				console.log(r);
				if(typeof process !== "undefined") process.exit();
			});
		} else {
			console.log(compiled_code);
			if(typeof process !== "undefined") process.exit();
		}
	}

	return {
		compiled_code,
		file: f,
	};
};
