conf = imp 'conf'

config = conf.create 'com.app.name'

animations = config.optionCenter 'animations', enable: false

print animations.get('enable')