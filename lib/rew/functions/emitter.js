module.exports = function emitter() {
	const events = [];
	const on = (event, callback, props = {}) => {
		const addEvent = (event) => events.push({ ...props, event, callback });
		if (Array.isArray(event)) {
			event.forEach(addEvent);
		} else {
			addEvent(event);
		}
		return listener;
	};
	const off = (event, callback, removable = null) => {
		const rm = (event) => {
			if (removable) {
				removable(event);
			}
			events.splice(events.indexOf(event), 1);
		};
		const rmEvent = (event) => {
			if (callback) {
				const _events = events.filter(({ callback: c }) => c == callback);
				_events.forEach((e) => {
					if (Array.isArray(event)) {
						if (event.includes(e.event)) rm(e);
					} else {
						if (e.event == event) rm(e);
					}
				});
			} else {
				const _events = events.filter(({ event: e }) => e == event);
				_events.forEach((e) => {
					rm(e);
				});
			}
		};
		if (Array.isArray(event)) {
			event.forEach(rmEvent);
		} else {
			rmEvent(event);
		}
		return listener;
	};
	const emit = (event, ...data) => {
		const emitEvent = (event) => events.filter(({ event: e }) => e == event).forEach(({ callback }) => callback(...data));
		if (Array.isArray(event)) {
			event.forEach(emitEvent);
		} else {
			emitEvent(event);
		}
		return listener;
	};
	const listener = { on, off, emit };
	return listener;
};
