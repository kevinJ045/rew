#!/usr/bin/env node

const yargs = require('yargs/yargs');
const path = require('path');
const { hideBin } = require('yargs/helpers');
const { fork } = require('child_process');
const { watch } = require('chokidar');
const utils = require('./utils');

yargs(hideBin(process.argv))
  .command(
    '$0 <file>', 
    'Run the specified file', 
    (yargs) => {
      yargs
        .positional('file', {
          describe: 'File to run',
          type: 'string',
        })
        .option('watch', {
          alias: 'w',
          describe: 'Watch the file for changes',
          type: 'boolean',
        });
    },
    (argv) => {
      const filePath = path.resolve(process.cwd(), argv.file);
      const watching = [];
      const watchIt = (file) => {
        if(watching.includes(file)) return;
        watch(file).on('change', () => runIt());
        watching.push(file);
      }
      let prevFork;
      const runIt = () => {
        if(argv.watch) console.clear();
        if(prevFork && !prevFork.killed) prevFork.kill?.();
        prevFork = fork(path.resolve(__dirname, './run.js'))
        .on('message', (data) => {
          if(argv.watch){
            data.forEach(file => {
              watchIt(file);
            });
          } else {
            process.exit();
          }
        }).send({ filePath, watch: argv.watch });
        if(argv.watch) watchIt(filePath);
      }
      runIt();
    }
  )
  .command(
    'conf <command> [path] [key] [value]', 
    'Configuration management', 
    (yargs) => {
      yargs
        .positional('command', {
          describe: 'Configuration command (get, set, remove)',
          type: 'string',
          choices: ['get', 'set', 'remove'],
        })
        .positional('path', {
          describe: 'Configuration path',
          type: 'string',
          default: '',
        })
        .positional('key', {
          describe: 'Key to get/set/remove',
          type: 'string',
          default: '',
        })
        .positional('value', {
          describe: 'Value to set (only used with "set" command)',
          type: 'string',
          default: '',
        });
    },
    (argv) => {
      const { command, path, key, value } = argv;
      const result = utils.conf(command, path, key, value);
      if(result) console.log(result);
    }
  )
  .command('create <path>',  'Create a new project', (yargs) => {
    yargs
      .positional('path', {
        describe: 'Path of the project to create',
        type: 'string',
      });
    },
    (argv) => {
      utils.createProject(argv.path);
    }
  )
  .command('run <path | package>',  'Run an app', (yargs) => {
    yargs
      .positional('path', {
        describe: 'Path of the app to run',
        type: 'string',
      });
    },
    (argv) => {
      utils.runApp(argv.path);
    }
  )
  .command('build <file>', 'Build the specified file', (yargs) => {
    yargs
      .positional('file', {
        describe: 'File to build',
        type: 'string',
      });
  }, (argv) => {
    console.log(`Building file: ${argv.file}`);
  })
  .help()
  .argv;
