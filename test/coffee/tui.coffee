blessed = require 'blessed'

screen = blessed.screen smartCSR: true

screen.title = 'hello'

box = blessed.box({
  top: 'center',
  left: 'center',
  width: '50%',
  height: '50%',
  content: 'Hello {bold}world{/bold}!',
  tags: true,
  border: {
    type: 'line'
  },
  style: {
    fg: 'white',
    bg: 'magenta',
    border: {
      fg: '#f0f0f0'
    },
    hover: {
      bg: 'green'
    }
  }
})

screen.append(box)


box.on('click', (data) ->
  box.setContent('{center}Some different {red-fg}content{/red-fg}.{/center}')
  screen.render()
)

box.key('enter', (ch, key) ->
  box.setContent('{right}Even different {black-fg}content{/black-fg}.{/right}\n')
  box.setLine(1, 'bar')
  box.insertLine(1, 'foo')
  screen.render()
)

screen.key(['escape', 'q', 'C-c'], (ch, key) -> 
  return process.exit(0)
)

box.focus()

screen.render()