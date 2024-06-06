const { spawnSync } = require('child_process');

const print = (module.exports.print = function print(...args) {
	return console.log(...args);
});

print.stdout = process.stdout;
print.stdin = process.stdin;

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
