const fs = require('fs');
const { v4: uuidv4 } = require('uuid');
const path = require('path');
const { CONFIG_PATH } = require('../const/config_path');
const { serializeData, deserializeData, gen_key } = require('../misc/bin');
const RuneDB = require('./modules/rune/db');
const { typeis } = require('../const/default');

const ENCRYPTION_KEY = 'e6ad8b0792b9e0472ea44d1f3adfd1d503182efcce25991b05cc5ef83f307ffc';
const PAGE_LIMIT = 100;

class Change {
	constructor(values) {
		this.values = values;
	}
}

class PopChange extends Change {}

class PushChange extends Change {}

const runePush = (...values) => new PushChange(values);
const runePop = (...values) => new PopChange(values);

function makeRef(value, props = '') {
	if (!value['@rune.id']) return null;
	const collection = getCollectionFromID(value['@rune.id']);
	const ref = collection + '.' + value['@rune.id'];
	return '@rune.ref ' + ref + props;
}

const eid = (s, diff) =>
	s
		.split('')
		.map((i) => {
			let charCode = i.charCodeAt(0) + diff;
			if (charCode > 122) {
				charCode -= 26;
			}
			return String.fromCharCode(charCode);
		})
		.join('');

function generateID(id, collection) {
	return eid(collection, 5) + '+' + id;
}

function getCollectionFromID(id) {
	return eid(id.split('+')[0], -5);
}

function coltypedef(cb) {
	const typedef = {};
	const ctx = {
		opt: (name, type, defaultValue) => {
			typedef[name] = type;
			if(typeof defaultValue !== "undefined"){
				if(!typedef['@rune.default']){
					typedef['@rune.default'] = {};
				}
				typedef['@rune.default'][name] = defaultValue;
			}
		},
		req: (name, type, defaultValue) => {
			if(!typedef['@rune.required']){
				typedef['@rune.required'] = {};
			}
			typedef['@rune.required'][name] = true;
			ctx.opt(name, type, defaultValue);
		},
		unique: (name, type, defaultValue) => {
			if(!typedef['@rune.unique']){
				typedef['@rune.unique'] = {};
			}
			typedef['@rune.unique'][name] = true;
			ctx.req(name, type, defaultValue);
		}
	};
	cb.call(ctx);
	return typedef;
}

