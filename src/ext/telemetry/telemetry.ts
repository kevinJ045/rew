// Copyright 2018-2025 the Deno authors. MIT license.
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
var __classPrivateFieldIn = (this && this.__classPrivateFieldIn) || function(state, receiver) {
    if (receiver === null || (typeof receiver !== "object" && typeof receiver !== "function")) throw new TypeError("Cannot use 'in' operator on non-object");
    return typeof state === "function" ? receiver === state : state.has(receiver);
};
var __rest = (this && this.__rest) || function (s, e) {
    var t = {};
    for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p) && e.indexOf(p) < 0)
        t[p] = s[p];
    if (s != null && typeof Object.getOwnPropertySymbols === "function")
        for (var i = 0, p = Object.getOwnPropertySymbols(s); i < p.length; i++) {
            if (e.indexOf(p[i]) < 0 && Object.prototype.propertyIsEnumerable.call(s, p[i]))
                t[p[i]] = s[p[i]];
        }
    return t;
};
var _Tracer_tracer, _Span_otelSpan, _Span_spanContext, _Context_data, _BatchObservableResult_observables, _Meter_meter, _Counter_instrument, _Counter_upDown, _Gauge_instrument, _Histogram_instrument, _Observable_result, _ObservableResult_instrument, _ObservableResult_isRegularCounter, _BaggageImpl_entries;
import { core, primordials } from "ext:core/mod.js";
import { op_otel_collect_isolate_metrics, op_otel_enable_isolate_metrics, op_otel_log, op_otel_log_foreign, op_otel_metric_attribute3, op_otel_metric_observable_record0, op_otel_metric_observable_record1, op_otel_metric_observable_record2, op_otel_metric_observable_record3, op_otel_metric_observation_done, op_otel_metric_record0, op_otel_metric_record1, op_otel_metric_record2, op_otel_metric_record3, op_otel_metric_wait_to_observe, op_otel_span_add_link, op_otel_span_attribute1, op_otel_span_attribute2, op_otel_span_attribute3, op_otel_span_update_name, OtelMeter, OtelTracer, } from "ext:core/ops";
import { Console } from "ext:deno_console/01_console.js";
const { ArrayFrom, ArrayIsArray, ArrayPrototypeFilter, ArrayPrototypeForEach, ArrayPrototypeJoin, ArrayPrototypeMap, ArrayPrototypePush, ArrayPrototypeReduce, ArrayPrototypeReverse, ArrayPrototypeShift, ArrayPrototypeSlice, DatePrototype, DatePrototypeGetTime, Error, MapPrototypeEntries, MapPrototypeKeys, Number, NumberParseInt, NumberPrototypeToString, ObjectAssign, ObjectDefineProperty, ObjectEntries, ObjectKeys, ObjectPrototypeIsPrototypeOf, ObjectValues, ReflectApply, SafeArrayIterator, SafeIterator, SafeMap, SafePromiseAll, SafeRegExp, SafeSet, SafeWeakSet, StringPrototypeIndexOf, StringPrototypeSlice, StringPrototypeSplit, StringPrototypeSubstring, StringPrototypeTrim, SymbolFor, TypeError, decodeURIComponent, encodeURIComponent, } = primordials;
const { AsyncVariable, getAsyncContext, setAsyncContext } = core;
export let TRACING_ENABLED = false;
export let METRICS_ENABLED = false;
export let PROPAGATORS = [];
let ISOLATE_METRICS = false;
// Note: These start at 0 in the JS library,
// but start at 1 when serialized with JSON.
var SpanKind;
(function (SpanKind) {
    SpanKind[SpanKind["INTERNAL"] = 0] = "INTERNAL";
    SpanKind[SpanKind["SERVER"] = 1] = "SERVER";
    SpanKind[SpanKind["CLIENT"] = 2] = "CLIENT";
    SpanKind[SpanKind["PRODUCER"] = 3] = "PRODUCER";
    SpanKind[SpanKind["CONSUMER"] = 4] = "CONSUMER";
})(SpanKind || (SpanKind = {}));
var SpanStatusCode;
(function (SpanStatusCode) {
    SpanStatusCode[SpanStatusCode["UNSET"] = 0] = "UNSET";
    SpanStatusCode[SpanStatusCode["OK"] = 1] = "OK";
    SpanStatusCode[SpanStatusCode["ERROR"] = 2] = "ERROR";
})(SpanStatusCode || (SpanStatusCode = {}));
function hrToMs(hr) {
    return (hr[0] * 1e3 + hr[1] / 1e6);
}
function isTimeInput(input) {
    return typeof input === "number" ||
        (input && (ArrayIsArray(input) || isDate(input)));
}
function timeInputToMs(input) {
    if (input === undefined)
        return;
    if (ArrayIsArray(input)) {
        return hrToMs(input);
    }
    else if (isDate(input)) {
        return DatePrototypeGetTime(input);
    }
    return input;
}
function countAttributes(attributes) {
    return attributes ? ObjectKeys(attributes).length : 0;
}
export function enterSpan(span) {
    var _a;
    if (!span.isRecording())
        return undefined;
    const context = ((_a = CURRENT.get()) !== null && _a !== void 0 ? _a : ROOT_CONTEXT).setValue(SPAN_KEY, span);
    return CURRENT.enter(context);
}
export const currentSnapshot = getAsyncContext;
export const restoreSnapshot = setAsyncContext;
function isDate(value) {
    return ObjectPrototypeIsPrototypeOf(DatePrototype, value);
}
var SpanAttributesLocation;
(function (SpanAttributesLocation) {
    SpanAttributesLocation[SpanAttributesLocation["SELF"] = 0] = "SELF";
    SpanAttributesLocation[SpanAttributesLocation["LAST_EVENT"] = 1] = "LAST_EVENT";
    SpanAttributesLocation[SpanAttributesLocation["LAST_LINK"] = 2] = "LAST_LINK";
})(SpanAttributesLocation || (SpanAttributesLocation = {}));
function spanAddAttributes(span, attributesLocation, attributes) {
    const attributeKvs = ObjectEntries(attributes);
    let i = 0;
    while (i < attributeKvs.length) {
        if (i + 2 < attributeKvs.length) {
            op_otel_span_attribute3(span, attributesLocation, attributeKvs[i][0], attributeKvs[i][1], attributeKvs[i + 1][0], attributeKvs[i + 1][1], attributeKvs[i + 2][0], attributeKvs[i + 2][1]);
            i += 3;
        }
        else if (i + 1 < attributeKvs.length) {
            op_otel_span_attribute2(span, attributesLocation, attributeKvs[i][0], attributeKvs[i][1], attributeKvs[i + 1][0], attributeKvs[i + 1][1]);
            i += 2;
        }
        else {
            op_otel_span_attribute1(span, attributesLocation, attributeKvs[i][0], attributeKvs[i][1]);
            i += 1;
        }
    }
}
class TracerProvider {
    constructor() {
        throw new TypeError("TracerProvider can not be constructed");
    }
    static getTracer(name, version, options) {
        const tracer = new OtelTracer(name, version, options === null || options === void 0 ? void 0 : options.schemaUrl);
        return new Tracer(tracer);
    }
}
class Tracer {
    constructor(tracer) {
        _Tracer_tracer.set(this, void 0);
        __classPrivateFieldSet(this, _Tracer_tracer, tracer, "f");
    }
    startActiveSpan(name, optionsOrFn, fnOrContext, maybeFn) {
        var _a;
        let options;
        let context;
        let fn;
        if (typeof optionsOrFn === "function") {
            options = undefined;
            fn = optionsOrFn;
        }
        else if (typeof fnOrContext === "function") {
            options = optionsOrFn;
            fn = fnOrContext;
        }
        else if (typeof maybeFn === "function") {
            options = optionsOrFn;
            context = fnOrContext;
            fn = maybeFn;
        }
        else {
            throw new Error("startActiveSpan requires a function argument");
        }
        if (options === null || options === void 0 ? void 0 : options.root) {
            context = ROOT_CONTEXT;
        }
        else {
            context = (_a = context !== null && context !== void 0 ? context : CURRENT.get()) !== null && _a !== void 0 ? _a : ROOT_CONTEXT;
        }
        const span = this.startSpan(name, options, context);
        const ctx = CURRENT.enter(context.setValue(SPAN_KEY, span));
        try {
            return ReflectApply(fn, undefined, [span]);
        }
        finally {
            setAsyncContext(ctx);
        }
    }
    startSpan(name, options, context) {
        var _a, _b, _c;
        if (options === null || options === void 0 ? void 0 : options.root) {
            context = undefined;
        }
        else {
            context = context !== null && context !== void 0 ? context : CURRENT.get();
        }
        const startTime = timeInputToMs(options === null || options === void 0 ? void 0 : options.startTime);
        const parentSpan = context === null || context === void 0 ? void 0 : context.getValue(SPAN_KEY);
        const attributesCount = countAttributes(options === null || options === void 0 ? void 0 : options.attributes);
        const parentOtelSpan = parentSpan !== undefined
            ? (_a = getOtelSpan(parentSpan)) !== null && _a !== void 0 ? _a : undefined
            : undefined;
        let otelSpan;
        if (parentOtelSpan || !parentSpan) {
            otelSpan = __classPrivateFieldGet(this, _Tracer_tracer, "f").startSpan(parentOtelSpan, name, (_b = options === null || options === void 0 ? void 0 : options.kind) !== null && _b !== void 0 ? _b : 0, startTime, attributesCount);
        }
        else {
            const spanContext = parentSpan.spanContext();
            otelSpan = __classPrivateFieldGet(this, _Tracer_tracer, "f").startSpanForeign(spanContext.traceId, spanContext.spanId, name, (_c = options === null || options === void 0 ? void 0 : options.kind) !== null && _c !== void 0 ? _c : 0, startTime, attributesCount);
        }
        const span = new Span(otelSpan);
        if (options === null || options === void 0 ? void 0 : options.links)
            span.addLinks(options === null || options === void 0 ? void 0 : options.links);
        if (options === null || options === void 0 ? void 0 : options.attributes)
            span.setAttributes(options === null || options === void 0 ? void 0 : options.attributes);
        return span;
    }
}
_Tracer_tracer = new WeakMap();
const SPAN_KEY = SymbolFor("OpenTelemetry Context Key SPAN");
let getOtelSpan;
class Span {
    constructor(otelSpan) {
        _Span_otelSpan.set(this, void 0);
        _Span_spanContext.set(this, void 0);
        __classPrivateFieldSet(this, _Span_otelSpan, otelSpan, "f");
    }
    spanContext() {
        if (!__classPrivateFieldGet(this, _Span_spanContext, "f")) {
            if (__classPrivateFieldGet(this, _Span_otelSpan, "f")) {
                __classPrivateFieldSet(this, _Span_spanContext, __classPrivateFieldGet(this, _Span_otelSpan, "f").spanContext(), "f");
            }
            else {
                __classPrivateFieldSet(this, _Span_spanContext, {
                    traceId: "00000000000000000000000000000000",
                    spanId: "0000000000000000",
                    traceFlags: 0,
                }, "f");
            }
        }
        return __classPrivateFieldGet(this, _Span_spanContext, "f");
    }
    addEvent(name, attributesOrStartTime, startTime) {
        if (!__classPrivateFieldGet(this, _Span_otelSpan, "f"))
            return this;
        let attributes;
        if (isTimeInput(attributesOrStartTime)) {
            startTime = attributesOrStartTime;
        }
        else {
            attributes = attributesOrStartTime;
        }
        const startTimeMs = timeInputToMs(startTime);
        __classPrivateFieldGet(this, _Span_otelSpan, "f").addEvent(name, startTimeMs !== null && startTimeMs !== void 0 ? startTimeMs : NaN);
        if (attributes) {
            spanAddAttributes(__classPrivateFieldGet(this, _Span_otelSpan, "f"), SpanAttributesLocation.LAST_EVENT, attributes);
        }
        return this;
    }
    addLink(link) {
        var _a, _b;
        if (!__classPrivateFieldGet(this, _Span_otelSpan, "f"))
            return this;
        const valid = op_otel_span_add_link(__classPrivateFieldGet(this, _Span_otelSpan, "f"), link.context.traceId, link.context.spanId, link.context.traceFlags, (_a = link.context.isRemote) !== null && _a !== void 0 ? _a : false, (_b = link.droppedAttributesCount) !== null && _b !== void 0 ? _b : 0);
        if (link.attributes) {
            spanAddAttributes(__classPrivateFieldGet(this, _Span_otelSpan, "f"), SpanAttributesLocation.LAST_LINK, link.attributes);
        }
        if (!valid)
            return this;
        return this;
    }
    addLinks(links) {
        for (let i = 0; i < links.length; i++) {
            this.addLink(links[i]);
        }
        return this;
    }
    end(endTime) {
        var _a;
        (_a = __classPrivateFieldGet(this, _Span_otelSpan, "f")) === null || _a === void 0 ? void 0 : _a.end(timeInputToMs(endTime) || NaN);
    }
    isRecording() {
        return __classPrivateFieldGet(this, _Span_otelSpan, "f") !== undefined;
    }
    recordException(exception, time) {
        if (typeof exception === "string") {
            this.addEvent("exception", {
                "exception.message": exception,
            }, time);
            return;
        }
        const attributes = {};
        if (exception.code) {
            if (typeof exception.code === "number") {
                attributes["exception.type"] = NumberPrototypeToString(exception.code);
            }
            else {
                attributes["exception.type"] = exception.code;
            }
        }
        else if (exception.name) {
            attributes["exception.type"] = exception.name;
        }
        if (exception.message) {
            attributes["exception.message"] = exception.message;
        }
        if (exception.stack) {
            attributes["exception.stacktrace"] = exception.stack;
        }
        this.addEvent("exception", attributes, time);
    }
    setAttribute(key, value) {
        if (!__classPrivateFieldGet(this, _Span_otelSpan, "f"))
            return this;
        op_otel_span_attribute1(__classPrivateFieldGet(this, _Span_otelSpan, "f"), SpanAttributesLocation.SELF, key, value);
        return this;
    }
    setAttributes(attributes) {
        if (!__classPrivateFieldGet(this, _Span_otelSpan, "f"))
            return this;
        spanAddAttributes(__classPrivateFieldGet(this, _Span_otelSpan, "f"), SpanAttributesLocation.SELF, attributes);
        return this;
    }
    setStatus(status) {
        var _a, _b;
        (_a = __classPrivateFieldGet(this, _Span_otelSpan, "f")) === null || _a === void 0 ? void 0 : _a.setStatus(status.code, (_b = status.message) !== null && _b !== void 0 ? _b : "");
        return this;
    }
    updateName(name) {
        if (!__classPrivateFieldGet(this, _Span_otelSpan, "f"))
            return this;
        op_otel_span_update_name(__classPrivateFieldGet(this, _Span_otelSpan, "f"), name);
        return this;
    }
}
_Span_otelSpan = new WeakMap(), _Span_spanContext = new WeakMap();
(() => {
    // deno-lint-ignore prefer-primordials
    getOtelSpan = (span) => (__classPrivateFieldIn(_Span_otelSpan, span) ? __classPrivateFieldGet(span, _Span_otelSpan, "f") : undefined);
})();
const CURRENT = new AsyncVariable();
class Context {
    constructor(data) {
        // @ts-ignore __proto__ is not supported in TypeScript
        _Context_data.set(this, { __proto__: null });
        // @ts-ignore __proto__ is not supported in TypeScript
        __classPrivateFieldSet(this, _Context_data, Object.assign({ __proto__: null }, data), "f");
    }
    getValue(key) {
        return __classPrivateFieldGet(this, _Context_data, "f")[key];
    }
    setValue(key, value) {
        const c = new Context(__classPrivateFieldGet(this, _Context_data, "f"));
        __classPrivateFieldGet(c, _Context_data, "f")[key] = value;
        return c;
    }
    deleteValue(key) {
        const c = new Context(__classPrivateFieldGet(this, _Context_data, "f"));
        delete __classPrivateFieldGet(c, _Context_data, "f")[key];
        return c;
    }
}
_Context_data = new WeakMap();
// TODO(lucacasonato): @opentelemetry/api defines it's own ROOT_CONTEXT
const ROOT_CONTEXT = new Context();
// Context manager for opentelemetry js library
export class ContextManager {
    constructor() {
        throw new TypeError("ContextManager can not be constructed");
    }
    static active() {
        var _a;
        return (_a = CURRENT.get()) !== null && _a !== void 0 ? _a : ROOT_CONTEXT;
    }
    static with(context, fn, thisArg, ...args) {
        const ctx = CURRENT.enter(context);
        try {
            return ReflectApply(fn, thisArg, args);
        }
        finally {
            setAsyncContext(ctx);
        }
    }
    // deno-lint-ignore no-explicit-any
    static bind(context, target) {
        return ((...args) => {
            const ctx = CURRENT.enter(context);
            try {
                return ReflectApply(target, this, args);
            }
            finally {
                setAsyncContext(ctx);
            }
        });
    }
    static enable() {
        return this;
    }
    static disable() {
        return this;
    }
}
var ValueType;
(function (ValueType) {
    ValueType[ValueType["INT"] = 0] = "INT";
    ValueType[ValueType["DOUBLE"] = 1] = "DOUBLE";
})(ValueType || (ValueType = {}));
class MeterProvider {
    constructor() {
        throw new TypeError("MeterProvider can not be constructed");
    }
    static getMeter(name, version, options) {
        const meter = new OtelMeter(name, version, options === null || options === void 0 ? void 0 : options.schemaUrl);
        return new Meter(meter);
    }
}
let batchResultHasObservables;
class BatchObservableResult {
    constructor(observables) {
        _BatchObservableResult_observables.set(this, void 0);
        __classPrivateFieldSet(this, _BatchObservableResult_observables, observables, "f");
    }
    observe(metric, value, attributes) {
        if (!__classPrivateFieldGet(this, _BatchObservableResult_observables, "f").has(metric))
            return;
        getObservableResult(metric).observe(value, attributes);
    }
}
_BatchObservableResult_observables = new WeakMap();
(() => {
    batchResultHasObservables = (cb, observables) => {
        for (const observable of new SafeIterator(observables)) {
            if (!__classPrivateFieldGet(cb, _BatchObservableResult_observables, "f").has(observable))
                return false;
        }
        return true;
    };
})();
const BATCH_CALLBACKS = new SafeMap();
const INDIVIDUAL_CALLBACKS = new SafeMap();
class Meter {
    constructor(meter) {
        _Meter_meter.set(this, void 0);
        __classPrivateFieldSet(this, _Meter_meter, meter, "f");
    }
    createCounter(name, options) {
        if ((options === null || options === void 0 ? void 0 : options.valueType) !== undefined && (options === null || options === void 0 ? void 0 : options.valueType) !== 1) {
            throw new Error("Only valueType: DOUBLE is supported");
        }
        if (!METRICS_ENABLED)
            return new Counter(null, false);
        const instrument = __classPrivateFieldGet(this, _Meter_meter, "f").createCounter(name, 
        // deno-lint-ignore prefer-primordials
        options === null || options === void 0 ? void 0 : options.description, options === null || options === void 0 ? void 0 : options.unit);
        return new Counter(instrument, false);
    }
    createUpDownCounter(name, options) {
        if ((options === null || options === void 0 ? void 0 : options.valueType) !== undefined && (options === null || options === void 0 ? void 0 : options.valueType) !== 1) {
            throw new Error("Only valueType: DOUBLE is supported");
        }
        if (!METRICS_ENABLED)
            return new Counter(null, true);
        const instrument = __classPrivateFieldGet(this, _Meter_meter, "f").createUpDownCounter(name, 
        // deno-lint-ignore prefer-primordials
        options === null || options === void 0 ? void 0 : options.description, options === null || options === void 0 ? void 0 : options.unit);
        return new Counter(instrument, true);
    }
    createGauge(name, options) {
        if ((options === null || options === void 0 ? void 0 : options.valueType) !== undefined && (options === null || options === void 0 ? void 0 : options.valueType) !== 1) {
            throw new Error("Only valueType: DOUBLE is supported");
        }
        if (!METRICS_ENABLED)
            return new Gauge(null);
        const instrument = __classPrivateFieldGet(this, _Meter_meter, "f").createGauge(name, 
        // deno-lint-ignore prefer-primordials
        options === null || options === void 0 ? void 0 : options.description, options === null || options === void 0 ? void 0 : options.unit);
        return new Gauge(instrument);
    }
    createHistogram(name, options) {
        var _a;
        if ((options === null || options === void 0 ? void 0 : options.valueType) !== undefined && (options === null || options === void 0 ? void 0 : options.valueType) !== 1) {
            throw new Error("Only valueType: DOUBLE is supported");
        }
        if (!METRICS_ENABLED)
            return new Histogram(null);
        const instrument = __classPrivateFieldGet(this, _Meter_meter, "f").createHistogram(name, 
        // deno-lint-ignore prefer-primordials
        options === null || options === void 0 ? void 0 : options.description, options === null || options === void 0 ? void 0 : options.unit, (_a = options === null || options === void 0 ? void 0 : options.advice) === null || _a === void 0 ? void 0 : _a.explicitBucketBoundaries);
        return new Histogram(instrument);
    }
    createObservableCounter(name, options) {
        if ((options === null || options === void 0 ? void 0 : options.valueType) !== undefined && (options === null || options === void 0 ? void 0 : options.valueType) !== 1) {
            throw new Error("Only valueType: DOUBLE is supported");
        }
        if (!METRICS_ENABLED)
            new Observable(new ObservableResult(null, true));
        const instrument = __classPrivateFieldGet(this, _Meter_meter, "f").createObservableCounter(name, 
        // deno-lint-ignore prefer-primordials
        options === null || options === void 0 ? void 0 : options.description, options === null || options === void 0 ? void 0 : options.unit);
        return new Observable(new ObservableResult(instrument, true));
    }
    createObservableUpDownCounter(name, options) {
        if ((options === null || options === void 0 ? void 0 : options.valueType) !== undefined && (options === null || options === void 0 ? void 0 : options.valueType) !== 1) {
            throw new Error("Only valueType: DOUBLE is supported");
        }
        if (!METRICS_ENABLED)
            new Observable(new ObservableResult(null, false));
        const instrument = __classPrivateFieldGet(this, _Meter_meter, "f").createObservableUpDownCounter(name, 
        // deno-lint-ignore prefer-primordials
        options === null || options === void 0 ? void 0 : options.description, options === null || options === void 0 ? void 0 : options.unit);
        return new Observable(new ObservableResult(instrument, false));
    }
    createObservableGauge(name, options) {
        if ((options === null || options === void 0 ? void 0 : options.valueType) !== undefined && (options === null || options === void 0 ? void 0 : options.valueType) !== 1) {
            throw new Error("Only valueType: DOUBLE is supported");
        }
        if (!METRICS_ENABLED)
            new Observable(new ObservableResult(null, false));
        const instrument = __classPrivateFieldGet(this, _Meter_meter, "f").createObservableGauge(name, 
        // deno-lint-ignore prefer-primordials
        options === null || options === void 0 ? void 0 : options.description, options === null || options === void 0 ? void 0 : options.unit);
        return new Observable(new ObservableResult(instrument, false));
    }
    addBatchObservableCallback(callback, observables) {
        if (!METRICS_ENABLED)
            return;
        const result = new BatchObservableResult(new SafeWeakSet(observables));
        startObserving();
        BATCH_CALLBACKS.set(callback, result);
    }
    removeBatchObservableCallback(callback, observables) {
        if (!METRICS_ENABLED)
            return;
        const result = BATCH_CALLBACKS.get(callback);
        if (result && batchResultHasObservables(result, observables)) {
            BATCH_CALLBACKS.delete(callback);
        }
    }
}
_Meter_meter = new WeakMap();
function record(instrument, value, attributes) {
    if (instrument === null)
        return;
    if (attributes === undefined) {
        op_otel_metric_record0(instrument, value);
    }
    else {
        const attrs = ObjectEntries(attributes);
        if (attrs.length === 0) {
            op_otel_metric_record0(instrument, value);
        }
        let i = 0;
        while (i < attrs.length) {
            const remaining = attrs.length - i;
            if (remaining > 3) {
                op_otel_metric_attribute3(attrs.length, attrs[i][0], attrs[i][1], attrs[i + 1][0], attrs[i + 1][1], attrs[i + 2][0], attrs[i + 2][1]);
                i += 3;
            }
            else if (remaining === 3) {
                op_otel_metric_record3(instrument, value, attrs[i][0], attrs[i][1], attrs[i + 1][0], attrs[i + 1][1], attrs[i + 2][0], attrs[i + 2][1]);
                i += 3;
            }
            else if (remaining === 2) {
                op_otel_metric_record2(instrument, value, attrs[i][0], attrs[i][1], attrs[i + 1][0], attrs[i + 1][1]);
                i += 2;
            }
            else if (remaining === 1) {
                op_otel_metric_record1(instrument, value, attrs[i][0], attrs[i][1]);
                i += 1;
            }
        }
    }
}
function recordObservable(instrument, value, attributes) {
    if (instrument === null)
        return;
    if (attributes === undefined) {
        op_otel_metric_observable_record0(instrument, value);
    }
    else {
        const attrs = ObjectEntries(attributes);
        if (attrs.length === 0) {
            op_otel_metric_observable_record0(instrument, value);
        }
        let i = 0;
        while (i < attrs.length) {
            const remaining = attrs.length - i;
            if (remaining > 3) {
                op_otel_metric_attribute3(attrs.length, attrs[i][0], attrs[i][1], attrs[i + 1][0], attrs[i + 1][1], attrs[i + 2][0], attrs[i + 2][1]);
                i += 3;
            }
            else if (remaining === 3) {
                op_otel_metric_observable_record3(instrument, value, attrs[i][0], attrs[i][1], attrs[i + 1][0], attrs[i + 1][1], attrs[i + 2][0], attrs[i + 2][1]);
                i += 3;
            }
            else if (remaining === 2) {
                op_otel_metric_observable_record2(instrument, value, attrs[i][0], attrs[i][1], attrs[i + 1][0], attrs[i + 1][1]);
                i += 2;
            }
            else if (remaining === 1) {
                op_otel_metric_observable_record1(instrument, value, attrs[i][0], attrs[i][1]);
                i += 1;
            }
        }
    }
}
class Counter {
    constructor(instrument, upDown) {
        _Counter_instrument.set(this, void 0);
        _Counter_upDown.set(this, void 0);
        __classPrivateFieldSet(this, _Counter_instrument, instrument, "f");
        __classPrivateFieldSet(this, _Counter_upDown, upDown, "f");
    }
    add(value, attributes, _context) {
        if (value < 0 && !__classPrivateFieldGet(this, _Counter_upDown, "f")) {
            throw new Error("Counter can only be incremented");
        }
        record(__classPrivateFieldGet(this, _Counter_instrument, "f"), value, attributes);
    }
}
_Counter_instrument = new WeakMap(), _Counter_upDown = new WeakMap();
class Gauge {
    constructor(instrument) {
        _Gauge_instrument.set(this, void 0);
        __classPrivateFieldSet(this, _Gauge_instrument, instrument, "f");
    }
    record(value, attributes, _context) {
        record(__classPrivateFieldGet(this, _Gauge_instrument, "f"), value, attributes);
    }
}
_Gauge_instrument = new WeakMap();
class Histogram {
    constructor(instrument) {
        _Histogram_instrument.set(this, void 0);
        __classPrivateFieldSet(this, _Histogram_instrument, instrument, "f");
    }
    record(value, attributes, _context) {
        record(__classPrivateFieldGet(this, _Histogram_instrument, "f"), value, attributes);
    }
}
_Histogram_instrument = new WeakMap();
let getObservableResult;
class Observable {
    constructor(result) {
        _Observable_result.set(this, void 0);
        __classPrivateFieldSet(this, _Observable_result, result, "f");
    }
    addCallback(callback) {
        const res = INDIVIDUAL_CALLBACKS.get(this);
        if (res)
            res.add(callback);
        else
            INDIVIDUAL_CALLBACKS.set(this, new SafeSet([callback]));
        startObserving();
    }
    removeCallback(callback) {
        const res = INDIVIDUAL_CALLBACKS.get(this);
        if (res)
            res.delete(callback);
        if ((res === null || res === void 0 ? void 0 : res.size) === 0)
            INDIVIDUAL_CALLBACKS.delete(this);
    }
}
_Observable_result = new WeakMap();
(() => {
    getObservableResult = (observable) => __classPrivateFieldGet(observable, _Observable_result, "f");
})();
class ObservableResult {
    constructor(instrument, isRegularCounter) {
        _ObservableResult_instrument.set(this, void 0);
        _ObservableResult_isRegularCounter.set(this, void 0);
        __classPrivateFieldSet(this, _ObservableResult_instrument, instrument, "f");
        __classPrivateFieldSet(this, _ObservableResult_isRegularCounter, isRegularCounter, "f");
    }
    observe(value, attributes) {
        if (__classPrivateFieldGet(this, _ObservableResult_isRegularCounter, "f")) {
            if (value < 0) {
                throw new Error("Observable counters can only be incremented");
            }
        }
        recordObservable(__classPrivateFieldGet(this, _ObservableResult_instrument, "f"), value, attributes);
    }
}
_ObservableResult_instrument = new WeakMap(), _ObservableResult_isRegularCounter = new WeakMap();
async function observe() {
    if (ISOLATE_METRICS) {
        op_otel_collect_isolate_metrics();
    }
    const promises = [];
    // Primordials are not needed, because this is a SafeMap.
    // deno-lint-ignore prefer-primordials
    for (const { 0: observable, 1: callbacks } of INDIVIDUAL_CALLBACKS) {
        const result = getObservableResult(observable);
        // Primordials are not needed, because this is a SafeSet.
        // deno-lint-ignore prefer-primordials
        for (const callback of callbacks) {
            // PromiseTry is not in primordials?
            // deno-lint-ignore prefer-primordials
            ArrayPrototypePush(promises, Promise.try(callback, result));
        }
    }
    // Primordials are not needed, because this is a SafeMap.
    // deno-lint-ignore prefer-primordials
    for (const { 0: callback, 1: result } of BATCH_CALLBACKS) {
        // PromiseTry is not in primordials?
        // deno-lint-ignore prefer-primordials
        ArrayPrototypePush(promises, Promise.try(callback, result));
    }
    await SafePromiseAll(promises);
}
let isObserving = false;
function startObserving() {
    if (!isObserving) {
        isObserving = true;
        (async () => {
            while (true) {
                const promise = op_otel_metric_wait_to_observe();
                core.unrefOpPromise(promise);
                const ok = await promise;
                if (!ok)
                    break;
                await observe();
                op_otel_metric_observation_done();
            }
        })();
    }
}
const otelConsoleConfig = {
    ignore: 0,
    capture: 1,
    replace: 2,
};
function otelLog(message, level) {
    var _a;
    const currentSpan = (_a = CURRENT.get()) === null || _a === void 0 ? void 0 : _a.getValue(SPAN_KEY);
    const otelSpan = currentSpan !== undefined
        ? getOtelSpan(currentSpan)
        : undefined;
    if (otelSpan || currentSpan === undefined) {
        op_otel_log(message, level, otelSpan);
    }
    else {
        const spanContext = currentSpan.spanContext();
        op_otel_log_foreign(message, level, spanContext.traceId, spanContext.spanId, spanContext.traceFlags);
    }
}
/*
 * Copyright The OpenTelemetry Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
const VERSION = "00";
const VERSION_PART = "(?!ff)[\\da-f]{2}";
const TRACE_ID_PART = "(?![0]{32})[\\da-f]{32}";
const PARENT_ID_PART = "(?![0]{16})[\\da-f]{16}";
const FLAGS_PART = "[\\da-f]{2}";
const TRACE_PARENT_REGEX = new SafeRegExp(`^\\s?(${VERSION_PART})-(${TRACE_ID_PART})-(${PARENT_ID_PART})-(${FLAGS_PART})(-.*)?\\s?$`);
const VALID_TRACEID_REGEX = new SafeRegExp("^([0-9a-f]{32})$", "i");
const VALID_SPANID_REGEX = new SafeRegExp("^[0-9a-f]{16}$", "i");
const MAX_TRACE_STATE_ITEMS = 32;
const MAX_TRACE_STATE_LEN = 512;
const LIST_MEMBERS_SEPARATOR = ",";
const LIST_MEMBER_KEY_VALUE_SPLITTER = "=";
const VALID_KEY_CHAR_RANGE = "[_0-9a-z-*/]";
const VALID_KEY = `[a-z]${VALID_KEY_CHAR_RANGE}{0,255}`;
const VALID_VENDOR_KEY = `[a-z0-9]${VALID_KEY_CHAR_RANGE}{0,240}@[a-z]${VALID_KEY_CHAR_RANGE}{0,13}`;
const VALID_KEY_REGEX = new SafeRegExp(`^(?:${VALID_KEY}|${VALID_VENDOR_KEY})$`);
const VALID_VALUE_BASE_REGEX = new SafeRegExp("^[ -~]{0,255}[!-~]$");
const INVALID_VALUE_COMMA_EQUAL_REGEX = new SafeRegExp(",|=");
const TRACE_PARENT_HEADER = "traceparent";
const TRACE_STATE_HEADER = "tracestate";
const INVALID_TRACEID = "00000000000000000000000000000000";
const INVALID_SPANID = "0000000000000000";
const INVALID_SPAN_CONTEXT = {
    traceId: INVALID_TRACEID,
    spanId: INVALID_SPANID,
    traceFlags: 0,
};
const BAGGAGE_KEY_PAIR_SEPARATOR = "=";
const BAGGAGE_PROPERTIES_SEPARATOR = ";";
const BAGGAGE_ITEMS_SEPARATOR = ",";
const BAGGAGE_HEADER = "baggage";
const BAGGAGE_MAX_NAME_VALUE_PAIRS = 180;
const BAGGAGE_MAX_PER_NAME_VALUE_PAIRS = 4096;
const BAGGAGE_MAX_TOTAL_LENGTH = 8192;
class NonRecordingSpan {
    constructor(_spanContext = INVALID_SPAN_CONTEXT) {
        this._spanContext = _spanContext;
    }
    spanContext() {
        return this._spanContext;
    }
    setAttribute(_key, _value) {
        return this;
    }
    setAttributes(_attributes) {
        return this;
    }
    addEvent(_name, _attributes) {
        return this;
    }
    addLink(_link) {
        return this;
    }
    addLinks(_links) {
        return this;
    }
    setStatus(_status) {
        return this;
    }
    updateName(_name) {
        return this;
    }
    end(_endTime) { }
    isRecording() {
        return false;
    }
    // deno-lint-ignore no-explicit-any
    recordException(_exception, _time) { }
}
const otelPropagators = {
    traceContext: 0,
    baggage: 1,
    none: 2,
};
function parseTraceParent(traceParent) {
    const match = TRACE_PARENT_REGEX.exec(traceParent);
    if (!match)
        return null;
    // According to the specification the implementation should be compatible
    // with future versions. If there are more parts, we only reject it if it's using version 00
    // See https://www.w3.org/TR/trace-context/#versioning-of-traceparent
    if (match[1] === "00" && match[5])
        return null;
    return {
        traceId: match[2],
        spanId: match[3],
        traceFlags: NumberParseInt(match[4], 16),
    };
}
function isTracingSuppressed(context) {
    return context.getValue(SymbolFor("OpenTelemetry SDK Context Key SUPPRESS_TRACING")) === true;
}
function isValidTraceId(traceId) {
    return VALID_TRACEID_REGEX.test(traceId) && traceId !== INVALID_TRACEID;
}
function isValidSpanId(spanId) {
    return VALID_SPANID_REGEX.test(spanId) && spanId !== INVALID_SPANID;
}
function isSpanContextValid(spanContext) {
    return (isValidTraceId(spanContext.traceId) && isValidSpanId(spanContext.spanId));
}
function validateKey(key) {
    return VALID_KEY_REGEX.test(key);
}
function validateValue(value) {
    return (VALID_VALUE_BASE_REGEX.test(value) &&
        !INVALID_VALUE_COMMA_EQUAL_REGEX.test(value));
}
class TraceStateClass {
    constructor(rawTraceState) {
        this._internalState = new SafeMap();
        if (rawTraceState)
            this._parse(rawTraceState);
    }
    set(key, value) {
        const traceState = this._clone();
        if (traceState._internalState.has(key)) {
            traceState._internalState.delete(key);
        }
        traceState._internalState.set(key, value);
        return traceState;
    }
    unset(key) {
        const traceState = this._clone();
        traceState._internalState.delete(key);
        return traceState;
    }
    get(key) {
        return this._internalState.get(key);
    }
    serialize() {
        return ArrayPrototypeJoin(ArrayPrototypeReduce(this._keys(), (agg, key) => {
            ArrayPrototypePush(agg, key + LIST_MEMBER_KEY_VALUE_SPLITTER + this.get(key));
            return agg;
        }, []), LIST_MEMBERS_SEPARATOR);
    }
    _parse(rawTraceState) {
        if (rawTraceState.length > MAX_TRACE_STATE_LEN)
            return;
        this._internalState = ArrayPrototypeReduce(ArrayPrototypeReverse(StringPrototypeSplit(rawTraceState, LIST_MEMBERS_SEPARATOR)), (agg, part) => {
            const listMember = StringPrototypeTrim(part); // Optional Whitespace (OWS) handling
            const i = StringPrototypeIndexOf(listMember, LIST_MEMBER_KEY_VALUE_SPLITTER);
            if (i !== -1) {
                const key = StringPrototypeSlice(listMember, 0, i);
                const value = StringPrototypeSlice(listMember, i + 1, part.length);
                if (validateKey(key) && validateValue(value)) {
                    agg.set(key, value);
                }
            }
            return agg;
        }, new SafeMap());
        // Because of the reverse() requirement, trunc must be done after map is created
        if (this._internalState.size > MAX_TRACE_STATE_ITEMS) {
            this._internalState = new SafeMap(ArrayPrototypeSlice(ArrayPrototypeReverse(ArrayFrom(MapPrototypeEntries(this._internalState))), 0, MAX_TRACE_STATE_ITEMS));
        }
    }
    _keys() {
        return ArrayPrototypeReverse(ArrayFrom(MapPrototypeKeys(this._internalState)));
    }
    _clone() {
        const traceState = new TraceStateClass();
        traceState._internalState = new SafeMap(this._internalState);
        return traceState;
    }
}
class W3CTraceContextPropagator {
    inject(context, carrier, setter) {
        var _a;
        const spanContext = (_a = context.getValue(SPAN_KEY)) === null || _a === void 0 ? void 0 : _a.spanContext();
        if (!spanContext ||
            isTracingSuppressed(context) ||
            !isSpanContextValid(spanContext)) {
            return;
        }
        const traceParent = `${VERSION}-${spanContext.traceId}-${spanContext.spanId}-0${NumberPrototypeToString(Number(spanContext.traceFlags || 0), 16)}`;
        setter.set(carrier, TRACE_PARENT_HEADER, traceParent);
        if (spanContext.traceState) {
            setter.set(carrier, TRACE_STATE_HEADER, spanContext.traceState.serialize());
        }
    }
    extract(context, carrier, getter) {
        const traceParentHeader = getter.get(carrier, TRACE_PARENT_HEADER);
        if (!traceParentHeader)
            return context;
        const traceParent = ArrayIsArray(traceParentHeader)
            ? traceParentHeader[0]
            : traceParentHeader;
        if (typeof traceParent !== "string")
            return context;
        const spanContext = parseTraceParent(traceParent);
        if (!spanContext)
            return context;
        spanContext.isRemote = true;
        const traceStateHeader = getter.get(carrier, TRACE_STATE_HEADER);
        if (traceStateHeader) {
            // If more than one `tracestate` header is found, we merge them into a
            // single header.
            const state = ArrayIsArray(traceStateHeader)
                ? ArrayPrototypeJoin(traceStateHeader, ",")
                : traceStateHeader;
            spanContext.traceState = new TraceStateClass(typeof state === "string" ? state : undefined);
        }
        return context.setValue(SPAN_KEY, new NonRecordingSpan(spanContext));
    }
    fields() {
        return [TRACE_PARENT_HEADER, TRACE_STATE_HEADER];
    }
}
const baggageEntryMetadataSymbol = SymbolFor("BaggageEntryMetadata");
export function baggageEntryMetadataFromString(str) {
    if (typeof str !== "string") {
        str = "";
    }
    return {
        __TYPE__: baggageEntryMetadataSymbol,
        toString() {
            return str;
        },
    };
}
function serializeKeyPairs(keyPairs) {
    return ArrayPrototypeReduce(keyPairs, (hValue, current) => {
        const value = `${hValue}${hValue !== "" ? BAGGAGE_ITEMS_SEPARATOR : ""}${current}`;
        return value.length > BAGGAGE_MAX_TOTAL_LENGTH ? hValue : value;
    }, "");
}
function getKeyPairs(baggage) {
    return ArrayPrototypeMap(baggage.getAllEntries(), (baggageEntry) => {
        let entry = `${encodeURIComponent(baggageEntry[0])}=${encodeURIComponent(baggageEntry[1].value)}`;
        // include opaque metadata if provided
        // NOTE: we intentionally don't URI-encode the metadata - that responsibility falls on the metadata implementation
        if (baggageEntry[1].metadata !== undefined) {
            entry += BAGGAGE_PROPERTIES_SEPARATOR +
                // deno-lint-ignore prefer-primordials
                baggageEntry[1].metadata.toString();
        }
        return entry;
    });
}
function parsePairKeyValue(entry) {
    const valueProps = StringPrototypeSplit(entry, BAGGAGE_PROPERTIES_SEPARATOR);
    if (valueProps.length <= 0)
        return;
    const keyPairPart = ArrayPrototypeShift(valueProps);
    if (!keyPairPart)
        return;
    const separatorIndex = StringPrototypeIndexOf(keyPairPart, BAGGAGE_KEY_PAIR_SEPARATOR);
    if (separatorIndex <= 0)
        return;
    const key = decodeURIComponent(StringPrototypeTrim(StringPrototypeSubstring(keyPairPart, 0, separatorIndex)));
    const value = decodeURIComponent(StringPrototypeTrim(StringPrototypeSubstring(keyPairPart, separatorIndex + 1)));
    let metadata;
    if (valueProps.length > 0) {
        metadata = baggageEntryMetadataFromString(ArrayPrototypeJoin(valueProps, BAGGAGE_PROPERTIES_SEPARATOR));
    }
    return { key, value, metadata };
}
class BaggageImpl {
    constructor(entries) {
        _BaggageImpl_entries.set(this, void 0);
        __classPrivateFieldSet(this, _BaggageImpl_entries, entries ? new SafeMap(entries) : new SafeMap(), "f");
    }
    getEntry(key) {
        const entry = __classPrivateFieldGet(this, _BaggageImpl_entries, "f").get(key);
        if (!entry) {
            return undefined;
        }
        return ObjectAssign({}, entry);
    }
    getAllEntries() {
        return ArrayPrototypeMap(ArrayFrom(MapPrototypeEntries(__classPrivateFieldGet(this, _BaggageImpl_entries, "f"))), (entry) => [entry[0], entry[1]]);
    }
    setEntry(key, entry) {
        const newBaggage = new BaggageImpl(__classPrivateFieldGet(this, _BaggageImpl_entries, "f"));
        __classPrivateFieldGet(newBaggage, _BaggageImpl_entries, "f").set(key, entry);
        return newBaggage;
    }
    removeEntry(key) {
        const newBaggage = new BaggageImpl(__classPrivateFieldGet(this, _BaggageImpl_entries, "f"));
        __classPrivateFieldGet(newBaggage, _BaggageImpl_entries, "f").delete(key);
        return newBaggage;
    }
    removeEntries(...keys) {
        const newBaggage = new BaggageImpl(__classPrivateFieldGet(this, _BaggageImpl_entries, "f"));
        for (const key of new SafeArrayIterator(keys)) {
            __classPrivateFieldGet(newBaggage, _BaggageImpl_entries, "f").delete(key);
        }
        return newBaggage;
    }
    clear() {
        return new BaggageImpl();
    }
}
_BaggageImpl_entries = new WeakMap();
export class W3CBaggagePropagator {
    inject(context, carrier, setter) {
        const baggage = context.getValue(baggageEntryMetadataSymbol);
        if (!baggage || isTracingSuppressed(context))
            return;
        const keyPairs = ArrayPrototypeSlice(ArrayPrototypeFilter(getKeyPairs(baggage), (pair) => {
            return pair.length <= BAGGAGE_MAX_PER_NAME_VALUE_PAIRS;
        }), 0, BAGGAGE_MAX_NAME_VALUE_PAIRS);
        const headerValue = serializeKeyPairs(keyPairs);
        if (headerValue.length > 0) {
            setter.set(carrier, BAGGAGE_HEADER, headerValue);
        }
    }
    extract(context, carrier, getter) {
        const headerValue = getter.get(carrier, BAGGAGE_HEADER);
        const baggageString = ArrayIsArray(headerValue)
            ? ArrayPrototypeJoin(headerValue, BAGGAGE_ITEMS_SEPARATOR)
            : headerValue;
        if (!baggageString)
            return context;
        const baggage = {};
        if (baggageString.length === 0) {
            return context;
        }
        const pairs = StringPrototypeSplit(baggageString, BAGGAGE_ITEMS_SEPARATOR);
        ArrayPrototypeForEach(pairs, (entry) => {
            const keyPair = parsePairKeyValue(entry);
            if (keyPair) {
                const baggageEntry = { value: keyPair.value };
                if (keyPair.metadata) {
                    baggageEntry.metadata = keyPair.metadata;
                }
                baggage[keyPair.key] = baggageEntry;
            }
        });
        if (ObjectEntries(baggage).length === 0) {
            return context;
        }
        return context.setValue(baggageEntryMetadataSymbol, new BaggageImpl(new SafeMap(ObjectEntries(baggage))));
    }
    fields() {
        return [BAGGAGE_HEADER];
    }
}
let builtinTracerCache;
export function builtinTracer() {
    if (!builtinTracerCache) {
        builtinTracerCache = new Tracer(OtelTracer.builtin());
    }
    return builtinTracerCache;
}
function enableIsolateMetrics() {
    op_otel_enable_isolate_metrics();
    ISOLATE_METRICS = true;
    startObserving();
}
// We specify a very high version number, to allow any `@opentelemetry/api`
// version to load this module. This does cause @opentelemetry/api to not be
// able to register anything itself with the global registration methods.
const OTEL_API_COMPAT_VERSION = "1.999.999";
export function bootstrap(config) {
    var _a;
    var _b;
    const { 0: tracingEnabled, 1: metricsEnabled, 2: consoleConfig } = config, propagators = __rest(config, ["0", "1", "2"]);
    TRACING_ENABLED = tracingEnabled === 1;
    METRICS_ENABLED = metricsEnabled === 1;
    PROPAGATORS = ArrayPrototypeMap(ArrayPrototypeFilter(ObjectValues(propagators), (propagator) => propagator !== otelPropagators.none), (propagator) => {
        switch (propagator) {
            case otelPropagators.traceContext:
                return new W3CTraceContextPropagator();
            case otelPropagators.baggage:
                return new W3CBaggagePropagator();
        }
    });
    switch (consoleConfig) {
        case otelConsoleConfig.capture:
            core.wrapConsole(globalThis.console, new Console(otelLog));
            break;
        case otelConsoleConfig.replace:
            ObjectDefineProperty(globalThis, "console", core.propNonEnumerable(new Console(otelLog)));
            break;
        default:
            break;
    }
    if (TRACING_ENABLED || METRICS_ENABLED) {
        const otel = (_a = globalThis[_b = SymbolFor("opentelemetry.js.api.1")]) !== null && _a !== void 0 ? _a : (globalThis[_b] = {
            version: OTEL_API_COMPAT_VERSION,
        });
        if (TRACING_ENABLED) {
            otel.trace = TracerProvider;
            otel.context = ContextManager;
        }
        if (METRICS_ENABLED) {
            otel.metrics = MeterProvider;
            enableIsolateMetrics();
        }
    }
}
export const telemetry = {
    tracerProvider: TracerProvider,
    contextManager: ContextManager,
    meterProvider: MeterProvider,
};

