ui = imp 'ui'

{ Widget, Text, findElement, StyleSheet } = await ui.start
  port: 4532,
  style: """body{ background: black; }"""

w = new Widget
  id: 'mainguy'
  parent: 'null'
  style: 
    color: 'red'
  attr:
    title: 'Hello'
  children:
    [
      new Text 'Hello'
    ]

w.on 'click', () -> w.add new Text 'Hello'

# print await findElement 'mainguy'

print(w.children[0].options.data)
