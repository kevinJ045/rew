aug = imp '../aug.js', type: 'js'

f = aug ->
  print @a


f a: 'aa'