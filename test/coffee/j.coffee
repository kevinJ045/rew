


j = yaml 'name: !i2 1', {
  key: '!i2',
  kind: 'scalar',
  construct: (a) => a + ' true'
}
print j
