let start = true;
const startPrefix = '╭';
const middlePrefix = '├';
const separator = '│';
const endPrefix = '╰';

let last = '';

const log = (module.exports.log = function (...toPrint) {
	let prefix = start ? startPrefix : middlePrefix;
	let returns = false, nosep = false;
	if (toPrint[toPrint.length - 1] == ':end') {
		prefix = endPrefix;
		toPrint.pop();
	}
	if (toPrint[toPrint.length - 1] == ':returns') {
		returns = true;
		toPrint.pop();
	}
	if (toPrint[toPrint.length - 1] == ':nosep') {
		nosep = true;
		toPrint.pop();
	}
	if (prefix == endPrefix && start) prefix = separator;
	// if(last == endPrefix && prefix == separator) prefix = startPrefix;
	if (!start && !returns && !nosep) console.log(last == endPrefix ? startPrefix : separator);
	if (start) start = false;
	last = prefix;
	if (returns) return [prefix, ...toPrint].join(' ');
	else if (toPrint.length) console.log(prefix, ...toPrint);
});

module.exports.logget = function (...toPrint) {
	let args = [...toPrint];
	if (toPrint[toPrint.length - 1] == ':end') {
		let l = args.pop();
		args.push(':returns', l);
	} else {
		args.push(':returns');
	}
	return log(...args);
};

log.startPrefix = startPrefix;
log.middlePrefix = middlePrefix;
log.separator = separator;
log.endPrefix = endPrefix;
