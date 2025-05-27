
var _InnerRequest_external, _InnerRequest_context, _InnerRequest_methodAndUri, _InnerRequest_streamRid, _InnerRequest_body, _InnerRequest_upgraded, _InnerRequest_urlValue, _InnerRequest_completed, _ServeHandlerInfo_inner;
import { core, internals, primordials } from "ext:core/mod.js";
const { BadResourcePrototype, InterruptedPrototype, Interrupted, internalRidSymbol, } = core;
import { op_http_cancel, op_http_close, op_http_close_after_finish, op_http_get_request_headers, op_http_get_request_method_and_url, op_http_metric_handle_otel_error, op_http_notify_serving, op_http_read_request_body, op_http_request_on_cancel, op_http_serve, op_http_serve_address_override, op_http_serve_on, op_http_set_promise_complete, op_http_set_response_body_bytes, op_http_set_response_body_resource, op_http_set_response_body_text, op_http_set_response_header, op_http_set_response_headers, op_http_set_response_trailers, op_http_try_wait, op_http_upgrade_raw, op_http_upgrade_websocket_next, op_http_wait, } from "ext:core/ops";
const { ArrayPrototypeFind, ArrayPrototypeMap, ArrayPrototypePush, ObjectHasOwn, ObjectPrototypeIsPrototypeOf, PromisePrototypeCatch, SafeArrayIterator, SafePromisePrototypeFinally, SafePromiseAll, PromisePrototypeThen, StringPrototypeIncludes, StringPrototypeSlice, StringPrototypeStartsWith, Symbol, TypeError, TypedArrayPrototypeGetSymbolToStringTag, Uint8Array, Promise, Number, } = primordials;
import { InnerBody } from "ext:deno_fetch/22_body.js";
import { Event } from "ext:deno_web/02_event.js";
import { fromInnerResponse, newInnerResponse, ResponsePrototype, toInnerResponse, } from "ext:deno_fetch/23_response.js";
import { abortRequest, fromInnerRequest, toInnerRequest, } from "ext:deno_fetch/23_request.js";
import { AbortController } from "ext:deno_web/03_abort_signal.js";
import { _eventLoop, _idleTimeoutDuration, _idleTimeoutTimeout, _readyState, _rid, _role, _serverHandleIdleTimeout, SERVER, WebSocket, } from "ext:deno_websocket/01_websocket.js";
import { Deferred, getReadableStreamResourceBacking, readableStreamForRid, ReadableStreamPrototype, resourceForReadableStream, } from "ext:deno_web/06_streams.js";
import { listen, listenOptionApiName, UpgradedConn, } from "ext:deno_net/01_net.js";
import { hasTlsKeyPairOptions, listenTls } from "ext:deno_net/02_tls.js";
import { SymbolAsyncDispose } from "ext:deno_web/00_infra.js";
import { builtinTracer, ContextManager, currentSnapshot, enterSpan, METRICS_ENABLED, PROPAGATORS, restoreSnapshot, TRACING_ENABLED, } from "ext:deno_telemetry/telemetry.ts";
import { updateSpanFromRequest, updateSpanFromResponse, } from "ext:deno_telemetry/util.ts";
// Copyright 2018-2025 the Deno authors. MIT license.

let console = globalThis.console;

var __classPrivateFieldSet = (this && this.__classPrivateFieldSet) || function (receiver, state, value, kind, f) {
    if (kind === "m") throw new TypeError("Private method is not writable");
    if (kind === "a" && !f) throw new TypeError("Private accessor was defined without a setter");
    if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver)) throw new TypeError("Cannot write private member to an object whose class did not declare it");
    return (kind === "a" ? f.call(receiver, value) : f ? f.value = value : state.set(receiver, value)), value;
};
var __classPrivateFieldGet = (this && this.__classPrivateFieldGet) || function (receiver, state, kind, f) {
    if (kind === "a" && !f) throw new TypeError("Private accessor was defined without a getter");
    if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver)) throw new TypeError("Cannot read private member from an object whose class did not declare it");
    return kind === "m" ? f : kind === "a" ? f.call(receiver) : f ? f.value : state.get(receiver);
};

