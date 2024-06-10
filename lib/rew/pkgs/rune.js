const fs = require('fs');
const { v4: uuidv4 } = require('uuid');
const path = require('path');
const { CONFIG_PATH } = require('../const/config_path');
const { serializeData, deserializeData, gen_key } = require('../misc/bin');

const ENCRYPTION_KEY = 'e6ad8b0792b9e0472ea44d1f3adfd1d503182efcce25991b05cc5ef83f307ffc';

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

	const collection = (collectionName) => {
		const collectionFilePath = path.join(dbDirPath, `${collectionName}.col`);

		const insert = (record) => {
			const mainData = readMainData();
			if (!mainData.collections.includes(collectionName)) {
				mainData.collections.push(collectionName);
				writeMainData(mainData);
			}

			let data = [];
			if (fs.existsSync(collectionFilePath)) {
				data = readDataFile(collectionFilePath);
			}
			const id = uuidv4();
			record['@rune.id'] = generateID(id, collectionName);
			data.push(record);
			writeDataFile(collectionFilePath, data);
			return record;
		};

		const read = (id, evaluate = true) => {
			if (typeof id == 'object' && '@rune.id' in id) id = id['@rune.id'];
			if (!fs.existsSync(collectionFilePath)) return null;
			const data = readDataFile(collectionFilePath);
			const record = data.find((record) => record['@rune.id'] === id);
			if (record) {
				return evaluateRecord(record);
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

		const update = (caseRecord, newRecord) => {

			let id;
			if (typeof caseRecord === 'string') {
				id = caseRecord;
			} else if (typeof caseRecord === 'object') {
				const data = readDataFile(collectionFilePath);
				const record = data.find((record) => {
					for (const key in caseRecord) {
						if (record[key] !== caseRecord[key]) return false;
					}
					return true;
				});
				if (record) {
					id = record['@rune.id'];
				} else {
					return null; // No matching record found
				}
			}

			if (!id) return null;

			const data = readDataFile(collectionFilePath);
			const index = data.findIndex((record) => record['@rune.id'] === id);
			if (index !== -1) {
				const oldRecord = data[index];
				for (const key in newRecord) {
					const value = newRecord[key];
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
						oldRecord[key] = value;
					}
				}
				data[index] = oldRecord;
				writeDataFile(collectionFilePath, data);
				return data[index];
			}
			return null;
		};

		const find = (criteria) => {
			if (typeof criteria == 'string') return read(criteria);
			if (!criteria || typeof criteria !== 'object') return null;

			
			if (!fs.existsSync(collectionFilePath)) writeDataFile(collectionFilePath, []);

			const data = readDataFile(collectionFilePath);
			const record =
				data.find((record) => {
					for (const key in criteria) {
						if (record[key] !== criteria[key]) return false;
					}
					return true;
				}) || null;
			if (record) {
				return evaluateRecord(record);
			}
			return null;
		};

		const remove = (id) => {
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

		const list = () => {
			if (!fs.existsSync(collectionFilePath)) return [];
			const data = readDataFile(collectionFilePath);
			return data.map((rec) => evaluateRecord(rec));
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

		if (!fs.existsSync(collectionFilePath)) writeDataFile(collectionFilePath, []);

		return {
			insert,
			read,
			update,
			remove,
			find,
			map,
			transform,
			filter,
			sort,
			list,
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

	readMainData();

	return { setData, getData, collection, findRef, makeRef, map };
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
});
