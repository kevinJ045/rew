const { spawnSync } = require('child_process');
const r = require('colors')

const baseAnsii = '\x1b';
const fg = {
	o: ["30", '39'],
	r: ["31", '39'],
	g: ["32", '39'],
	y: ["33", '39'],
	b: ["34", '39'],
	m: ["35", '39'],
	c: ["36", '39'],
	w: ["37", '39'],
	a: ["90", '39']
}
const bg = {
	O: ["40", '49'],
	R: ["41", '49'],
	G: ["42", '49'],
	Y: ["43", '49'],
	B: ["44", '49'],
	M: ["45", '49'],
	C: ["46", '49'],
	W: ["47", '49'],
	A: ["48", '49']
}
const styles = {
	"!": ["1", '22'], // bold
	"'": ["2", '22'], // dim
	"/": ["3", '23'], // italic
	"_": ["4", '24'], // underline
	"i": ["7", '27'],
	"h": ["8", '28'],
	"-": ["9", '29'],
}

const findStyle = (name) => {
	return styles[name] || bg[name] || fg[name];
}

const addColorize = (applied) => {
	return `${baseAnsii}[${applied.fg}m${applied.bg ? `${baseAnsii}[${applied.bg}m` : ''}${applied.style ? `${baseAnsii}[${applied.style}m` : ''}`
}

const colorize = (baseString = '') => {
	const applied = {
		style: '',
		fg: "39",
		bg: "49",
	};
	const stringParts = baseString.replace(/%([orgybmcwaORGYBMCWA!'\/_ih-])/gmi, (_, col) => {
		const val = findStyle(col);
		if(!val) return _;
		const tag = col in styles ? 'style' :
			col in fg ? 'fg' : 'bg';
		if(applied[tag] == val[0]){
			applied[tag] = val[1];
		} else {
			applied[tag] = val[0];
		}
		return addColorize(applied);
	});
	return stringParts + addColorize({
		style: '',
		fg: "39",
		bg: "49",
	});
}

(module.exports.print = function print(...args) {
	if(!args.some(i => typeof i !== "string")){
		args = [args.join(' ')]
	}
	if(typeof args[0] === "string" && args[0].startsWith('%c')){
		args[0] = colorize(args[0]);
	}
	return console.log(...args);
});

module.exports.printf = function printf(buffer, cb) {
	if(typeof buffer === "string" && buffer.startsWith('%c')){
		buffer = colorize(buffer);
	}
	return process.stdout.write(buffer, cb);
};

module.exports.input = function input(prompt) {
	process.stdout.write(prompt);

	let cmd;
	let args;
	if ('null' == 'win32') {
		cmd = 'cmd';
		args = ['/V:ON', '/C', 'set /p response= && echo !response!'];
	} else {
		cmd = 'bash';
		args = ['-c', 'read response; echo "$response"'];
	}

	let opts = {
		stdio: ['inherit', 'pipe'],
		shell: false,
	};

	return spawnSync(cmd, args, opts).stdout.toString().trim();
};

module.exports.clear = () => {
	console.clear();
}
