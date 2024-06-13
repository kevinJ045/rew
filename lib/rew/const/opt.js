const { REW_FILE_TYPE } = require("./ext");

const execOptions = {
	sharedContext: true,
	resolveExtensions: [REW_FILE_TYPE.EXTENSION, ".coffee", { ext: '.js', options: { type: 'js' } }, { ext: '.qrew', options: { qrew: true } }],
	nativeRequire: false,
	useImport: false,
	cwdAlias: '$',
	jsxPragma: 'createElement',
	jsx: false
};

module.exports.execOptions = execOptions;
