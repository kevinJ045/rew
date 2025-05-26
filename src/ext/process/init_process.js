import { run, Command, ChildProcess, kill } from 'ext:deno_process/40_process.js';
globalThis.Deno.run = run;
globalThis.Deno.kill = kill;
globalThis.Deno.Command = Command;
globalThis.Deno.ChildProcess = ChildProcess;