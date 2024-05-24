const path = require('path');
const { spawn, exec } = require('child_process');
const WebSocket = require('ws');
const http = require('http');
const fs = require('fs');

const BIN_PATH = path.resolve(__dirname, '../../bin/ui');
const HTML_STRING = fs.readFileSync(path.resolve(__dirname, '../html/ui.html'), { encoding: 'utf-8' });

const defaultOptions = {
  port: 14473,
  title: 'Title',
  onExit: () => process.exit()
};

module.exports = (context) => ({
  start: (o = {}) => {
    const options = {
      ...o,
      ...defaultOptions
    }

    const svr = http.createServer((req, res) => {
      res.write(HTML_STRING.replace(/\%OPTIONS\(([^)]+)\)/g, (_, n) => console.log(options[n], n) || options[n] || _));
      res.end();
    });

    const wss = new WebSocket.Server({ server: svr });

    wss.on('connection', (ws) => {
      console.log('Client connected');

      // Send a command to create a button
      ws.send(JSON.stringify({ action: 'init', data: options }));

      ws.on('message', (message) => {
        console.log(`Received: ${message}`);
      });

      ws.on('close', () => {
        console.log('Client disconnected');
      });
    });

    svr.listen(options.port);

    const url = `http://localhost:${options.port}`;

    const p = spawn(BIN_PATH, [url]);

    p.stdout.on('data', (data) => {
      console.log(`stdout: ${data}`);
    });

    p.stderr.on('data', (data) => {
      console.error(`stderr: ${data}`);
    });

    p.on('close', (code) => {
      options.onExit(code);
    });
    // p.on('message', console.log);
    // p.on('error', console.log);
    // exec(BIN_PATH+' '+'http://localhost:' + port, console.log)
  }
})