// run.js
const { REW_FILE_TYPE } = require('../const/ext');
const { run } = require('../main');
const { watch } = require('chokidar');

function exec(filePath, argv, options = {}) {
	return run(filePath, { argv, ...options })?.context?.module?.imports || [];
}

function runFile(filePath, options = {}, argv){
	const watching = [];
	const watchIt = (file) => {
		if (watching.includes(file)) return;
		watch(file).on('change', () => runIt());
		watching.push(file);
	};

	const runIt = () => {
		if (options.watch) console.clear();
		const imports = exec(filePath, [filePath, ...(argv || [])], { onlyCompile: options?.onlyCompile, async: options?.async });
		if (options.watch) {
			imports.forEach((file) => {
				watchIt(file);
			});
			watchIt(filePath);
		}
	};

	runIt();
}

function runFileWithArgv(filePath, options = {}, cargv) {
	let argv = cargv || process.argv;
	argv.shift();
	if (argv[0].endsWith(REW_FILE_TYPE.EXTENSION) || argv[0].endsWith('.coffee')) {
		if (argv[1] == 'run') {
			argv.splice(0, 3);
		} else if(argv[1] == '-w' || argv[1] == '--watch'){
			argv.splice(0, 3);
		} else argv.splice(0, 2);
	}
	if (argv[1] == 'exec') {
		argv.splice(0, 2);
	}
	if (argv.includes('--')) {
		argv = argv.slice(argv.indexOf('--') + 1, argv.length);
	} else {
		const index = argv.find(p => filePath.endsWith(p.replace(/(\.|\.\.)\//, '/')));
		if(index){
			argv = argv.slice(argv.indexOf(index) + 1, argv.length);
		}
	}
	runFile(filePath, options, argv);
}

module.exports = { runFileWithArgv, runFile, execRewFile: exec };
