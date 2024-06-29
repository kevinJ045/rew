const { REW_FILE_TYPE } = require("./ext");

const execOptions = {
	sharedContext: true,
	resolveExtensions: [REW_FILE_TYPE.EXTENSION, ".coffee", { ext: '.js', options: { type: 'js' } }, { ext: '.qrew', options: { qrew: true } }],
	nativeRequire: false,
	useImport: false,
	cwdAlias: '$',
	jsxPragma: '__using__.JSX.createElement',
	jsxPragmaFrag: '__using__.JSX.Fragment',
	jsx: false,
	typescript: false,
	decorators: false,

	_syntaxAliases: {}
};

module.exports.execOptions = execOptions;
