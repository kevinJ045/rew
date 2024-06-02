const path = require('path');
const fs = require('fs');

module.exports = {
	findPackage(pkg) {
		if (pkg == 'pkgs') return false;
		return fs.existsSync(path.resolve(__dirname, './' + pkg + '.js'));
	},
	getPackage(pkg) {
		return require('./' + pkg);
	},
};
