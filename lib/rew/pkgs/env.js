

module.exports = (context) => ({
  has: (key) => key in process.env,
  get: (key) => process.env[key],
  set: (key, value) => process.env[key] = value,
  rm: (key) => delete process.env[key],
  is: (key, value) => process.env[key] == value
})