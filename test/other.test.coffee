import "#std.testing";

function add(a, b)
  a + b

function sub(a, b)
  a - b

rew::testing::describe 'Adding', (it) ->
  it('should be 4', -> rew::testing::assert_eq(add(1, 2), 3))

rew::testing::describe 'Subbing', (it) ->
  it('should be 1', -> rew::testing::assert_eq(sub(2, 1), 2))
  import "#std.types!";
using namespace rew::ns;

enum Animal {
  Cat
  Dog
  Capibara
}

isItCat = (a: Animal) -> match(a)
  .enum(Animal)
  .on 'Cat', -> print('It\'s a cat') ?? true
  .on Animal.Dog, -> print('It\'s a dog') ?? false
  .default -> print('Who knows') ?? false
  .end

isItCat Animal.Cat
isItCat Animal.Dog
isItCat Animal.Capibara

g := const_rec {
  myName: "sss"
}

g.myName = 'hhh'

print gimport "./e.coffee"
using namespace rew::ns

print module.options
print "Imported Script from"


sayhello = (...a) ->
  print "hello", ...a

sayhello g = ""

export default class Gangarmada
  mmm = "mmm"
export class GGG
  name = "sss"
export hello = "shhshsh"
export name = "jjj"import "#std.conf";
import "#std.encoding";
using namespace rew::ns
print "c", Object.keys(rew::),typeof rew::conf
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
x = 1

rew::io::out.print "eeeeeeeeeeeeeeeeeeeeeee"



using namespace rew::ns;

e = rew::channel::emitter()

e
  .on 'hi', (i) -> print 'hi', i
  .on 'hello', -> print 'hello'
  .emit ['hi', 'hello'], 'Yo'
  
import "./d.coffee";

ffk::sss

import "#std.shell";
using namespace rew::ns;

l = rew::shell::spawn 'sleep 40'
l.status()
print l.pid
rew::shell::kill l.pid
print 'output:', rew::shell::sync 'echo hii'
import "#std.fs"

something = null

using namespace rew::ns
# print rew::fs::read "./exec.coffee!"
export main = ->
  print rew::fs::open "./ffi.coffee", write: true
  print rew::fs::sha './ffi.coffee'
  print rew::fs::read "./ffi.coffee"



package MyPkg;

native MyPkg;
import "#std.http";
import "#std.net";
import "#std.encoding";

rew::http::withOptions(port: 3000) (req) ->
  rew::http::Response::new("Hello, World!")

rew::net::listen(hostname: "0.0.0.0", port: 4444) (conn) ->
  try
    buffer = new Uint8Array(1024);
    n = await conn.read(buffer)
    rew::io::out.print "Data received from connection", rew::encoding::bytesToString(buffer.subarray(0, n))
  catch err
    rew::io::out.print "Error reading from connection:", err

  await conn.write(rew::encoding::stringToBytes("Hello from the server!"))

rew::channel::timeout 1000, ->
  rew::net::fetch("http://localhost:3000")
  .then (res) ->
    rew::io::out.print "Response received:", await res.text()
  .catch (err) ->
    rew::io::out.print "Error fetching:", err
  
  rew::net::connect({ hostname: "127.0.0.1", port: 4444 }) (conn) ->
    message = "Hello from client!";
    await conn.write(rew::encoding::stringToBytes(message));
    buffer = new Uint8Array(1024);
    n = await conn.read(buffer)
    rew::io::out.print "Data received from connection", rew::encoding::bytesToString(buffer.subarray(0, n))
    conn.close()
import "#std.http";

rew::http::withOptions(port: 3000) (req) ->
  rew::http::Response::new("Hello, World!!")
import "#std.ffi!";
import imported, { hello } from "./d.coffee"
import * as smn from "agamada.domago"
import * as tst from "agamada.domago/test"



using namespace rew::ns

# print 'ffi: ', rew::ffi

my_linked_fn = ->->->->->->->->-> '=============>> linked fn result'

print my_linked_fn()()()()()()()()()

