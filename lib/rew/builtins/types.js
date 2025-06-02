"no-compile"

//declare* "=typedef" = rew::types::typedef;
//declare* "=int" = rew::types::int;
//declare* "=str" = rew::types::str;
//declare* "=float" = rew::types::float;
//declare* "=num" = rew::types::num;
//declare* "=bool" = rew::types::bool;
//declare* "=typef" = rew::types::typef;
//declare* "=struct" = struct;

const _defaultConstructors = {
	string: String,
	array: Array,
	number: Number,
	bigint: BigInt,
	boolean: Boolean,
	symbol: Symbol,
	undefined: Object,
	object: Object,
	function: Function,
};

function getType(value) {
	return typeof value === 'object' ? (Array.isArray(value) ? 'array' : typeof value) : typeof value;
}

class Type{
	constructor(o){
		for(let i in o){
			this[i] = o[i];
		}
	}

}

function typedef(value, strict = false) {
	if(typeof value == "function" && value.type instanceof Type){
		value = value.type;
	}
	return value instanceof Type ? value : new Type({
		strict,
		defaultValue: value,
		class:
			typeof value == 'function'
				? value
				: typeof value === 'object' && value !== null && !Array.isArray(value)
					? value.constructor
					: _defaultConstructors[getType(value)],
		type: getType(value),
		isConstructed: typeof value === 'object' && value !== null && !Array.isArray(value),
		isEmpty: typeof value == 'object' ? !Object.keys(value).length : typeof value == 'string' ? value == '' : typeof value !== 'function',
		or(...others){
			return [this, ...others];
		}
	});
}

function typef(fn, returnType, argumentsTypes) {
	if(typeof returnType == "function"){
		const ref = fn;
		fn = returnType;
		returnType = ref;
	}
	if (typeof fn !== 'function') {
		throw new Error('First argument must be a function');
	}
	if (typeof returnType == 'function' && returnType.type instanceof Type) returnType = returnType.type;
	const requiredArguments = Array.isArray(argumentsTypes) ? argumentsTypes.filter(i => Array.isArray(i) ? !i.includes(null) : true) : [];
	const wrappedFn = function(...args){
		if(argumentsTypes && Array.isArray(argumentsTypes)){
			if(args.length !== requiredArguments.length && args.length !== argumentsTypes.length){
				throw new TypeError(`Function ${fn.name || '<anonymous>'} takes exactly ${requiredArguments.length} parameters`)
			}	
			const argumentsTyped = typeAre(args, argumentsTypes);
			if(argumentsTyped !== false){
				throw new TypeError(`Function ${fn.name || '<anonymous>'} call error: Parameter at index ${argumentsTyped} is of the wrong type`);
			}
		}
		const result = fn.call(this, ...args);
		if(!typeis(result, wrappedFn.returnType)){
			throw new TypeError(`Function ${fn.name || '<anonymous>'} does not return it's own return type.`);
		}
		return result;
	}
	wrappedFn.returnType = returnType;
	wrappedFn.type = returnType;
	wrappedFn.argumentsTypes = argumentsTypes;
	return wrappedFn;
}
typef.is = function(func, returnType, argumentsTypes){
	return typeis(func.returnType.defaultValue, returnType);
}

const typeAre = (values, types) => {
	const verified = values.map((t, i) => Array.isArray(types[i]) ? (types[i].map((t2) => typeis(t, t2)).includes(true)) : typeis(t, types[i]));
	const hasWrong = verified.indexOf(false);
	return hasWrong > -1 ? hasWrong : false;
}

