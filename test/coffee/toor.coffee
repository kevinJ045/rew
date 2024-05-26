threads = imp 'threads'

myThread = threads.thread (data) -> 
  print(data)
  @process.on 'event', () => 
    print('Event Happened')
    @process.finish 'je;;o'
  # @process.finish ''
  # print(data)

proc = myThread.start data: 'data'

proc.get().then(print)

sleep 1000
  .then () -> proc.emit 'event'