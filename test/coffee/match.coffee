i = 's'

match(i, {
  'ss': () -> print('true'),
  's': () -> print('huh')
})

s = struct name: ''

f = s name: 'ss'

match(f, [
  [s, () -> print 'kkk']
])

match(f, map s, () -> print 'sss')

match('hello_world', map /hello/i, (match) -> print(match))