const { parentPort, workerData } = require('worker_threads');
const { exec } = require('../../../modules/runtime');

const target = {};
target.events = [];
target.on = (e, cb) => {
	target.events.push({ e, cb });
};
target.off = (e) => {
	target.events = target.events.filter((i) => i.e !== e);
};
target.emit = (e, data) => {
	target.events.filter((i) => i.e == e).forEach((i) => i.cb(data));
};

parentPort.on('message', (data) => {
	if (data.type === 'event') {
		target.emit(data.event, data.data);
	} else if (data.type === 'start') {
		(async () => {
			try {
				exec(`(${workerData.cb}).call({ process }, context)`, {
					print: (...a) => console.log(...a),
					process: {
						on: (e, cb) => {
							target.on(e, cb);
						},
						off: (e) => {
							target.off(e);
						},
						emit: (e, data) => {
							parentPort.postMessage({ type: 'event', event: e, data });
						},
						exit: (code) => process.exit(code),
						finish: (data) => {
							parentPort.postMessage({ type: 'result', result: data });
							process.exit(0);
						},
					},
					context: workerData.context,
				}, '', 'thread');
			} catch (e) {
				parentPort.postMessage({ type: 'error', error: e.message });
			}
		})();
	}
});