const createDB = (dbName, dirname, dbData = {}, encryptionKey) => {
	const dbDirPath = path.join(dirname, dbName);
	const mainFilePath = path.join(dbDirPath, 'main.bin');

	if (!fs.existsSync(dbDirPath)) {
		fs.mkdirSync(dbDirPath);
	}

	const getData = () => {
		return readMainData().data;
	};
	getData.key = (key) => {
		return getData()[key];
	};

	const setData = (data) => {
		const newData = readMainData();
		for (let i in data) if (data[i] !== undefined) newData.data[i] = data[i];
		writeMainData(newData);
	};

	setData.key = (key, value) => {
		setData({ [key]: value });
	};

	setData.rm = (key) => {
		setData({ [key]: undefined });
	};

	setData.reset = () => {
		writeMainData({
			...readMainData(),
			data: { ...dbData, name: dbName },
		});
	};

	const readMainData = () => {
		if (!fs.existsSync(mainFilePath)) {
			writeMainData({
				collections: [],
				data: { ...dbData, name: dbName },
				maps: [],
			});
		}
		const buffer = fs.readFileSync(mainFilePath);
		return deserializeData(buffer, encryptionKey);
	};

	const writeMainData = (data) => {
		const buffer = serializeData(data, encryptionKey);
		fs.writeFileSync(mainFilePath, buffer);
	};

	const readDataFile = (filePath) => {
		const buffer = fs.readFileSync(filePath);
		return deserializeData(buffer, encryptionKey);
	};

	const writeDataFile = (filePath, data) => {
		const buffer = serializeData(data, encryptionKey);
		fs.writeFileSync(filePath, buffer);
	};

	const collection = (collectionName, {
		model,
		exclude
	} = {}) => {
		const collectionFilePath = path.join(dbDirPath, `${collectionName}.col`);

		const validateFields = (definition, data, optional = false) => {
			if(!definition){
				return data;
			}

      const validatedData = {};
      for (const [field, type] of Object.entries(definition)) {
				if(field.startsWith('@rune.')) continue;
				if(!data[field] && typeof definition['@rune.default']?.[field] !== "undefined") data[field] = definition['@rune.default']?.[field];
				
				if(typeof data[field] == "function"){
					data[field] = data[field](data, definition);
				}

        if (data[field] === undefined) {
					if(optional) continue;
					else if(definition['@rune.required']){
						if(definition['@rune.required'][field]){
							throw new ReferenceError(`Field ${field} is required, yet not provided.`)
						} else continue;
					} else continue;
				}

        const value = data[field];
        if (!typeis(value, type) && value != type) {
          throw new TypeError(`Invalid type for field "${field}". Expected ${
            type?.type?.type || type?.type || type
          }, got ${typeof value}`);
        }
        validatedData[field] = value;
      }
      return validatedData;
    };

		const validateUniqueFields = (record, data) => {
			if(!model) return null;
			if(!model['@rune.unique']) return null;

			const uniqueFields = Object.keys(model['@rune.unique']);

			return uniqueFields.find(
				(field) => data.find(storedRecord => storedRecord[field] == record[field])
			);
		}

    const applyFieldSelection = (record, select) => {
			const newRecord = {...record};
			if(exclude) for (const key of exclude) {
				if (record[key]) delete newRecord[key];
			}
      if (!select) return newRecord;
      const selectedRecord = {};
      for (const key of Array.isArray(select) ? select : Object.keys(select)) {
        if (record[key]) selectedRecord[key] = record[key];
      }
      return selectedRecord;
    };

		const insert = (record, fields) => {

			if(Array.isArray(record)){
				return record.map((item) => insert(item));
			}

			const mainData = readMainData();
			if (!mainData.collections.includes(collectionName)) {
				mainData.collections.push(collectionName);
				writeMainData(mainData);
			}

			let data = [];
			if (fs.existsSync(collectionFilePath)) {
				data = readDataFile(collectionFilePath);
			}
			if(model){
				record = validateFields(model, record);
				const invalidUniqueFields = validateUniqueFields(record, data);
				if(invalidUniqueFields){
					throw new ReferenceError(`Duplicate value for field ${invalidUniqueFields}`);
				}
			}
			const id = uuidv4();
			record['@rune.id'] = generateID(id, collectionName);
			data.push(record);
			writeDataFile(collectionFilePath, data);
			return applyFieldSelection(record, fields);
		};

		const read = (id, fields) => {
			if (typeof id == 'object' && '@rune.id' in id) id = id['@rune.id'];
			if (!fs.existsSync(collectionFilePath)) return null;
			const data = readDataFile(collectionFilePath);
			const record = data.find((record) => record['@rune.id'] === id);
			if (record) {
				return applyFieldSelection(evaluateRecord(record), fields);
			}
			return null;
		};

		const evaluateRecord = (record, prevRecord) => {
			const evaluateValue = (val) => {
				if (typeof val == 'string' && val.startsWith('@rune.ref')) {
					const ref = val.split('@rune.ref')[1].trim();
					const refData = findRef(ref, false);
					if (!refData) {
						return null;
					} else {
						let value = refData;
						if (refData['@rune.id']) {
							value = prevRecord && prevRecord['@rune.id'] == refData['@rune.id'] ? prevRecord : evaluateRecord(refData, record);
						}
						return value;
					}
				}
				if (Array.isArray(val)) {
					val = val.map((i) => evaluateValue(i));
				}
				return val;
			};
			for (let i in record) {
				const val = record[i];
				record[i] = evaluateValue(val);
			}
			return record;
		};

		const update = (caseRecord, newRecord, limit = 0, fields) => {
			let updatedRecords = [];
			const data = readDataFile(collectionFilePath);
			const validatedNewRecord = model ? validateFields(model, newRecord, true) : newRecord;
			const invalidUniqueFields = validateUniqueFields(validatedNewRecord, data);
			if(invalidUniqueFields){
				throw new ReferenceError(`Duplicate value for field ${invalidUniqueFields}`);
			}
		
			const matches = data.filter((record) => {
				if (typeof caseRecord === 'string') {
					return record['@rune.id'] === caseRecord;
				} else if (typeof caseRecord === 'object') {
					return Object.keys(caseRecord).every((key) => record[key] === caseRecord[key]);
				}
				return false;
			});
			if (matches.length === 0) return null;
	
			matches.forEach((oldRecord, index) => {
				if(limit > 0 && updatedRecords.length > limit) return;
				for (const key in validatedNewRecord) {
					const value = validatedNewRecord[key];
					if (value instanceof PushChange) {
						if (!oldRecord[key] || !Array.isArray(oldRecord[key])) {
							oldRecord[key] = [];
						}
						oldRecord[key].push(...value.values);
					} else if (value instanceof PopChange) {
						if (oldRecord[key] && Array.isArray(oldRecord[key])) {
							value.values.forEach((val) => {
								const index = oldRecord[key].indexOf(val);
								if (index !== -1) {
									oldRecord[key].splice(index, 1);
								}
							});
						}
					} else {
						oldRecord[key] = typeof value == "function" ? value(oldRecord, index) : value;
					}
				}
				updatedRecords.push(applyFieldSelection(evaluateRecord(oldRecord), fields));
			});
			writeDataFile(collectionFilePath, data);
		
			return updatedRecords;		
		};

		const find = (criteria, fields, limit = 0, index = 0) => {
			if (typeof criteria == 'string') return read(criteria);
			if (!criteria || typeof criteria !== 'object') return null;

			
			if (!fs.existsSync(collectionFilePath)) writeDataFile(collectionFilePath, []);

			const data = readDataFile(collectionFilePath);
			const record =
				data[limit > 0 || limit == -1 ? 'filter' : 'find']((record) => {
					for (const key in criteria) {
						if (record[key] !== criteria[key]) return false;
					}
					return true;
				}) || null;
			if (record) {
				return Array.isArray(record) ? (limit > 0 ? record.slice(index, index + limit) : record).map(i => applyFieldSelection(evaluateRecord(i), fields)) : applyFieldSelection(evaluateRecord(record), fields);
			}
			return null;
		};

		const removeOne = (id) => {
			if ('@rune.id' in id) id = id['@rune.id'];
			let data = readDataFile(collectionFilePath);
			const index = data.findIndex((record) => record['@rune.id'] === id);
			if (index !== -1) {
				data.splice(index, 1);
				writeDataFile(collectionFilePath, data);
				return true;
			}
			return false;
		};

		const remove = (criteria, limit = Infinity) => {
			let data = readDataFile(collectionFilePath);
			let deletedCount = 0;
			
			const filteredData = data.filter((record, index) => {
				if (deletedCount >= limit) return true;
		
				const matches = Object.keys(criteria).every((key) => typeof criteria[key] == 'function' ? criteria[key](record[key], record, index) : record[key] === criteria[key]);
				if (matches) {
					deletedCount++;
					return false;
				}
		
				return true;
			});
		
			if (deletedCount === 0) return false;

			writeDataFile(collectionFilePath, filteredData);
		
			return true;
		};
		

		const list = (fields) => {
			if (!fs.existsSync(collectionFilePath)) return [];
			const data = readDataFile(collectionFilePath);
			return data.map((rec) => applyFieldSelection(evaluateRecord(rec), fields));
		};

		const map = (cb, mutate = false) => {
			const data = readDataFile(collectionFilePath);
			const mappedData = data.map(cb);
			if (mutate) {
				writeDataFile(collectionFilePath, mappedData);
			}
			return mappedData;
		};

		const transform = (cb, mutate = true) => {
			const data = readDataFile(collectionFilePath);
			const transformedData = cb(data);
			if (mutate) {
				writeDataFile(collectionFilePath, transformedData);
			}
			return transformedData;
		};

		const filter = (cb, mutate = false) => {
			const data = readDataFile(collectionFilePath);
			const filteredData = data.filter(cb);
			if (mutate) {
				writeDataFile(collectionFilePath, filteredData);
			}
			return filteredData;
		};

		const sort = (cb, mutate = false) => {
			const data = readDataFile(collectionFilePath);
			const sortedData = data.sort(cb);
			if (mutate) {
				writeDataFile(collectionFilePath, sortedData);
			}
			return sortedData;
		};

		const empty = () => {
			writeDataFile(collectionFilePath, []);
		}

		if (!fs.existsSync(collectionFilePath)) writeDataFile(collectionFilePath, []);

		return {
			insert,
			read,
			update,
			remove,
			removeOne,
			find,
			map,
			transform,
			filter,
			sort,
			list,
			empty
		};
	};

	const findRef = (ref, evaluate = true) => {
		const [name, id, ...rest] = ref.split('.');
		const col = collection(name);
		const record = col.read(id, evaluate);
		if (rest.length === 0) return record;
		let value = record;
		for (const prop of rest) {
			if (typeof value != 'object') break;
			if (!(prop in value)) return null;
			value = value[prop];
		}
		return value;
	};

	const map = (mapName) => {
		const mapFilePath = path.join(dbDirPath, `${mapName}.map`);

		const set = (key, value) => {
			const mainData = readMainData();
			if (!mainData.maps.includes(mapName)) {
				mainData.maps.push(mapName);
				writeMainData(mainData);
			}

			let data = {};
			if (fs.existsSync(mapFilePath)) {
				data = readDataFile(mapFilePath);
			}
			data[key] = value;
			writeDataFile(mapFilePath, data);
		};

		const get = (key) => {
			if (!fs.existsSync(mapFilePath)) return null;
			const data = readDataFile(mapFilePath);
			return data[key] || null;
		};

		const remove = (key) => {
			if (!fs.existsSync(mapFilePath)) return false;
			let data = {};
			if (fs.existsSync(mapFilePath)) {
				data = readDataFile(mapFilePath);
			}
			if (data[key]) {
				delete data[key];
				writeDataFile(mapFilePath, data);
				return true;
			}
			return false;
		};

		const transform = (cb, mutate = true) => {
			let data = {};
			if (fs.existsSync(mapFilePath)) {
				data = readDataFile(mapFilePath);
			}
			const transformedData = cb(data);
			if (mutate) {
				writeDataFile(mapFilePath, transformedData);
			}
			return transformedData;
		};

		const list = () => {
			if (!fs.existsSync(mapFilePath)) return {};
			const data = readDataFile(mapFilePath);
			return data;
		};

		if (!fs.existsSync(mapFilePath)) writeDataFile(mapFilePath, {});

		return { set, get, remove, list, transform };
	};

	collection.type = coltypedef;

	readMainData();

	return new RuneDB({ setData, getData, collection, findRef, makeRef, map });
};

module.exports = (context) => ({
	_onImport() {
		delete this.createDB;
		return this;
	},
	db(dbname, data = {}, encryptionKey) {
		if (!context.app) throw new Error('rune can only be used in apps');
		const pkg = path.join(CONFIG_PATH, context.app.config.manifest.package, 'db');
		if (!fs.existsSync(pkg)) fs.mkdirSync(pkg, { recursive: true });
		return createDB(dbname, pkg, data, encryptionKey || ENCRYPTION_KEY);
	},
	genKey(secret){
		return gen_key(secret);
	},
	makeRef,
	push: runePush,
	pop: runePop,
	createDB,
	localState(name, options){
		const map = this.db('_' + name).map('_' + name + '_map');
		const firstValue = map.get(name);
		const _set = () => {
			let item = options.get();
			if(item.then){
				item.then((i) => map.set(i));
			} else {
				map.set(name, item);
			}
		}
		if(!firstValue) {
			_set();
		} else {
			options.set(firstValue);
		}
		if(options.trigger){
			options.trigger(() => {
				_set();
			});
		}
	}
});
