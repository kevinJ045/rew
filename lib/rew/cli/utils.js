const path = require('path');
const fs = require('fs');
const conf = require('../pkgs/conf');
const jsYaml = require('js-yaml');
const readline = require('readline');
const { log, logget } = require('./log');
const { execSync, exec } = require('child_process');
const { run } = require('../main');
const { generateRandomID } = require('../functions/id');

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
		log('Crating at', ppath);
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
				execSync('cd ' + projectPath + ' && git init .');
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
			log('Done.', ':end');
			rl.close();
		};
		if (!fs.existsSync(projectPath)) {
			rl.question(logget('Package Name: '), (pkg) => {
				if (pkg.trim()) {
					project.package = pkg.trim();
					rl.question(logget('Use git(y/N): '), (use_git) => {
						project.git = use_git.toLowerCase() == 'y' || use_git.toLowerCase() == 'yes';
						create();
					});
				} else {
					rl.close();
				}
			});
		} else {
			log(`Project ${ppath} already exists at ${projectPath}`, ':end');
			rl.close();
		}
	},
	runApp(pathOrPackage) {
		const apppath = path.resolve(process.cwd(), pathOrPackage);
		const appConfpath = path.join(apppath, 'app.yaml');

		const runAppRoot = (root, confPath) => {
			const c = jsYaml.load(fs.readFileSync(confPath, { encoding: 'utf-8' }));
			if (c.entry) {
				const r = path.resolve(root, c.entry);
				const mod_path = path.resolve(root, 'snode_moduless/@makano/rew');
				const mod_path_lib = path.join(mod_path, 'lib/rew/cli');
				if (fs.existsSync(mod_path) && __dirname !== mod_path_lib) {
					const mod_path_utilsjs = path.join(mod_path_lib, '../main.js');
					require(mod_path_utilsjs).run(r);
				} else run(r);
			}
		};

		if (fs.existsSync(apppath) && fs.existsSync(appConfpath)) {
			runAppRoot(apppath, appConfpath);
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
			log('Installing ' + pname);
			log('Package: ' + p.name + '@' + p.version);
			if (p.description) {
				log('Description: ' + p.description);
			}
			rl.question(logget('Install ' + pname + '? (y/N)'), (f) => {
				if (f.toLowerCase() == 'y') {
					if (fs.existsSync(installPath)) {
						execSync(`rm -r ${installPath}`);
					}
					execSync(`cp -r ${apppath} ${installPath}`);
					execSync(`chmod 444 ${installPath}/app.yaml`);
					if (rmidir) {
						execSync(`rm -r ${apppath}`);
					}
					log('Installed ' + pname, ':end');
					rl.close();
				} else {
					if (rmidiri) {
						execSync(`rm -r ${apppath}`);
					}
					log('Canceled install', ':end');
					rl.close();
				}
			});
		} else {
			log('Path is not a rew app', ':end');
		}
	},
	async cloneGit(gitpath) {
		const p = gitpath.split('github:')[1];
		const url = `https://github.com/${p}`;
		const apiurl = `https://api.github.com/repos/${p}`;
		return await fetch(apiurl)
			.then((r) => {
				if (r.status !== 200) return log('Repo not found', ':end');
				log('Cloning from github');
				const tempPath = '/tmp/rew-git-clone-' + p.replace(/\//g, '_') + '-' + generateRandomID();
				execSync(`git clone ${url} ${tempPath}`, { stdio: 'ignore' });
				log('Installing deps...');
				execSync(`cd ${tempPath} && npm i`);
				return tempPath;
			})
			.catch((r) => log('Repo not found', ':end'));
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
			const text = await (await fetch(repoUrl.startsWith('//.') ? 'http://'+repoUrl.slice(3) : repoUrl.startsWith('//') ? 'https://'+repoUrl : repoUrl)).text();
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
			return {};
		}
	},
  async fromRepo(repoAndPkg){
    const [repo, pkg] = repoAndPkg.slice(1).split('/');
		const repoUrl = this.findRepo(repo);
		if(!repoUrl){
		 	log(`Repository "${repo}" not found.`);
			return log(`Add with "rew repo add ${repo} URL"`, ':end');
		} else {
			const repoJson = await this.getRepoJson(repoUrl);
			if(repoJson?.packages?.[pkg]){
				await this.installAppFrom(repoJson.packages[pkg]);
			} else {
				log(`Package "${pkg}" is not in repo "${repo}"`, ":end");
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
				const url = confInstance.get(key) || {};
				if(!url) return log('Repo not found', ':end');
				const json = await this.getRepoJson(url);
				if(json.name) log(json.name);
				log('Packages:')
				if(json.packages) Object.keys(json.packages).forEach(name => log(name)) || log(`${Object.keys(json.packages).length} Packages in ${key}`, ':end');
				else log('none', ':end')
			} else {
				console.log(Object.keys(repos).join('\n'));
			}
		} else if (command === 'delete') {
			confInstance.remove('repos');
		} else {
			log('Invalid command', ':end');
		}
	},	
	initFirst(){
		conf({}).create('').optionCenter('repos').set('rewpkgs', '//raw.githubusercontent.com/kevinJ045/rewpkgs/main/main.yaml');
	}
};
