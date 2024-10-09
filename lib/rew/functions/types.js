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

function int(str) {
	return parseInt(str);
}
int.type = typedef(1);

function float(str) {
	return parseFloat(str);
}
float.type = typedef(1.0);

function num(str) {
	return Number(str);
}
num.type = typedef(1);

function str(str) {
	return str ? str.toString() : '';
}
str.type = typedef('');

function bool(value) {
	return typeof value == 'string' ? (value == 'true' ? true : false) : value !== null && value !== undefined;
}
bool.type = typedef(true);

module.exports = {
	typex,
	typei,
	typeis,
	typedef,
	typef,

	int,
	float,
	num,
	str,
	bool,
};

// const f = typedef('');
// const fg = typedef({ g: f });

// const g = { g: 'sss' };

// class L {}

// class N {}

// class M extends N{}

// let n = new N
// let m = new M
// let l = new L

// let tn = typedef(n)

// // console.log(typeis(g, fg), typeis(1, f), typei('', String));
// console.log(typeis(l, tn));
