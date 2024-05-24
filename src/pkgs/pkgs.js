const conf = require('./conf');
const ui = require('./ui');

const packages = {};

packages.ui = ui;
packages.conf = conf;

module.exports = {
	findPackage(pkg) {
		if (pkg in packages) return packages[pkg];
		return null;
	},
};
