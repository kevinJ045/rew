import "#std.threads"
using namespace rew::ns()

print "Starting threads demo..."

print "\nCreating a Worker object..."

calculatorWorker = threads::create ->
  total = 0
  
  onmessage (data) ->
    if data.op == "add"
      total += data.value
      postMessage({ result: total })
    else if data.op == "subtract"
      total -= data.value
      postMessage({ result: total })
    else if data.op == "multiply"
      total *= data.value
      postMessage({ result: total })
    else if data.op == "divide"
      total /= data.value
      postMessage({ result: total })
    else
      postMessage({ error: "Unknown operation" })

calculatorWorker.onmessage (event) ->
  print "Calculator result:", event.data

calculatorWorker.postMessage({ op: "add", value: 5 })
calculatorWorker.postMessage({ op: "multiply", value: 10 })
calculatorWorker.postMessage({ op: "subtract", value: 15 })
calculatorWorker.postMessage({ op: "divide", value: 2 })


channel = rew::channel::new 1000, ->
  activeThreads = threads::list()
  print "Active threads:", activeThreads.length


rew::channel::timeout 3000, ->
  print "Terminating the calculator worker..."
  calculatorWorker.terminate()

rew::channel::timeout 5000, ->
  channel.stop()
