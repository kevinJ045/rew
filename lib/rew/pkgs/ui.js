const path = require('path');
const { spawn, exec } = require('child_process');
const fs = require('fs');
const { uiClasses } = require('./modules/ui/classes');
const { generateRandomID } = require('../functions/id');
const { THEME_PATH } = require('../const/files');
const readline = require('readline');
const emitter = require('../functions/emitter');

const BIN_PATH = path.resolve(__dirname, '../../../bin/ui');
const HTML_STRING = fs.readFileSync(path.resolve(__dirname, '../html/ui.html'), { encoding: 'utf-8' });
const JS_STRING = fs.readFileSync(path.resolve(__dirname, '../html/ui.js'), {
	encoding: 'utf-8',
});

const replaceString = (string, options) =>
	string.replace(/\$OPTIONS\(([^)]+)\)/g, (_, n) =>
		n.startsWith('json.') ? JSON.stringify(options[n.split('json.')[1]] || '{}') : options[n] || _,
	);

const defaultOptions = {
	title: 'Title',
	onExit: () => process.exit(),
	style: '',
	stylePath: THEME_PATH,
	exec: () => {},
	execContext: {},
};

module.exports = (context) => ({
	start: (o = {}) => {
		const options = {
			...defaultOptions,
			...o,
		};

		const hookedSocketListeners = {};

		const runId = generateRandomID();
		const tmpFile = '/tmp/' + runId + '.ruw.ui.socket';

		options.runId = runId;

		if (fs.existsSync(options.stylePath)) options.style = fs.readFileSync(options.stylePath, { encoding: 'utf-8' }) + '\n' + options.style;

		options.style = ' */\n' + options.style + '\n/* ';

		const HTML = replaceString(HTML_STRING, options);
		const JS = replaceString(JS_STRING, options);

		/**
		 * Queue for future writes
		 * @type {string[]}
		 * */
		const queue = [];

		const send = (data) => {
			const content = fs.readFileSync(tmpFile, { encoding: 'utf-8' });
			if (content) {
				queue.push(data);
			} else {
				fs.writeFileSync(tmpFile, typeof data !== 'string' ? JSON.stringify(data) : data);
			}
		};

		const sendEvent = (data) => {
			send({
				action: 'JS',
				data: `window.recieveMessage(${JSON.stringify(data)})`,
			});
		};

		const g_emitter = emitter();

		const recieve = (data) => {
			g_emitter.emit('recieve', data);
		};

		const rl = readline.createInterface({
			input: process.stdin,
			output: process.stdout,
		});

		rl.question('', () => {});

		fs.writeFileSync(tmpFile, '');

		fs.watch(tmpFile, { encoding: 'utf-8' }).on('change', () => {
			if (queue.length) {
				send(queue.pop());
			}
		});

		const p = spawn(options.bin || BIN_PATH, [runId]);

		p.on('close', (code) => {
			rl.close();
			options.onExit(code);
		});

		process.on('beforeExit', () => {
			p.kill();
			fs.unlinkSync(tmpFile);
		});

		g_emitter.on('recieve', (edata) => {
			if (edata.action.startsWith('hook:')) {
				const hook = hookedSocketListeners[edata.data.rid];
				const type = edata.action.split('hook:')[1];
				if (hook && hook.type == type) {
					hookedSocketListeners[edata.data.rid].cb(edata.data.object);
					if (hook.once) delete hookedSocketListeners[edata.data.rid];
				}
			}
		});

		return new Promise((r) => {
			p.stdout.on('data', (data) => {
				if (data.toString().startsWith('RESPONSE::')) {
					const d = data.toString().split('RESPONSE::')[1];
					const jd = JSON.parse(d);
					recieve(jd);
				} else if (data.toString().trim().endsWith('SETUP::READY')) {
					console.log('READY');
					r(
						uiClasses(
							context,
							options,
							sendEvent,
							(cb) => {
								g_emitter.on('recieve', cb);
							},
							(rid, type, cb, once = true) => {
								// Add hook
								hookedSocketListeners[rid] = { type, cb, once };
							},
							(rid) => {
								// Remove hook
								delete hookedSocketListeners[rid];
							},
						),
					);
				} else if (data.toString().endsWith('SETUP::HTML')) {
					send({ action: 'JS2', data: JS, isSetup: true });
				} else if (data.toString() == 'INIT::READY') {
					send({ action: 'HTML', data: HTML });
				} else {
					console.log(data.toString());
				}
			});

			p.stderr.on('data', (data) => {
				console.error(data.toString());
			});
		});
	},
});
