ui = imp 'ui'

{ Widget, Text } = await ui.start
  port: 4532

w = new Widget
  parent: 'null'
  style: 
    color: 'red'
  children:
    [
      new Text 'Hello'
    ]

print(w.children[0].options.data)