const _upgraded = Symbol("_upgraded");
function internalServerError() {
    // "Internal Server Error"
    return new Response(new Uint8Array([
        73,
        110,
        116,
        101,
        114,
        110,
        97,
        108,
        32,
        83,
        101,
        114,
        118,
        101,
        114,
        32,
        69,
        114,
        114,
        111,
        114,
    ]), { status: 500 });
}
// Used to ensure that user returns a valid response (but not a different response) from handlers that are upgraded.
const UPGRADE_RESPONSE_SENTINEL = fromInnerResponse(newInnerResponse(101), "immutable");
function upgradeHttpRaw(req, conn) {
    const inner = toInnerRequest(req);
    if (inner._wantsUpgrade) {
        return inner._wantsUpgrade("upgradeHttpRaw", conn);
    }
    throw new TypeError("'upgradeHttpRaw' may only be used with serve");
}
function addTrailers(resp, headerList) {
    const inner = toInnerResponse(resp);
    op_http_set_response_trailers(inner.external, headerList);
}
class InnerRequest {
    constructor(external, context) {
        _InnerRequest_external.set(this, void 0);
        _InnerRequest_context.set(this, void 0);
        _InnerRequest_methodAndUri.set(this, void 0);
        _InnerRequest_streamRid.set(this, void 0);
        _InnerRequest_body.set(this, void 0);
        _InnerRequest_upgraded.set(this, void 0);
        _InnerRequest_urlValue.set(this, void 0);
        _InnerRequest_completed.set(this, void 0);
        __classPrivateFieldSet(this, _InnerRequest_external, external, "f");
        __classPrivateFieldSet(this, _InnerRequest_context, context, "f");
        __classPrivateFieldSet(this, _InnerRequest_upgraded, false, "f");
        __classPrivateFieldSet(this, _InnerRequest_completed, undefined, "f");
    }
    close(success = true) {
        // The completion signal fires only if someone cares
        if (__classPrivateFieldGet(this, _InnerRequest_completed, "f")) {
            if (success) {
                __classPrivateFieldGet(this, _InnerRequest_completed, "f").resolve(undefined);
            }
            else {
                if (!__classPrivateFieldGet(this, _InnerRequest_context, "f").legacyAbort) {
                    abortRequest(this.request);
                }
                __classPrivateFieldGet(this, _InnerRequest_completed, "f").reject(new Interrupted("HTTP response was not sent successfully"));
            }
        }
        if (__classPrivateFieldGet(this, _InnerRequest_context, "f").legacyAbort) {
            abortRequest(this.request);
        }
        __classPrivateFieldSet(this, _InnerRequest_external, null, "f");
    }
    get [(_InnerRequest_external = new WeakMap(), _InnerRequest_context = new WeakMap(), _InnerRequest_methodAndUri = new WeakMap(), _InnerRequest_streamRid = new WeakMap(), _InnerRequest_body = new WeakMap(), _InnerRequest_upgraded = new WeakMap(), _InnerRequest_urlValue = new WeakMap(), _InnerRequest_completed = new WeakMap(), _upgraded)]() {
        return __classPrivateFieldGet(this, _InnerRequest_upgraded, "f");
    }
    _wantsUpgrade(upgradeType, ...originalArgs) {
        if (__classPrivateFieldGet(this, _InnerRequest_upgraded, "f")) {
            throw new errors.Http("Already upgraded");
        }
        if (__classPrivateFieldGet(this, _InnerRequest_external, "f") === null) {
            throw new errors.Http("Already closed");
        }
        // upgradeHttpRaw is sync
        if (upgradeType == "upgradeHttpRaw") {
            const external = __classPrivateFieldGet(this, _InnerRequest_external, "f");
            const underlyingConn = originalArgs[0];
            this.url();
            this.headerList;
            this.close();
            __classPrivateFieldSet(this, _InnerRequest_upgraded, () => { }, "f");
            const upgradeRid = op_http_upgrade_raw(external);
            const conn = new UpgradedConn(upgradeRid, underlyingConn === null || underlyingConn === void 0 ? void 0 : underlyingConn.remoteAddr, underlyingConn === null || underlyingConn === void 0 ? void 0 : underlyingConn.localAddr);
            return { response: UPGRADE_RESPONSE_SENTINEL, conn };
        }
        // upgradeWebSocket is sync
        if (upgradeType == "upgradeWebSocket") {
            const response = originalArgs[0];
            const ws = originalArgs[1];
            const external = __classPrivateFieldGet(this, _InnerRequest_external, "f");
            this.url();
            this.headerList;
            this.close();
            const goAhead = new Deferred();
            __classPrivateFieldSet(this, _InnerRequest_upgraded, () => {
                goAhead.resolve();
            }, "f");
            const wsPromise = op_http_upgrade_websocket_next(external, response.headerList);
            // Start the upgrade in the background.
            (async () => {
                try {
                    // Returns the upgraded websocket connection
                    const wsRid = await wsPromise;
                    // We have to wait for the go-ahead signal
                    await goAhead.promise;
                    ws[_rid] = wsRid;
                    ws[_readyState] = WebSocket.OPEN;
                    ws[_role] = SERVER;
                    const event = new Event("open");
                    ws.dispatchEvent(event);
                    ws[_eventLoop]();
                    if (ws[_idleTimeoutDuration]) {
                        ws.addEventListener("close", () => clearTimeout(ws[_idleTimeoutTimeout]));
                    }
                    ws[_serverHandleIdleTimeout]();
                }
                catch (error) {
                    const event = new ErrorEvent("error", { error });
                    ws.dispatchEvent(event);
                }
            })();
            return { response: UPGRADE_RESPONSE_SENTINEL, socket: ws };
        }
    }
    url() {
        var _a;
        if (__classPrivateFieldGet(this, _InnerRequest_urlValue, "f") !== undefined) {
            return __classPrivateFieldGet(this, _InnerRequest_urlValue, "f");
        }
        if (__classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f") === undefined) {
            if (__classPrivateFieldGet(this, _InnerRequest_external, "f") === null) {
                throw new TypeError("Request closed");
            }
            // TODO(mmastrac): This is quite slow as we're serializing a large number of values. We may want to consider
            // splitting this up into multiple ops.
            __classPrivateFieldSet(this, _InnerRequest_methodAndUri, op_http_get_request_method_and_url(__classPrivateFieldGet(this, _InnerRequest_external, "f")), "f");
        }
        const method = __classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[0];
        const scheme = __classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[5] !== undefined
            ? `${__classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[5]}://`
            : __classPrivateFieldGet(this, _InnerRequest_context, "f").scheme;
        const authority = (_a = __classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[1]) !== null && _a !== void 0 ? _a : __classPrivateFieldGet(this, _InnerRequest_context, "f").fallbackHost;
        const path = __classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[2];
        // * is valid for OPTIONS
        if (method === "OPTIONS" && path === "*") {
            return (__classPrivateFieldSet(this, _InnerRequest_urlValue, scheme + authority + "/" + path, "f"));
        }
        // CONNECT requires an authority
        if (method === "CONNECT") {
            return (__classPrivateFieldSet(this, _InnerRequest_urlValue, scheme + __classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[1], "f"));
        }
        return __classPrivateFieldSet(this, _InnerRequest_urlValue, scheme + authority + path, "f");
    }
    get completed() {
        if (!__classPrivateFieldGet(this, _InnerRequest_completed, "f")) {
            // NOTE: this is faster than Promise.withResolvers()
            let resolve, reject;
            const promise = new Promise((r1, r2) => {
                resolve = r1;
                reject = r2;
            });
            __classPrivateFieldSet(this, _InnerRequest_completed, { promise, resolve, reject }, "f");
        }
        return __classPrivateFieldGet(this, _InnerRequest_completed, "f").promise;
    }
    get remoteAddr() {
        var _a;
        if (__classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f") === undefined) {
            if (__classPrivateFieldGet(this, _InnerRequest_external, "f") === null) {
                throw new TypeError("Request closed");
            }
            __classPrivateFieldSet(this, _InnerRequest_methodAndUri, op_http_get_request_method_and_url(__classPrivateFieldGet(this, _InnerRequest_external, "f")), "f");
        }
        const transport = (_a = __classPrivateFieldGet(this, _InnerRequest_context, "f").listener) === null || _a === void 0 ? void 0 : _a.addr.transport;
        if (__classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[3] === "unix") {
            return {
                transport,
                path: __classPrivateFieldGet(this, _InnerRequest_context, "f").listener.addr.path,
            };
        }
        if (StringPrototypeStartsWith(__classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[3], "vsock:")) {
            return {
                transport,
                cid: Number(StringPrototypeSlice(__classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[3], 6)),
                port: __classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[4],
            };
        }
        return {
            transport: "tcp",
            hostname: __classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[3],
            port: __classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[4],
        };
    }
    get method() {
        if (__classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f") === undefined) {
            if (__classPrivateFieldGet(this, _InnerRequest_external, "f") === null) {
                throw new TypeError("Request closed");
            }
            __classPrivateFieldSet(this, _InnerRequest_methodAndUri, op_http_get_request_method_and_url(__classPrivateFieldGet(this, _InnerRequest_external, "f")), "f");
        }
        return __classPrivateFieldGet(this, _InnerRequest_methodAndUri, "f")[0];
    }
    get body() {
        if (__classPrivateFieldGet(this, _InnerRequest_external, "f") === null) {
            throw new TypeError("Request closed");
        }
        if (__classPrivateFieldGet(this, _InnerRequest_body, "f") !== undefined) {
            return __classPrivateFieldGet(this, _InnerRequest_body, "f");
        }
        // If the method is GET or HEAD, we do not want to include a body here, even if the Rust
        // side of the code is willing to provide it to us.
        if (this.method == "GET" || this.method == "HEAD") {
            __classPrivateFieldSet(this, _InnerRequest_body, null, "f");
            return null;
        }
        __classPrivateFieldSet(this, _InnerRequest_streamRid, op_http_read_request_body(__classPrivateFieldGet(this, _InnerRequest_external, "f")), "f");
        __classPrivateFieldSet(this, _InnerRequest_body, new InnerBody(readableStreamForRid(__classPrivateFieldGet(this, _InnerRequest_streamRid, "f"), false, undefined, (controller, error) => {
            if (ObjectPrototypeIsPrototypeOf(BadResourcePrototype, error)) {
                // TODO(kt3k): We would like to pass `error` as `cause` when BadResource supports it.
                controller.error(new error.constructor(`Cannot read request body as underlying resource unavailable`));
            }
            else {
                controller.error(error);
            }
        })), "f");
        return __classPrivateFieldGet(this, _InnerRequest_body, "f");
    }
    get headerList() {
        if (__classPrivateFieldGet(this, _InnerRequest_external, "f") === null) {
            throw new TypeError("Request closed");
        }
        const headers = [];
        const reqHeaders = op_http_get_request_headers(__classPrivateFieldGet(this, _InnerRequest_external, "f"));
        for (let i = 0; i < reqHeaders.length; i += 2) {
            ArrayPrototypePush(headers, [reqHeaders[i], reqHeaders[i + 1]]);
        }
        return headers;
    }
    get external() {
        return __classPrivateFieldGet(this, _InnerRequest_external, "f");
    }
    onCancel(callback) {
        if (__classPrivateFieldGet(this, _InnerRequest_external, "f") === null) {
            if (__classPrivateFieldGet(this, _InnerRequest_context, "f").legacyAbort)
                callback();
            return;
        }
        PromisePrototypeThen(op_http_request_on_cancel(__classPrivateFieldGet(this, _InnerRequest_external, "f")), (r) => {
            return !__classPrivateFieldGet(this, _InnerRequest_context, "f").legacyAbort ? r && callback() : callback();
        });
    }
}
class CallbackContext {
    constructor(signal, args, listener) {
        this.asyncContextSnapshot = currentSnapshot();
        // The abort signal triggers a non-graceful shutdown
        signal === null || signal === void 0 ? void 0 : signal.addEventListener("abort", () => {
            op_http_cancel(this.serverRid, false);
        }, { once: true });
        this.abortController = new AbortController();
        this.serverRid = args[0];
        this.scheme = args[1];
        this.fallbackHost = args[2];
        this.legacyAbort = args[3] == false;
        this.closed = false;
        this.listener = listener;
    }
    close() {
        try {
            this.closed = true;
            core.tryClose(this.serverRid);
        }
        catch (_a) {
            // Pass
        }
    }
}
class ServeHandlerInfo {
    constructor(inner) {
        _ServeHandlerInfo_inner.set(this, void 0);
        __classPrivateFieldSet(this, _ServeHandlerInfo_inner, inner, "f");
    }
    get remoteAddr() {
        return __classPrivateFieldGet(this, _ServeHandlerInfo_inner, "f").remoteAddr;
    }
    get completed() {
        return __classPrivateFieldGet(this, _ServeHandlerInfo_inner, "f").completed;
    }
}
_ServeHandlerInfo_inner = new WeakMap();
function fastSyncResponseOrStream(req, respBody, status, innerRequest) {
    if (respBody === null || respBody === undefined) {
        // Don't set the body
        innerRequest === null || innerRequest === void 0 ? void 0 : innerRequest.close();
        op_http_set_promise_complete(req, status);
        return;
    }
    const stream = respBody.streamOrStatic;
    const body = stream.body;
    if (body !== undefined) {
        // We ensure the response has not been consumed yet in the caller of this
        // function.
        stream.consumed = true;
    }
    if (TypedArrayPrototypeGetSymbolToStringTag(body) === "Uint8Array") {
        innerRequest === null || innerRequest === void 0 ? void 0 : innerRequest.close();
        op_http_set_response_body_bytes(req, body, status);
        return;
    }
    if (typeof body === "string") {
        innerRequest === null || innerRequest === void 0 ? void 0 : innerRequest.close();
        op_http_set_response_body_text(req, body, status);
        return;
    }
    // At this point in the response it needs to be a stream
    if (!ObjectPrototypeIsPrototypeOf(ReadableStreamPrototype, stream)) {
        innerRequest === null || innerRequest === void 0 ? void 0 : innerRequest.close();
        throw new TypeError("Invalid response");
    }
    const resourceBacking = getReadableStreamResourceBacking(stream);
    let rid, autoClose;
    if (resourceBacking) {
        rid = resourceBacking.rid;
        autoClose = resourceBacking.autoClose;
    }
    else {
        rid = resourceForReadableStream(stream);
        autoClose = true;
    }
    PromisePrototypeThen(op_http_set_response_body_resource(req, rid, autoClose, status), (success) => {
        innerRequest === null || innerRequest === void 0 ? void 0 : innerRequest.close(success);
        op_http_close_after_finish(req);
    });
}
/**
 * Maps the incoming request slab ID to a fully-fledged Request object, passes it to the user-provided
 * callback, then extracts the response that was returned from that callback. The response is then pulled
 * apart and handled on the Rust side.
 *
 * This function returns a promise that will only reject in the case of abnormal exit.
 */
