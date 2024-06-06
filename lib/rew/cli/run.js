const { run } = require('../main');

function exec(filePath, argv) {
	return run(filePath, { argv })?.context?.module?.imports || [];
}

const onmsg = ({ filePath, argv, watch }) => {
	const imports = exec(filePath,  [filePath, ...(argv || [])]);
	if (watch) {
		process.send(imports);
	}
	process.off('message', onmsg);
};

process.on('message', onmsg);
