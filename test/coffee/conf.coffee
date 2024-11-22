conf = imp 'conf'

file = conf.staticFile 'folder/example.txt', 'Hello'
  .create()

animations = conf.optionCenter 'animations', enable: false

animations.set 'id',
  name: 4
  type: 'int'

animations.set 'enable', true

print animations.get('enable')

print animations.get 'id'