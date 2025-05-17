import "#std!"
using namespace rew::ns()
data = rew::conf

# Working with text data
data::write "notes/notes.txt", "This is a simple text note."
data::write "notes.txt", "This is a simple text note."
text = data::read "notes.txt"
print "Text note:", text

# Working with JSON data
userData = {
  name: "User",
  age: 30,
  preferences: {
    theme: "dark",
    fontSize: 14
  }
}
data::writeJSON "user.json", userData
user = data::readJSON "user.json"
print "User name:", user.name
print "User preferences:", user.preferences

# Working with YAML data
configData = {
  server: {
    host: "localhost",
    port: 8080
  },
  database: {
    url: "postgres://user:pass@localhost/db",
    maxConnections: 10
  },
  features: [
    "authentication",
    "logging",
    "api"
  ]
}
data::writeYAML "config.yaml", configData
config = data::readYAML "config.yaml"
print "Server host:", config.server.host
print "Enabled features:", config.features.join(", ")

# Working with binary data
binaryData = new Uint8Array([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])  # PNG header
data::writeBinary "sample.bin", binaryData
readBinary = data::readBinary "sample.bin"
print "Binary data length:", readBinary.length
print "First bytes:", rew::encoding::bytesToHex(readBinary.slice(0, 4))

# Using auto-detection
data::writeAuto "config2.yaml", configData  # Will be written as YAML
data::writeAuto "user2.json", userData      # Will be written as JSON
data::writeAuto "binary.dat", binaryData    # Will be written as binary

# Reading with auto-detection
config2 = data::readAuto "config2.yaml"
print "Auto-detected YAML:", config2.server.port

# Get file info
info = data::getInfo "user.json"
print "File exists:", info.exists
print "File format:", info.format