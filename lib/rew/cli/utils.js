const path = require('path');
const fs = require('fs');
const conf = require('../pkgs/conf');
const jsYaml = require('js-yaml');
const readline = require('readline');
const { log, logget } = require('./log');
const { fork, execSync, exec } = require('child_process');
const { run } = require('../main');
const { generateRandomID } = require('../functions/id');
const { compile } = require('../modules/compiler');
const { to_qrew } = require('../qrew/compile');
const { findAppInfo } = require('../misc/findAppInfo');
const { req } = require('../misc/req');
const { CONFIG_PATH } = require('../const/config_path');
const { watch } = require('chokidar');
const { execRewFile } = require('./run');
const { seededID } = require('../misc/seededid');
const loading = require('loading-cli');
const sleep = require('../functions/sleep');
const { gen_key } = require('../misc/bin');
const { REW_FILE_TYPE } = require('../const/ext');

const binpath = path.join(conf({}).create('').root, '.bin');
const logspath = path.join(conf({}).create('').root, '.logs');
const cachepath = path.join(conf({}).create('').root, '.cache');
const localBinPath = path.join(binpath, '../../../', 'bin');

module.exports = {
	runFile(filePath, options = {}, argv) {
		const watching = [];
		const watchIt = (file) => {
			if (watching.includes(file)) return;
			watch(file).on('change', () => runIt());
			watching.push(file);
		};

		const runIt = () => {
			if (options.watch) console.clear();
			const imports = execRewFile(filePath, [filePath, ...(argv || [])], { onlyCompile: options?.onlyCompile, async: options?.async });
			if (options.watch) {
				imports.forEach((file) => {
					watchIt(file);
				});
				watchIt(filePath);
			}
		};

		runIt();
	},
	runFileWithArgv(filePath, options = {}, cargv) {
		let argv = cargv || process.argv;
		argv.shift();
		if (argv[0].endsWith(REW_FILE_TYPE.EXTENSION) || argv[0].endsWith('.coffee')) {
			if (argv[1] == 'run') {
				argv.splice(0, 3);
			} else if(argv[1] == '-w' || argv[1] == '--watch'){
				argv.splice(0, 3);
			} else argv.splice(0, 2);
		}
		if (argv[1] == 'exec') {
			argv.splice(0, 2);
		}
		if (argv.includes('--')) {
			argv = argv.slice(argv.indexOf('--') + 1, argv.length);
		}
		this.runFile(filePath, options, argv)
	},
	conf(command, fullPath, key, value) {
		const con = conf({});
		if (command == 'get') {
			if (!fullPath || fullPath == 'list') {
				return fs.readdirSync(con.CONFIG_PATH).join('\n');
			} else {
				let name = fullPath.indexOf('/') ? fullPath.split('/')[0] : fullPath;
				let dpath = fullPath.indexOf('/') ? fullPath.split('/').slice(1).join('/') : '';
				if(fullPath.startsWith('/')){ 
					dpath = name;
					name = '';
				}
				const root = con.create(name);
				if (dpath) {
					const fp = path.join(root.root, dpath);
					if (!fullPath.startsWith('/') && fs.existsSync(fp) && fs.statSync(fp).isDirectory()) {
						return fs.readdirSync(fp).join('\n');
					} else {
						const o = dpath && dpath !== '/' ? root.optionCenter(dpath) : root.optionCenter('_default');
						return key ? o.get(key) : o.getAll(true);
					}
				} else {
					return fs.readdirSync(root.root).join('\n');
				}
			}
		} else {
			let name = fullPath.indexOf('/') ? fullPath.split('/')[0] : fullPath;
			let dpath = fullPath.indexOf('/') ? fullPath.split('/')[1] : '';
			if(fullPath.startsWith('/')){ 
				dpath = name == '/' ? '_default' : name;
				name = '';
			}
			if (key) {
				const root = con.create(name);
				const o = dpath ? root.optionCenter(dpath) : root;
				if (command == 'set') {
					if (value) {
						o.set(key, value == 'false' || value == 'true' ? (value == 'true' ? true : false) : !isNaN(parseFloat(value)) ? parseFloat(value) : value);
					} else {
						log('Value not specified', ':end');
					}
				} else {
					o.remove(key);
				}
			} else {
				log('Key not specified', ':end');
			}
		}
	},
	createProject: (ppath) => {
		const projectPath = path.join(process.cwd(), ppath);
		log(''.cyan, 'Creating at'.blue, ppath.yellow);
		const rl = readline.createInterface({
			input: process.stdin,
			output: process.stdout,
		});
		const project = {};
		const create = () => {
			fs.mkdirSync(projectPath, { recursive: true });
			const confPath = path.join(projectPath, 'app.yaml');
			const entryFile = path.join(projectPath, 'main'+(project.civet ? REW_FILE_TYPE.EXTENSION : '.coffee'));
			fs.writeFileSync(confPath, jsYaml.dump({ manifest: { package: project.package, private: false }, exec: { entry: 'main'+(project.civet ? REW_FILE_TYPE.EXTENSION : '.coffee') }, assets: { icon: 'assets/icon.png', folder: './assets' }, install: { requirements: [] } }));
			fs.writeFileSync(entryFile, `print("Hello World!")`);
			fs.mkdirSync(path.join(projectPath, 'assets'), { recursive: true });
			if (project.git) {
				fs.writeFileSync(path.join(projectPath, '.gitignore'), `node_modules/\npackage-lock.json`);
				execSync('cd ' + projectPath + ' && git init . && git branch -m main', { stdio: 'ignore' });
			}
			if(project.intellisense){
				fs.copyFileSync(path.join(__dirname, '../../../tsconfig.json'), path.join(projectPath, 'tsconfig.json'));
				fs.copyFileSync(path.join(__dirname, '../../../runtime.d.ts'), path.join(projectPath, 'runtime.d.ts'));
			}
			execSync('cd ' + projectPath + ' && npm init -y', { stdio: 'ignore' });
			if(project.civet){
				log('Installing NPM Packages');
				execSync('cd '+projectPath+' && npm i @types/node --no-save', { stdio: 'ignore' });
			}
			// log('Installing '+npm_package_name);
			// exec('cd '+projectPath+' && npm i '+npm_package_name, (err) => {
			//   if(err){
			//     console.error(err);
			//     process.exit(0);
			//   } else {
			//     rl.close();
			//   }
			// });
			log('Done.'.blue.bold, ':end');
			rl.close();
		};
		if (!fs.existsSync(projectPath)) {
			rl.question(logget(' Package Name: '.blue), (pkg) => {
				if (pkg.trim()) {
					project.package = pkg.trim();
					rl.question(logget(' Use intellisense declarations ? (y/N): '.magenta.bold), (inteli) => {
						project.intellisense = inteli.toLowerCase() == 'y' || inteli.toLowerCase() == 'yes';
						rl.question(logget(' Use Civet For main ? (y/N): '.blue.bold), (civet) => {
							project.civet = civet.toLowerCase() == 'y' || civet.toLowerCase() == 'yes';
							rl.question(logget('󰊢 Use git ? (y/N): '.yellow.bold), (use_git) => {
								project.git = use_git.toLowerCase() == 'y' || use_git.toLowerCase() == 'yes';
								create();
							});
						});
					});
				} else {
					rl.close();
				}
			});
		} else {
			log(` Project ${ppath} already exists at ${projectPath}`.red.bold, ':end');
			rl.close();
		}
	},
	runApp(pathOrPackage, options) {
		const apppath = path.resolve(process.cwd(), pathOrPackage);
		const appConfpath = path.join(apppath, 'app.yaml');

		const runAppRoot = (root, confPath, byPath) => {
			const c = jsYaml.load(fs.readFileSync(confPath, { encoding: 'utf-8' }));
			if (options.entry) {
				c.exec.entry = c.exec[options.entry] || c.exec.entry;
			}
			if (c.exec.entry) {
				let r = path.resolve(root, c.exec.entry);
				if (byPath && options.dev) r = r.endsWith('.qrew') ? r.replace(/\.qrew$/, (a, b) => fs.existsSync(r.replace(a, '.coffee')) ? '.coffee' : REW_FILE_TYPE.EXTENSION) : r;
				if (options.build) {
					this.build({
						file: r,
						translate: options.translate || false
					});
					r = path.resolve(root, c.exec.entry.replace(new RegExp(path.extname(c.exec.entry).replace('.', '\\.') + '$'), options.translate ? '.js' : '.qrew'));
				}
				this.runFileWithArgv(r, { async: !process.stdin.isTTY });
			}
		};

		if (fs.existsSync(apppath) && fs.existsSync(appConfpath)) {
			runAppRoot(apppath, appConfpath, true);
		} else {
			const con = conf({});
			const apppath = path.resolve(con.CONFIG_PATH, pathOrPackage, 'app');
			const appConfpath = path.join(apppath, 'app.yaml');
			if (fs.existsSync(apppath) && fs.existsSync(appConfpath)) {
				runAppRoot(apppath, appConfpath);
			}
		}
	},
	installApp(pathname, opts, rmidir, rmidiri) {
		if (!pathname) {
			return;
		}
		const apppath = path.resolve(process.cwd(), pathname);
		const appConfpath = path.join(apppath, 'app.yaml');
		const appPackagepath = path.join(apppath, 'package.json');
		if (fs.existsSync(apppath) && fs.existsSync(appConfpath)) {
			const c = jsYaml.load(fs.readFileSync(appConfpath, { encoding: 'utf-8' }));
			const p = JSON.parse(fs.readFileSync(appPackagepath, { encoding: 'utf-8' }));
			const pname = c.manifest.package;
			const installPath = path.join(conf({}).create(pname).root, 'app');
			const rl = readline.createInterface({
				input: process.stdin,
				output: process.stdout,
			});
			log(' Installing '.blue + pname.green.bold);
			log(' Package'.blue + ': ' + p.name.green + '@' + p.version.yellow);
			if (p.description) {
				log(' Description'.blue + '\n' + p.description.split('\n').map((i, ind, a) => '  '+(ind == 0 && a.length > 1 ? log.startPrefix : (a.length-1 == ind ? log.endPrefix : log.middlePrefix))+' '+i).join('\n'), ':end');
			}
			if (p.keywords && p.keywords.length) {
				log(' Tags'.blue + '\n  ' + log.endPrefix + p.keywords.map(i => '#'+i).join(' '), ':end')
			}
			const done = (f) => {
				if (f.toLowerCase() == 'y' || f.toLowerCase() == 'yes') {
					if (fs.existsSync(installPath)) {
						execSync(`rm -r ${installPath}`);
					}
					if (c.install?.build) {
						log(' Building'.blue);
						try{
							this.build({
								...c.install.build,
								file: path.join(apppath, c.exec[c.install.build.file] || c.install.build.file)
							});
						} catch(e){}
					}
					execSync(`cp -r ${apppath} ${installPath}`);
					execSync(`chmod 444 ${installPath}/app.yaml`);
					if (c.install) {
						if (c.install.commands) {
							for (let command of c.install.commands) {
								try{
									execSync(command.replace(/\$installPath/g, installPath), { stdio: 'inherit' });
								} catch(e){
									const logFile = path.join(logspath, 'logs-'+Date.now()+'.log');
									fs.writeFileSync(logFile, e.toString() +'\n'+ e.stack);
									log(` Command Failed: ${command}, check logs at ${logFile}`);
								}
							}
						}
						if (c.install.requirements) {
							this.installReq(c, opts);
						}
						if (c.install.file) {
							this.runFileWithArgv(path.join(installPath, c.exec[c.install.file] || c.install.file), {}, [installPath]);
						}
						if (c.install.exec) {
							// this.installReq(c);
							if(conf({}).create('').get('executables') == false){
								log(' Ignoring executables'.blue);
							} else {
								for (let i in c.install.exec) {
									let iff = c.install.exec[i];
									if (iff in c.exec) iff = c.exec[iff];
									const file = path.join(installPath, iff);
									const filepath = path.join(binpath, i);
									const binfp = path.join(localBinPath, i);
									if (!fs.existsSync(localBinPath)) fs.mkdirSync(localBinPath, { recursive: true });
									fs.writeFileSync(filepath, `#!/usr/bin/env bash\n#@app.${pname}\nrew ${file} $*`);
									fs.chmodSync(filepath, '755');
									if(fs.existsSync(binfp)) fs.unlinkSync(binfp);
									fs.linkSync(filepath, binfp);
								}
							}
						}

						log(' Installed '.green + pname.cyan.bold, ':end');
					}
					rl.close();
				} else {
					log(' Canceled install'.red.bold, ':end');
					rl.close();
				}
			};
			if (fs.existsSync(installPath) && !opts.update) {
				rl.close();
				log(` App Already Installed`.green.bold);
				return log(`  Run With --update or -u to update.`.green.bold, ':end');
			}
			if(opts.yes) done('y');
			else rl.question(logget('Install '.blue + pname.green.bold + '? (y/N) '), done);
		} else {
			log(' Path is not a rew app'.red.bold, ':end');
		}
	},
	installReq(config, opts) {
		if (typeof config !== "object") {
			const confPath = path.join(config, './app.yaml');
			if (!fs.existsSync(confPath)) return log(' Path is not a rew app'.red.bold, ':end');
			config = jsYaml.load(fs.readFileSync(confPath, { encoding: 'utf-8' }));
		}
		if (config.install?.requirements) {
			if (!Array.isArray(config.install.requirements)) return log(' Requirements must be an array'.red.bold, ':end');
			config.install.requirements.forEach(req => {
				log('Finding '.cyan + req.green);
				this.installAppFrom(req, opts);
			});
		}
	},
	build(argv) {
		function readFile(filePath) {
			return fs.readFileSync(filePath, { encoding: 'utf-8' });
		}

		function extractImports(content) {
			const customImportRegex = /(\w+)\s*=\s*(imp|inc)\s*['"](.+?)['"]/g;
			const jsImportRegex = /import\s+((?:\w+\s*,?\s*)?{?[^{]*}?)\s*from\s*['"](.+?)['"]/g;
			const imports = [];
			let match;

			while ((match = customImportRegex.exec(content)) !== null) {
				imports.push({ variable: match[1], url: match[3] });
			}

			while ((match = jsImportRegex.exec(content)) !== null) {
				const variables = match[1].trim().replace(/[{}]/g, '').split(',').map(v => v.trim()).filter(v => v);
				const url = match[2];
				variables.forEach(variable => {
					imports.push({ variable, url });
				});
			}

			return imports;
		}

		function writeCompiledFile(filePath, compiledCode) {
			const dirName = outputDir ? outputDir : path.dirname(filePath);
			if (!fs.existsSync(dirName)) fs.mkdirSync(dirName, { recursive: true });
			const baseName = path.basename(filePath, path.extname(filePath));
			const newFilePath = path.join(dirName, `${baseName}.${argv.translate ? 'js' : 'qrew'}`);
			fs.writeFileSync(newFilePath, compiledCode);
			log(`${'Compiled'.green.bold}: ${newFilePath.yellow}`);
			if (argv.remove) {
				fs.unlinkSync(filePath);
				log(`${'Removed'.red.bold}: ${filePath.yellow}`);
			}
		}

		function processFile(filePath, importsArray) {
			const content = readFile(filePath);
			const imports = argv.single ? [] : extractImports(content);

			imports.forEach((importStatement) => {
				const importedFilePath = path.resolve(path.dirname(filePath), importStatement.url);
				if (!importsArray.some((importObj) => importObj.url === importStatement.url)) {
					if (fs.existsSync(importedFilePath)) {
						importsArray.push(importStatement);
						processFile(importedFilePath, importsArray);
					} else if (fs.existsSync(importedFilePath + REW_FILE_TYPE.EXTENSION)) {
						importsArray.push(importStatement);
						processFile(importedFilePath + REW_FILE_TYPE.EXTENSION, importsArray);
					} else if (fs.existsSync(importedFilePath + '.coffee')) {
						importsArray.push(importStatement);
						processFile(importedFilePath + '.coffee', importsArray);
					} else if (fs.existsSync(importedFilePath + '.js')) {
						importsArray.push(importStatement);
						processFile(importedFilePath + '.js', importsArray);
					}
				}
			});

			const appPath = findAppInfo(filePath);

			const compiled = argv.translate ? compile({ content }, {}) : to_qrew(`"initFile ${filePath}"\n${path.basename(content)}`, appPath?.config?.manifest?.package || path.basename(filePath).split('.').slice(0, -1).join('.'));
			writeCompiledFile(filePath, compiled);
		}

		const filePath = path.resolve(process.cwd(), argv.file);
		const importsArray = [];
		const outputDir = argv.output ? path.resolve(process.cwd(), argv.output) : null;
		log(' Start compile at'.yellow, (outputDir || 'default path').green);
		processFile(filePath, importsArray);
		log('󰈔 Compiled'.yellow, (importsArray.length + 1 + '').blue, `file${importsArray.length ? 's' : ''}.`.yellow, ':end');
	},
	cache(command){
		if(command == 'list'){
			console.log(fs.readdirSync(cachepath).join('\n').trim());
		} else {
			fs.readdirSync(cachepath).forEach(file => fs.rmSync(path.join(cachepath, file), { recursive: true }));
		}
	},
	async cloneGit(gitpath, opts) {
		const p = gitpath.split('github:')[1];
		const clonePath = path.join(cachepath, 'rew-git-clone-'+gen_key(gitpath).substring(0, 12));
		const url = `https://github.com/${p}`;
		const apiurl = `https://api.github.com/repos/${p}/commits`;
		const load = loading("Finding Repo...".yellow).start();
		try {
			const response = await req(apiurl);
			if (response.status !== 200) {
				load.stop();
				return log(' Package not found in github'.red.bold, ':end');
			}
			let pull = false;
			if(fs.existsSync(clonePath)){
				if(fs.existsSync(path.join(clonePath, response.data[0].sha))){
					load.stop();
					log('Found Cache'.yellow);
					return clonePath+'/clone';
				} else {
					pull = true;
				}
			}
			fs.mkdirSync(clonePath, { recursive: true });
			fs.writeFileSync(path.join(clonePath, response.data[0].sha), '');
			load.text = 'Cloning from github'.blue.bold;
			await sleep(100)
			if(pull) execSync(`cd ${clonePath}/clone && git pull`, { stdio: opts.verbose ? 'inherit' : 'ignore' });
			else execSync(`git clone ${url} ${clonePath}/clone`, { stdio: opts.verbose ? 'pipe' : 'ignore' });
			load.text = 'Installing npm packages'.green.bold;
			await sleep(100)
			execSync(`cd ${clonePath}/clone && npm i`, { stdio: opts.verbose ? 'inherit' : 'ignore' });
			load.stop();
			return clonePath+'/clone';
		} catch (e) {
			const logFile = path.join(logspath, 'logs-'+Date.now()+'.log');
			fs.writeFileSync(logFile, e.toString() +'\n'+ e.stack);
			load.stop();
			log(' Something went wrong, check logs at'.red.bold, logFile.green, ':end');
		}
	},
	findRepo(repo) {
		const repos = conf({}).create('').optionCenter('repos');
		return repos.get(repo);
	},
	async installAppFrom(path, opts) {
		if (path.startsWith('github:')) this.installApp(await this.cloneGit(path, opts), opts, true);
		else if (path.startsWith('@')) this.fromRepo(path, opts);
		else this.installApp(path, opts, null, null);
	},
	uninstall(packageName, all) {
		const confPath = path.join(CONFIG_PATH, packageName);
		const apppath = path.resolve(confPath, 'app');
		const appConfpath = path.join(apppath, 'app.yaml');
		if (!fs.existsSync(appConfpath) && fs.existsSync(confPath) && !all) {
			log(` App ${packageName.green}`.red.bold, `not found`.red.bold, `but configs are found.`.green.bold);
			return log(`Use the`.cyan, '--all'.green, 'flag to remove them.'.cyan, ':end');
		} else if (!fs.existsSync(appConfpath) && !all) {
			return log(` App ${packageName.green}`.red.bold, `not found.`.red.bold, ':end');
		}
		log('Uninstalling'.cyan, packageName.green);
		execSync('rm -rf ' + (all ? confPath : apppath));
		fs.readdirSync(binpath)
			.forEach(filename => {
				const filepath = path.join(binpath, filename);
				const lfilepath = path.join(localBinPath, filename);
				const content = fs.readFileSync(filepath, { encoding: 'utf-8' });
				if (content.split('\n')[1].startsWith('#@app.' + packageName)) {
					fs.unlinkSync(lfilepath);
					fs.unlinkSync(filepath);
				}
			});
		log('Uninstalled'.cyan, ':end');
	},
	async getRepoJson(repoUrl) {
		try {
			const text = (await req(repoUrl.startsWith('//.') ? 'http://' + repoUrl.slice(3) : repoUrl.startsWith('//') ? 'https://' + repoUrl : repoUrl)).data;
			const json = text.startsWith('---') || text.startsWith('%YAML') ? jsYaml.loadAll(text)[0] : JSON.parse(text);
			if (Array.isArray(json.include)) {
				for (let i of json.include) {
					json.packages = {
						...json.packages,
						...((await this.getRepoJson(i.startsWith('.') ? path.join(path.dirname(repoUrl), i) : i)).packages || {})
					};
				}
			}
			return json;
		} catch (e) {
			log(` Fetch Error. Check your connection.`.red.bold);
			return {};
		}
	},
	async fromRepo(repoAndPkg, opts) {
		const [repo, pkg] = repoAndPkg.slice(1).split('/');
		const repoUrl = this.findRepo(repo);
		if (!repoUrl) {
			log(` Repository "${repo.green}"`.red.bold, `not found.`.red.bold);
			return log(`Add with:`.yellow, '\n\t$'.green, `rew repo add ${repo} URL`.cyan.bold, ':end');
		} else {
			const repoJson = await this.getRepoJson(repoUrl);
			if (repoJson?.packages?.[pkg]) {
				await this.installAppFrom(repoJson.packages[pkg], opts);
			} else {
				log(` Package "${pkg.cyan}" is not in repo "${repo.green}"`.red.bold, ":end");
			}
		}
	},
	async repo(command, key, value, options) {
		const confInstance = conf({}).create('').optionCenter('repos') || {};

		if (command === 'add' || command === 'set') {
			confInstance.set(key, value.replace('https://', '//').replace('http://', '//.'));
		} else if (command === 'get') {
			if (key) {
				console.log(confInstance.get(key) || 'Not found');
			} else {
				if(options.json) return console.log(JSON.stringify(confInstance.getAll()));
				console.log(Object.keys(confInstance.getAll()).join('\n'));
			}
		} else if (command === 'view') {
			if (key) {
				const url = confInstance.get(key);
				if (!url) return log(' Repo not found'.red.bold, ':end');
				const json = await this.getRepoJson(url);
				if(options.json) return console.log(JSON.stringify(json));
				if (json.name) log(json.name);
				log('Packages:'.yellow)
				if (json.packages) Object.keys(json.packages).forEach(name => log(name)) || log(`${Object.keys(json.packages).length} Packages in ${key}`, ':end');
				else log('None'.blue, ':end')
			} else {
				if(options.json) return JSON.stringify(confInstance.getAll());
				console.log(Object.keys(confInstance.getAll()).join('\n'));
			}
		} else if (command === 'delete') {
			confInstance.remove(key);
		} else {
			log(' Invalid command'.red.bold, ':end');
		}
	},
	initFirst() {
		log('First time init')
		conf({}).create('').optionCenter('repos').set('rewpkgs', '//raw.githubusercontent.com/kevinJ045/rewpkgs/main/main.yaml');
		fs.mkdirSync(binpath, { recursive: true });
		fs.mkdirSync(cachepath, { recursive: true });
		fs.mkdirSync(logspath, { recursive: true });
	},
	getAllPipeInput(){
		return new Promise((resolve) => {
			let data = '';
			process.stdin.setEncoding('utf8');
			
			process.stdin.on('data', (chunk) => {
				data += chunk;
			});
			
			process.stdin.on('end', () => {
				resolve(data);
			});
		});
	}
};
