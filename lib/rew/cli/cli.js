#!/usr/bin/env node

const colors = require('colors');
const yargs = require('yargs/yargs');
const path = require('path');
const { hideBin } = require('yargs/helpers');
const { existsSync, readFileSync, writeFileSync, statSync, unlinkSync } = require('fs');
const { log } = require('./log');
const rune = require('../pkgs/rune');
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
				.option('compile', {
					alias: 'c',
					describe: 'Compile and output the javascript',
					type: 'boolean',
				});
		},
		(argv) => {
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
			});
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
				});
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
			});
		},
		(argv) => {
			require('./utils').createProject(argv.path, argv);
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
		'nodify',
		'Generate a js header file to run rew natively inside of nodejs',
		(yargs) => {
		},
		async (argv) => {
			const webpack = require('webpack');
			const TerserPlugin = require('terser-webpack-plugin');

			const compiler = webpack({
				entry: path.resolve(__dirname, '../misc/qexec.js'),
				output: {
					filename: 'out.js',
					path: path.resolve(process.cwd()),
					libraryTarget: 'commonjs2'
				},
				module: {
					rules: [
						{
							test: /\.js$/, // Handle .js files
							exclude: /node_modules/, // Exclude node_modules
							use: {
								loader: 'babel-loader', // Optional: Transpile with Babel
							},
						}
					],
				},
				resolve: {
					extensions: ['.js'],
					fallback: {
						fs: false, // Adjust Node.js built-ins if necessary
						path: false,
					},
				},
				externals: {
					'vite': 'commonjs vite', // Exclude Vite
					'bindings': 'commonjs bindings', // Exclude Vite
					'deasync': 'commonjs deasync', // Exclude Vite
				},
				ignoreWarnings: [
					{
						module: /esbuild\/lib\/main\.d\.ts/, // Ignore esbuild main.d.ts
					},
				],
				optimization: {
					minimize: true,
					minimizer: [new TerserPlugin({
						terserOptions: {
							mangle: false,
							compress: {
								drop_console: false,
							},
							format: {
								comments: false,
							},
						},
					})],
				},		
				mode: 'development',	
				target: 'node'
			})

			compiler.run((err, stats) => {
				if (err || stats.hasErrors()) {
					console.error('Webpack compilation error:', err || stats.toString('errors-only'));
					return;
				}
			
				console.log(stats.toString({
					chunks: false,  // Makes the build log less noisy
					colors: true,   // Shows colors in the console
				}));
			
				console.log('Bundling complete. Output file:', path.resolve(__dirname, 'dist/bundle.js'));
			});
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
				});
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
			});
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
			});
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
			yargs.positional('command', {
				describe: 'Command to clear/list',
				type: 'string',
			});
		},
		async (argv) => {
			require('./utils').cache(argv.command)
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