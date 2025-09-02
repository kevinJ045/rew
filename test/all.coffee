import "#std.testing";
import "#std.types!";
import "#std.conf";
import "#std.encoding";
import "#std.shell";
import "#std.fs";
import "#std.os";
import "#std.threads";

using namespace rew::ns;

rew::testing::describe 'const_rec', (it) ->
  it 'should make object properties unwritable', ->
    g := const_rec {
      myName: "sss"
    }
    g.myName = 'hhh'
    rew::testing::assert_eq g.myName, "sss"

rew::testing::describe 'Imports', (it) ->

  it 'should import yaml, json and txt', ->
    import txt from "./test.txt"
    import json from "./some.json"
    import yaml from "./app.yaml"

    rew::testing::assert_eq typeof txt, "string"
    rew::testing::assert_eq typeof json, "object"
    rew::testing::assert_eq typeof yaml, "object"

  it 'should import async', ->
    imported = await imp "./to_import.coffee"

    rew::testing::assert_eq imported.demo_data, "demo_data"

  # Comment this if it's making errors
  it 'should import external', ->
    import j from "gaga.daga!"

    rew::testing::assert_eq j.isMain, true

rew::testing::describe 'Exports', (it) ->
  it 'should export classes and variables', ->
    # This is a mock of what would be in a separate file
    # to test the module.exports functionality.
    # In a real scenario, we would import from another file.
  
    export default class Gangarmada
      mmm = "mmm"
    export class GGG
      name = "sss"
    export hello = "shhshsh"
    export name = "jjj"

    rew::testing::assert_eq typeof module.exports.default, "function"
    rew::testing::assert_eq typeof module.exports.GGG, "function"
    rew::testing::assert_eq module.exports.hello, "shhshsh"
    rew::testing::assert_eq module.exports.name, "jjj"

rew::testing::describe 'rew::os::clamp', (it) ->
  it 'should return the correct value for Linux', ->
    # On Linux, it should return the second value
    rew::testing::assert_eq rew::os::clamp("win", "linux", "mac"), "linux"
    rew::testing::assert_eq rew::os::clamp("win", "mac_lin"), "mac_lin"

rew::testing::describe 'Packages', (it) ->
  it 'should create in-file submodules', ->
    package myp;
    pub package ss;
    myp::s = 'ss';
    rew::testing::assert_eq myp::s, 'ss'

  it 'should export a native package', ->
    # native myp;
    # import c from "./p.coffee"; # Assuming p.coffee exports myp
    # This is hard to test without actual files, but we can check the concept
    # We'd need to mock the import. For now, this is a placeholder.
    # rew::testing::assert_eq typeof myp, "object"


rew::testing::describe '#declare directive', (it) ->
  it 'should create a custom declarator', ->
    #declare "=mything*" = something;
    class something
      constructor(@value)
    
    mything(10) ff = "ss" # This should be equivalent to ff = something 10
    
    # This is conceptually what we want to test.
    # The actual test might need to be different based on how #declare is implemented.
    # For now, we assume it creates an instance of `something`.
    rew::testing::assert_eq ff.value, 10

rew::testing::describe 'Enums and Match', (it) ->
  enum Animal { Cat, Dog, Capibara }
  
  isItCat = (a: Animal) -> match(a)
    .enum(Animal)
    .on('Cat', -> true)
    .on(Animal.Dog, -> false)
    .default(-> false)
    .end

  it 'should correctly identify Cat', ->
    rew::testing::assert_eq isItCat(Animal.Cat), true
  
  it 'should correctly identify Dog', ->
    rew::testing::assert_eq isItCat(Animal.Dog), false
    
  it 'should handle default case', ->
    rew::testing::assert_eq isItCat(Animal.Capibara), false

rew::testing::describe 'rew::conf', (it) ->
  data = rew::conf
  
  it 'should write and read text files', ->
    data::write "test.txt", "hello world"
    text = data::read "test.txt"
    rew::testing::assert_eq text, "hello world"
    
  it 'should write and read JSON files', ->
    userData = { name: "test" }
    data::writeJSON "test.json", userData
    user = data::readJSON "test.json"
    rew::testing::assert_eq user.name, "test"

rew::testing::describe 'rew::channel::emitter', (it) ->
  it 'should emit and receive events', () ->
    await new Promise (resolve, reject) ->
      e = rew::channel::emitter()
      e.on 'test_event', (data) ->
        rew::testing::assert_eq data, "test_data"
        resolve()
      e.emit 'test_event', 'test_data'

rew::testing::describe 'rew::shell', (it) ->
  it 'should execute a command with sync and return output', ->
    output = rew::shell::sync 'echo hello', onlyString: 'true'
    rew::testing::assert_eq output.trim(), 'hello'

rew::testing::describe 'rew::fs', (it) ->
  it 'should perform file operations', ->
    await rew::fs::write "test_fs.txt", "content"
    content = rew::fs::read "test_fs.txt"
    rew::testing::assert_eq content, "content"
    sha = rew::fs::sha "test_fs.txt"
    rew::testing::assert_eq typeof sha, "string"
    rew::fs::rm "test_fs.txt"

rew::testing::describe 'rew::ptr', (it) ->
  it 'should write and read from a pointer', ->
    bytes = rew::encoding::stringToBytes "test\0"
    ptr = rew::ptr::of bytes
    rew::testing::assert_eq rew::ptr::string(ptr), "test"

    name = &"hello"
    id = &11
    someVal = &4.5
    isCool = &true
    rew::testing::assert_eq *name, "hello"
    rew::testing::assert_eq *id, 11
    rew::testing::assert_eq *someVal, 4.5
    rew::testing::assert_eq *isCool, true


rew::testing::describe 'Random Functions', (it) ->
  it 'genUid should generate a UID of specified length', ->
    rew::testing::assert_eq genUid(10).length, 10
  
  it 'randFrom should generate a number within a range', ->
    num = randFrom 1, 10
    rew::testing::assert_eq num >= 1 && num <= 10, true

rew::testing::describe 'rew::threads', (it) ->
  it 'should create a worker and communicate with it', () ->
    await new Promise (resolve, reject) ->
      worker = threads::create ->
        onmessage (data) ->
          postMessage data * 2
      
      worker.onmessage (event) ->
        rew::testing::assert_eq event.data, 10
        worker.terminate()
        resolve()
        
      worker.postMessage 5

rew::testing::describe 'Types', (it) ->
  it 'should create and match structs', ->
    _person = struct { name: str, age: int }
    sample = _person::new name: 'test', age: 20
    
    result = match(sample)
      .on(_person, -> true)
      .default(-> false)
      .end
      
    rew::testing::assert_eq result, true
    
  it 'typeis should check the type of an instance', ->
    class MyClass
    inst = new MyClass
    rew::testing::assert_eq typeis(inst, MyClass), true

rew::testing::describe 'rew::vfile', (it) ->
  it 'should add and find a virtual file', ->
    rew::vfile::add "test.vfile", "virtual content"
    file = rew::vfile::find "test.vfile"
    rew::testing::assert_eq file, "virtual content"
