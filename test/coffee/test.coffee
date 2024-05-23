yaml = imp './something.yaml', type: 'yaml'
# js = imp './test.js', type: 'js'
# js = imp './test.json', type: 'json'

person = struct name: '', age: 0, hobby: ''

parent = struct.inherits person, people: []

people = [
  person name: 'Sean', age: 60, hobby: 'being dumb'
  person name: 'Makano', age: 19, hobby: 'being cool'
]

print yaml

# for man in people
#   print man.name, 'is', man.age

# print parent people: people