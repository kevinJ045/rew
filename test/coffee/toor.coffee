{ thread } = imp 'threads'

myThread = thread () ->
	@process.on 'myEvent', (data) =>
		print(data)
		@process.emit 'myEventBack', data: 'smn'

runningThread = myThread.start()

runningThread.on 'myEventBack', (data) ->
  print data, 'back'
  runningThread.stop()

sleep 1000
	.then () ->
		runningThread.emit 'myEvent', data: 'Hello'

# print 'It Exists' if exists './conf.coffee'