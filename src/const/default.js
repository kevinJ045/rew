const { cenum } = require('../models/enum');
const { struct } = require('../models/struct');
const emitter = require('../functions/emitter');
const future = require('../functions/future');
const sleep = require('../functions/sleep');

module.exports = {
	cenum,
	struct,
	future,
	emitter,
	sleep,
	print: function (...arguments) {
		return console.log(...arguments);
	},
};
