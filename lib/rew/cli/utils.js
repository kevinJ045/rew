const path = require('path');
const fs = require('fs');
const conf = require('../pkgs/conf');
const jsYaml = require('js-yaml');
const readline = require('readline');
const { log, logget } = require('./log');
const { execSync, exec } = require('child_process');
const { run } = require('../main');
const { generateRandomID } = require('../functions/id');
const { compile } = require('../modules/compiler');
const { to_qrew } = require('../qrew/compile');
const { findAppInfo } = require('../misc/findAppInfo');
const { req } = require('../misc/req');

const npm_package_name = '@makano/rew';

module.exports = {
	conf(command, fullPath, key, value) {
		const con = conf({});
		if (command == 'get') {
			if (!fullPath || fullPath == 'list') {
				return fs.readdirSync(con.CONFIG_PATH).join('\n');
			} else {
				const name = fullPath.indexOf('/') ? fullPath.split('/')[0] : fullPath;
				const dpath = fullPath.indexOf('/') ? fullPath.split('/').slice(1).join('/') : '';
				const root = con.create(name);
				if (dpath) {
					const fp = path.join(root.root, dpath);
					if (fs.existsSync(fp) && fs.statSync(fp).isDirectory()) {
						return fs.readdirSync(fp).join('\n');
					} else {
						const o = con.create(name).optionCenter(dpath);
						return key ? o.get(key) : o.getAll(true);
					}
				} else {
					return fs.readdirSync(root.root).join('\n');
				}
			}
		} else {
			const name = fullPath.indexOf('/') ? fullPath.split('/')[0] : fullPath;
			const dpath = fullPath.indexOf('/') ? fullPath.split('/')[1] : '';
			if (name && key) {
				const root = con.create(name);
				const o = dpath ? root.optionCenter(dpath) : root;
				if (command == 'set') {
					if (value) {
						o.set(key, value);
					} else {
						log('Value not specified', ':end');
					}
				} else {
					o.remove(key);
				}
			} else {
				log(!name ? 'Path not specified' : 'Key not specified', ':end');
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
			const entryFile = path.join(projectPath, 'main.coffee');
			fs.writeFileSync(confPath, jsYaml.dump({ package: project.package, entry: 'main.coffee' }));
			fs.writeFileSync(entryFile, `print("Hello World!")`);
			if (project.git) {
				fs.writeFileSync(path.join(projectPath, '.gitignore'), `node_modules/\npackage-lock.json`);
				execSync('cd ' + projectPath + ' && git init . && git branch -m main', { stdio: 'ignore' });
			}
			execSync('cd ' + projectPath + ' && npm init -y', { stdio: 'ignore' });
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
					rl.question(logget('󰊢 Use git(y/N): '.yellow.bold), (use_git) => {
						project.git = use_git.toLowerCase() == 'y' || use_git.toLowerCase() == 'yes';
						create();
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
			if (c.entry) {
				if(byPath && options.dev) c.entry = c.entry.endsWith('.qrew') ? c.entry.replace(/\.qrew$/, '.coffee') : c.entry;
				let r = path.resolve(root, c.entry);
				if(options.build) {
					this.build({
						file: r,
						translate: options.translate || false
					});
					r = path.resolve(root, c.entry.replace(new RegExp(path.extname(c.entry).replace('.', '\\.') + '$'), options.translate ? '.js' : '.qrew'));
				}
				const mod_path = path.resolve(root, 'snode_moduless/@makano/rew');
				const mod_path_lib = path.join(mod_path, 'lib/rew/cli');
				if (fs.existsSync(mod_path) && __dirname !== mod_path_lib) {
					const mod_path_utilsjs = path.join(mod_path_lib, '../main.js');
					require(mod_path_utilsjs).run(r);
				} else run(r);
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
	installApp(pathname, rmidir, rmidiri) {
		if (!pathname) {
			return;
		}
		const apppath = path.resolve(process.cwd(), pathname);
		const appConfpath = path.join(apppath, 'app.yaml');
		const appPackagepath = path.join(apppath, 'package.json');
		if (fs.existsSync(apppath) && fs.existsSync(appConfpath)) {
			const c = jsYaml.load(fs.readFileSync(appConfpath, { encoding: 'utf-8' }));
			const p = JSON.parse(fs.readFileSync(appPackagepath, { encoding: 'utf-8' }));
			const pname = c.package;
			const installPath = path.join(conf({}).create(pname).root, 'app');
			const rl = readline.createInterface({
				input: process.stdin,
				output: process.stdout,
			});
			log(' Installing '.blue + pname.green.bold);
			log(' Package'.blue + ': ' + p.name.green + '@' + p.version.yellow);
			if (p.description) {
				log(' Description'.blue + ': ' + p.description);
			}
			rl.question(logget('Install '.blue + pname.green.bold + '? (y/N) '), (f) => {
				if (f.toLowerCase() == 'y') {
					if (fs.existsSync(installPath)) {
						execSync(`rm -r ${installPath}`);
					}
					execSync(`cp -r ${apppath} ${installPath}`);
					execSync(`chmod 444 ${installPath}/app.yaml`);
					if (rmidir) {
						execSync(`rm -r ${apppath}`);
					}
					log(' Installed '.green + pname.cyan.bold, c.install ? '' : ':end');
					if(c.install){
						if(c.install.build){
							log(' Building'.blue);
							this.build({
								...c.install.build,
								file: path.join(installPath, c.install.build.file)
							});
						}
					}
					rl.close();
				} else {
					if (rmidiri) {
						execSync(`rm -r ${apppath}`);
					}
					log(' Canceled install'.red.bold, ':end');
					rl.close();
				}
			});
		} else {
			log(' Path is not a rew app'.red.bold, ':end');
		}
	},
	build(argv){
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
			if(argv.remove){
				fs.unlinkSync(filePath);
				log(`${'Removed'.red.bold}: ${filePath.yellow}`);
			}
		}

		function processFile(filePath, importsArray) {
			const content = readFile(filePath);
			const imports = extractImports(content);

			imports.forEach((importStatement) => {
				const importedFilePath = path.resolve(path.dirname(filePath), importStatement.url);
				if (!importsArray.some((importObj) => importObj.url === importStatement.url)) {
					if (fs.existsSync(importedFilePath)) {
						importsArray.push(importStatement);
						processFile(importedFilePath, importsArray);
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

			const compiled = argv.translate ? compile({ content }, {}) : to_qrew(content, appPath?.config?.package || path.basename(filePath).split('.').slice(0, -1).join('.'));
			writeCompiledFile(filePath, compiled);
		}

		const filePath = path.resolve(process.cwd(), argv.file);
		const importsArray = [];
		const outputDir = argv.output ? path.resolve(process.cwd(), argv.output) : null;
		log(' Start compile at'.yellow, (outputDir || 'default path').green);
		processFile(filePath, importsArray);
		log('󰈔 Compiled'.yellow, (importsArray.length + 1 + '').blue, `file${importsArray.length ? 's' : ''}.`.yellow, ':end');
	},
	async cloneGit(gitpath) {
		const p = gitpath.split('github:')[1];
		const url = `https://github.com/${p}`;
		const apiurl = `https://api.github.com/repos/${p}`;
		try{
			const response = await req(apiurl);
			if (response.status !== 200) return log(' Repo not found'.red.bold, ':end');
			log(''.blue, 'Cloning from github'.yellow);
			const tempPath = '/tmp/rew-git-clone-' + p.replace(/\//g, '_') + '-' + generateRandomID();
			execSync(`git clone ${url} ${tempPath}`, { stdio: 'ignore' });
			log(''.blue, 'Installing deps...'.yellow);
			execSync(`cd ${tempPath} && npm i`);
			return tempPath;
		} catch(e){
			log(' Repo not found'.red.bold, ':end');
		}
	},
	findRepo(repo){
		const repos = conf({}).create('').optionCenter('repos');
		return repos.get(repo);
	},
	async installAppFrom(path){
		if (path.startsWith('github:')) this.installApp(await this.cloneGit(path), true, true);
		else if(path.startsWith('@')) this.fromRepo(path);
		else this.installApp(path);
	},
	async getRepoJson(repoUrl){
		try{
			const text = (await req(repoUrl.startsWith('//.') ? 'http://'+repoUrl.slice(3) : repoUrl.startsWith('//') ? 'https://'+repoUrl : repoUrl)).data;
			const json = text.startsWith('---') || text.startsWith('%YAML') ? jsYaml.loadAll(text)[0] : JSON.parse(text);
			if(Array.isArray(json.include)){
				for(let i of json.include){
					json.packages = {
						...json.packages,
						...((await this.getRepoJson(i.startsWith('.') ? path.join(path.dirname(repoUrl), i) : i)).packages || {})
					};
				}
			}
			return json;
		} catch(e){
			log(` Fetch Error. Check your connection.`.red.bold);
			return {};
		}
	},
  async fromRepo(repoAndPkg){
    const [repo, pkg] = repoAndPkg.slice(1).split('/');
		const repoUrl = this.findRepo(repo);
		if(!repoUrl){
		 	log(` Repository "${repo.green}"`.red.bold, `not found.`.red.bold);
			return log(`Add with:`.yellow, '\n\t$'.green, `rew repo add ${repo} URL`.cyan.bold, ':end');
		} else {
			const repoJson = await this.getRepoJson(repoUrl);
			if(repoJson?.packages?.[pkg]){
				await this.installAppFrom(repoJson.packages[pkg]);
			} else {
				log(` Package "${pkg}" is not in repo "${repo.green}"`.red.bold, ":end");
			}
		}
  },
	async repo(command, key, value) {
		const confInstance = conf({}).create('').optionCenter('repos') || {};
	
		if (command === 'add' || command === 'set') {
			confInstance.set(key, value.replace('https://', '//').replace('http://', '//.'));
		} else if (command === 'get') {
			if (key) {
				console.log(confInstance.get(key) || 'Not found');
			} else {
				console.log(Object.keys(confInstance.getAll()).join('\n'));
			}
		} else if (command === 'view') {
			if (key) {
				const url = confInstance.get(key);
				if(!url) return log(' Repo not found'.red.bold, ':end');
				const json = await this.getRepoJson(url);
				if(json.name) log(json.name);
				log('Packages:'.yellow)
				if(json.packages) Object.keys(json.packages).forEach(name => log(name)) || log(`${Object.keys(json.packages).length} Packages in ${key}`, ':end');
				else log('None'.blue, ':end')
			} else {
				console.log(Object.keys(repos).join('\n'));
			}
		} else if (command === 'delete') {
			confInstance.remove('repos');
		} else {
			log(' Invalid command'.red.bold, ':end');
		}
	},	
	initFirst(){
		conf({}).create('').optionCenter('repos').set('rewpkgs', '//raw.githubusercontent.com/kevinJ045/rewpkgs/main/main.yaml');
	}
};
