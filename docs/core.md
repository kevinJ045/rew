# Rew Core
Rew comes with a standard library of some functions.

### `struct`
A simple structure manager.
```coffee
file = struct name: '', content: '', length: 0

myTextFile = file name: 'myfile'

print myTextFile
```

### `future`
A simple promise manager.
```coffee
longTask = future (resolve, reject) -> ### long task here ###
  .pipe (data) -> print data # As a promise.then
  .last (data) -> print data # As a promise.finallt
  .catch (data) -> print data # As a promise.catch

otherTask = await future.wait() # To await

# to resolve later externally
longTask.resolve "customData"
```

### `emitter`
A simple event target.
```coffee
target = emitter()
target.on 'something', (data) -> print data
tatget.emit 'something', 'Data'

target.on ['something', 'anotherthing'], (data) -> ...
```

### `sleep`
A simple waiter for the parent thread.
```coffee
print 'Waiting...'
await sleep 1000
print 'Done!'
```

### `match`
A simple switch statement.
```coffee
myString = 'Hello!'

match myString,
  # Matches what the value is calls the match
  'Hello!': () -> print 'Hey!!'
  'Hi!': () -> print '...hey?'
```
#### `match` with  `struct`
When using match with values that are not strings, we have to use maps, or arrays mapped like this:
```coffee
myStruct = struct foo: 'bar'

myInstance = myStruct foo: 'not bar'

match myInstance, [
  [myStruct, () -> print 'myInstance is an instance of myStruct']
]
```

#### `match` with `map`
```coffee{5}
class myClass

myInstance = new myClass

match myInstance, map myClass, () -> print 'myInstance is a myClass'
```

### `map`
A simple map implementation.
```coffee
myMap = map 'key', 'value', 
  'key2', 'value2'

print myMap
```

### `imp`
A simple `require` implementation only for `rew` modules.
```coffee
{ LinkedList } = imp 'data'
list = new LinkedList

list.add('value')
```
#### importing files
```coffee
myFile = imp './myfile.coffee'
myJs = imp './myfile.js', type: 'js'
myJson = imp './myfile.json', type: 'json'
myYaml = imp './myfile.yaml', type: 'yaml'
myText = imp './myfile.txt', type: 'text'
```

### `exports`
A simple export implementation.
```coffee
# You can use module.exports
myVariable = ""
module.exports = { myVariable }
# or
exports { myVariable }
```

### `require`
NodeJS's require
```coffee
fs = require 'fs'
path = require 'path'
```