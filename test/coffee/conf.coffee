conf = imp 'conf'

config = conf.create 'com.app.name'

animations = config.optionCenter 'animations', enable: false

animations.set 'id',
  name: 4
  type: 'int'

animations.set 'enable', true

print animations.get('enable')

print animations.get 'id'