#d=eclare "=default" = ONLYIF(prev="export") rew::mod::export; 
#de=clare "=export" = rew::mod::export;
#dec=lare "export" = ONLYIF(next="default"); 
f = 1
export { f }
# export default f = 1


export default f

#ifdef Garmenanarnarnaruman
print 'Chaugemagangemaug', 'Gaugemachangemaug', 'Garmanarnar'
#endif

print smn
print tst
print imported, hello
# print magnificento

print module.app

print ffi

# print rew::encoding::toBase64 await rew::fs::read './d.coffee', { binary: true }
import "#std.os";

using namespace rew::ns;

print rew::os::
print rew::os::userInfo()
print rew::os::clamp("a", "b")

rew::process::exit()package myp;
pub package ss;

myp::s = 'ss';

native myp;
import c from "./p.coffee";

rew::io::out.print c;import "#std.encoding!";

array = rew::ptr::of rew::encoding::stringToBytes "sss"

a = rew::ptr::writeArray array, rew::encoding::stringToBytes "sss"

rew::io::out.print rew::ptr::readArray a, 3using namespace rew::ns;

print pickRandom genUid()
print pickRandom genUid(24)
print pickRandom genUid(24, "kkiiggllmmffx")
print pickRandom 1
print pickRandom "ss", "sss", "ssss", "sssss"
print randFrom 1, 10
print randFrom 1, 10, "sss"
import s from "./d.coffee"

using namespace rew::ns
rew::io::out.print typeof s.default, new s.default
rew::io::out.print typeof s.GGG, new s.GGG
rew::io::out.print rew::process::args
rew::io::out.print s
time = Date.now()
smn = 0
for i = 0; i < 10_000_000_000; i++
  smn++

tim2 = Date.now()

rew::io::out.print smn, (tim2 - time) / 1000import "#std!";

#declare "=mything*" = something;

class something
  s = 'ss'
  constructor(
    @ss,
    @dd
  )

mything("a") ff = "ss"

@print ffimport "#std.threads"
using namespace rew::ns

print "Starting threads demo..."

print "\nCreating a Worker object..."

calculatorWorker = threads::create ->
  total = 0
  
  onmessage (data) ->
    if data.op == "add"
      total += data.value
      postMessage({ result: total })
    else if data.op == "subtract"
      total -= data.value
      postMessage({ result: total })
    else if data.op == "multiply"
      total *= data.value
      postMessage({ result: total })
    else if data.op == "divide"
      total /= data.value
      postMessage({ result: total })
    else
      postMessage({ error: "Unknown operation" })

calculatorWorker.onmessage (event) ->
  print "Calculator result:", event.data

calculatorWorker.postMessage({ op: "add", value: 5 })
calculatorWorker.postMessage({ op: "multiply", value: 10 })
calculatorWorker.postMessage({ op: "subtract", value: 15 })
calculatorWorker.postMessage({ op: "divide", value: 2 })


channel = rew::channel::new 1000, ->
  activeThreads = threads::list()
  print "Active threads:", activeThreads.length


rew::channel::timeout 3000, ->
  print "Terminating the calculator worker..."
  calculatorWorker.terminate()

rew::channel::timeout 5000, ->
  channel.stop()
import "#std.types!";
using namespace rew::ns;

_person = struct {
  name: str,
  age: int
}

_not_person = struct {
  kk: str
}

sample = _person::new name: 'ss', age: 4

match(sample)
  .on _not_person, -> print('Matched person:', sample.name, sample.age)
  .default -> print 'nope'
  .end

class MyClass

inst = new MyClass

print typeis inst, MyClass

person = instantiate class
  str name = input('Name: ')
  int age = input('Age: ')

print person.name, 'is', person.age, 'years old.'

match(person.name)
  .on 'Makano', -> print('Hello, Makano!')
  .on /jo/i, -> print('Hello, John!')
  .default -> print('Hello, stranger!')  
  .end
rew::vfile::add "fff.ccc", "GGG"

rew::io::out.print rew::vfile::find "fff.ccc"
rew::io::out.print rew::io::out.size()