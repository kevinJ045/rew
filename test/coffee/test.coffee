
person = struct name: '', age: 0, hobby: ''

parent = struct.inherits person, people: []

people = [
  person name: 'Sean', age: 60, hobby: 'being dumb'
  person name: 'Makano', age: 19, hobby: 'being cool'
]

for man in people
  print man.name, 'is', man.age

print parent people: people