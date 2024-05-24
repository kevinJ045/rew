module.exports.exportsFunction = function (context) {
	return function (item, name) {
		if (name) {
			context.module.exports[name] = item;
		} else {
			context.module.exports = item;
		}
	};
};
