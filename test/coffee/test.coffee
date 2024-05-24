ui = imp 'ui'

{ Widget, Text, findElement, StyleSheet } = await ui.start
  port: 4532,
  style: """body{ color: white; }"""
  exec: () -> 
    document.body.appendChild(document.createTextNode('before_init'))

w = new Widget
  id: 'mainguy'
  parent: 'null'
  attr:
    title: 'Hello'
  children:
    [
      new Text 'Hello'
    ]

w.on 'click', () -> w.add new Text 'Hello', 
  style: 
    color: 'red'

# print await findElement 'mainguy'

print(w.children[0].options.data)
