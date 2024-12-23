function isEmpty(value) {
	if (Array.isArray(value) || typeof value === 'string') {
		return value.length === 0;
	} else if (typeof value === 'object') {
		return Object.keys(value).length === 0;
	} else {
		return true;
	}
}

function clone(value) {
	if (Array.isArray(value)) {
		return value.slice();
	} else if (typeof value === 'object') {
		return Object.assign({}, value);
	} else {
		return value;
	}
}

function deepClone(value) {
	if (Array.isArray(value)) {
		return value.map((item) => deepClone(item));
	} else if (typeof value === 'object') {
		const obj = {};
		for (const key in value) {
			if (value.hasOwnProperty(key)) {
				obj[key] = deepClone(value[key]);
			}
		}
		return obj;
	} else {
		return value;
	}
}

function isObject(item) {
  return (item && typeof item === 'object' && !Array.isArray(item));
}

function deepMerge(target, ...sources) {
  if (!sources.length) return target;
  const source = sources.shift();

  if (isObject(target) && isObject(source)) {
    for (const key in source) {
      if (isObject(source[key])) {
        if (!target[key]) Object.assign(target, { [key]: {} });
        deepMerge(target[key], source[key]);
      } else {
        Object.assign(target, { [key]: source[key] });
      }
    }
  }

  return deepMerge(target, ...sources);
}

function merge(obj1, obj2) {
	return Object.assign({}, obj1, obj2);
}

const uniqueId = (() => {
	let id = 0;
	return () => {
		id += 1;
		return `id-${id}`;
	};
})();

function filter(arr, fn) {
	return arr.filter(fn);
}

function reduce(arr, fn, initial) {
	return arr.reduce(fn, initial);
}

function compose(...fns) {
	return (initialValue) => {
		return fns.reduceRight((acc, fn) => fn(acc), initialValue);
	};
}

function curry(fn) {
	const curried = (...args) => {
		if (args.length >= fn.length) {
			return fn.apply(null, args);
		} else {
			return (...moreArgs) => curried.apply(null, args.concat(moreArgs));
		}
	};
	return curried;
}

function getters(object, getters) {
	for (let prop in getters) {
		if (getters.hasOwnProperty(prop) && typeof getters[prop] === 'function') {
			Object.defineProperty(object, prop, {
				get: getters[prop],
				enumerable: true,
				configurable: true
			});
		}
	}
}

function setters(object, setters) {
	for (let prop in setters) {
		if (setters.hasOwnProperty(prop) && typeof setters[prop] === 'function') {
			Object.defineProperty(object, prop, {
				set: setters[prop],
				enumerable: true,
				configurable: true
			});
		}
	}
}

module.exports = {
	isEmpty,
	clone,
	deepClone,
	merge,
	deepMerge,
	uniqueId,
	filter,
	reduce,
	compose,
	curry,
	getters,
	setters
};
