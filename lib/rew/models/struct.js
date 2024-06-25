const { generateRandomID } = require('../functions/id');

module.exports.struct = function struct(template) {
	var key, types, value;

	types = {};
	for (key in template) {
		value = template[key];
		types[key] = typeof value;
	}

	const fun = function (properties = {}, extra) {
		var defaultValue, instance;
		instance = {};
		for (key in template) {
			defaultValue = template[key];
			if(key.startsWith('@') && typeof template[key] == "function"){
				const realname = key.slice(1);
				instance[realname] = defaultValue(properties[realname]);
			} else if (key in properties) {
				value = properties[key];
				if (defaultValue != '!any' && typeof value !== types[key] && types[key] !== '!any') {
					throw new Error(`Type error: Expected ${types[key]} for ${key}, got ${typeof value}`);
				}
				instance[key] = value;
			} else {
				instance[key] = defaultValue == '!any' ? null : defaultValue;
			}
		}
		if(typeof extra == "object"){
			for(let i in extra){
				instance[i] = extra[i];
			}
		}
		instance.__proto__ = { '@instance': fun };
		return instance;
	};

	fun.extends = (stuff) => struct({ ...template, ...stuff });

	return fun;
};

module.exports.struct.inherits = function (struct, template) {
	return module.exports.struct({
		...struct(),
		...template,
	});
};
