#!/usr/bin/env node

const yargs = require('yargs/yargs');
const path = require('path');
const { hideBin } = require('yargs/helpers');
const { fork, exec, execSync } = require('child_process');
const { watch } = require('chokidar');
const utils = require('./utils');
const { existsSync, readFileSync, writeFileSync, mkdirSync } = require('fs');
const { log } = require('./log');
const os = require('os');
const crypto = require('crypto');
const { CONFIG_PATH } = require('../const/config_path');
const rune = require('../pkgs/rune');
const { to_qrew, from_qrew } = require('../qrew/compile');
const { findAppInfo } = require('../misc/findAppInfo');
const { print, input } = require('../functions/stdout');
const colors = require('colors');

if (!existsSync(CONFIG_PATH)) {
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
		'rune-keygen',
		'Generate a rune encryption key',
		(yargs) => {
		},
		(argv) => {
			console.log('Encryption Key:', rune({}).genKey(input('Secret Value: ') || null));
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
			})
			.option('dev', {
				describe: 'If your entry file is a .qrew, then just use the .coffee instead',
				type: 'boolean',
			})
			.option('build', {
				alias: 'b',
				describe: 'Builds to a .qrew before running',
				type: 'boolean',
			})
			.option('translate', {
				alias: 't',
				describe: 'Builds to a .js before running, only used when --build is passed',
				type: 'boolean',
			});
		},
		(argv) => {
			utils.runApp(argv.path, argv);
		},
	)
	.command(
		'secret <command> [key]',
		'Add secrets to the current path',
		(yargs) => {
			yargs.positional('command', {
				describe: 'Path of the app to run',
				type: 'string',
			});
		},
		(argv) => {
			const appPath = findAppInfo(path.join(process.cwd(), 'app.yaml'));

			if (!appPath) return log(''.red.bold, 'Secrets only available in apps'.red.bold, ':end');

			const qrewPath = path.join(appPath.path, 'secrets.qrew');

			const getHost = () => `${process.env.USER}@${os.platform()}.${os.hostname()}`;

			const verifyUser = (content) => {
				const owner = content.match(/^owner = "(.*)" # end$/m)?.[1];
				if (owner == getHost()) return true;
				return false;
			};

			if (argv.command == 'init') {
				writeFileSync(qrewPath, to_qrew(`secrets = {} # end\n\nowner = "${getHost()}" # end\n \nexports { ...secrets }`, appPath.config.package))
			} else {
				const currentFileContent = from_qrew(readFileSync(qrewPath), appPath.config.package).toString();
				if (!verifyUser(currentFileContent)) return log(''.red.bold, 'You are not allowed to change this data'.red.bold, ':end');

				const secrets = currentFileContent.match(/^secrets = (.*) # end$/m)?.[1];

				let secretsJson = JSON.parse(secrets);

				if (argv.command == 'set' || argv.command == 'remove') {
					if (argv.command == 'set') {
						let val = input('Secret Value: ');

						secretsJson[argv.key] = val;
					} else {
						delete secretsJson[argv.key];
					}

					const newSecrets = `secrets = ${JSON.stringify(secretsJson)} # end`;
					const newFileContent = currentFileContent.replace(/^secrets = .* # end$/m, newSecrets);

					writeFileSync(qrewPath, to_qrew(newFileContent, appPath.config.package))
				} else if (argv.command == 'get') {
					if (argv.key) {
						console.log(argv.key.yellow, '=', secretsJson[argv.key].green);
					}
					else {
						for (let key in secretsJson) {
							console.log(key.yellow, '=', secretsJson[key].green);
						}
					}
				}
			}
		},
	)
	.command(
		'install <path>',
		'Install an app',
		(yargs) => {
			yargs.positional('path', {
				describe: 'Path or github or repo id of the app to install',
				type: 'string',
			});
		},
		async (argv) => {
			utils.installAppFrom(argv.path);
		},
	)
	.command(
		'uninstall <package>',
		'Unnstall an app',
		(yargs) => {
			yargs.positional('package', {
				describe: 'Package of the app to uninstall',
				type: 'string',
			}).option('all', {
				alias: 'a',
				describe: 'Remove the configs as well',
				type: 'boolean',
			});
		},
		async (argv) => {
			utils.uninstall(argv.package, argv.all);
		},
	)
	.command(
		'version',
		'Rew Version',
		(yargs) => {
		},
		async (argv) => {
			const pkg = JSON.parse(readFileSync(path.resolve(__dirname, '../../../package.json'), { encoding: 'utf-8' }));
			const getLatest = async () => {
				try{
					return (await (await fetch(`https://registry.npmjs.org/${pkg.name}`)).json()).dist_tags.latest.yellow.bold
				} catch(e) {
					return `(${'!err'.blue.bgRed}, see ${`https://npmjs.com/package/${pkg.name}`.blue.underline})`;
				}
			}
			log(`${'Rew'.red.bold} ${'RUNTIME'.yellow}`);
			log(`Version: ${pkg.name.green}@${pkg.version.yellow.bold}`);
			log(`Latest: ${pkg.name}@${await getLatest()}`, ':end');
		},
	)
	.command(
		'repo <command> [name] [url]',
		'Manage install repositories',
		(yargs) => {
			yargs.positional('command', {
				describe: 'Command to add/remove/set/get/view',
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
				})
				.option('translate', {
					alias: 't',
					describe: 'Translate to js',
					type: 'boolean',
				})
				.option('single', {
					alias: 's',
					describe: 'Build single file(don\'t build imports)',
					type: 'boolean',
				})
				.option('remove', {
					alias: 'r',
					describe: 'Remove all coffee',
					type: 'boolean',
				});
		},
		(argv) => {
			utils.build(argv);
		},
	)
	.help().argv;
