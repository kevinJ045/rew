import * as _console from 'ext:deno_console/01_console.js';
globalThis.Deno.inspect = _console.inspect
let _log_out = console.log;
let Deno = globalThis.Deno || {};
globalThis.console = new _console.Console(
  (output, code) => Deno[code ? 'stdout' : 'stderr'].writeSync(Deno.core.encode(output)),
)