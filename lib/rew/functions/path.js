const path = require('path');

module.exports = (currentFile) => {
	const e = {};
	e.basename = (pathname, suffix) => path.basename(pathname, suffix);
	e.dirname = (pathname) => path.dirname(pathname);
	e.extname = (pathname) => path.extname(pathname);

	e.pjoin = (...paths) => path.join(...paths);
	e.presolve = (...paths) => path.resolve(...paths);

	return e;
};
