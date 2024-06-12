const { Readable, Writable, Transform, Duplex, pipeline, finished } = require('stream');

module.exports = (context) => ({
  Readable,
  Writable,
  Transform,
  Duplex,
  pipeline,
  finished
})