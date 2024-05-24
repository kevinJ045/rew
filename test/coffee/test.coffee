ui = imp 'ui'

{ Widget } = await ui.start
  port: 4532

w = new Widget
  data: 
    text: 'sss'
  parent: 'null'
  children:
    [
      new Widget
        data: 
          text: 'hello'
    ]

# print(w)
