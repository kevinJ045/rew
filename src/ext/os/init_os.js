
import * as os from 'ext:deno_os/30_os.js';
import * as signals from "ext:deno_os/40_signals.js";

// import * as imports from "ext:core/ops";

// console.log(Object.keys(imports))

globalThis.Deno.os = os;
globalThis.Deno.signals = signals;