#!/usr/bin/env node

const yargs = require('yargs/yargs');
const path = require('path');
const { hideBin } = require('yargs/helpers');
const { fork, exec, execSync } = require('child_process');
const { watch } = require('chokidar');
const utils = require('./utils');
const { existsSync, readFileSync, writeFileSync, mkdirSync } = require('fs');
const { log } = require('./log');
const { compile } = require('../modules/compiler');
const crypto = require('crypto');
const { CONFIG_PATH } = require('../const/config_path');

if(!existsSync(CONFIG_PATH)){
	mkdirSync(CONFIG_PATH, { recursive: true });
	utils.initFirst();
}

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
			if (!existsSync(filePath)) {
				log('File not found:', argv.file, ':end');
				return;
			}
			const watching = [];
			const watchIt = (file) => {
				if (watching.includes(file)) return;
				watch(file).on('change', () => runIt());
				watching.push(file);
			};
			let prevFork;
			const runIt = () => {
				if (argv.watch) console.clear();
				if (prevFork && !prevFork.killed) prevFork.kill?.();
				prevFork = fork(path.resolve(__dirname, './run.js'))
					.on('message', (data) => {
						if (argv.watch) {
							data.forEach((file) => {
								watchIt(file);
							});
						}
					})
					.send({ filePath, watch: argv.watch });
				if (argv.watch) watchIt(filePath);
			};
			runIt();
		},
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
			if (result) console.log(result);
		},
	)
	.command(
		'create <path>',
		'Create a new project',
		(yargs) => {
			yargs.positional('path', {
				describe: 'Path of the project to create',
				type: 'string',
			});
		},
		(argv) => {
			utils.createProject(argv.path);
		},
	)
	.command(
		'rune-keygen <secret>',
		'Generate a rune encryption key',
		(yargs) => {
			yargs.option('secret', {
				describe: 'Secret used to generate encryption key',
				type: 'string',
			});
		},
		(argv) => {
			const generateEncryptionKey = (secret) => {
				if (secret) {
					return crypto.createHash('sha256').update(secret).digest('hex');
				} else {
					return crypto.randomBytes(32).toString('hex');
				}
			};

			const encryptionKey = generateEncryptionKey(argv.secret);
			console.log('Encryption Key:', encryptionKey);
		},
	)
	.command(
		'ui-bin <path>',
		'Build the UI bin for your own app',
		(yargs) => {
			yargs.positional('path', {
				describe: 'Path of the output bin',
				type: 'string',
			});
		},
		(argv) => {
			execSync('sh ' + path.resolve(__dirname, '../../../build.sh') + ' ' + argv.path);
		},
	)
	.command(
		'run <path | package>',
		'Run an app',
		(yargs) => {
			yargs.positional('path', {
				describe: 'Path of the app to run',
				type: 'string',
			});
		},
		(argv) => {
			utils.runApp(argv.path);
		},
	)
	.command(
		'install <path>',
		'Install an app',
		(yargs) => {
			yargs.positional('path', {
				describe: 'Path of the app to install',
				type: 'string',
			});
		},
		async (argv) => {
			utils.installAppFrom(argv.path);
		},
	)
	.command(
		'repo <command> [name] [url]',
		'Manage install repositories',
		(yargs) => {
			yargs.positional('command', {
				describe: 'Command to add/remove/set',
				type: 'string',
			});
			yargs.positional('name', {
				describe: 'name of the repo',
				type: 'string',
			});
			yargs.positional('url', {
				describe: 'url of the repo',
				type: 'string',
			});
		},
		async (argv) => {
			utils.repo(argv.command, argv.name, argv.url);
		},
	)
	.command(
		'build <file>',
		'Build the specified file',
		(yargs) => {
			yargs
				.positional('file', {
					describe: 'File to build',
					type: 'string',
				})
				.option('output', {
					alias: 'o',
					describe: 'Output directory',
					type: 'string',
				});
		},
		(argv) => {
			function readFile(filePath) {
				return readFileSync(filePath, { encoding: 'utf-8' });
			}

			function extractImports(content) {
				const importRegex = /(\w+)\s*=\s*imp\s*['"](.+?)['"]/g;
				const imports = [];
				let match;
				while ((match = importRegex.exec(content)) !== null) {
					imports.push({ variable: match[1], url: match[2] });
				}
				return imports;
			}

			function writeCompiledFile(filePath, compiledCode) {
				const dirName = outputDir ? outputDir : path.dirname(filePath);
				if (!existsSync(dirName)) mkdirSync(dirName, { recursive: true });
				const baseName = path.basename(filePath, path.extname(filePath));
				const newFilePath = path.join(dirName, `${baseName}.js`);
				writeFileSync(newFilePath, compiledCode, { encoding: 'utf-8' });
				log(`Compiled: ${newFilePath}`);
			}

			function processFile(filePath, importsArray) {
				const content = readFile(filePath);
				const imports = extractImports(content);

				imports.forEach((importStatement) => {
					const importedFilePath = path.resolve(path.dirname(filePath), importStatement.url);
					if (!importsArray.some((importObj) => importObj.url === importStatement.url)) {
						if (existsSync(importedFilePath)) {
							importsArray.push(importStatement);
							processFile(importedFilePath, importsArray);
						} else if (existsSync(importedFilePath + '.coffee')) {
							importsArray.push(importStatement);
							processFile(importedFilePath + '.coffee', importsArray);
						} else if (existsSync(importedFilePath + '.js')) {
							importsArray.push(importStatement);
							processFile(importedFilePath + '.js', importsArray);
						}
					}
				});

				const compiled = compile({ content }, {});
				writeCompiledFile(filePath, compiled);
			}

			const filePath = path.resolve(process.cwd(), argv.file);
			const importsArray = [];
			const outputDir = argv.output ? path.resolve(process.cwd(), argv.output) : null;
			log('Start compile at', outputDir || 'default path');
			processFile(filePath, importsArray);
			log('Compiled', importsArray.length + 1, 'files.', ':end');
		},
	)
	.help().argv;
