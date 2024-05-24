module.exports.struct = function struct(template) {
	var key, types, value;

	types = {};
	for (key in template) {
		value = template[key];
		types[key] = typeof value;
	}

	return function (properties = {}) {
		var defaultValue, instance;
		instance = {};
		for (key in template) {
			defaultValue = template[key];
			if (key in properties) {
				value = properties[key];
				if (defaultValue != '!any' && typeof value !== types[key]) {
					throw new Error(`Type error: Expected ${types[key]} for ${key}, got ${typeof value}`);
				}
				instance[key] = value;
			} else {
				instance[key] = defaultValue == '!any' ? null : defaultValue;
			}
		}
		return instance;
	};
};

module.exports.struct.inherits = function (struct, template) {
	return module.exports.struct({
		...struct(),
		...template,
	});
};
