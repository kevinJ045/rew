"no-compile"
class ExecPipedError extends Error {
  constructor(message) {
    super(message);
    this.name = "ExecPipedError";
  }
}
if(!rew.extensions.has('shell')) rew.extensions.add('shell', (Deno) => rew.extensions.createClass({
  _namespace(){
    return 'shell';
  },
  get ChildProcess(){
    return Deno.ChildProcess;
  },
  kill(pid, signal = "SIGTERM") {
    Deno.kill(pid, signal);
  },
  spawn(command, options = {}) {
    return Deno.run({
      cmd: Array.isArray(command) ? command : command.split(" "),
      ...options,
    });
  },
  async wait(process) {
    const status = await process.status();
    process.close();
    return status;
  },
  fexec(command, options = {}) {
    const process = this.spawn(command, { ...options, stdout: "piped", stderr: "piped" });
    return this.wait(process).then((status) => {
      return {
        status,
        output: process.output(),
        error: process.stderrOutput(),
      };
    });
  },
  sync(command, options = {}){
    return this.exec(command, { ...options, stdout: "piped", stderr: "piped" });
  },
  command(command, options = {}) {
    if (typeof command === "string") {
      command = command.split(" ");
    }
    return new Deno.Command(command[0], {
      args: command.slice(1),
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
      ...options,
    });
  },
  exec(command, options = {}) {
    let commands = Array.isArray(command) ? command : command.split(" ");
    let { onlyString } = options; 
    delete options.onlyString; 
    const cmd = new Deno.Command(commands.shift(), {
      stdin: "inherit",
      stdout: "inherit",
      ...options,
      args: commands,
    });
    if(!onlyString) return cmd.outputSync();
    if(options.stdout === "inherit" || !options.stdout) {
      return Deno.core.decode(cmd.outputSync().stdout);
    } else {
      const { success, code, stdout, stderr } = cmd.outputSync();
      if (!success) {
        throw new ExecPipedError(`Command failed with exit code ${code}: ${Deno.core.decode(stderr)}`);
      }
      return Deno.core.decode(stdout);
    }
  }
}));