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
    
    const id = rew.ops.op_thread_spawn(code, JSON.stringify({
      path: module.app.path,
      config: {
        manifest: {
          package: module.app.config.manifest.package,
          version: module.app.config.manifest.version,
        },
        entries: {
          ...module.app.config.entries
        }
      }
    }));
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
        this._polling = false;
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