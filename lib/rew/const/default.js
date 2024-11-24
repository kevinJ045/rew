const { cenum } = require('../models/enum');
const { struct } = require('../models/struct');
const emitter = require('../functions/emitter');
const future = require('../functions/future');
const sleep = require('../functions/sleep');
const { match } = require('../functions/match');
const { map } = require('../functions/map');
const { typex, typeis, typedef, typei, int, float, num, str, bool, typef } = require('../functions/types');
const { isEmpty, clone, deepClone, merge, uniqueId, compose, curry, getters, setters, deepMerge } = require('../functions/core');
const { print, input, clear, printf } = require('../functions/stdout');
const { curl } = require('../functions/curl');
const { wait } = require('../functions/wait');
const { scheduleFrame } = require('../functions/misc');
const { jsons, yaml, json, yamls } = require('../functions/json');
const { generateRandomID } = require('../functions/id');
const { namespace } = require('./usage');

module.exports = {
	cenum,
	struct,
	future,
	emitter,
	sleep,
	wait,
	scheduleFrame,
	match,
	map,
	clear,

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

	isEmpty,
	clone,
	deepClone,
	merge,
	deepMerge,
	uniqueId,
	compose,
	curry,
	getters,
	setters,

	json,
	jsons,
	yaml,
	yamls,

	namespace,

	genID: generateRandomID,

	curl,

	print,
	printf,
	input,
};