function mapToCallback(context, callback, onError) {
    let mapped = async function (req, span) {
        // Get the response from the user-provided callback. If that fails, use onError. If that fails, return a fallback
        // 500 error.
        let innerRequest;
        let response;
        try {
            innerRequest = new InnerRequest(req, context);
            const request = fromInnerRequest(innerRequest, "immutable");
            innerRequest.request = request;
            if (span) {
                updateSpanFromRequest(span, request);
            }
            response = await callback(request, new ServeHandlerInfo(innerRequest));
            // Throwing Error if the handler return value is not a Response class
            if (!ObjectPrototypeIsPrototypeOf(ResponsePrototype, response)) {
                throw new TypeError("Return value from serve handler must be a response or a promise resolving to a response");
            }
            if (response.type === "error") {
                throw new TypeError("Return value from serve handler must not be an error response (like Response.error())");
            }
            if (response.bodyUsed) {
                throw new TypeError("The body of the Response returned from the serve handler has already been consumed");
            }
        }
        catch (error) {
            try {
                response = await onError(error);
                if (!ObjectPrototypeIsPrototypeOf(ResponsePrototype, response)) {
                    throw new TypeError("Return value from onError handler must be a response or a promise resolving to a response");
                }
            }
            catch (error) {
                if (METRICS_ENABLED) {
                    op_http_metric_handle_otel_error(req);
                }
                console.log("error", "Exception in onError while handling exception", error);
                response = internalServerError();
            }
        }
        if (span) {
            updateSpanFromResponse(span, response);
        }
        const inner = toInnerResponse(response);
        if (innerRequest === null || innerRequest === void 0 ? void 0 : innerRequest[_upgraded]) {
            // We're done here as the connection has been upgraded during the callback and no longer requires servicing.
            if (response !== UPGRADE_RESPONSE_SENTINEL) {
                console.log("error", "Upgrade response was not returned from callback");
                context.close();
            }
            innerRequest === null || innerRequest === void 0 ? void 0 : innerRequest[_upgraded]();
            return;
        }
        // Did everything shut down while we were waiting?
        if (context.closed) {
            // We're shutting down, so this status shouldn't make it back to the client but "Service Unavailable" seems appropriate
            innerRequest === null || innerRequest === void 0 ? void 0 : innerRequest.close();
            op_http_set_promise_complete(req, 503);
            return;
        }
        const status = inner.status;
        const headers = inner.headerList;
        if (headers && headers.length > 0) {
            if (headers.length == 1) {
                op_http_set_response_header(req, headers[0][0], headers[0][1]);
            }
            else {
                op_http_set_response_headers(req, headers);
            }
        }
        fastSyncResponseOrStream(req, inner.body, status, innerRequest);
    };
    if (TRACING_ENABLED) {
        const origMapped = mapped;
        mapped = function (req, _span) {
            const snapshot = currentSnapshot();
            restoreSnapshot(context.asyncContext);
            const reqHeaders = op_http_get_request_headers(req);
            const headers = [];
            for (let i = 0; i < reqHeaders.length; i += 2) {
                ArrayPrototypePush(headers, [reqHeaders[i], reqHeaders[i + 1]]);
            }
            let activeContext = ContextManager.active();
            for (const propagator of new SafeArrayIterator(PROPAGATORS)) {
                activeContext = propagator.extract(activeContext, headers, {
                    get(carrier, key) {
                        var _a;
                        return (_a = ArrayPrototypeFind(carrier, (carrierEntry) => carrierEntry[0] === key)) === null || _a === void 0 ? void 0 : _a[1];
                    },
                    keys(carrier) {
                        return ArrayPrototypeMap(carrier, (carrierEntry) => carrierEntry[0]);
                    },
                });
            }
            const span = builtinTracer().startSpan("serve", { kind: 1 }, activeContext);
            enterSpan(span);
            try {
                return SafePromisePrototypeFinally(origMapped(req, span), () => span.end());
            }
            finally {
                restoreSnapshot(snapshot);
            }
        };
    }
    else {
        const origMapped = mapped;
        mapped = function (req, span) {
            const snapshot = currentSnapshot();
            restoreSnapshot(context.asyncContext);
            try {
                return origMapped(req, span);
            }
            finally {
                restoreSnapshot(snapshot);
            }
        };
    }
    return mapped;
}
const kLoadBalanced = Symbol("kLoadBalanced");
function formatHostName(hostname) {
    // If the hostname is "0.0.0.0", we display "localhost" in console
    // because browsers in Windows don't resolve "0.0.0.0".
    // See the discussion in https://github.com/denoland/deno_std/issues/1165
    if (core.build.os === "windows" &&
        (hostname == "0.0.0.0" || hostname == "::")) {
        return "localhost";
    }
    // Add brackets around ipv6 hostname
    return StringPrototypeIncludes(hostname, ":") ? `[${hostname}]` : hostname;
}
function serve(arg1, arg2) {
    let options;
    let handler;
    if (typeof arg1 === "function") {
        handler = arg1;
    }
    else if (typeof arg2 === "function") {
        handler = arg2;
        options = arg1;
    }
    else {
        options = arg1;
    }
    if (handler === undefined) {
        if (options === undefined) {
            throw new TypeError("Cannot serve HTTP requests: either a `handler` or `options` must be specified");
        }
        handler = options.handler;
    }
    if (typeof handler !== "function") {
        throw new TypeError(`Cannot serve HTTP requests: handler must be a function, received ${typeof handler}`);
    }
    if (options === undefined) {
        options = { __proto__: null };
    }
    const { 0: overrideKind, 1: overrideHost, 2: overridePort, 3: duplicateListener, } = op_http_serve_address_override();
    if (overrideKind) {
        let envOptions = duplicateListener ? { __proto__: null } : options;
        switch (overrideKind) {
            case 1: {
                // TCP
                envOptions = Object.assign(Object.assign({}, envOptions), { hostname: overrideHost, port: overridePort });
                delete envOptions.path;
                delete envOptions.cid;
                break;
            }
            case 2: {
                // Unix
                envOptions = Object.assign(Object.assign({}, envOptions), { path: overrideHost });
                delete envOptions.hostname;
                delete envOptions.cid;
                delete envOptions.port;
                break;
            }
            case 3: {
                // Vsock
                envOptions = Object.assign(Object.assign({}, envOptions), { cid: Number(overrideHost), port: overridePort });
                delete envOptions.hostname;
                delete envOptions.path;
                break;
            }
        }
        if (duplicateListener) {
            envOptions.onListen = () => {
                // override default console.log behavior
            };
            const envListener = serveInner(envOptions, handler);
            const userListener = serveInner(options, handler);
            return {
                addr: userListener.addr,
                finished: SafePromiseAll([envListener.finished, userListener.finished]),
                shutdown() {
                    return SafePromiseAll([
                        envListener.shutdown(),
                        userListener.shutdown(),
                    ]);
                },
                ref() {
                    envListener.ref();
                    userListener.ref();
                },
                unref() {
                    envListener.unref();
                    userListener.unref();
                },
                [SymbolAsyncDispose]() {
                    return this.shutdown();
                },
            };
        }
        options = envOptions;
    }
    return serveInner(options, handler);
}
function serveInner(options, handler) {
    var _a, _b, _c, _d, _e;
    const wantsHttps = hasTlsKeyPairOptions(options);
    const wantsUnix = ObjectHasOwn(options, "path");
    const wantsVsock = ObjectHasOwn(options, "cid");
    const signal = options.signal;
    const onError = (_a = options.onError) !== null && _a !== void 0 ? _a : function (error) {
        console.log("error", error);
        return internalServerError();
    };
    if (wantsUnix) {
        const listener = listen({
            transport: "unix",
            path: options.path,
            [listenOptionApiName]: "serve",
        });
        const path = listener.addr.path;
        return serveHttpOnListener(listener, signal, handler, onError, () => {
            if (options.onListen) {
                options.onListen(listener.addr);
            }
            else {
                console.log("info", `Listening on ${path}`);
            }
        });
    }
    if (wantsVsock) {
        const listener = listen({
            transport: "vsock",
            cid: options.cid,
            port: options.port,
            [listenOptionApiName]: "serve",
        });
        const { cid, port } = listener.addr;
        return serveHttpOnListener(listener, signal, handler, onError, () => {
            if (options.onListen) {
                options.onListen(listener.addr);
            }
            else {
                console.log("info", `Listening on vsock:${cid}:${port}`);
            }
        });
    }
    const listenOpts = {
        hostname: (_b = options.hostname) !== null && _b !== void 0 ? _b : "0.0.0.0",
        port: (_c = options.port) !== null && _c !== void 0 ? _c : 8000,
        reusePort: (_d = options.reusePort) !== null && _d !== void 0 ? _d : false,
        loadBalanced: (_e = options[kLoadBalanced]) !== null && _e !== void 0 ? _e : false,
    };
    if (options.certFile || options.keyFile) {
        throw new TypeError("Unsupported 'certFile' / 'keyFile' options provided: use 'cert' / 'key' instead.");
    }
    if (options.alpnProtocols) {
        throw new TypeError("Unsupported 'alpnProtocols' option provided. 'h2' and 'http/1.1' are automatically supported.");
    }
    let listener;
    if (wantsHttps) {
        if (!options.cert || !options.key) {
            throw new TypeError("Both 'cert' and 'key' must be provided to enable HTTPS");
        }
        listenOpts.cert = options.cert;
        listenOpts.key = options.key;
        listenOpts.alpnProtocols = ["h2", "http/1.1"];
        listener = listenTls(listenOpts);
        listenOpts.port = listener.addr.port;
    }
    else {
        listener = listen(listenOpts);
        listenOpts.port = listener.addr.port;
    }
    const addr = listener.addr;
    const onListen = (scheme) => {
        if (options.onListen) {
            options.onListen(addr);
        }
        else {
            const host = formatHostName(addr.hostname);
            const url = `${scheme}${host}:${addr.port}/`;
            const helper = addr.hostname === "0.0.0.0" || addr.hostname === "::"
                ? ` (${scheme}localhost:${addr.port}/)`
                : "";
            console.log("info", `Listening on ${url}${helper}`);
        }
    };
    return serveHttpOnListener(listener, signal, handler, onError, onListen);
}
/**
 * Serve HTTP/1.1 and/or HTTP/2 on an arbitrary listener.
 */