function typeis(obj, typeDef, missingObjects = false) {

	
	if(Array.isArray(typeDef)){
		return typeDef.some((item) => typeis(obj, item));
	}

	if(obj == null && typeDef === null) return true;
	else if(obj == null) return false;
	if(obj == undefined && typeDef === undefined) return true;
	else if(obj == undefined) return false;

	if(typeDef == null && obj === null) return true;
	else if(typeDef == null) return false;
	if(typeDef == undefined && obj === undefined) return true;
	else if(typeDef == undefined) return false;


	// Resolve Type
	if (typeof typeDef == 'function' && typeDef.type instanceof Type) typeDef = typeDef.type;
	else if(typeof obj == "object" && typeof typeDef == "function" && obj instanceof typeDef) return true;

	if (typeDef.isConstructed && typeDef.class && !(obj instanceof typeDef.class)) {
		return missingObjects ? [false] : false;
	}

	if (getType(obj) == 'object' && typeDef.type == 'function') {
		return missingObjects ? [obj instanceof typeDef.class] : obj instanceof typeDef.class;
	}

	if (getType(obj) !== typeDef.type) {
		return missingObjects ? [false] : false;
	}

	if (!typeDef.isEmpty) {
		if (typeDef.type == 'object') {
			for (const key in typeDef.defaultValue) {
				let propTypeDef = typeDef.defaultValue[key];
				// Resolve type
				if (typeof propTypeDef == 'function' && propTypeDef.type) propTypeDef = propTypeDef.type;

				if (typeof propTypeDef === 'object') {
					if (!typeis(obj[key], propTypeDef)) {
						return missingObjects ? [false, {
							[key]: {
								type_mismatch: propTypeDef,
								given: obj[gen_key]
							}
						}] : false;
					}
				} else if (typeof obj[key] !== typeof propTypeDef) {
					return missingObjects ? [false, {
						[key]: obj[key] ? {
							type_mismatch: typeof propTypeDef,
							given: typeof obj[key]
						} : {
							not_found: true
						}
					}] : false;
				}
			}
			if (typeDef.strict) {
				if (Object.keys(obj).some((key) => !Object.keys(typeDef.defaultValue).includes(key))) return missingObjects ?
					[false, Object.fromEntries(Object.keys(obj).filter((key) => !Object.keys(typeDef.defaultValue).includes(key)).map((key) => [key, { is_extra: true }]))]
				: false;
			}
		} else if (typeDef.type == 'string') {
			return typeDef.defaultValue == obj;
		} else if (typeDef.type == 'function') {
			return typeDef.defaultValue == obj;
		}
	}

	return missingObjects ? [true] : true;
}
typeis.multi = (values, types) => typeAre(values, types);

function typex(child, parent) {
	return child.prototype instanceof parent || child === parent;
}

function typei(child, parent) {
	return child instanceof parent || child.constructor === parent;
}

const _supportsFor = (item) => {
	item.or = (...others) => [item, ...others]
}
function int(str) {
	return parseInt(str);
}
int.type = typedef(1);
_supportsFor(int);

function float(str) {
	return parseFloat(str);
}
float.type = typedef(1.0);
_supportsFor(float);

function num(str) {
	return Number(str);
}
_supportsFor(num);
num.type = typedef(1);

function str(str) {
	return str ? str.toString() : '';
}
str.type = typedef('');
_supportsFor(str);

function bool(value) {
	return typeof value == 'string' ? (value == 'true' ? true : false) : value !== null && value !== undefined;
}
bool.type = typedef(true);
_supportsFor(bool);

const SerializableData = ['string', 'number', 'boolean'];
const isRegExp = (obj) => Object.prototype.toString.call(obj) === '[object RegExp]';
const AnySymbol = Symbol('any');
const ExistsSymbol = Symbol('exists');

function deepMatch(obj, pattern) {

	if (pattern instanceof RegExp && typeof obj === 'string') {
		return pattern.test(obj);
	}

	if (typeis(obj, pattern)) {
		return true;
	}

	if (pattern === null || obj === null) return pattern === obj;
	if (typeof pattern !== 'object' || typeof obj !== 'object') return pattern === obj;


	for (const key of Object.keys(pattern)) {
		const expected = pattern[key];

		if (!(key in obj)) {
			if (expected === ExistsSymbol) continue; // allow ExistsSymbol to pass if key is missing
			return false;
		}

		const actual = obj[key];

		if (expected === ExistsSymbol) {
			// Just existence check
			continue;
		} else if (Array.isArray(expected)) {
			// Match if actual matches any of the values
			if (!expected.some(val => deepMatch(actual, val))) return false;
		} else if (!deepMatch(actual, expected)) {
			return false;
		}
	}

	return true;
}

function fixArray(arr) {
	let result = [];
	for (let i = 0; i < arr.length; i += 2) {
		const key = arr[i];
		const value = arr[i + 1];
		if (Array.isArray(key)) {
			for (let k of key) result.push([k, value]);
		} else {
			result.push([key, value]);
		}
	}
	return result;
}

function _raw_match(value, templates, props) {
	const entries = templates instanceof Map
		? templates.entries()
		: Array.isArray(templates)
			? fixArray(templates)
			: Object.entries(templates);

	let any = null;

	for (const [pattern, callback] of entries) {
		let matched = false;

		if (pattern === AnySymbol) {
			any = callback;
			continue;
		}

		if (typeof pattern === 'function') {
			matched = value instanceof pattern || pattern(value);
		} else if(pattern instanceof Struct) {
			matched =  value['@instance'] == pattern;
		} else if (isRegExp(pattern)) {
			matched = pattern.test(value);
		} else if (SerializableData.includes(typeof value)) {
			matched = pattern === value;
		} else if (typeof pattern === 'object') {
			matched = deepMatch(value, pattern);
		}

		if (matched && props) {
			if (typeof props === 'object') {
				matched = deepMatch(value, props);
			} else if (typeof props === 'function') {
				matched = props(pattern, value);
			}
		}

		if (matched) {
			return callback(...(isRegExp(pattern) ? pattern.exec(value) : [value]));
		}
	}

	if (any) {
		return any(value);
	}

	return null;
}

