const execOptions = {
	sharedContext: true,
	resolveExtensions: ['.coffee', { ext: '.js', options: { type: 'js' } }, { ext: '.qrew', options: { qrew: true } }],
	nativeRequire: false,
	useImport: false,
	cwdAlias: '$',
	jsxPragma: 'createElement',
	jsx: false
};

module.exports.execOptions = execOptions;
