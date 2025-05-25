"no-compile"
if(!rew.extensions.has('shell')) rew.extensions.add('shell', (Deno) => rew.extensions.createClass({
  _namespace(){
    return 'shell';
  },
  // kill process
  kill(pid, signal = "SIGTERM") {
  },
  // spawn process
  async spawn(command, args = []) {
    return Deno.run(command)
  },
  // process waiter
  async wait(process) {
    
  },
  // sync function to exec and return the output
  exec(command, options = {}) {
    
  },
}));