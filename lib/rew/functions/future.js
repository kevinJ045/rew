const emitter = require('./emitter');

function future(callback, timeout = 0, defData = null) {
	const listener = emitter();
	const promise = new Promise((resolve, reject) => {
		listener.on('resolve', (data) => {
			resolve(data);
		});
		listener.on('reject', (data) => {
			reject(data);
		});
		callback(resolve, reject);
		if (timeout) setTimeout(() => resolve(defData), timeout);
	});
	return {
		pipe: (callback) => promise.then(callback),
		last: (callback) => promise.finally(callback),
		catch: (callback) => promise.catch(callback),
		resolve: (data) => listener.emit('resolve', data),
		reject: (data) => listener.emit('reject', data),
		wait: async () => await promise,
	};
};

future.promise = (promse, timeout = 0, defData = null) => {
  return future((resolve, reject) => {
    promse.then(resolve).catch(reject);
  }, timeout, defData);
}

module.exports = future;