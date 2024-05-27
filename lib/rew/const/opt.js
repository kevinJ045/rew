const execOptions = {
  sharedContext: true,
  resolveExtensions: [{ext: '.js', options: { type: 'js' }}, '.coffee'],
  nativeRequire: false
}

module.exports.execOptions = execOptions;