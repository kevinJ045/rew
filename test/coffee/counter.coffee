num = 0
myFunc = () ->
  num++
  scheduleFrame myFunc
scheduleFrame myFunc

myFunc2 = ->
  await sleep(1000)
  clear()
  print 'FPS:', num
  num = 0
  myFunc2()

myFunc2()