rew.prototype.mod.prototype.defineNew("/home/makano/workspace/rew-rust/test/threads.coffee", function(globalThis){
with (globalThis) {
  var calculatorWorker;
rew.prototype.mod.prototype.find(module, "#std.threads")
using(namespace(rew.prototype.ns()))

print("Starting threads demo...")

print("\nCreating a Worker object...")

calculatorWorker = threads.prototype.create(function() {
  var total;
  total = 0
  
  return onmessage(function(data) {
    if (data.op === "add") {
      total += data.value
      return postMessage({ result: total })
    }
    else if (data.op === "subtract") {
      total -= data.value
      return postMessage({ result: total })
    }
    else if (data.op === "multiply") {
      total *= data.value
      return postMessage({ result: total })
    }
    else if (data.op === "divide") {
      total /= data.value
      return postMessage({ result: total })
    }
    else {
      return postMessage({ error: "Unknown operation" })
    }
  })
})

calculatorWorker.onmessage(function(event) {
  return print("Calculator result:", event.data)
})

calculatorWorker.postMessage({ op: "add", value: 5 })
calculatorWorker.postMessage({ op: "multiply", value: 10 })
calculatorWorker.postMessage({ op: "subtract", value: 15 })
calculatorWorker.postMessage({ op: "divide", value: 2 })


rew.prototype.channel.prototype.new(1000, function() {
  var activeThreads;
  activeThreads = threads.prototype.list()
  return print("Active threads:", activeThreads.length)
})

rew.prototype.channel.prototype.timeout(3000, function() {
  print("Terminating the calculator worker...")
  return calculatorWorker.terminate()
})

}
return globalThis.module.exports;
}, ["app://test.app/threads"]);(function(module){
"no-compile"
const liveThreads = [];
if(!rew.extensions.has('threads')) rew.extensions.add('threads', (Deno, module) => rew.extensions.createClass({
  _namespace(){
    return "threads";
  },

  spawn(code) {
    if (typeof code === 'function') {
      code = `(${code.toString()})();`;
    }
    
    const id = rew.ops.op_thread_spawn(code);
    liveThreads.push(id);
    return id;
  },

  list(){
    return liveThreads;
  },

  terminate(...ids) {
    return ids.map((id) => {
      liveThreads.splice(liveThreads.indexOf(id), 1);
      rew.ops.op_thread_terminate(id);
    });
  },
  
  create(code) {
    const threadId = this.spawn(code);
    
    return {
      id: threadId,
      
      postMessage(message) {
        return rew.ops.op_thread_message(this.id, JSON.stringify(message));
      },
      
      terminate() {
        liveThreads.splice(liveThreads.indexOf(this.id), 1);
        return rew.ops.op_thread_terminate(this.id);
      },
      
      receiveMessage(timeout) {
        if(!liveThreads.includes(this.id)) return null;
        return rew.ops.op_thread_receive(this.id, timeout);
      },
      
      _startPolling() {
        if (!this._polling && this._onmessage) {
          this._polling = true;
          
          const poll = () => {
            if (!this._polling) return;
            
            const message = this.receiveMessage(100);
            if (message) {
              this._onmessage({ data: message });
            }
            
            setTimeout(poll, 10);
          };
          
          poll();
        }
      },
      
      onmessage(fn) {
        this._onmessage = fn;
        if (fn) {
          this._polling = false;
          this._startPolling();
        } else {
          this._polling = false;
        }
      }

    };
  }
}));
})({filename: "#std.threads"});
rew.prototype.mod.prototype.get('/home/makano/workspace/rew-rust/test/threads.coffee');