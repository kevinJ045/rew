const execOptions = {
	sharedContext: true,
	resolveExtensions: ['.coffee', { ext: '.js', options: { type: 'js' } }, { ext: '.qrew', options: { qrew: true } }],
	nativeRequire: false,
	cwdAlias: '$',
	jsxPragma: 'createElement',
	jsx: false,
};

module.exports.execOptions = execOptions;
