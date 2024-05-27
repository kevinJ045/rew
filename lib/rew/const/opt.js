const execOptions = {
  sharedContext: true,
  resolveExtensions: [{ext: '.js', options: { type: 'js' }}, '.coffee'],
  nativeRequire: false,
  cwdAlias: '$'
}

module.exports.execOptions = execOptions;