function match(value, props) {
	let templates = [];
	return {
		on(_case, fn) {
			templates.push(_case, fn);
			return this;
		},
		default(fn) {
			templates.push(AnySymbol, fn);
			return this;
		},
		get end() {
			return _raw_match(value, templates, props);
		}
	};
}


match.prototype.any = AnySymbol
match.prototype.exists = ExistsSymbol

function map(...args) {
	if (args.length % 2 !== 0) {
		throw new Error('Arguments must be in key-value pairs');
	}

	const result = new Map();
	for (let i = 0; i < args.length; i += 2) {
		const key = args[i];
		const value = args[i + 1];
		// rew.prototype.io.prototype.out.print(key, value);
		result.set(key, value);
	}

	return result;
};

Object.without = function(object, ...keys){
	let newObject = {...object};
	for(let i = 0; i < keys.length; i++){
		delete newObject[keys[i]];
	}
	return newObject;
}

class Struct {
	#template = {};
	#types = {};
	constructor(a, t){
		this.#template = a;
		this.#types = t;
	}

	validate(properties){
		let instance = {};
		for (let key in this.#template) {
			let defaultValue = this.#template[key];
			if(key.startsWith('@') && typeof this.#template[key] == "function"){
				const realname = key.slice(1);
				instance[realname] = defaultValue(properties[realname]);
			} else if (key in properties) {
				let value = properties[key];
				if (defaultValue != '!any' && typeof value !== this.#types[key] && this.#types[key] !== '!any' && !typeis(value, this.#types[key])) {
					return [false, (this.#types[key]?.type?.type ?? this.#types[key]), key, typeof value];
				}
				instance[key] = value;
			} else {
				instance[key] = defaultValue == '!any' ? null : defaultValue?.type instanceof Type ? defaultValue.type.defaultValue : defaultValue;
			}
		}
		return instance;
	}

}
function struct(template) {
	var key, types, value;

	types = {};
	for (key in template) {
		value = template[key];
		types[key] = typeof template[key] == 'function' && template[key].type instanceof Type ? template[key] : typeof value;
	}

	let s = new Struct(template, types);
	s.prototype = {};
	s.prototype.extends = (stuff) => struct({ ...template, ...stuff });
	s.prototype.new = function StructFactory(properties, extra){
		var instance = s.validate(properties);
		if(instance?.[0] == false){
			throw new Error(`Type error: Expected ${instance[1]} for ${instance[2]}, got ${instance[3]}`);
		}
		if(typeof extra == "object"){
			for(let i in extra){
				instance[i] = extra[i];
			}
		}
		instance.__proto__ = { '@instance': s };
		return instance;
	}
	return s;
};


function macro(_, _fn){
  return function(name, ...args){
    let fn = args.pop()
    let full_args = args.length == 1 && args[0] == null ? [] : args;
    return _fn(name, fn, ...full_args);
	};
}

function proto(name, ...args){
	let _strict = false;
	let fn = args.pop()
	let full_args = args.length == 1 && args[0] == null ? [] : args;
	full_args = full_args.filter(i => {
		if(i == "strict"){
			_strict = true;
			return false;
		} else return true;
	});
	let parameter_types = !full_args.length ? [[], undefined] :
		full_args.length == 1 ? [[], full_args[0]] :
		[
			Array.isArray(full_args[0]) ? full_args[0] : [full_args[0]],
			full_args[1]
		];
	return function(...args){
		const checked_args = args.map((arg, index) => {
			// rew.prototype.io.prototype.out.print("ARGS", arg, parameter_types[0]);
			if(typeis(arg, parameter_types[0][index])){
				return arg;
			} else if(_strict){
				throw new TypeError(`Argument for function ${name || '<anonymous>'} at index ${index} is of the wrong type.`);
			} else if(typeof parameter_types[0][index] == "function"){
				return parameter_types[0][index](arg);
			}
			return arg;
		});
		const result = fn.call(this, ...checked_args);
		if(typeis(result, parameter_types[1])){
			return result;
		} else if(_strict){
			throw new TypeError(`Function ${name || '<anonymous>'} returned the wrong type.`);
		} else if(typeof parameter_types[1] == "function"){
			return parameter_types[1](result);
		}
		return result;
	}
}
proto.strict = (name, ...a) => proto(name, "strict", ...a);

if(!rew.extensions.has('types')) rew.extensions.add('types', () => rew.extensions.createClass({
	_namespace(){
		return this;
	},
	typex,
	typei,
	typeis,
	typedef,
	typef,
	match,
	map,
	int,
	float,
	num,
	str,
	bool,
	struct,
	macro,
	proto
}));