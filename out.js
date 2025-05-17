rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/s.coffee", function(context){
            with (context) {
              var data, text, userData, user, configData, config, binaryData, readBinary;
using(namespace(rew.prototype.ns()))
data = rew.prototype.data

// Working with text data
data.prototype.write("notes.txt", "This is a simple text note.")
text = data.prototype.read("notes.txt")
print("Text note:", text)

// Working with JSON data
userData = {
  name: "User",
  age: 30,
  preferences: {
    theme: "dark",
    fontSize: 14
  }
}
data.prototype.writeJSON("user.json", userData)
user = data.prototype.readJSON("user.json")
print("User name:", user.name)
print("User preferences:", user.preferences)

// Working with YAML data
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
data.prototype.writeYAML("config.yaml", configData)
config = data.prototype.readYAML("config.yaml")
print("Server host:", config.server.host)
print("Enabled features:", config.features.join(", "))

// Working with binary data
binaryData = new Uint8Array([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])  // PNG header
data.prototype.writeBinary("sample.bin", binaryData)
readBinary = data.prototype.readBinary("sample.bin")
print("Binary data length:", readBinary.length)
print("First bytes:", rew.prototype.encoding.prototype.bytesToHex(readBinary.slice(0, 4)))

// Using auto-detection
data.prototype.writeAuto("config2.yaml", configData)  // Will be written as YAML
data.prototype.writeAuto("user2.json", userData)      // Will be written as JSON
data.prototype.writeAuto("binary.dat", binaryData)    // Will be written as binary

// Reading with auto-detection

            }
            return context.module.exports;
          }, );
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/s.coffee');