function serveHttpOnListener(listener, signal, handler, onError, onListen) {
    const context = new CallbackContext(signal, op_http_serve(listener[internalRidSymbol]), listener);
    const callback = mapToCallback(context, handler, onError);
    onListen(context.scheme);
    return serveHttpOn(context, listener.addr, callback);
}
/**
 * Serve HTTP/1.1 and/or HTTP/2 on an arbitrary connection.
 */
function serveHttpOnConnection(connection, signal, handler, onError, onListen) {
    const context = new CallbackContext(signal, op_http_serve_on(connection[internalRidSymbol]), null);
    const callback = mapToCallback(context, handler, onError);
    onListen(context.scheme);
    return serveHttpOn(context, connection.localAddr, callback);
}
function serveHttpOn(context, addr, callback) {
    let ref = true;
    let currentPromise = null;
    const promiseErrorHandler = (error) => {
        // Abnormal exit
        console.log("error", "Terminating serve loop due to unexpected error", error);
        context.close();
    };
    // Run the server
    const finished = (async () => {
        const rid = context.serverRid;
        while (true) {
            let req;
            try {
                // Attempt to pull as many requests out of the queue as possible before awaiting. This API is
                // a synchronous, non-blocking API that returns u32::MAX if anything goes wrong.
                while ((req = op_http_try_wait(rid)) !== null) {
                    PromisePrototypeCatch(callback(req, undefined), promiseErrorHandler);
                }
                currentPromise = op_http_wait(rid);
                if (!ref) {
                    core.unrefOpPromise(currentPromise);
                }
                req = await currentPromise;
                currentPromise = null;
            }
            catch (error) {
                if (ObjectPrototypeIsPrototypeOf(BadResourcePrototype, error)) {
                    break;
                }
                if (ObjectPrototypeIsPrototypeOf(InterruptedPrototype, error)) {
                    break;
                }
                throw new errors.Http(error);
            }
            if (req === null) {
                break;
            }
            PromisePrototypeCatch(callback(req, undefined), promiseErrorHandler);
        }
        try {
            if (!context.closing && !context.closed) {
                context.closing = await op_http_close(rid, false);
                context.close();
            }
            await context.closing;
        }
        catch (error) {
            if (ObjectPrototypeIsPrototypeOf(InterruptedPrototype, error)) {
                return;
            }
            if (ObjectPrototypeIsPrototypeOf(BadResourcePrototype, error)) {
                return;
            }
            throw error;
        }
        finally {
            context.close();
            context.closed = true;
        }
    })();
    op_http_notify_serving();
    return {
        addr,
        finished,
        async shutdown() {
            try {
                if (!context.closing && !context.closed) {
                    // Shut this HTTP server down gracefully
                    context.closing = op_http_close(context.serverRid, true);
                }
                await context.closing;
            }
            catch (error) {
                // The server was interrupted
                if (ObjectPrototypeIsPrototypeOf(InterruptedPrototype, error)) {
                    return;
                }
                if (ObjectPrototypeIsPrototypeOf(BadResourcePrototype, error)) {
                    return;
                }
                throw error;
            }
            finally {
                context.closed = true;
            }
        },
        ref() {
            ref = true;
            if (currentPromise) {
                core.refOpPromise(currentPromise);
            }
        },
        unref() {
            ref = false;
            if (currentPromise) {
                core.unrefOpPromise(currentPromise);
            }
        },
        [SymbolAsyncDispose]() {
            return this.shutdown();
        },
    };
}
internals.addTrailers = addTrailers;
internals.upgradeHttpRaw = upgradeHttpRaw;
internals.serveHttpOnListener = serveHttpOnListener;
internals.serveHttpOnConnection = serveHttpOnConnection;
function registerDeclarativeServer(exports) {
    if (!ObjectHasOwn(exports, "fetch"))
        return;
    if (typeof exports.fetch !== "function") {
        throw new TypeError("Invalid type for fetch: must be a function");
    }
    return ({ servePort, serveHost, workerCountWhenMain, }) => {
        serve({
            port: servePort,
            hostname: serveHost,
            [kLoadBalanced]: workerCountWhenMain == null
                ? true
                : workerCountWhenMain > 0,
            onListen: ({ transport, port, hostname, path, cid }) => {
                if (workerCountWhenMain != null) {
                    const nThreads = workerCountWhenMain > 0
                        ? ` with ${workerCountWhenMain + 1} threads`
                        : "";
                    let target;
                    switch (transport) {
                        case "tcp":
                            target = `http://${formatHostName(hostname)}:${port}/`;
                            break;
                        case "unix":
                            target = path;
                            break;
                        case "vsock":
                            target = `vsock:${cid}:${port}`;
                            break;
                    }
                    console.log("info", `%cdeno serve%c: Listening on %c${target}%c${nThreads}`, "color: green", "color: inherit", "color: yellow", "color: inherit");
                }
            }
        }, (req, connInfo) => {
            return exports.fetch(req, connInfo);
        });
    };
}
export { addTrailers, registerDeclarativeServer, serve, serveHttpOnConnection, serveHttpOnListener, upgradeHttpRaw, };

