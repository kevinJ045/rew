const path = require('path');
const fs = require('fs');
const conf = require('../pkgs/conf');
const jsYaml = require('js-yaml');
const readline = require('readline');
const { log, logget } = require('./log');
const { execSync } = require('child_process');
const { compile } = require('../modules/compiler');
const { to_qrew } = require('../qrew/compile');
const { findAppInfo } = require('../misc/findAppInfo');
const { req } = require('../misc/req');
const { CONFIG_PATH } = require('../const/config_path');
const { runFileWithArgv } = require('./run');
const loading = require('loading-cli');
const sleep = require('../functions/sleep');
const { gen_key } = require('../misc/bin');
const { REW_FILE_TYPE } = require('../const/ext');

const {
  binpath,
  logspath,
  cachepath,
  localBinPath,
	hashTags,
	FILE_DL_EXTRACT_REGEX
} = require('./helpers');
const { input } = require('../functions/stdout');



module.exports = {
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
	createProject: (ppath, argv) => {
		const projectPath = path.join(process.cwd(), ppath);
		log(''.cyan, 'Creating at'.blue, ppath.yellow);

		const b = (value, type) => type == "boolean" ? (value.toString() == 'y' || value.toString() == "yes" ? true : false) : value;

		const registerInput = (name, promptText, type, argvName, defaultValue) => {
			if(type == 'boolean') defaultValue = false;
			let prompted = false;
			project[name] = argv[argvName] ?? (argv.ignore ? defaultValue ?? b(input(promptText, prompted = log() || true), type) : b(input(promptText, prompted = log() || true), type));
			log(name.grey+'?'.grey, type == 'string' ? project[name].green : (project[name] == true ? 'yes'.cyan : 'no'.yellow), prompted ? ':nosep' : '');
		}

		const project = {};
		const create = () => {
			fs.mkdirSync(projectPath, { recursive: true });
			const confObj = { manifest: { package: project.package, private: false }, exec: { entry: 'main'+(project.civet ? REW_FILE_TYPE.EXTENSION : '.coffee') }, assets: { icon: 'assets/icon.png', folder: './assets' }, install: { requirements: [] } };
			const confPath = path.join(projectPath, 'app.yaml');
			const entryFile = path.join(projectPath, 'main'+(project.civet ? REW_FILE_TYPE.EXTENSION : '.coffee'));
			if(project.intellisense) {
				confObj.types = ['rew'];
			}
			fs.writeFileSync(confPath, jsYaml.dump(confObj));
			fs.writeFileSync(entryFile, `using namespace std::ns ->
  define Main class
    @main: (argv) ->
      print 'Hello, World!'`);
			fs.mkdirSync(path.join(projectPath, 'assets'), { recursive: true });
			if (project.git) {
				fs.writeFileSync(path.join(projectPath, '.gitignore'), `node_modules/\npackage-lock.json`);
				execSync('cd ' + projectPath + ' && git init . && git branch -m main', { stdio: 'ignore' });
			}
			if(project.intellisense){
				require('./miscUtils')
				.types(projectPath);
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
		};
		if (!fs.existsSync(projectPath)) {

			registerInput(
				'package',
				logget(' Package Name: '.blue),
				'string',
				'name',
				path.basename(projectPath)
			)

			registerInput(
				'intellisense',
				logget(' Use intellisense declarations ? (y/N): '.magenta),
				'boolean',
				'types'
			)

			registerInput(
				'civet',
				logget(' Use Civet For main ? (y/N): '.blue),
				'boolean',
				'civet'
			)

			registerInput(
				'git',
				logget('󰊢 Use git ? (y/N): '.yellow),
				'boolean',
				'git'
			)

			create();
		} else {
			log(` Project ${ppath} already exists at ${projectPath}`.red.bold, ':end');
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
				runFileWithArgv(r, { async: !process.stdin.isTTY });
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
			} else {
				log('App does not exist'.red.bold, ':end');
				process.exit(1);
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
					execSync(`${process.platform == "win32" ? 'copy' : 'cp'} -r ${apppath} ${installPath}`);
					if(process.platform != "win32") execSync(`chmod 444 ${installPath}/app.yaml`);
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
							runFileWithArgv(path.join(installPath, c.exec[c.install.file] || c.install.file), {}, [installPath]);
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
									fs.writeFileSync(filepath, `#!/usr/bin/env bash\n#@app.${pname}\nrew ${file} -- $*`);
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

			const compiled = argv.translate ? compile({ content }, {}) : to_qrew(`"initFile ${path.basename(filePath)}"\n${content}`, appPath?.config?.manifest?.package || path.basename(filePath).split('.').slice(0, -1).join('.'));
			writeCompiledFile(filePath, compiled);
		}

		const filePath = path.resolve(process.cwd(), argv.file);
		const importsArray = [];
		const outputDir = argv.output ? path.resolve(process.cwd(), argv.output) : null;
		log(' Start compile at'.yellow, (outputDir || 'default path').green);
		processFile(filePath, importsArray);
		log('󰈔 Compiled'.yellow, (importsArray.length + 1 + '').blue, `file${importsArray.length ? 's' : ''}.`.yellow, ':end');
	},
	cache(command, file){

		const findGitCommitPath = (p) => {
			const heads = path.join(p, '.git/refs/heads');
			if(!fs.existsSync(heads)) return '';
			const refs = fs.readdirSync(heads);
			if(!refs.length) return '';
			return fs.readFileSync(path.join(heads, refs[0]), { encoding: 'utf-8' }).trim();
		}

		if(command == 'list'){
			console.log(fs.readdirSync(cachepath).join('\n').trim());
		} else if(command == 'clear'){
			if(file && file !== 'all') fs.rmSync(path.join(cachepath, file), { recursive: true });
			else if(file == 'all' || !file) fs.readdirSync(cachepath).forEach(file => fs.rmSync(path.join(cachepath, file), { recursive: true }));
		} else if(command == 'install'){
			if(!file) return process.exit(1);
			this.installApp(path.join(cachepath, file, 'clone'), {
				update: true
			});
		} else if(command == 'show'){
			if(!file) return console.log(fs.readdirSync(cachepath).join('\n').trim());

			const hashed = hashTags(file, {
				tag: (str) => ({ $set: str + '#name#commit' }),
				commit: (file) => findGitCommitPath(path.join(cachepath, file.replace(/#name/, ''), 'clone')),
				name: (file) => jsYaml.load(fs.readFileSync(path.join(cachepath, file, 'clone/app.yaml'), { encoding: 'utf-8' }).trim()).manifest.package
			});
			file = hashed.string;

			if(hashed.tag){
				return console.log(hashed.name +':'+ hashed.commit);
			}
			if(hashed.commit) return console.log(hashed.commit);
			if(hashed.name) return console.log(hashed.name);

			const f = path.join(cachepath, file);
			if(!fs.existsSync(f)) return;
			if(fs.statSync(f).isDirectory()) console.log(
				fs.readdirSync(f).join('\n')
			);
			else if(file.endsWith('app.yaml')){
				console.log(fs.readFileSync(f, { encoding: 'utf-8' }).trim());
			} else console.log(f);
		}
	},
	async cloneGit(gitpath, opts) {
		let p = gitpath.split('github:')[1];
		const clonePath = path.join(cachepath, 'rew-git-clone-'+gen_key(gitpath).substring(0, 12));
		const load = loading("Finding Repo...".yellow).start();

		let branch = null; // Default branch
    let commit = null;

    // Extracting branch or commit if specified
    if (p.includes('@')) {
			[p, branch] = p.split('@');
    } 
		if (branch?.includes('#')) {
			[branch, commit] = branch.split('#');
    }
		if (p.includes('#')) {
			[p, commit] = p.split('#');
    }

		const url = `https://github.com/${p}`;
		const apiurl = `https://api.github.com/repos/${p}/commits`;

		try {
			const response = await req(apiurl);
			if(!commit) commit = response.data[0].sha;
			if (response.status !== 200) {
				load.stop();
				return log(' Package not found in github'.red.bold, ':end');
			}
			let pull = false;
			if(fs.existsSync(clonePath)){
				if(fs.existsSync(path.join(clonePath, commit))){
					load.stop();
					log('Found Cache'.yellow);
					return clonePath+'/clone';
				} else {
					pull = true;
				}
			}
			fs.mkdirSync(clonePath, { recursive: true });
			fs.writeFileSync(path.join(clonePath, commit), '');
			load.text = pull ? 'Updating repository'.blue.bold : 'Cloning from github'.blue.bold;
			await sleep(100)
			if(pull) execSync(`cd ${clonePath}/clone && git fetch --all${response.data[0].sha !== commit ? ` && git reset --hard ${commit}` : ''}`, { stdio: opts.verbose ? 'inherit' : 'ignore' });
			else execSync(`git clone ${url} ${clonePath}/clone && cd ${clonePath}/clone${response.data[0].sha !== commit ? ` && git reset --hard ${commit}` : ''}${branch ? ' && git checkout '+branch : ''}`, { stdio: opts.verbose ? 'pipe' : 'ignore' });
			if (fs.existsSync(path.join(clonePath, 'clone', 'package.json'))) {
				load.text = 'Installing npm packages'.green.bold;
				await sleep(100);
				execSync(`cd ${clonePath}/clone && npm install`, { stdio: opts.verbose ? 'inherit' : 'ignore' });
			}
			load.stop();
			return clonePath+'/clone';
		} catch (e) {
			const logFile = path.join(logspath, 'logs-'+Date.now()+'.log');
			fs.writeFileSync(logFile, e.toString() +'\n'+ e.stack);
			load.stop();
			if(opts.verbose) console.error(e);
			log(' Something went wrong, check logs at'.red.bold, logFile.green, ':end');
		}
	},
	findRepo(repo) {
		const repos = conf({}).create('').optionCenter('repos');
		return repos.get(repo);
	},
	isCommandAvailable(command) {
    try {
			execSync(`command -v ${command}`, { stdio: 'ignore' });
			return true;
    } catch (e) {
			return false;
    }
	},
	downloadFileFromUrl(url, dest, sha, opts) {
    const isCurlAvailable = this.isCommandAvailable('curl');
    const isWgetAvailable = this.isCommandAvailable('wget');
    
    if (!isCurlAvailable && !isWgetAvailable) {
			log(` Failed to download: Nither curl nor wget command were found.`.red.bold, ':end');
			return false;
		}

		if(opts.c || opts.clean){
			if(fs.existsSync(dest)) fs.rmSync(dest);
		}
		let tries = 0;

		const checkSha = (throwErr) => {
			if(sha){
				const thisSha = execSync(`sha256sum "${dest}"`).toString().split(' ')[0];
				if(thisSha === sha){
					return true;
				} else {
					if(throwErr) {
						log(` Failed to verify file checksum, ${tries == 0 ? 'trying again...' : 'try again.'}`.red.bold, ':end');
						fs.rmSync(dest);
						tries++;
						if(tries < 2) return tryDl();
					}
					return false;
				}
			} else return true;
		}
    
    const downloadCommand = isCurlAvailable 
			? `curl -L -o "${dest}" "${url}"` 
			: `wget -O "${dest}" "${url}"`;

		const tryDl = () => {
			try {
				if(fs.existsSync(dest)){
					return checkSha(true);
				} else {
					execSync(downloadCommand, { stdio: opts.verbose ? 'inherit' : 'ignore' });
					return checkSha(true);
				}
			} catch(e){
				log(` Failed to download ${url}: ${e.name}: ${e.message}.`.red.bold, ':end');
				return false;
			}
		}

		return tryDl();
	},
	downloadFile(fullPath, opts){
		const pathToDownloadInto = path.join(cachepath, 'rew-file-download-'+gen_key(fullPath).substring(0, 12));

		const commandMatch = fullPath.match(FILE_DL_EXTRACT_REGEX);

		let shaHash = null;
    let command = null;
    let args = [];
    let url = null;

    if (commandMatch) {
			// If sha() is present in the format file+sha(SHA-AAAA)+command(args):URL
			if (commandMatch[1]) {
				shaHash = commandMatch[1];
				command = commandMatch[2];
				args = commandMatch[3] ? commandMatch[3].slice(1, -1).split(' ') : [];
				url = commandMatch[4];
			} else {
				command = commandMatch[5];
				args = commandMatch[6] ? commandMatch[6].slice(1, -1).split(' ') : [];
				url = commandMatch[7];
			}

			fs.mkdirSync(pathToDownloadInto, { recursive: true });
    } else {
			log(` Format not correct`.red.bold, ':end');
			process.exit(1);
    }

		const fileName = path.basename(url);
		const filePath = path.join(pathToDownloadInto, fileName);

		const extractPath = path.join(pathToDownloadInto, 'extract');
		if(opts.c || opts.clean || opts.u){
			if(fs.existsSync(extractPath)) execSync(`rm -rf ${extractPath}`);
		}

    const fixedArgs = args.join(' ')
			.replaceAll('$path', extractPath)
			.replaceAll('$file', filePath) || (
				command == 'unzip' ?
				`${filePath} -d ${extractPath}` :
				`${filePath} ${extractPath}`
			);
		
		if(opts.verbose){
			log('Download started for'.cyan, url.yellow, 'at'.cyan, filePath.yellow, shaHash ? 'with sha'.cyan : '', shaHash ? shaHash.yellow : '');
		} else {
			log(`Downloading`.cyan, fileName.yellow);
		}
		const downloaded = fs.existsSync(extractPath) ? true : this.downloadFileFromUrl(url, filePath, shaHash, opts);
		if(downloaded){
			log(`Download Complete for`.cyan, fileName.yellow);
			log('Extracting'.cyan, fileName.yellow);
			if(opts.verbose){
				log('Running command:'.cyan, command.green.bold, 'for'.cyan, filePath.yellow, 'at'.cyan, extractPath.yellow);
			}
			if(this.isCommandAvailable(command)){
				try{
					if(fs.existsSync(extractPath)) {
						log('Found Extracted Directory'.cyan);
					} else execSync(`${command} ${fixedArgs}`, {
						stdio: opts.verbose ? 'inherit' : 'ignore'
					});
				} catch(e){
					log(` Failed to extract`.red.bold, ':end');
					return null;
				}
			} else {
				log(` Command "${command}" not found.`.red.bold, ':end');
				return null;
			}

			return extractPath;
		} else {
			return null;
		}
	},
	async installAppFrom(path, opts) {
		if (path.startsWith('github:')) this.installApp(await this.cloneGit(path, opts), opts, true);
		if (path.match(FILE_DL_EXTRACT_REGEX)) this.installApp(this.downloadFile(path, opts), opts, true);
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
	}
};
