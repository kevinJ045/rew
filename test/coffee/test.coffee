ui = imp 'ui'

{ Widget, Text } = await ui.start
  port: 4532

w = new Widget
  parent: 'null'
  children:
    [
      new Text 'Hello'
    ]
  styles:
    color: 'red'

print(w.children[0].options.data)
