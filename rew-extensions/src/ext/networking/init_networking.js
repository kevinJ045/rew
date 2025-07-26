
import * as net from 'ext:deno_net/01_net.js';
import * as fetch from 'ext:deno_fetch/26_fetch.js';
import { Response } from 'ext:deno_fetch/23_response.js';
import { Request } from 'ext:deno_fetch/23_request.js';
import * as event_source from 'ext:deno_fetch/27_eventsource.js';
import * as websocketstream from 'ext:deno_websocket/02_websocketstream.js';

// console.log('Initializing networking extensions...');

globalThis.Deno.fetch = fetch;
globalThis.Deno.Response = Response;
globalThis.Deno.Request = Request;
globalThis.Request = Request;
globalThis.Response = Response;
globalThis.Deno.net = net;
globalThis.Deno.event_source = event_source;
globalThis.Deno.websocketstream = websocketstream;