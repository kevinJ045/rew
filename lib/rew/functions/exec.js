const shell = require('child_process');

module.exports = (currentFile) => {
	function exec(command, options) {
		return shell.execSync(command, {
			stdio: options?.output == false ? null : 'inherit',
		});
	}

	exec.background = function execAsync(command, options, callback) {
		if (typeof options == 'function' && !callback) {
			callback = options;
			options = {};
		}
		if (!options) options = {};
		if (!callback) callback = () => {};
		return shell.exec(
			command,
			{
				...options,
			},
			callback,
		);
	};

	function spawn(command, options) {
		return shell.spawn(command, options);
	}

	return { exec, spawn };
};
