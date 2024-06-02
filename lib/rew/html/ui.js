try {
	window.execContext = $OPTIONS(json.execContext);
} catch (e) {
	window.execContext = {};
}

try {
	window.exec = $OPTIONS(exec);
} catch (e) {
	window.exec = function () {};
}

const DOM = [];

const findInDom = (id) => DOM.find((el) => el.widgetOptions.uuid == id) || DOM.find((el) => el.id == id);

const parseStyleValue = (val) => val;

const addTo = (el, parent) => {
	if (parent == 'null') {
		document.body.appendChild(el);
	} else {
		findInDom(parent).appendChild(el);
	}
};

const initElement = (el, options, update = false) => {
	if (el.widgetOptions) {
		if (el.widgetOptions.style) {
			for (let i in options.style) {
				el.style.removeProperty(i, el.widgetOptions.style[i]);
			}
		}
		if (el.widgetOptions.attr) {
			for (let i in el.widgetOptions.attr) {
				el.removeAttribute(i);
			}
		}
	}

	el.widgetOptions = options;
	el.id = options.id;
	el.textContent = options.data.text;

	if (options.style) {
		for (let i in options.style) {
			el.style.setProperty(i, options.style[i]);
		}
	}

	if (options.attr) {
		for (let i in options.attr) {
			el.setAttribute(i, options.attr[i]);
		}
	}

	if (options.children.length) {
		options.children.forEach((option) => {
			option.parent = options.uuid;
			if (update) updateElement(findInDom(option.uuid), option);
			else createElement(option);
		});
	}

	if (options.parent) {
		addTo(el, options.parent);
	}
};

const updateElement = (el, options) => {
	if (!el) return;
	initElement(el, options, true);
	return el;
};

const events = [
	'click',
	'dblclick',
	'mousedown',
	'mouseup',
	'mouseover',
	'mouseout',
	'mousemove',
	'mouseenter',
	'mouseleave',
	'keydown',
	'keypress',
	'keyup',
	'change',
	'input',
	'submit',
	'focus',
	'blur',
	'copy',
	'cut',
	'paste',
	'scroll',
	'wheel',
	'resize',
	'contextmenu',
	'drag',
	'dragstart',
	'dragend',
	'dragenter',
	'dragleave',
	'dragover',
	'drop',
	'error',
	'load',
	'abort',
];
const handleListeners = (el) => {
	events.forEach((event) => {
		el.addEventListener(event, (e) => {
			sendData({
				action: 'hook:eventTrigger',
				data: {
					rid: 'event_trigger',
					object: {
						uuid: el.widgetOptions.uuid,
						event,
						data: {
							mouse: { x: e.clientX, y: e.clientY },
							key: { code: e.keyCode, key: e.key },
						},
					},
				},
			});
		});
	});
};

function eventHandlerFunction({ uuid, hookID, event }) {
	return function (e) {
		sendData({
			action: 'hook:event_' + event,
			data: {
				rid: hookID,
				object: {
					uuid,
					event,
					data: {
						mouse: { x: e.clientX, y: e.clientY },
						key: { code: e.keyCode, key: e.key },
					},
				},
			},
		});
	};
}

const createElement = (options) => {
	const el = document.createElement(options.element);
	DOM.push(el);
	initElement(el, options);
	return el;
};

const stringifyJSON = (json) => {
	try {
		return JSON.stringify(json, null, 4);
	} catch (e) {
		return json.toString();
	}
};

const log = (...strings) => {
	window.webkit.messageHandlers.external.postMessage(
		JSON.stringify(
			{
				action: 'log',
				data: strings
					.map((r) => (typeof r == 'object' ? stringifyJSON(r) : `${r.toString()}`))
					// .map((i) => i.replace(/\"/g, '\\\\"').replace(/\n/g, "\\\\n"))
					.join('\n'),
			},
			null,
			4,
		),
	);
};

const sendData = (data) => {
	log('RESPONSE::' + stringifyJSON(data));
};

function process_data(data) {
	return JSON.parse(data);
}

window.recieveMessage = (data) => {
	const edata = data;
	if (edata.action == 'eventListen') {
		const el = findInDom(edata.data.uuid);
		if (el) {
			el.addEventListener(edata.data.event, eventHandlerFunction(edata.data));
		}
	} else if (edata.action == 'createElement') {
		const options = edata.data;
		try {
			createElement(options);
		} catch (e) {
			log(e.toString());
		}
	} else if (edata.action == 'addStyleSheet') {
		const style = document.createElement('style');
		style.textContent = edata.data;
		document.head.appendChild(style);
	} else if (edata.action == 'updateElement') {
		const options = edata.data;
		try {
			updateElement(findInDom(options.uuid), options);
		} catch (e) {
			log(e.toString());
		}
	} else if (edata.action == 'findElement') {
		const id = edata.data.id;
		const rid = edata.data.rid;
		try {
			sendData({
				action: 'hook:findElement',
				data: { rid, object: findInDom(id)?.widgetOptions },
			});
		} catch (e) {
			log(e.toString());
		}
	} else if (edata.action == 'message') {
		window.dispatchEvent(
			new CustomEvent('message', {
				detail: edata.data,
			}),
		);
	}
};

window.addEventListener('load', () => {
	window.exec({
		...window.execContext,
		window,
		log,
		send: (data) => sendData({ action: 'message', data }),
		onRecieve: (cb) => window.addEventListener('message', (e) => cb(e.detail || {})),
	});
	log('SETUP::READY');
});
