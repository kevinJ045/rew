const execOptions = {
	sharedContext: true,
	resolveExtensions: [{ ext: '.js', options: { type: 'js' } }, '.coffee'],
	nativeRequire: false,
	cwdAlias: '$',
	jsxPragma: 'createElement',
	jsx: false,
};

module.exports.execOptions = execOptions;
