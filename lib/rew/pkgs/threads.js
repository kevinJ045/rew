const { Worker } = require('worker_threads');
const emitter = require('../functions/emitter');
const future = require('../functions/future');
const { struct } = require('../models/struct');
const path = require('path');

module.exports = (context) => ({
  thread: (cb) => {
    const workers = [];

    const stopWorker = (worker) => {
      workers.splice(workers.indexOf(worker), 1);
      worker.terminate()
    };

    const athread = {
      stopAll: () => {
        if (!run) return;
        workers.forEach(stopWorker);
      },
      start: (context) => {
        let dataResult = future(() => {}), listener = emitter();
        const worker = new Worker(path.resolve(__dirname, './modules/threads/worker.js'), {
          workerData: { context, cb: cb.toString() }
        });
        workers.push(worker);

        const stop = () => stopWorker(worker);

        worker.on('message', (data) => {
          if (data.type === 'result') {
            dataResult.resolve(data.result);
            stop();
          } else if (data.type === 'error') {
            reject(new Error(data.error));
            stop();
          } else if (data.type === 'event') {
            listener.emit(data.event, data.data);
          }
        });

        worker.on('error', (error) => {
          console.log(error);
          stop();
        });

        worker.on('exit', (code) => {
          stop();
          if (code !== 0) {
            throw new Error(`Worker stopped with exit code ${code}`);
          }
        });

        worker.postMessage({ type: 'start' });

        return {
          on: (e, cb) => listener.on(e, cb),
          off: (e, cb) => listener.off(e, cb),
          emit: (e, data) => worker?.postMessage({ type: 'event', event: e, data }),
          get: () => dataResult.wait()
        };
      }
    };

    return athread;
  }
});
