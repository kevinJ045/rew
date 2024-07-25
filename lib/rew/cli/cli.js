#!/usr/bin/env node

const colors = require('colors');
const yargs = require('yargs/yargs');
const path = require('path');
const { hideBin } = require('yargs/helpers');
const { existsSync, readFileSync, writeFileSync, statSync, unlinkSync } = require('fs');
const { log } = require('./log');
const { to_qrew, from_qrew } = require('../qrew/compile');
const { findAppInfo } = require('../misc/findAppInfo');
const { input } = require('../functions/stdout');
const { req } = require('../misc/req');
const { gen_key } = require('../misc/bin');
const { REW_FILE_TYPE } = require('../const/ext');
const { generateRandomID } = require('../functions/id');
const { runFileWithArgv } = require('./run');
const { npm_package_name, getAllPipeInput } = require('./helpers');

function isFileArgument(file) {
	try {
		return existsSync(file) && statSync(file).isFile();
	} catch {
		return false;
	}
}

const isFileGiven = isFileArgument(hideBin(process.argv)[0]) || hideBin(process.argv)[0] == 'run';
try{
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
				})
				.option('strace', {
					alias: 's',
					describe: 'Log everything',
					type: 'boolean',
				})
				.option('compile', {
					alias: 'c',
					describe: 'Compile and output the javascript',
					type: 'boolean',
				});
		},
		(argv) => {
			if(argv.strace){
				process.straceMode = true;
			}
			const filePath = path.resolve(process.cwd(), argv.file);
			if (!existsSync(filePath)) {
				log('File not found:'.red.bold, argv.file, ':end');
				return;
			}
			runFileWithArgv(filePath, { async: !process.stdin.isTTY, onlyCompile: argv.compile, watch: argv.watch });
		},
	)
	.command(
		'exec [code]',
		'Executes in REPL',
		(yargs) => {
			yargs
			.option('compile', {
				alias: 'c',
				describe: 'Compile and output the javascript',
				type: 'boolean',
			})
			.example('rew exec "print \'hi\'"', "Executes code")
			.example('echo "print \\hi\'" | rew exec', "Executes code from pipe")
			.example('rew exec "print \'hi\'" -- arg1', "Executes code with arguments")
			.example('echo "print \\hi\'" | rew exec -- arg1', "Executes code from pipe with arguments");
		},
		async (argv) => {
			const replFile = '/tmp/rew-'+generateRandomID()+'-'+Date.now()+'.coffee';
			let code = argv.code;
			if(!process.stdin.isTTY) {
				code = await getAllPipeInput();
			}
			writeFileSync(replFile, code);
			try{
				runFileWithArgv(replFile, { async: !process.stdin.isTTY, onlyCompile: argv.compile });
			} catch(e){
				console.error(e);
			} finally {
				unlinkSync(replFile);
			}
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
				})
				.example('rew conf get', 'Get all the possible conf-sets')
				.example('rew conf get app.package', 'Get all files for app.package')
				.example('rew conf get app.package/_default', 'Get all default configs for app.package')
				.example('rew conf get app.package/[optionGroup]', 'Get all configs for app.package/[optionGroup]')
				.example('rew conf get app.package/[optionGroup] [key]', 'Get value of \'key\' for app.package/[optionGroup]')
				.example('rew conf get app.package/app', 'List all files for the app if the app is installed')
				.example('rew conf get app.package/path/to/file', 'Reads file or lists directory')

				.example('rew conf set app.package/[optionGroup] [key] "[value]"', "Sets 'key' to 'value' for app.package/[optionGroup]")
				.example('rew conf remove app.package/[optionGroup] [key]', "Removes 'key' from app.package/[optionGroup]");
		},
		(argv) => {
			const { command, path, key, value } = argv;
			const result = require('./utils').conf(command, path, key, value);
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
			}).option('git', {
				alias: 'g',
				describe: `Enable Git Option`,
				type: 'boolean',
			}).option('civet', {
				alias: 'c',
				describe: `Use civet for main`,
				type: 'boolean',
			}).option('types', {
				alias: 't',
				describe: `Create @types/rew in node modules`,
				type: 'boolean',
			}).option('name', {
				alias: 'n',
				describe: `The package name`,
				type: 'string'
			}).option('ignore', {
				alias: 'i',
				describe: `Use default options`,
				type: 'boolean',
				default: false
			})
			.example('rew create /path/to/project', 'Open interactive shell to create app at path')
			.example('rew create -i /path/to/project', 'Create without interactive shell')
			.example('rew create -n package.name /path/to/project', 'Create with "-n" as a package name')
			.example('rew create -t /path/to/project', 'Enable types')
			.example('rew create -c /path/to/project', 'Enable civet for main')
			.example('rew create -g /path/to/project', 'Setup git')
			.example('rew create -git /path/to/project', '(Recommended) Setup git, no interactive shell, enable types.')
			.example('rew create -gitn package.name /path/to/project', '(Recommended) Setup git, no interactive shell, enable types, set package name.');
		},
		(argv) => {
			require('./utils').createProject(argv.path, argv);
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
					describe: `If your entry file is a .qrew, then just use .coffe or ${REW_FILE_TYPE.EXTENSION} instead`,
					type: 'boolean',
				})
				.option('entry', {
					alias: 'e',
					describe: 'Choose entry file from app.config.exec',
					type: 'string',
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
				})
				.example('rew run .', "Run the current directory as an app")
				.example('rew run package.name', "Run 'package.name' if it's installed")
				.example('rew run . -b', "Build files into .qrew before running")
				.example('rew run . --dev -b', `If current entry file ends with .qrew it changes it to .coffee or ${REW_FILE_TYPE.EXTENSION} instead, and builds it before running the build`)
				.example('rew run . -e test', "Runs the 'test' entry from the 'app.yaml' if exists")
		},
		(argv) => {
			require('./utils').runApp(argv.path, argv);
		},
	)
	.command(
		'secret <command> [key]',
		'Add secrets to the current path',
		(yargs) => {
			yargs
				.positional('command', {
					describe: 'Path of the app to run',
					type: 'string',
				})
				.option('file', {
					alias: 'f',
					describe: 'Set file name',
					type: 'string',
					default: 'secrets.qrew'
				})
		},
		(argv) => {
			const appPath = findAppInfo(path.join(process.cwd(), 'app.yaml'));

			if (!appPath) return log(''.red.bold, 'Secrets only available in apps'.red.bold, ':end');

			const qrewPath = path.join(appPath.path, argv.file || 'secrets.qrew');

			const getPass = () => gen_key(input('Secret Encryptor: '));//`${process.env.USER}@${os.platform()}.${os.hostname()}`;

			const verifyUser = (content) => {
				const owner = content.match(/^owner = "(.*)" # end$/m)?.[1];
				if (owner == getPass()) return true;
				return false;
			};

			if (argv.command == 'init') {
				writeFileSync(qrewPath, to_qrew(`secrets = {} # end\n\nowner = "${getPass()}" # end\n \nexports { ...secrets }`, appPath.config.manifest.package))
			} else {
				const currentFileContent = from_qrew(readFileSync(qrewPath), appPath.config.manifest.package).toString();
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

					writeFileSync(qrewPath, to_qrew(newFileContent, appPath.config.manifest.package))
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
			}).option('requirements', {
				alias: 'r',
				describe: 'Install requirements of the app',
				type: 'boolean',
			}).option('verbose', {
				alias: 'v',
				describe: 'Verbose',
				type: 'boolean',
			}).option('update', {
				alias: 'u',
				describe: 'Update the app',
				type: 'boolean',
			}).option('yes', {
				alias: 'y',
				describe: 'Auto yes',
				type: 'boolean',
			})
			.example('rew install /path/to/app', "Installs path into the rew conf directory.")
			.example('rew install -r /path/to/app', "Installs all the required libraries/apps for the specified app.")
			.example('rew install github:username/repo', "Clones the repo and installs it to rew")
			.example('rew install -yu github:username/repo', "Installs from github without asking for confirmation and auto updates")
			.example('rew install github:username/repo@branch', "Installs a specific branch from github(default is main)")
			.example('rew install github:username/repo#commit', "Installs a specific commit from github(default is latest)")
			.example('rew install github:username/repo@branch#commit', "Installs a specific commit from a branch in a github repository");
		},
		async (argv) => {
			if (argv.requirements) require('./utils').installReq(argv.path, argv);
			else require('./utils').installAppFrom(argv.path, argv);
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
			})
			.example('rew uninstall package.name', "Uninstalls app package.name, but keeps the data/configs")
			.example('rew uninstall -a package.name', "Uninstalls app package.name entirely");
		},
		async (argv) => {
			require('./utils').uninstall(argv.package, argv.all);
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
				try {
					return (await req(`https://registry.npmjs.org/${pkg.name}`)).data['dist-tags'].latest
				} catch (e) {
					return `(${'!err'.blue.bgRed}, see ${`https://npmjs.com/package/${pkg.name}`.blue.underline})`;
				}
			}
			log(`${'Rew'.red.bold} ${'RUNTIME'.yellow}`);
			log(`Version: ${pkg.name.green.bold}@${pkg.version.yellow.bold}`.magenta.bold);
			const latest = await getLatest();
			const isLatest = latest === pkg.version;
			log(`Latest: ${pkg.name.cyan.bold}@${latest.yellow.bold}`.green.bold, isLatest ? ':end' : '');
			if (!isLatest) {
				log(`There is an update available`.cyan.bold);
				log('Update With:'.yellow, `npm i -g ${npm_package_name}`.green.bold, ':end');
			}
		},
	)
	.command(
		'cache <command>',
		'Manage cache',
		(yargs) => {
			yargs
			.positional('command', {
				describe: 'Command to clear/list/show',
				type: 'string',
			})
			.example('rew cache list', 'Lists all caches')
			.example('rew cache clear', 'Clears all caches')
			.example('rew cache clear all', 'Clears all caches')
			.example('rew cache clear [id]', 'Clears all caches for id [id]')
			.example('rew cache show [id]', 'Shows commits downloaded for [id]')
			.example('rew cache show [id]', 'Shows commits downloaded for [id]')
			.example('rew cache show [id]/clone', 'Shows the contents for [id]')
			.example('rew cache show [id]#tag', 'Shows the tag for [id]')
			.example('rew cache show [id]#commit', 'Shows the current commit for [id]')
			.example('rew cache show [id]#name', 'Shows the name for [id]')
			.example('rew cache show [id]/clone/app.yaml', 'Shows the app config for [id]')
			.example('rew cache show [id]/clone/path/to/file', 'Gives you the path to the file inside [id]')
			.example('rew cache show [id]/clone/path/to/file', 'Gives you the path to the file inside [id]')
			.example('rew cache install [id]', 'Installs cache')
		},
		async (argv) => {
			require('./utils').cache(argv.command, ...argv._.slice(1));
		},
	)
	.command(
		'misc <command>',
		'Misc functions',
		(yargs) => {
			yargs.positional('command', {
				describe: 'Misc command name',
				type: 'string',
			})
			.example('rew misc types', 'Fixes types for libraries and rew runtime')
			.example('rew misc keygen', 'Generate a rune encryption key')
		},
		(argv) => {
			require('./miscUtils')[argv.command]?.(...argv._.slice(1));
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
			yargs.option('json', {
				describe: 'Return a json output',
				type: 'boolean',
			});
		},
		async (argv) => {
			require('./utils').repo(argv.command, argv.name, argv.url, argv);
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
					describe: 'Remove all .coffee and '+REW_FILE_TYPE.EXTENSION,
					type: 'boolean',
				});
		},
		(argv) => {
			require('./utils').build(argv);
		},
	)
	.help(!isFileGiven).argv;
} catch(e) {
	console.error(e);
	process.exit(1);
}