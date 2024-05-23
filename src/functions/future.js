const emitter = require("./emitter");


module.exports = function future(callback, timeout = 0, defData = null) {
  const listener = emitter();
  const promise = new Promise((resolve, reject) => {
    listener.on('resolve', (data) => {
      resolve(data);
    });
    listener.on('reject', (data) => {
      reject(data);
    });
    callback(resolve, reject);
    if(timeout) setTimeout(() => resolve(defData), timeout);
  });
  return {
    pipe: (callback) => promise.then(callback),
    pipex: (callback) => promise.finally(callback),
    catch: (callback) => promise.catch(callback),
    resolve: (data) => listener.emit('resolve', data),
    reject: (data) => listener.emit('reject', data)
  }
}