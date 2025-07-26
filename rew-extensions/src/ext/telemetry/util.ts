// Copyright 2018-2025 the Deno authors. MIT license.
import { primordials } from "ext:core/mod.js";
const { String, StringPrototypeSlice } = primordials;
export function updateSpanFromRequest(span, request) {
    span.updateName(request.method);
    span.setAttribute("http.request.method", request.method);
    const url = new URL(request.url);
    span.setAttribute("url.full", request.url);
    span.setAttribute("url.scheme", StringPrototypeSlice(url.protocol, 0, -1));
    span.setAttribute("url.path", url.pathname);
    span.setAttribute("url.query", StringPrototypeSlice(url.search, 1));
}
export function updateSpanFromResponse(span, response) {
    span.setAttribute("http.response.status_code", String(response.status));
    if (response.status >= 400) {
        span.setAttribute("error.type", String(response.status));
        span.setStatus({ code: 2, message: response.statusText });
    }
}
// deno-lint-ignore no-explicit-any
export function updateSpanFromError(span, error) {
    var _a, _b;
    span.setAttribute("error.type", (_a = error.name) !== null && _a !== void 0 ? _a : "Error");
    span.setStatus({ code: 2, message: (_b = error.message) !== null && _b !== void 0 ? _b : String(error) });
}

