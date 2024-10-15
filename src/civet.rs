
pub fn get_civet_script() -> String {
  return String::from(r#"
    "use strict";
var module = {};
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __commonJS = (cb, mod) => function __require() {
  return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// node_modules/@danielx/hera/dist/machine.js
var require_machine = __commonJS({
  "node_modules/@danielx/hera/dist/machine.js"(exports2, module2) {
    "use strict";
    var __defProp2 = Object.defineProperty;
    var __getOwnPropDesc2 = Object.getOwnPropertyDescriptor;
    var __getOwnPropNames2 = Object.getOwnPropertyNames;
    var __hasOwnProp2 = Object.prototype.hasOwnProperty;
    var __export2 = (target, all) => {
      for (var name in all)
        __defProp2(target, name, { get: all[name], enumerable: true });
    };
    var __copyProps2 = (to, from, except, desc) => {
      if (from && typeof from === "object" || typeof from === "function") {
        for (let key of __getOwnPropNames2(from))
          if (!__hasOwnProp2.call(to, key) && key !== except)
            __defProp2(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc2(from, key)) || desc.enumerable });
      }
      return to;
    };
    var __toCommonJS2 = (mod) => __copyProps2(__defProp2({}, "__esModule", { value: true }), mod);
    var machine_exports = {};
    __export2(machine_exports, {
      $C: () => $C2,
      $E: () => $E2,
      $EVENT: () => $EVENT2,
      $EVENT_C: () => $EVENT_C2,
      $EXPECT: () => $EXPECT2,
      $L: () => $L231,
      $N: () => $N2,
      $P: () => $P2,
      $Q: () => $Q2,
      $R: () => $R95,
      $R$0: () => $R$02,
      $S: () => $S2,
      $T: () => $T2,
      $TEXT: () => $TEXT2,
      $TR: () => $TR2,
      $TS: () => $TS2,
      $TV: () => $TV2,
      $Y: () => $Y2,
      ParseError: () => ParseError2,
      Validator: () => Validator2
    });
    module2.exports = __toCommonJS2(machine_exports);
    function $EXPECT2(parser2, expectation) {
      return function(ctx, state2) {
        const result = parser2(ctx, state2);
        if (!result)
          ctx.fail(state2.pos, expectation);
        return result;
      };
    }
    function $L231(str) {
      return function(_ctx, state2) {
        const { input, pos } = state2, { length } = str, end = pos + length;
        if (input.substring(pos, end) === str) {
          return {
            loc: {
              pos,
              length
            },
            pos: end,
            value: str
          };
        }
        return;
      };
    }
    function $R95(regExp) {
      return function(_ctx, state2) {
        const { input, pos } = state2;
        regExp.lastIndex = state2.pos;
        let l, m, v;
        if (m = input.match(regExp)) {
          v = m[0];
          l = v.length;
          return {
            loc: {
              pos,
              length: l
            },
            pos: pos + l,
            value: m
          };
        }
        return;
      };
    }
    function $C2(...terms) {
      return (ctx, state2) => {
        let i = 0;
        const l = terms.length;
        while (i < l) {
          const r = terms[i++](ctx, state2);
          if (r)
            return r;
        }
        return;
      };
    }
    function $S2(...terms) {
      return (ctx, state2) => {
        let { input, pos } = state2, i = 0, value;
        const results = [], s = pos, l = terms.length;
        while (i < l) {
          const r = terms[i++](ctx, { input, pos });
          if (r) {
            ({ pos, value } = r);
            results.push(value);
          } else
            return;
        }
        return {
          loc: {
            pos: s,
            length: pos - s
          },
          pos,
          value: results
        };
      };
    }
    function $E2(fn) {
      return (ctx, state2) => {
        const r = fn(ctx, state2);
        if (r)
          return r;
        const { pos } = state2;
        return {
          loc: {
            pos,
            length: 0
          },
          pos,
          value: void 0
        };
      };
    }
    function $Q2(fn) {
      return (ctx, state2) => {
        let { input, pos } = state2;
        let value;
        const s = pos;
        const results = [];
        while (true) {
          const prevPos = pos;
          const r = fn(ctx, { input, pos });
          if (!r)
            break;
          ({ pos, value } = r);
          if (pos === prevPos)
            break;
          else
            results.push(value);
        }
        return {
          loc: {
            pos: s,
            length: pos - s
          },
          pos,
          value: results
        };
      };
    }
    function $P2(fn) {
      return (ctx, state2) => {
        const { input, pos: s } = state2;
        let value;
        const first = fn(ctx, state2);
        if (!first)
          return;
        let { pos } = first;
        const results = [first.value];
        while (true) {
          const prevPos = pos;
          const r = fn(ctx, { input, pos });
          if (!r)
            break;
          ({ pos, value } = r);
          if (pos === prevPos)
            break;
          results.push(value);
        }
        return {
          loc: {
            pos: s,
            length: pos - s
          },
          value: results,
          pos
        };
      };
    }
    function $TEXT2(fn) {
      return (ctx, state2) => {
        const newState = fn(ctx, state2);
        if (!newState)
          return;
        newState.value = state2.input.substring(state2.pos, newState.pos);
        return newState;
      };
    }
    function $N2(fn) {
      return (ctx, state2) => {
        const newState = fn(ctx, state2);
        if (newState)
          return;
        return {
          loc: {
            pos: state2.pos,
            length: 0
          },
          value: void 0,
          pos: state2.pos
        };
      };
    }
    function $Y2(fn) {
      return (ctx, state2) => {
        const newState = fn(ctx, state2);
        if (!newState)
          return;
        return {
          loc: {
            pos: state2.pos,
            length: 0
          },
          value: void 0,
          pos: state2.pos
        };
      };
    }
    function $T2(parser2, fn) {
      return function(ctx, state2) {
        const result = parser2(ctx, state2);
        if (!result)
          return;
        if (ctx.tokenize)
          return result;
        const { value } = result;
        const mappedValue = fn(value);
        result.value = mappedValue;
        return result;
      };
    }
    function $TR2(parser2, fn) {
      return function(ctx, state2) {
        const result = parser2(ctx, state2);
        if (!result)
          return;
        if (ctx.tokenize)
          return result;
        const { loc, value } = result;
        const mappedValue = fn(SKIP, loc, ...value);
        if (mappedValue === SKIP) {
          return;
        }
        result.value = mappedValue;
        return result;
      };
    }
    function $TS2(parser2, fn) {
      return function(ctx, state2) {
        const result = parser2(ctx, state2);
        if (!result)
          return;
        if (ctx.tokenize)
          return result;
        const { loc, value } = result;
        const mappedValue = fn(SKIP, loc, value, ...value);
        if (mappedValue === SKIP) {
          return;
        }
        result.value = mappedValue;
        return result;
      };
    }
    function $TV2(parser2, fn) {
      return function(ctx, state2) {
        const result = parser2(ctx, state2);
        if (!result)
          return;
        if (ctx.tokenize)
          return result;
        const { loc, value } = result;
        const mappedValue = fn(SKIP, loc, value, value);
        if (mappedValue === SKIP) {
          return;
        }
        result.value = mappedValue;
        return result;
      };
    }
    function $R$02(parser2) {
      return function(ctx, state2) {
        const result = parser2(ctx, state2);
        if (!result)
          return;
        const value = result.value[0];
        result.value = value;
        return result;
      };
    }
    function $EVENT2(ctx, state2, name, fn) {
      let eventData, enter, exit;
      if (enter = ctx.enter) {
        const result2 = enter(name, state2);
        if (result2) {
          if ("cache" in result2)
            return result2.cache;
          eventData = result2.data;
        }
      }
      let result = fn(ctx, state2);
      if (result && ctx.tokenize) {
        result = $TOKEN(name, state2, result);
      }
      if (exit = ctx.exit)
        exit(name, state2, result, eventData);
      return result;
    }
    function $EVENT_C2(ctx, state2, name, fns) {
      let eventData, enter, exit;
      if (enter = ctx.enter) {
        const result2 = enter(name, state2);
        if (result2) {
          if ("cache" in result2)
            return result2.cache;
          eventData = result2.data;
        }
      }
      let result, i = 0, l = fns.length;
      while (!result && i < l) {
        if (result = fns[i](ctx, state2))
          break;
        i++;
      }
      if (result && ctx.tokenize) {
        result = $TOKEN(name, state2, result);
      }
      if (exit = ctx.exit)
        exit(name, state2, result, eventData);
      return result;
    }
    function $TOKEN(name, state2, newState) {
      if (!newState)
        return;
      newState.value = {
        type: name,
        children: [newState.value].flat(),
        token: state2.input.substring(state2.pos, newState.pos),
        loc: newState.loc
      };
      return newState;
    }
    var SKIP = {};
    function Validator2() {
      const failHintRegex = /\S+|\s+|$/y;
      const failExpected = Array(16);
      let failIndex = 0;
      let maxFailPos = 0;
      function fail(pos, expected) {
        if (pos < maxFailPos)
          return;
        if (pos > maxFailPos) {
          maxFailPos = pos;
          failExpected.length = failIndex = 0;
        }
        failExpected[failIndex++] = expected;
        return;
      }
      function location(input, pos) {
        const [line, column] = input.split(/\n|\r\n|\r/).reduce(([row, col], line2) => {
          const l = line2.length + 1;
          if (pos >= l) {
            pos -= l;
            return [row + 1, 1];
          } else if (pos >= 0) {
            col += pos;
            pos = -1;
            return [row, col];
          } else {
            return [row, col];
          }
        }, [1, 1]);
        return [line, column];
      }
      function validate(input, result, { filename: filename2 }) {
        if (result && result.pos === input.length)
          return result.value;
        const expectations = Array.from(new Set(failExpected.slice(0, failIndex)));
        let l = location(input, maxFailPos), [line, column] = l;
        if (result && result.pos > maxFailPos) {
          l = location(input, result.pos);
          throw new Error(`${filename2}:${line}:${column} Unconsumed input at #{l}

${input.slice(result.pos)}
`);
        }
        if (expectations.length) {
          failHintRegex.lastIndex = maxFailPos;
          let [hint] = input.match(failHintRegex);
          if (hint.length)
            hint = JSON.stringify(hint);
          else
            hint = "EOF";
          const error = new ParseError2("Failed to parse", `Expected:
	${expectations.join("\n	")}
Found: ${hint}
`, filename2, line, column, maxFailPos);
          throw error;
        }
        if (result) {
          throw new Error(`
Unconsumed input at ${l}

${input.slice(result.pos)}
`);
        }
        throw new Error("No result");
      }
      function reset() {
        failIndex = 0;
        maxFailPos = 0;
        failExpected.length = 0;
      }
      return {
        fail,
        validate,
        reset
      };
    }
    var ParseError2 = class extends Error {
      constructor(header, body, filename2, line, column, offset) {
        let message = `${filename2}:${line}:${column} ${header}`;
        if (body)
          message += `
${body}`;
        super(message);
        this.header = header;
        this.body = body;
        this.filename = filename2;
        this.line = line;
        this.column = column;
        this.offset = offset;
        this.name = "ParseError";
        this.message = message;
      }
    };
  }
});

// source/main.civet
var main_exports = {};
__export(main_exports, {
  ParseError: () => import_lib3.ParseError,
  ParseErrors: () => ParseErrors,
  SourceMap: () => SourceMap2,
  compile: () => compile,
  default: () => main_default,
  generate: () => generate_default,
  isCompileError: () => isCompileError,
  lib: () => lib_exports,
  parse: () => parse,
  parseProgram: () => parseProgram,
  prune: () => prune,
  sourcemap: () => sourcemap_exports
});
module.exports = __toCommonJS(main_exports);

// source/parser.hera
var import_lib3 = __toESM(require_machine());

// source/parser/lib.civet
var lib_exports = {};
__export(lib_exports, {
  addPostfixStatement: () => addPostfixStatement,
  adjustBindingElements: () => adjustBindingElements,
  adjustIndexAccess: () => adjustIndexAccess,
  attachPostfixStatementAsExpression: () => attachPostfixStatementAsExpression,
  blockWithPrefix: () => blockWithPrefix,
  convertNamedImportsToObject: () => convertNamedImportsToObject,
  convertObjectToJSXAttributes: () => convertObjectToJSXAttributes,
  convertWithClause: () => convertWithClause,
  dedentBlockString: () => dedentBlockString,
  dedentBlockSubstitutions: () => dedentBlockSubstitutions,
  deepCopy: () => deepCopy,
  dynamizeImportDeclaration: () => dynamizeImportDeclaration,
  dynamizeImportDeclarationExpression: () => dynamizeImportDeclarationExpression,
  expressionizeTypeIf: () => expressionizeTypeIf,
  forRange: () => forRange,
  gatherBindingCode: () => gatherBindingCode,
  gatherRecursive: () => gatherRecursive,
  gatherRecursiveAll: () => gatherRecursiveAll,
  gatherRecursiveWithinFunction: () => gatherRecursiveWithinFunction,
  getHelperRef: () => getHelperRef,
  getIndentLevel: () => getIndentLevel,
  getPrecedence: () => getPrecedence,
  getTrimmingSpace: () => getTrimmingSpace,
  hasAwait: () => hasAwait,
  hasImportDeclaration: () => hasImportDeclaration,
  hasYield: () => hasYield,
  insertTrimmingSpace: () => insertTrimmingSpace,
  isEmptyBareBlock: () => isEmptyBareBlock,
  isFunction: () => isFunction,
  isWhitespaceOrEmpty: () => isWhitespaceOrEmpty,
  lastAccessInCallExpression: () => lastAccessInCallExpression,
  literalValue: () => literalValue,
  makeAmpersandFunction: () => makeAmpersandFunction,
  makeEmptyBlock: () => makeEmptyBlock,
  makeExpressionStatement: () => makeExpressionStatement,
  makeGetterMethod: () => makeGetterMethod,
  makeLeftHandSideExpression: () => makeLeftHandSideExpression,
  makeRef: () => makeRef,
  maybeRef: () => maybeRef,
  maybeRefAssignment: () => maybeRefAssignment,
  modifyString: () => modifyString,
  negateCondition: () => negateCondition,
  precedenceStep: () => precedenceStep,
  prepend: () => prepend,
  processAssignmentDeclaration: () => processAssignmentDeclaration,
  processBinaryOpExpression: () => processBinaryOpExpression,
  processCallMemberExpression: () => processCallMemberExpression,
  processCoffeeDo: () => processCoffeeDo,
  processCoffeeInterpolation: () => processCoffeeInterpolation,
  processForInOf: () => processForInOf,
  processProgram: () => processProgram,
  processProgramAsync: () => processProgramAsync,
  processTryBlock: () => processTryBlock,
  processUnaryExpression: () => processUnaryExpression,
  quoteString: () => quoteString,
  reorderBindingRestProperty: () => reorderBindingRestProperty,
  replaceNode: () => replaceNode,
  replaceNodes: () => replaceNodes,
  skipImplicitArguments: () => skipImplicitArguments,
  trimFirstSpace: () => trimFirstSpace,
  typeOfJSX: () => typeOfJSX,
  wrapIIFE: () => wrapIIFE
});

// source/parser/binding.civet
function adjustAtBindings(statements, asThis = false) {
  gatherRecursiveAll(statements, (n) => n.type === "AtBindingProperty").forEach((binding) => {
    const { ref } = binding;
    if (asThis) {
      const atBinding = binding.binding;
      atBinding.children.pop();
      atBinding.type = void 0;
      binding.children.unshift(ref.id, ": this.", ref.base);
      binding.type = "Property";
      binding.ref = void 0;
      return;
    }
    if (ref.names[0] !== ref.base) {
      return binding.children.unshift(ref.base, ": ");
    }
    ;
    return;
  });
}
function adjustBindingElements(elements) {
  const names = elements.flatMap((p) => p.names || []), { length } = elements;
  let blockPrefix, restIndex = -1, restCount = 0;
  elements.forEach(({ type }, i) => {
    if (type === "BindingRestElement") {
      if (restIndex < 0)
        restIndex = i;
      return restCount++;
    }
    ;
    return;
  });
  if (restCount === 0) {
    return {
      children: elements,
      names,
      blockPrefix,
      length
    };
  } else if (restCount === 1) {
    const rest = elements[restIndex];
    const after = elements.slice(restIndex + 1);
    const restIdentifier = rest.binding.ref || rest.binding;
    names.push(...rest.names || []);
    let l = after.length;
    if (l) {
      if (arrayElementHasTrailingComma(after[l - 1]))
        l++;
      blockPrefix = {
        type: "PostRestBindingElements",
        children: ["[", insertTrimmingSpace(after, ""), "] = ", restIdentifier, ".splice(-", l.toString(), ")"],
        names: after.flatMap((p) => p.names)
      };
    }
    return {
      names,
      children: [...elements.slice(0, restIndex), {
        ...rest,
        children: rest.children.slice(0, -1)
        // remove trailing comma
      }],
      blockPrefix,
      length
    };
  }
  const err = {
    type: "Error",
    children: ["Multiple rest elements in array pattern"]
  };
  return {
    names,
    children: [...elements, err],
    blockPrefix,
    length
  };
}
function gatherBindingCode(statements, opts) {
  const thisAssignments = [];
  const splices = [];
  function insertRestSplices(s, p, thisAssignments2) {
    gatherRecursiveAll(s, (n) => n.blockPrefix || opts?.injectParamProps && n.accessModifier || n.type === "AtBinding").forEach((n) => {
      if (n.type === "AtBinding") {
        const { ref } = n;
        const { id } = ref;
        thisAssignments2.push([`this.${id} = `, ref]);
        return;
      }
      if (opts?.injectParamProps && n.type === "Parameter" && n.accessModifier) {
        n.names.forEach((id) => ({
          push: thisAssignments2.push({
            type: "AssignmentExpression",
            children: [`this.${id} = `, id],
            js: true
          })
        }));
        return;
      }
      const { blockPrefix } = n;
      p.push(blockPrefix);
      return insertRestSplices(blockPrefix, p, thisAssignments2);
    });
  }
  insertRestSplices(statements, splices, thisAssignments);
  return [splices, thisAssignments];
}
function arrayElementHasTrailingComma(elementNode) {
  let ref1;
  const lastChild = (ref1 = elementNode.children)[ref1.length - 1];
  return lastChild && lastChild[lastChild.length - 1]?.token === ",";
}

// source/parser/ref.civet
function makeRef(base = "ref", id = base) {
  return {
    type: "Ref",
    base,
    id
  };
}
function needsRef(expression, base = "ref") {
  if (!(expression != null && typeof expression === "object")) {
    return;
  }
  if (Array.isArray(expression)) {
    const nonempty = ((s, e) => {
      let step = e > s ? 1 : -1;
      return Array.from({ length: Math.abs(e - s) }, (_2, i) => s + i * step);
    })(0, expression.length).filter((i) => !isWhitespaceOrEmpty(expression[i]));
    if (nonempty.length === 1) {
      let ref1;
      if (ref1 = needsRef(expression[nonempty[0]], base)) {
        const ref = ref1;
        return ref;
      }
      return;
    } else {
      return makeRef(base);
    }
  }
  switch (expression.type) {
    case "Ref":
    case "Identifier":
    case "Literal":
      return;
  }
  return makeRef(base);
}
function maybeRef(exp, base = "ref") {
  return needsRef(exp, base) || exp;
}
function makeRefAssignment(ref, exp) {
  const refAssignment = {
    type: "AssignmentExpression",
    children: [ref, " = ", exp]
  };
  return {
    hoistDec: {
      type: "Declaration",
      children: ["let ", ref],
      names: []
    },
    refAssignment,
    refAssignmentComma: refAssignment ? [refAssignment, ","] : []
  };
}
function maybeRefAssignment(exp, base = "ref") {
  let hoistDec, refAssignment;
  const ref = maybeRef(exp, base);
  if (ref === exp) {
    return { ref, refAssignmentComma: [] };
  } else {
    return { ref, ...makeRefAssignment(ref, exp) };
  }
}

// source/parser/function.civet
function isVoidType(t) {
  return t?.type === "LiteralType" && t.t.type === "VoidType";
}
function isPromiseVoidType(t) {
  return t?.type === "IdentifierType" && t.raw === "Promise" && t.args?.types?.length === 1 && isVoidType(t.args.types[0]);
}
function isGeneratorVoidType(t) {
  return t?.type === "IdentifierType" && (t.raw === "Iterator" || t.raw === "Generator") && t.args?.types?.length >= 2 && isVoidType(t.args.types[1]);
}
function isAsyncGeneratorVoidType(t) {
  return t?.type === "IdentifierType" && (t.raw === "AsyncIterator" || t.raw === "AsyncGenerator") && t.args?.types?.length >= 2 && isVoidType(t.args.types[1]);
}
function implicitFunctionBlock(f) {
  if (f.abstract || f.block || f.signature?.optional)
    return;
  const { name, parent } = f;
  if (parent?.type === "ExportDeclaration")
    return;
  const expressions = parent?.expressions ?? parent?.elements;
  const currentIndex = expressions?.findIndex(([, def]) => def === f);
  const following = currentIndex >= 0 && expressions[currentIndex + 1]?.[1];
  if (f.type === following?.type && name && name === following.name) {
    f.ts = true;
  } else {
    const block = makeEmptyBlock();
    block.parent = f;
    f.block = block;
    f.children.push(block);
    f.ts = false;
  }
}
function processReturn(f, implicitReturns) {
  let { returnType } = f.signature;
  if (returnType && returnType.optional) {
    convertOptionalType(returnType);
  }
  if (!processReturnValue(f) && implicitReturns) {
    const { signature, block } = f;
    const { modifier, name, returnType: returnType2 } = signature;
    const { async, generator, set } = modifier;
    const isMethod = f.type === "MethodDefinition";
    const isConstructor = isMethod && name === "constructor";
    const isVoid = isVoidType(returnType2?.t) || async && (isPromiseVoidType(returnType2?.t) || generator && isAsyncGeneratorVoidType(returnType2?.t)) || !async && generator && isGeneratorVoidType(returnType2?.t);
    if (block?.type === "BlockStatement") {
      if (isVoid || set || isConstructor) {
        if (block.bare && block.implicitlyReturned) {
          braceBlock(block);
        }
      } else {
        if (!block.implicitlyReturned) {
          insertReturn(block);
        }
      }
    }
  }
}
function processReturnValue(func) {
  const { block } = func;
  const values = gatherRecursiveWithinFunction(
    block,
    ({ type }) => type === "ReturnValue"
  );
  if (!values.length) {
    return false;
  }
  const ref = makeRef("ret");
  let declaration;
  values.forEach((value) => {
    value.children = [ref];
    const { ancestor, child } = findAncestor(
      value,
      ({ type }) => type === "Declaration",
      isFunction
    );
    if (ancestor) {
      return declaration ??= child;
    }
    ;
    return;
  });
  let returnType = func.returnType ?? func.signature?.returnType;
  if (returnType) {
    const { t } = returnType;
    let m;
    if (m = t.type, m === "TypePredicate") {
      returnType = ": boolean";
    } else if (m === "AssertsType") {
      returnType = void 0;
    }
  }
  if (declaration) {
    if (!(declaration.suffix != null)) {
      declaration.children[1] = declaration.suffix = returnType;
    }
  } else {
    block.expressions.unshift([
      getIndent(block.expressions[0]),
      {
        type: "Declaration",
        children: ["let ", ref, returnType],
        names: []
      },
      ";"
    ]);
  }
  gatherRecursiveWithinFunction(
    block,
    (r) => r.type === "ReturnStatement" && !r.expression
  ).forEach((r) => {
    r.expression = ref;
    return r.children.splice(-1, 1, " ", ref);
  });
  let ref1;
  if (!((ref1 = block.children)[ref1.length - 2]?.type === "ReturnStatement")) {
    let ref2;
    const indent = getIndent((ref2 = block.expressions)[ref2.length - 1]) || ";";
    block.expressions.push([
      [indent],
      {
        type: "ReturnStatement",
        expression: ref,
        children: ["return ", ref]
      }
    ]);
  }
  return true;
}
function patternAsValue(pattern) {
  switch (pattern.type) {
    case "ArrayBindingPattern": {
      const children = [...pattern.children];
      const index = children.indexOf(pattern.elements);
      if (index < 0)
        throw new Error("failed to find elements in ArrayBindingPattern");
      children[index] = pattern.elements.map((el) => {
        const [ws, e, delim] = el.children;
        return { ...el, children: [ws, patternAsValue(e), delim] };
      });
      return { ...pattern, children };
    }
    case "ObjectBindingPattern": {
      const children = [...pattern.children];
      const index = children.indexOf(pattern.properties);
      if (index < 0)
        throw new Error("failed to find properties in ArrayBindingPattern");
      children[index] = pattern.properties.map(patternAsValue);
      return { ...pattern, children };
    }
    case "Identifier":
    case "BindingProperty": {
      const children = [
        // { name: value } = ... declares value, not name
        pattern.value ?? pattern.name,
        pattern.delim
      ];
      if (isWhitespaceOrEmpty(pattern.children[0])) {
        children.unshift(pattern.children[0]);
      }
      return { ...pattern, children };
    }
    default:
      return pattern;
  }
}
function assignResults(node, collect) {
  if (!node)
    return;
  switch (node.type) {
    case "BlockStatement":
      if (node.expressions.length) {
        let ref3;
        assignResults((ref3 = node.expressions)[ref3.length - 1], collect);
      } else {
        node.expressions.push(["", collect("void 0"), ";"]);
      }
      return;
    case "CaseBlock":
      node.clauses.forEach((clause) => {
        return assignResults(clause, collect);
      });
      return;
    case "WhenClause":
    case "DefaultClause":
    case "PatternClause": {
      assignResults(node.block, collect);
      return;
    }
  }
  if (!Array.isArray(node)) {
    return;
  }
  let [, exp, semi] = node;
  if (semi?.type === "SemicolonDelimiter") {
    return;
  }
  if (!exp) {
    return;
  }
  if (isExit(exp)) {
    return;
  }
  const outer = exp;
  let { type } = exp;
  if (type === "LabelledStatement") {
    exp = exp.statement;
    ({ type } = exp);
  }
  let ref4;
  switch (exp.type) {
    case "BreakStatement":
    case "ContinueStatement":
    case "DebuggerStatement":
    case "EmptyStatement":
    case "ReturnStatement":
    case "ThrowStatement": {
      return;
    }
    case "Declaration": {
      let ref5;
      if (exp.bindings?.length) {
        ref5 = patternAsValue((ref4 = exp.bindings)[ref4.length - 1].pattern);
      } else {
        ref5 = "void 0";
      }
      ;
      const value = ref5;
      exp.children.push([
        "",
        [";", collect(value)]
      ]);
      return;
    }
    case "FunctionExpression": {
      if (exp.id) {
        exp.children.push([
          "",
          [";", collect(exp.id)]
        ]);
        return;
      }
      break;
    }
    case "ForStatement":
    case "IterationStatement":
    case "DoStatement":
    case "ComptimeStatement": {
      wrapIterationReturningResults(exp, outer, collect);
      return;
    }
    case "BlockStatement": {
      if (node.expressions.some(isExit)) {
        return;
      }
      assignResults(exp.expressions[exp.expressions.length - 1], collect);
      return;
    }
    case "IfStatement": {
      assignResults(exp.then, collect);
      if (exp.then.bare && !exp.then.semicolon) {
        exp.then.children.push(exp.then.semicolon = ";");
      }
      if (exp.else) {
        assignResults(exp.else.block, collect);
      } else {
        exp.children.push([" else {", collect("void 0"), "}"]);
      }
      return;
    }
    case "PatternMatchingStatement": {
      assignResults(exp.children[0], collect);
      return;
    }
    case "SwitchStatement": {
      assignResults(exp.children[2], collect);
      return;
    }
    case "TryStatement": {
      exp.blocks.forEach((block) => assignResults(block, collect));
      return;
    }
  }
  if (node[node.length - 1]?.type === "SemicolonDelimiter") {
    return;
  }
  node[1] = collect(node[1]);
}
function insertReturn(node, outerNode = node) {
  if (!node)
    return;
  switch (node.type) {
    case "BlockStatement": {
      if (node.expressions.length) {
        if (node.expressions.some(([, exp2]) => isExit(exp2))) {
          return;
        }
        const last = node.expressions[node.expressions.length - 1];
        insertReturn(last);
      } else {
        if (node.parent.type === "CatchClause") {
          node.expressions.push(["return"]);
        }
      }
      return;
    }
    case "WhenClause": {
      if (node.break) {
        node.children.splice(node.children.indexOf(node.break), 1);
      }
      if (node.block.expressions.length) {
        insertReturn(node.block);
      } else {
        node.block.expressions.push(wrapWithReturn());
      }
      return;
    }
    case "DefaultClause": {
      insertReturn(node.block);
      return;
    }
  }
  if (!Array.isArray(node))
    return;
  let [, exp, semi] = node;
  if (semi?.type === "SemicolonDelimiter") {
    return;
  }
  if (!exp) {
    return;
  }
  if (isExit(exp)) {
    return;
  }
  const outer = exp;
  let { type } = exp;
  if (type === "LabelledStatement") {
    exp = exp.statement;
    ({ type } = exp);
  }
  let ref6;
  switch (type) {
    case "BreakStatement":
    case "ContinueStatement":
    case "DebuggerStatement":
    case "EmptyStatement":
    case "ReturnStatement":
    case "ThrowStatement": {
      return;
    }
    case "Declaration": {
      let ref7;
      if (exp.bindings?.length) {
        ref7 = [" ", patternAsValue((ref6 = exp.bindings)[ref6.length - 1].pattern)];
      } else {
        ref7 = [];
      }
      ;
      const value = ref7;
      exp.children.push(["", {
        type: "ReturnStatement",
        children: [
          ";return",
          ...value
        ],
        parent: exp
      }]);
      return;
    }
    case "FunctionExpression": {
      if (exp.id) {
        exp.children.push([
          "",
          {
            type: "ReturnStatement",
            children: [";return ", exp.id],
            parent: exp
          }
        ]);
        return;
      }
      break;
    }
    case "ForStatement":
    case "IterationStatement":
    case "DoStatement":
    case "ComptimeStatement": {
      wrapIterationReturningResults(exp, outer);
      return;
    }
    case "BlockStatement": {
      insertReturn(exp.expressions[exp.expressions.length - 1]);
      return;
    }
    case "IfStatement": {
      insertReturn(exp.then);
      if (exp.else)
        insertReturn(exp.else.block);
      else
        exp.children.push(["", {
          type: "ReturnStatement",
          // NOTE: add a prefixed semi-colon because the if block may not be braced
          children: [";return"],
          parent: exp
        }]);
      return;
    }
    case "PatternMatchingStatement": {
      insertReturn(exp.children[0]);
      return;
    }
    case "SwitchStatement": {
      insertSwitchReturns(exp);
      return;
    }
    case "TryStatement": {
      exp.blocks.forEach((block) => insertReturn(block));
      return;
    }
  }
  if (node[node.length - 1]?.type === "SemicolonDelimiter") {
    return;
  }
  const returnStatement = wrapWithReturn(node[1]);
  node.splice(1, 1, returnStatement);
}
function insertSwitchReturns(exp) {
  exp.caseBlock.clauses.forEach((clause) => {
    return insertReturn(clause);
  });
}
function wrapIterationReturningResults(statement, outer, collect) {
  if (statement.type === "DoStatement" || statement.type === "ComptimeStatement") {
    if (collect) {
      assignResults(statement.block, collect);
    } else {
      insertReturn(statement.block, outer);
    }
    return;
  }
  assert.equal(
    statement.resultsRef,
    void 0,
    "wrapIterationReturningResults should not be called twice on the same statement"
  );
  const resultsRef = statement.resultsRef = makeRef("results");
  const declaration = {
    type: "Declaration",
    children: ["const ", resultsRef, "=[]"]
  };
  outer.children.unshift(["", declaration, ";"]);
  assignResults(statement.block, (node) => {
    return [resultsRef, ".push(", node, ")"];
  });
  if (collect) {
    statement.children.push(collect(resultsRef));
  } else {
    statement.children.push(";return ", resultsRef, ";");
  }
}
function processParams(f) {
  const { type, parameters, block } = f;
  const isConstructor = f.name === "constructor";
  if (type === "ArrowFunction" && parameters && parameters.tp && parameters.tp.parameters.length === 1) {
    parameters.tp.parameters.push(",");
  }
  if (!block)
    return;
  const { expressions } = block;
  if (!expressions)
    return;
  const { blockPrefix } = parameters;
  let indent;
  if (!expressions.length) {
    indent = "";
  } else {
    indent = expressions[0][0];
  }
  const [splices, thisAssignments] = gatherBindingCode(parameters, {
    injectParamProps: isConstructor
  });
  const delimiter = {
    type: "SemicolonDelimiter",
    children: [";"]
  };
  const prefix = splices.map((s) => ["let ", s]).concat(thisAssignments).map(
    (s) => s.type ? {
      // TODO: figure out how to get JS only statement tuples
      ...s,
      children: [indent, ...s.children, delimiter]
    } : [indent, s, delimiter]
  );
  if (!prefix.length)
    return;
  if (isConstructor) {
    const superCalls = gatherNodes(
      expressions,
      (a) => typeof a === "object" && a != null && "type" in a && a.type === "CallExpression" && "children" in a && Array.isArray(a.children) && a.children.length >= 1 && typeof a.children[0] === "object" && a.children[0] != null && "token" in a.children[0] && a.children[0].token === "super"
    );
    if (superCalls.length) {
      const { child } = findAncestor(superCalls[0], (a1) => a1 === block);
      const index = findChildIndex(expressions, child);
      if (index < 0) {
        throw new Error("Could not find super call within top-level expressions");
      }
      expressions.splice(index + 1, 0, ...prefix);
      return;
    }
  }
  expressions.unshift(...prefix);
}
function processSignature(f) {
  const { block, signature } = f;
  if (hasAwait(block) && !f.async?.length) {
    f.async.push("async ");
    signature.modifier.async = true;
  }
  if (hasYield(block) && !f.generator?.length) {
    if (f.type === "ArrowFunction") {
      gatherRecursiveWithinFunction(block, ($) => $.type === "YieldExpression").forEach((y) => {
        const i = y.children.findIndex(($1) => $1.type === "Yield");
        return y.children.splice(i + 1, 0, {
          type: "Error",
          message: "Can't use yield inside of => arrow function"
        });
      });
    } else {
      f.generator.push("*");
      signature.modifier.generator = true;
    }
  }
}
function processFunctions(statements, config2) {
  gatherRecursiveAll(statements, ({ type }) => type === "FunctionExpression" || type === "ArrowFunction").forEach((f) => {
    if (f.type === "FunctionExpression") {
      implicitFunctionBlock(f);
    }
    processSignature(f);
    processParams(f);
    return processReturn(f, config2.implicitReturns);
  });
  gatherRecursiveAll(statements, ({ type }) => type === "MethodDefinition").forEach((f) => {
    implicitFunctionBlock(f);
    processParams(f);
    return processReturn(f, config2.implicitReturns);
  });
}
function expressionizeIteration(exp) {
  const { async, subtype, block, children, statement } = exp;
  const i = children.indexOf(statement);
  if (i < 0) {
    throw new Error("Could not find iteration statement in iteration expression");
  }
  if (subtype === "DoStatement" || subtype === "ComptimeStatement") {
    children.splice(i, 1, wrapIIFE([["", statement, void 0]], async));
    updateParentPointers(exp);
    return;
  }
  exp.resultsRef ??= makeRef("results");
  const { resultsRef } = exp;
  assignResults(block, (node) => {
    return [resultsRef, ".push(", node, ")"];
  });
  braceBlock(block);
  children.splice(
    i,
    1,
    wrapIIFE([
      ["", ["const ", resultsRef, "=[]"], ";"],
      ...children.map((c) => ["", c, void 0]),
      ["", wrapWithReturn(resultsRef)]
    ], async)
  );
  updateParentPointers(exp);
}
function skipImplicitArguments(args) {
  if (args.length === 1) {
    let arg0 = args[0];
    if (arg0.type === "StatementExpression") {
      arg0 = arg0.statement;
    }
    return arg0.type === "IterationExpression" && arg0.subtype !== "DoStatement" && !arg0.async && isEmptyBareBlock(arg0.block);
  }
  return false;
}
function processCoffeeDo(ws, expression) {
  ws = insertTrimmingSpace(ws, "");
  const args = [];
  if (typeof expression === "object" && expression != null && "type" in expression && expression.type === "ArrowFunction" || typeof expression === "object" && expression != null && "type" in expression && expression.type === "FunctionExpression") {
    const { parameters } = expression;
    const newParameters = {
      ...parameters,
      children: (() => {
        const results = [];
        for (let ref8 = parameters.children, i1 = 0, len3 = ref8.length; i1 < len3; i1++) {
          let parameter = ref8[i1];
          if (typeof parameter === "object" && parameter != null && "type" in parameter && parameter.type === "Parameter") {
            let ref9;
            if (ref9 = parameter.initializer) {
              const initializer = ref9;
              args.push(initializer.expression, parameter.delim);
              parameter = {
                ...parameter,
                initializer: void 0,
                children: parameter.children.filter((a2) => a2 !== initializer)
              };
            } else {
              args.push(parameter.children.filter(
                (a3) => a3 !== parameter.typeSuffix
              ));
            }
          }
          results.push(parameter);
        }
        return results;
      })()
    };
    expression = {
      ...expression,
      parameters: newParameters,
      children: expression.children.map(($2) => $2 === parameters ? newParameters : $2)
    };
  }
  return {
    type: "CallExpression",
    children: [
      makeLeftHandSideExpression(expression),
      {
        type: "Call",
        args,
        children: ["(", args, ")"]
      }
    ]
  };
}
function makeAmpersandFunction(rhs) {
  let { ref, typeSuffix, body } = rhs;
  if (!(ref != null)) {
    ref = makeRef("$");
    inplacePrepend(ref, body);
  }
  if (body?.type === "ObjectExpression") {
    body = makeLeftHandSideExpression(body);
  }
  const parameters = makeNode({
    type: "Parameters",
    children: typeSuffix ? ["(", ref, typeSuffix, ")"] : [ref],
    names: []
  });
  const expressions = [[" ", body]];
  const block = makeNode({
    type: "BlockStatement",
    bare: true,
    expressions,
    children: [expressions],
    implicitlyReturned: true
  });
  const async = [];
  const children = [async, parameters, " =>", block];
  const fn = makeNode({
    type: "ArrowFunction",
    async,
    signature: {
      modifier: {
        async: !!async.length
      }
    },
    children,
    ref,
    block,
    parameters,
    ampersandBlock: true,
    body
  });
  if (isStatement(body)) {
    braceBlock(block);
    fn.ampersandBlock = false;
    delete fn.body;
  }
  return fn;
}

// source/parser/block.civet
function blockWithPrefix(prefixStatements, block) {
  if (prefixStatements && prefixStatements.length) {
    const expressions = [...prefixStatements, ...block.expressions];
    updateParentPointers(prefixStatements, block);
    block = {
      ...block,
      expressions,
      children: block.children === block.expressions ? expressions : block.children.map((c) => c === block.expressions ? expressions : c)
    };
    braceBlock(block);
    updateParentPointers(block);
  }
  return block;
}
function braceBlock(block) {
  if (block.bare && !block.root) {
    if (block.children === block.expressions) {
      block.children = [block.expressions];
    }
    block.children.unshift(" {");
    block.children.push("}");
    const { implicitlyReturned } = block;
    block.bare = block.implicitlyReturned = false;
    if (implicitlyReturned) {
      processReturn(block.parent, true);
    }
  }
}
function duplicateBlock(block) {
  const expressions = [...block.expressions];
  let children;
  if (block.children === block.expressions) {
    children = expressions;
  } else {
    children = [...block.children];
    children.splice(children.indexOf(block.expressions), 1, expressions);
  }
  return {
    ...block,
    expressions,
    children
  };
}
function makeEmptyBlock() {
  const expressions = [];
  return {
    type: "BlockStatement",
    expressions,
    children: ["{", expressions, "}"],
    bare: false,
    empty: true
  };
}
function makeBlockFragment() {
  const expressions = [];
  return {
    type: "BlockStatement",
    children: expressions,
    parent: void 0,
    expressions,
    bare: false,
    root: false
  };
}
function replaceBlockExpression(node, child, replacement) {
  let found = false;
  const { expressions } = node;
  for (let i1 = 0, len3 = expressions.length; i1 < len3; i1++) {
    const statement = expressions[i1];
    const [, s] = statement;
    if (s === child) {
      statement[1] = replacement;
      replacement.parent = node;
      found = true;
      break;
    }
  }
  if (!found) {
    throw new Error("Could not find child to replace");
  }
}
function getIndent(statement) {
  let indent = statement?.[0];
  if (Array.isArray(indent)) {
    indent = indent.flat(Infinity);
    return indent.filter((n) => n && !(n.type === "Comment")).map((n) => {
      if (typeof n === "string")
        return n;
      if (n.token != null)
        return n.token;
      return "";
    });
  }
  return indent;
}
function hoistRefDecs(statements) {
  gatherRecursiveAll(statements, (s) => s.hoistDec).forEach((node) => {
    const { hoistDec } = node;
    node.hoistDec = null;
    const { ancestor, child } = findAncestor(node, (ancestor2) => {
      return ancestor2.type === "BlockStatement" && (!ancestor2.bare || ancestor2.root);
    });
    if (ancestor) {
      insertHoistDec(ancestor, child, hoistDec);
    } else {
      throw new Error("Couldn't find block to hoist declaration into.");
    }
    return;
  });
}
function insertHoistDec(block, node, dec) {
  const { expressions } = block;
  const index = findChildIndex(expressions, node);
  if (index < 0) {
    throw new Error("Couldn't find expression in block for hoistable declaration.");
  }
  const statement = [expressions[index][0], dec, ";"];
  expressions[index][0] = "";
  expressions.splice(index, 0, statement);
  updateParentPointers(dec, block);
}
function processBlocks(statements) {
  insertSemicolon(statements);
  gatherRecursive(statements, ($) => $.type === "BlockStatement").forEach(({ expressions }) => {
    return processBlocks(expressions);
  });
}
function insertSemicolon(statements) {
  const l = statements.length;
  for (let i2 = 0, len1 = statements.length; i2 < len1; i2++) {
    const i = i2;
    const s = statements[i2];
    if (i < l - 1) {
      if (needsPrecedingSemicolon(statements[i + 1][1])) {
        const delim = s[2];
        if (!delim) {
          s[2] = ";";
        } else if (typeof delim === "string" && !delim.match(/;/)) {
          s[2] = `;${delim}`;
        }
      }
    }
  }
}
function needsPrecedingSemicolon(exp) {
  if (!exp) {
    return false;
  }
  if (Array.isArray(exp)) {
    for (let i3 = 0, len22 = exp.length; i3 < len22; i3++) {
      const child = exp[i3];
      if (!(child != null)) {
        continue;
      }
      return needsPrecedingSemicolon(child);
    }
    return false;
  }
  if (isToken(exp)) {
    exp = exp.token;
  }
  if (typeof exp === "string") {
    return /^\s*[\(\[\`\+\-\/]/.test(exp);
  }
  switch (exp.type) {
    case "ParenthesizedExpression":
    case "ArrayExpression":
    case "ArrowFunction":
    case "TemplateLiteral":
    case "RegularExpressionLiteral":
    case "RangeExpression":
    case "ComputedPropertyName": {
      return true;
    }
    case "AssignmentExpression": {
      return startsWith(exp, /^(\[|\()/);
    }
    case "Literal": {
      return exp.raw?.startsWith("-") || exp.raw?.startsWith("+");
    }
    case "PipelineExpression":
    case "UnwrappedExpression": {
      return needsPrecedingSemicolon(exp.children[1]);
    }
    default: {
      if (exp.children) {
        return needsPrecedingSemicolon(exp.children);
      }
      ;
      return;
    }
  }
}

// source/parser/util.civet
function len(arr, length) {
  return arr.length === length;
}
var assert = {
  equal(a, b, msg) {
    if (a !== b) {
      throw new Error(`Assertion failed [${msg}]: ${a} !== ${b}`);
    }
  }
};
function addParentPointers(node, parent) {
  if (node == null)
    return;
  if (typeof node !== "object")
    return;
  if (Array.isArray(node)) {
    for (const child of node) {
      addParentPointers(child, parent);
    }
    return;
  }
  node = node;
  if (parent != null) {
    node.parent = parent;
  }
  if (node.children) {
    for (let ref = node.children, i1 = 0, len1 = ref.length; i1 < len1; i1++) {
      const child = ref[i1];
      addParentPointers(child, node);
    }
  }
}
function clone(node) {
  removeParentPointers(node);
  return deepCopy(node);
}
function removeParentPointers(node) {
  if (!(node != null && typeof node === "object")) {
    return;
  }
  if (Array.isArray(node)) {
    for (const child of node) {
      removeParentPointers(child);
    }
    return;
  }
  node.parent = null;
  if (node.children) {
    for (const child of node.children) {
      removeParentPointers(child);
    }
  }
}
function maybeWrap(node, parent) {
  if (!isASTNodeObject(node)) {
    updateParentPointers(node = {
      type: "Wrapper",
      children: [node],
      parent
    });
  }
  return node;
}
function maybeUnwrap(node) {
  if (node?.type === "Wrapper") {
    return node.children[0];
  } else {
    return node;
  }
}
function isASTNodeObject(node) {
  return typeof node === "object" && node != null && !Array.isArray(node);
}
function isParent(node) {
  return node != null && node.children != null;
}
function isToken(node) {
  return node != null && node.token != null;
}
function isEmptyBareBlock(node) {
  if (node?.type !== "BlockStatement")
    return false;
  const { bare, expressions } = node;
  return bare && (Array.isArray(expressions) && len(expressions, 0) || Array.isArray(expressions) && len(expressions, 1) && Array.isArray(expressions[0]) && expressions[0].length >= 2 && typeof expressions[0][1] === "object" && expressions[0][1] != null && "type" in expressions[0][1] && expressions[0][1].type === "EmptyStatement");
}
function isFunction(node) {
  if (node && typeof node === "object" && "type" in node) {
    const { type } = node;
    return type === "FunctionExpression" || type === "ArrowFunction" || type === "MethodDefinition";
  }
  return false;
}
var statementTypes = /* @__PURE__ */ new Set([
  "BlockStatement",
  "BreakStatement",
  "ComptimeStatement",
  "ContinueStatement",
  "DebuggerStatement",
  "Declaration",
  "DoStatement",
  "ForStatement",
  "IfStatement",
  "IterationStatement",
  "LabeledStatement",
  "ReturnStatement",
  "SwitchStatement",
  "ThrowStatement",
  "TryStatement"
]);
function isStatement(node) {
  return isASTNodeObject(node) && node.type != null && // forbid leaf
  statementTypes.has(node.type);
}
function isWhitespaceOrEmpty(node) {
  if (!node)
    return true;
  if (node.type === "Ref")
    return false;
  if (node.token)
    return /^\s*$/.test(node.token);
  if (node.children)
    node = node.children;
  if (!node.length)
    return true;
  if (typeof node === "string")
    return /^\s*$/.test(node);
  if (Array.isArray(node))
    return node.every(isWhitespaceOrEmpty);
  return false;
}
function isExit(node) {
  if (!(node != null)) {
    return false;
  }
  switch (node.type) {
    case "ReturnStatement":
    case "ThrowStatement":
    case "BreakStatement":
    case "ContinueStatement": {
      return true;
    }
    case "IfStatement": {
      return isExit(node.then) && isExit(node.else?.block);
    }
    case "BlockStatement": {
      return node.expressions.some((s) => isExit(s[1]));
    }
    case "IterationStatement": {
      return node.condition?.type === "ParenthesizedExpression" && node.condition.expression?.type === "Literal" && node.condition.expression?.raw === "true" && gatherRecursiveWithinFunction(node.block, ($) => $.type === "BreakStatement").length === 0;
    }
    default: {
      return false;
    }
  }
}
function isComma(node) {
  if (node?.token === ",") {
    return node;
  } else if (Array.isArray(node) && node[node.length - 1]?.token === ",") {
    return node[node.length - 1];
  }
  ;
  return;
}
function insertTrimmingSpace(target, c) {
  if (!(target != null)) {
    return target;
  }
  if (Array.isArray(target)) {
    if (target.length === 0) {
      return c;
    }
    return target.map((e, i) => {
      if (i === 0) {
        return insertTrimmingSpace(e, c);
      } else {
        return e;
      }
    });
  } else if (isParent(target)) {
    return {
      ...target,
      children: insertTrimmingSpace(target.children, c)
    };
  } else if (isToken(target)) {
    return {
      ...target,
      token: target.token.replace(/^ ?/, c)
    };
  } else if (typeof target === "string") {
    return target.replace(/^ ?/, c);
  } else {
    return target;
  }
}
function trimFirstSpace(target) {
  return insertTrimmingSpace(target, "");
}
function inplaceInsertTrimmingSpace(target, c) {
  if (!(target != null)) {
    return target;
  }
  if (Array.isArray(target)) {
    inplaceInsertTrimmingSpace(target[0], c);
  } else if (isParent(target)) {
    inplaceInsertTrimmingSpace(target.children, c);
  } else if (isToken(target)) {
    target.token = target.token.replace(/^ ?/, c);
  }
}
function getTrimmingSpace(target) {
  if (!(target != null)) {
    return;
  }
  if (Array.isArray(target)) {
    return getTrimmingSpace(target[0]);
  } else if (isParent(target)) {
    return getTrimmingSpace(target.children[0]);
  } else if (isToken(target)) {
    return target.token.match(/^ ?/)[0];
  }
  ;
  return;
}
function prepend(prefix, node) {
  if (!(prefix && prefix.length)) {
    return node;
  }
  if (Array.isArray(node)) {
    return [prefix, ...node];
  } else if (isParent(node)) {
    return {
      ...node,
      children: [prefix, ...node.children]
    };
  } else {
    return [prefix, node];
  }
}
function inplacePrepend(prefix, node) {
  if (!prefix) {
    return;
  }
  if (Array.isArray(prefix) && !prefix.length) {
    return;
  }
  if (Array.isArray(node)) {
    node.unshift(prefix);
  } else if (isParent(node)) {
    node.children.unshift(prefix);
  } else {
    throw new Error("Can't prepend to a leaf node");
  }
}
function literalValue(literal) {
  let { raw } = literal;
  switch (raw) {
    case "null":
      return null;
    case "true":
      return true;
    case "false":
      return false;
  }
  if (raw.startsWith('"') && raw.endsWith('"') || raw.startsWith("'") && raw.endsWith("'")) {
    return raw.slice(1, -1);
  }
  const numeric = literal.children.find(
    (child) => child.type === "NumericLiteral"
  );
  if (numeric) {
    raw = raw.replace(/_/g, "");
    const { token } = numeric;
    if (token.endsWith("n")) {
      return BigInt(raw.slice(0, -1));
    } else if (token.match(/[\.eE]/)) {
      return parseFloat(raw);
    } else if (token.startsWith("0")) {
      switch (token.charAt(1).toLowerCase()) {
        case "x":
          return parseInt(raw.replace(/0[xX]/, ""), 16);
        case "b":
          return parseInt(raw.replace(/0[bB]/, ""), 2);
        case "o":
          return parseInt(raw.replace(/0[oO]/, ""), 8);
      }
    }
    return parseInt(raw, 10);
  }
  throw new Error("Unrecognized literal " + JSON.stringify(literal));
}
function startsWith(target, value) {
  if (!target)
    return;
  if (Array.isArray(target)) {
    let i = 0;
    let l = target.length;
    while (i < l) {
      const t = target[i];
      if (t && (t.length || t.token || t.children)) {
        break;
      }
      i++;
    }
    if (i < l) {
      return startsWith(target[i], value);
    }
  }
  if (typeof target === "string")
    return value.test(target);
  if (target.children)
    return startsWith(target.children, value);
  if (target.token)
    return value.test(target.token);
  return;
}
function hasAwait(exp) {
  return gatherRecursiveWithinFunction(exp, ($1) => $1.type === "Await").length > 0;
}
function hasYield(exp) {
  return gatherRecursiveWithinFunction(exp, ($2) => $2.type === "Yield").length > 0;
}
function hasImportDeclaration(exp) {
  return gatherRecursiveWithinFunction(exp, ($3) => $3.type === "ImportDeclaration").length > 0;
}
function deepCopy(node) {
  if (node == null)
    return node;
  if (typeof node !== "object")
    return node;
  if (Array.isArray(node)) {
    return node.map(deepCopy);
  }
  if (node?.type === "Ref")
    return node;
  return Object.fromEntries(
    Object.entries(node).map(([key, value]) => {
      return [key, deepCopy(value)];
    })
  );
}
function removeHoistDecs(node) {
  if (node == null)
    return;
  if (typeof node !== "object")
    return;
  if ("hoistDec" in node) {
    node.hoistDec = void 0;
  }
  if (Array.isArray(node)) {
    for (const child of node) {
      removeHoistDecs(child);
    }
    return;
  }
  if (node.children) {
    for (const child of node.children) {
      removeHoistDecs(child);
    }
  }
}
var skipParens = /* @__PURE__ */ new Set([
  "AmpersandRef",
  "CallExpression",
  "Identifier",
  "JSXElement",
  "JSXFragment",
  "Literal",
  "MemberExpression",
  "NewExpression",
  "ParenthesizedExpression",
  "Ref",
  "Placeholder",
  "StatementExpression"
  // wrapIIFE
]);
function makeLeftHandSideExpression(expression) {
  if (isASTNodeObject(expression)) {
    if (expression.parenthesized) {
      return expression;
    }
    if (skipParens.has(expression.type)) {
      return expression;
    }
  }
  return makeNode({
    type: "ParenthesizedExpression",
    children: ["(", expression, ")"],
    expression,
    implicit: true
  });
}
function updateParentPointers(node, parent, depth = 1) {
  if (!(node != null)) {
    return;
  }
  if (!(typeof node === "object")) {
    return;
  }
  if (Array.isArray(node)) {
    for (let i2 = 0, len22 = node.length; i2 < len22; i2++) {
      const child = node[i2];
      updateParentPointers(child, parent, depth);
    }
    return;
  }
  node = node;
  if (parent != null) {
    node.parent = parent;
  }
  if (depth && isParent(node)) {
    for (let ref1 = node.children, i3 = 0, len3 = ref1.length; i3 < len3; i3++) {
      const child = ref1[i3];
      updateParentPointers(child, node, depth - 1);
    }
  }
}
function makeNode(node) {
  updateParentPointers(node);
  return node;
}
function skipIfOnlyWS(target) {
  if (!target)
    return target;
  if (Array.isArray(target)) {
    if (target.length === 1) {
      return skipIfOnlyWS(target[0]);
    } else if (target.every((e) => skipIfOnlyWS(e) === void 0)) {
      return void 0;
    }
    return target;
  }
  if (target.token != null && target.token.trim() === "") {
    return void 0;
  }
  return target;
}
function spliceChild(node, child, del, ...replacements) {
  const children = node?.children ?? node;
  if (!Array.isArray(children)) {
    throw new Error("spliceChild: non-array node has no children field");
  }
  const index = children.indexOf(child);
  if (index < 0) {
    throw new Error("spliceChild: child not found");
  }
  return children.splice(index, del, ...replacements);
}
function convertOptionalType(suffix) {
  if (suffix.t.type === "AssertsType") {
    spliceChild(suffix, suffix.optional, 1, suffix.optional = {
      type: "Error",
      message: "Can't use optional ?: syntax with asserts type"
    });
    return;
  }
  spliceChild(suffix, suffix.optional, 1, suffix.optional = void 0);
  const wrap = suffix.type === "ReturnTypeAnnotation";
  spliceChild(suffix, suffix.t, 1, suffix.t = [
    getTrimmingSpace(suffix.t),
    wrap && "(",
    // TODO: avoid parens if unnecessary
    "undefined | ",
    parenthesizeType(insertTrimmingSpace(suffix.t, "")),
    wrap && ")"
  ]);
}
var typeNeedsNoParens = /* @__PURE__ */ new Set([
  "IdentifierType",
  "ImportType",
  "LiteralType",
  "TupleType",
  "ParenthesizedType",
  "UnaryType"
]);
function parenthesizeType(type) {
  if (typeNeedsNoParens.has(type.type)) {
    return type;
  }
  return ["(", type, ")"];
}
function wrapIIFE(expressions, asyncFlag) {
  let prefix;
  const async = [];
  if (asyncFlag) {
    async.push("async ");
  } else if (hasAwait(expressions)) {
    async.push("async ");
    prefix = {
      type: "Await",
      children: ["await "]
    };
  }
  const block = makeNode({
    type: "BlockStatement",
    expressions,
    children: ["{", expressions, "}"],
    bare: false,
    root: false
  });
  const parameters = {
    type: "Parameters",
    children: ["()"],
    names: []
  };
  const signature = {
    modifier: {
      async: !!async.length
    },
    returnType: void 0
  };
  const fn = makeNode({
    type: "ArrowFunction",
    signature,
    parameters,
    returnType: void 0,
    ts: false,
    async,
    block,
    children: [async, parameters, "=>", block]
  });
  const exp = makeNode({
    type: "CallExpression",
    children: [makeLeftHandSideExpression(fn), "()"]
  });
  if (prefix) {
    return makeLeftHandSideExpression([prefix, exp]);
  }
  return exp;
}
function wrapWithReturn(expression) {
  const children = expression ? ["return ", expression] : ["return"];
  return makeNode({
    type: "ReturnStatement",
    children,
    expression,
    parent: expression?.parent
  });
}
function flatJoin(array, separator) {
  const result = [];
  for (let i4 = 0, len4 = array.length; i4 < len4; i4++) {
    const i = i4;
    const items = array[i4];
    if (i) {
      result.push(separator);
    }
    result.push(...items);
  }
  return result;
}

// source/parser/traversal.civet
function gatherRecursiveWithinFunction(node, predicate) {
  return gatherRecursive(node, predicate, isFunction);
}
function findChildIndex(parent, child) {
  if (!(parent != null)) {
    return -1;
  }
  const children = Array.isArray(parent) ? parent : parent.children;
  if (!(children != null)) {
    return -1;
  }
  for (let i1 = 0, len3 = children.length; i1 < len3; i1++) {
    const i = i1;
    const c = children[i1];
    if (c === child || Array.isArray(c) && arrayRecurse(c)) {
      return i;
    }
  }
  function arrayRecurse(array) {
    for (let i2 = 0, len1 = array.length; i2 < len1; i2++) {
      const c = array[i2];
      if (c === child || Array.isArray(c) && arrayRecurse(c)) {
        return true;
      }
    }
    return false;
  }
  return -1;
}
function findAncestor(node, predicate, stopPredicate) {
  let { parent } = node;
  while (parent && !stopPredicate?.(parent, node)) {
    if (predicate(parent, node)) {
      return { ancestor: parent, child: node };
    }
    node = parent;
    ({ parent } = node);
  }
  return { ancestor: void 0, child: node };
}
function gatherNodes(node, predicate) {
  if (node == null || typeof node === "string") {
    return [];
  }
  if (Array.isArray(node)) {
    return node.flatMap((n) => gatherNodes(n, predicate));
  }
  if (predicate(node)) {
    return [node];
  }
  switch (node.type) {
    case "BlockStatement": {
      return [];
    }
    case "ForStatement": {
      const isDec = node.declaration?.type === "Declaration";
      return node.children.flatMap((n) => {
        if (isDec && n === node.declaration) {
          return [];
        } else {
          return gatherNodes(n, predicate);
        }
      });
    }
    default: {
      return gatherNodes(
        //@ts-ignore
        node.children,
        predicate
      );
    }
  }
}
function gatherRecursive(node, predicate, skipPredicate) {
  if (node == null || typeof node === "string") {
    return [];
  }
  if (Array.isArray(node)) {
    return node.flatMap(($) => gatherRecursive($, predicate, skipPredicate));
  }
  if (skipPredicate?.(node)) {
    return [];
  }
  if (predicate(node)) {
    return [node];
  }
  return gatherRecursive(
    //@ts-ignore
    node.children,
    predicate,
    skipPredicate
  );
}
function gatherRecursiveAll(node, predicate) {
  if (node == null || typeof node === "string") {
    return [];
  }
  if (Array.isArray(node)) {
    return node.flatMap((n) => gatherRecursiveAll(n, predicate));
  }
  const nodes = gatherRecursiveAll(
    //@ts-ignore
    node.children,
    predicate
  );
  if (predicate(node)) {
    nodes.push(node);
  }
  return nodes;
}

// source/parser/op.civet
var precedenceOrder = [
  ["||", "??"],
  ["^^"],
  ["&&"],
  ["|"],
  ["^"],
  ["&"],
  // NOTE: Equality and inequality merged because of relational chaining
  [
    "==",
    "!=",
    "===",
    "!==",
    "<",
    "<=",
    ">",
    ">=",
    "in",
    "instanceof"
  ],
  // NOTE: Extra in-between level for default custom operators
  ["custom"],
  ["<<", ">>", ">>>"],
  ["+", "-"],
  ["*", "/", "%"],
  ["**"]
];
var precedenceMap = /* @__PURE__ */ new Map();
for (let i1 = 0, len3 = precedenceOrder.length; i1 < len3; i1++) {
  const prec = i1;
  const ops = precedenceOrder[i1];
  for (let i2 = 0, len1 = ops.length; i2 < len1; i2++) {
    const op = ops[i2];
    precedenceMap.set(op, prec);
  }
}
var precedenceStep = 1 / 64;
var precedenceRelational = precedenceMap.get("==");
var precedenceCustomDefault = precedenceMap.get("custom");
function getPrecedence(op) {
  if (typeof op === "string") {
    return precedenceMap.get(op) ?? (() => {
      throw new Error(`Unknown operator: ${op}`);
    })();
  } else if (op.type === "PatternTest") {
    return precedenceRelational;
  } else if (typeof op.prec === "number") {
    return op.prec;
  } else {
    return precedenceMap.get(op.prec ?? op.token) ?? (op.relational ? precedenceRelational : precedenceCustomDefault);
  }
}
function processBinaryOpExpression($0) {
  return recurse(expandChainedComparisons($0));
  function recurse(expandedOps) {
    let i = 2;
    while (i < expandedOps.length) {
      let op = expandedOps[i];
      if (op.special) {
        let advanceLeft2 = function(allowEqual) {
          while (start >= 4) {
            const prevPrec = getPrecedence(expandedOps[start - 2]);
            if (!(prevPrec > prec || allowEqual && prevPrec === prec)) {
              return prevPrec === prec;
            }
            start -= 4;
          }
          return false;
        }, advanceRight2 = function(allowEqual) {
          while (end + 4 < expandedOps.length) {
            const nextPrec = getPrecedence(expandedOps[end + 2]);
            if (!(nextPrec > prec || allowEqual && nextPrec === prec)) {
              return nextPrec === prec;
            }
            end += 4;
          }
          return false;
        };
        var advanceLeft = advanceLeft2, advanceRight = advanceRight2;
        let start = i - 2, end = i + 2;
        const prec = getPrecedence(op);
        let error;
        switch (op.assoc) {
          case "left":
          case void 0: {
            advanceLeft2(true);
            advanceRight2(false);
            break;
          }
          case "right": {
            advanceLeft2(false);
            advanceRight2(true);
            break;
          }
          case "non": {
            if (advanceLeft2(false) || advanceRight2(false)) {
              error = {
                type: "Error",
                message: `non-associative operator ${op.token} used at same precedence level without parenthesization`
              };
            }
            ;
            break;
          }
          case "arguments": {
            if (advanceLeft2(false)) {
              error = {
                type: "Error",
                message: `arguments operator ${op.token} used at same precedence level as ${expandedOps[start - 2].token} to the left`
              };
            }
            advanceRight2(true);
            break;
          }
          default: {
            throw new Error(`Unsupported associativity: ${op.assoc}`);
          }
        }
        let a = start === i - 2 ? expandedOps[start] : expandedOps.slice(start, i - 1);
        let wsOp = expandedOps[i - 1];
        let wsB = expandedOps[i + 1];
        let b = end === i + 2 ? expandedOps[i + 2] : expandedOps.slice(i + 2, end + 1);
        if (op.assoc === "arguments") {
          let i2 = 2;
          while (i2 < b.length) {
            if (prec === getPrecedence(b[i2])) {
              if (!(b[i2].token === op.token)) {
                error ??= {
                  type: "Error",
                  message: `arguments operator ${op.token} used at same precedence level as ${b[i2].token} to the right`
                };
              }
              b[i2] = ",";
            }
            i2 += 4;
          }
        } else {
          b = recurse(b);
        }
        if (op.token === "instanceof" && (typeof b === "object" && b != null && "type" in b && b.type === "Literal" && "children" in b && Array.isArray(b.children) && b.children.length >= 1 && typeof b.children[0] === "object" && b.children[0] != null && "type" in b.children[0] && b.children[0].type === "StringLiteral")) {
          a = ["typeof ", makeLeftHandSideExpression(a)];
          if (op.negated) {
            op = { ...op, token: "!==", negated: false };
          } else {
            op = { ...op, token: "===" };
          }
        }
        if (op.asConst) {
          a = makeAsConst(a);
          b = makeAsConst(b);
        }
        let children;
        if (op.type === "PatternTest") {
          children = [processPatternTest(a, b)];
        } else if (op.call) {
          wsOp = insertTrimmingSpace(wsOp, "");
          if (op.reversed) {
            wsB = insertTrimmingSpace(wsB, "");
            children = [wsOp, op.call, "(", wsB, b, ", ", a, ")", op.suffix];
          } else {
            children = [wsOp, op.call, "(", a, ",", wsB, b, ")", op.suffix];
          }
        } else if (op.method) {
          wsOp = insertTrimmingSpace(wsOp, "");
          wsB = insertTrimmingSpace(wsB, "");
          if (op.reversed) {
            if (end !== i + 2)
              b = makeLeftHandSideExpression(b);
            b = dotNumericLiteral(b);
            children = [wsB, b, wsOp, ".", op.method, "(", a, ")"];
          } else {
            if (start !== i - 2 || a.type === "NumericLiteral") {
              a = makeLeftHandSideExpression(a);
            }
            a = dotNumericLiteral(a);
            children = [a, wsOp, ".", op.method, "(", wsB, b, ")"];
          }
        } else if (op.token) {
          children = [a, wsOp, op, wsB, b];
          if (op.negated)
            children = ["(", ...children, ")"];
        } else {
          throw new Error("Unknown operator: " + JSON.stringify(op));
        }
        if (op.negated)
          children.unshift("!");
        if (error != null) {
          children.push(error);
        }
        expandedOps.splice(start, end - start + 1, {
          children
        });
        i = start + 2;
      } else {
        i += 4;
      }
    }
    return expandedOps;
  }
}
function dotNumericLiteral(literal) {
  if (literal?.type === "Literal" && /^[+-]?(?:0|[1-9](?:_[0-9]|[0-9])*)$/.test(literal.raw)) {
    literal.children.push(".");
    literal.raw += ".";
  }
  return literal;
}
var asConst = {
  ts: true,
  children: [" as const"]
};
function makeAsConst(node) {
  if (Array.isArray(node) && node.length === 1) {
    node = node[0];
  }
  if (node.type === "Literal" && node.raw !== "null" || node.type === "ArrayExpression" || node.type === "ObjectExpression") {
    return { ...node, children: [...node.children, asConst] };
  } else {
    return node;
  }
}
function isExistence(exp) {
  if (exp?.type === "ParenthesizedExpression" && exp.implicit) {
    exp = exp.expression;
  }
  if (exp?.type === "Existence") {
    return exp;
  }
  ;
  return;
}
function isRelationalOp(op) {
  return op.relational || getPrecedence(op) === precedenceRelational;
}
function expandChainedComparisons([first, binops]) {
  if (!binops.length) {
    return [first];
  }
  const results = [];
  let start = 0;
  const chains = [];
  var i = 0;
  for (let i3 = 0, len22 = binops.length; i3 < len22; i3++) {
    var i = i3;
    var [, op] = binops[i3];
    if (isRelationalOp(op)) {
      chains.push(i);
    } else if (getPrecedence(op) < precedenceRelational) {
      processChains();
      first = results.pop();
    }
  }
  processChains();
  return results;
  function processChains() {
    if (chains.length > 0) {
      first = expandExistence(first);
      for (let i4 = 0, len3 = chains.length; i4 < len3; i4++) {
        const k = i4;
        const index = chains[i4];
        if (k > 0) {
          results.push(" ", "&&", " ");
        }
        const binop = binops[index];
        const exp = binop[3] = expandExistence(binop[3]);
        results.push(first);
        const endIndex = chains[k + 1] ?? i + 1;
        results.push(...binops.slice(start, endIndex).flat());
        first = [exp].concat(binops.slice(index + 1, endIndex));
        start = endIndex;
      }
    } else {
      results.push(first);
      results.push(...binops.slice(start, i + 1).flat());
      start = i + 1;
    }
    chains.length = 0;
  }
  function expandExistence(exp) {
    let ref;
    if (ref = isExistence(exp)) {
      const existence = ref;
      results.push(existence, " ", "&&", " ");
      return existence.expression;
    } else {
      return exp;
    }
  }
  ;
}

// source/parser/helper.civet
var preludeVar = "var ";
function ts(children) {
  return {
    ts: true,
    children: Array.isArray(children) ? children : [children]
  };
}
var asAny = ts(" as any");
var declareHelper = {
  indexOf(indexOfRef) {
    state.prelude.push(["", [
      // [indent, statement]
      preludeVar,
      indexOfRef,
      ts([": <T>(this: T[], searchElement: T) => number"]),
      " = [].indexOf",
      asAny
    ], ";\n"]);
  },
  hasProp(hasPropRef) {
    state.prelude.push(["", [
      // [indent, statement]
      preludeVar,
      hasPropRef,
      ts(": <T>(object: T, prop: PropertyKey) => boolean"),
      " = ({}.constructor",
      asAny,
      ").hasOwn;\n"
    ]]);
  },
  is(isRef) {
    state.prelude.push(["", [
      // [indent, statement]
      preludeVar,
      isRef,
      ts(": { <B, A extends B> (a: A, b: B): b is A, <A, B> (a: A, b: B): a is A & B }"),
      " = Object.is",
      asAny,
      ";\n"
    ]]);
  },
  /**
   * Array length check with type guard.
   * From tlgreg https://discord.com/channels/933472021310996512/1012166187196629113/1157386582546976873
   */
  len(lenRef) {
    state.prelude.push(["", [
      ts(["function ", lenRef, "<T extends readonly unknown[], N extends number>(arr: T, length: N): arr is T & { length: N } { return arr.length === length }"]),
      {
        js: true,
        children: ["function ", lenRef, "(arr, length) { return arr.length === length }"]
      },
      "\n"
    ]]);
  },
  modulo(moduloRef) {
    state.prelude.push(["", [
      // [indent, statement]
      preludeVar,
      moduloRef,
      ts(": (a: number, b: number) => number"),
      " = (a, b) => (a % b + b) % b;\n"
    ]]);
  },
  Falsy(FalsyRef) {
    state.prelude.push([
      "",
      // [indent, statement]
      ts(["type ", FalsyRef, " = false | 0 | '' | 0n | null | undefined;\n"])
    ]);
  },
  xor(xorRef) {
    const Falsy = getHelperRef("Falsy");
    state.prelude.push(["", [
      // [indent, statement]
      preludeVar,
      xorRef,
      ts([
        ": <A, B>(a: A, b: B) => A extends ",
        Falsy,
        " ? B : B extends ",
        Falsy,
        " ? A : (false | (A & ",
        Falsy,
        " extends never ? never : B) | (B & ",
        Falsy,
        " extends never ? never : A))"
      ]),
      " = (a, b) => (a ? !b && a : b)",
      asAny,
      ";\n"
    ]]);
  },
  xnor(xnorRef) {
    const Falsy = getHelperRef("Falsy");
    state.prelude.push(["", [
      // [indent, statement]
      preludeVar,
      xnorRef,
      ts([
        ": <A, B>(a: A, b: B) => A & ",
        Falsy,
        " extends never ? B : (true | (B extends ",
        Falsy,
        " ? never : A) | (A extends ",
        Falsy,
        " ? never : B))"
      ]),
      " = (a, b) => (a ? b : !b || a)",
      asAny,
      ";\n"
    ]]);
  },
  concatAssign(ref) {
    state.prelude.push(["", [
      // [indent, statement]
      preludeVar,
      ref,
      ts([
        ": <B, A extends {push: (this: A, b: B) => void} | (B extends unknown[] ? {push: (this: A, ...b: B) => void} : never)>(lhs: A, rhs: B) => A"
      ]),
      " = (lhs, rhs) => (((rhs",
      asAny,
      ")?.[Symbol.isConcatSpreadable] ?? Array.isArray(rhs)) ? (lhs",
      asAny,
      ").push.apply(lhs, rhs",
      asAny,
      ") : (lhs",
      asAny,
      ").push(rhs), lhs);\n"
    ]]);
  },
  JSX(jsxRef) {
    state.prelude.push([
      "",
      // [indent, statement]
      ts(["import type { JSX as ", jsxRef, " } from 'solid-js'"]),
      ";\n"
    ]);
  },
  IntrinsicElements(intrinsicElementsRef) {
    const JSX = getHelperRef("JSX");
    state.prelude.push([
      "",
      // [indent, statement, delim]
      ts([
        "type ",
        intrinsicElementsRef,
        "<K extends keyof ",
        JSX,
        ".IntrinsicElements> =\n",
        "  ",
        JSX,
        ".IntrinsicElements[K] extends ",
        JSX,
        ".DOMAttributes<infer T> ? T : unknown"
      ]),
      ";\n"
    ]);
  }
};
function getHelperRef(base) {
  if (base in state.helperRefs) {
    return state.helperRefs[base];
  }
  const ref = makeRef(base);
  if (!(base in declareHelper)) {
    throw new Error(`Unknown helper function: ${base}`);
  }
  declareHelper[base](ref);
  return state.helperRefs[base] = ref;
}
function extractPreludeFor(node) {
  let helpers = new Set(Object.values(state.helperRefs));
  helpers = new Set(gatherRecursive(node, helpers.has.bind(helpers)));
  return state.prelude.filter((s) => gatherRecursive(s, helpers.has.bind(helpers)).length);
}

// source/parser/pattern-matching.civet
function processPatternTest(lhs, patterns) {
  const { ref, hoistDec, refAssignmentComma } = maybeRefAssignment(lhs, "m");
  const conditionExpression = flatJoin(patterns.map(($) => getPatternConditions($, ref)).map(($1) => flatJoin($1, " && ")), " || ");
  return makeLeftHandSideExpression({
    type: "PatternTest",
    hoistDec,
    children: [
      ...refAssignmentComma,
      conditionExpression
    ]
  });
}
function processPatternMatching(statements) {
  gatherRecursiveAll(statements, ($2) => $2.type === "SwitchStatement").forEach((s) => {
    const { caseBlock } = s;
    const { clauses } = caseBlock;
    for (let i1 = 0, len3 = clauses.length; i1 < len3; i1++) {
      const c = clauses[i1];
      if (c.type === "WhenClause" && c.break) {
        if (isExit(c.block)) {
          c.children.splice(c.children.indexOf(c.break), 1);
          c.break = void 0;
        }
      }
    }
    let errors = false;
    let isPattern = false;
    if (clauses.some(($3) => $3.type === "PatternClause")) {
      isPattern = true;
      for (let i2 = 0, len1 = clauses.length; i2 < len1; i2++) {
        const c = clauses[i2];
        if (!(c.type === "PatternClause" || c.type === "DefaultClause")) {
          errors = true;
          c.children.push({
            type: "Error",
            message: "Can't mix pattern matching and non-pattern matching clauses"
          });
        }
      }
    }
    if (errors || !isPattern) {
      return;
    }
    let { condition } = s;
    if (condition.type === "ParenthesizedExpression") {
      condition = condition.expression;
    }
    let { ref, hoistDec, refAssignmentComma } = maybeRefAssignment(condition, "m");
    const root = [];
    let prev = root;
    let e;
    const l = clauses.length;
    for (let i3 = 0, len22 = clauses.length; i3 < len22; i3++) {
      const i = i3;
      const c = clauses[i3];
      if (c.type === "DefaultClause") {
        if (e != null) {
          replaceNode(e.block, c.block, e);
        } else {
          prev.push(c.block);
        }
        break;
      }
      let { patterns, block } = c;
      let pattern = patterns[0];
      const conditionExpression = flatJoin(patterns.map(($4) => getPatternConditions($4, ref)).map(($5) => flatJoin($5, " && ")), " || ");
      const condition2 = makeNode({
        type: "ParenthesizedExpression",
        children: ["(", ...refAssignmentComma, conditionExpression, ")"],
        expression: conditionExpression
      });
      braceBlock(block);
      block = blockWithPrefix(getPatternBlockPrefix(pattern, ref), block);
      if (i < l - 1) {
        const expressions = [];
        const block2 = makeNode({
          type: "BlockStatement",
          expressions,
          children: [expressions],
          bare: true
        });
        e = makeNode({
          type: "ElseClause",
          block: block2,
          children: ["\n", "else ", block2]
        });
      } else {
        e = void 0;
      }
      prev.push(["", makeNode({
        type: "IfStatement",
        children: ["if", condition2, block, e],
        then: block,
        else: e,
        hoistDec
      })]);
      hoistDec = void 0;
      refAssignmentComma = [];
      if (e != null) {
        prev = e.block.expressions;
      }
    }
    s.type = "PatternMatchingStatement";
    s.children = root;
    return updateParentPointers(s);
  });
}
function getPatternConditions(pattern, ref, conditions = []) {
  if (pattern.rest)
    return conditions;
  switch (pattern.type) {
    case "ArrayBindingPattern": {
      const { elements, length } = pattern, hasRest = elements.some((e) => e.rest), l = (length - +hasRest).toString(), lengthCheck = hasRest ? [ref, ".length >= ", l] : [getHelperRef("len"), "(", ref, ", ", l, ")"];
      conditions.push(
        ["Array.isArray(", ref, ")"],
        lengthCheck
      );
      elements.forEach(({ children: [, e] }, i) => {
        const subRef = [ref, "[", i.toString(), "]"];
        return getPatternConditions(e, subRef, conditions);
      });
      const { blockPrefix } = pattern;
      if (blockPrefix) {
        const postElements = blockPrefix.children[1], { length: postLength } = postElements;
        postElements.forEach(({ children: [, e] }, i) => {
          const subRef = [ref, "[", ref, ".length - ", (postLength + i).toString(), "]"];
          return getPatternConditions(e, subRef, conditions);
        });
      }
      break;
    }
    case "ObjectBindingPattern": {
      conditions.push(
        ["typeof ", ref, " === 'object'"],
        [ref, " != null"]
      );
      pattern.properties.forEach((p) => {
        switch (p.type) {
          case "PinProperty":
          case "BindingProperty": {
            const { name, value } = p;
            let subRef;
            switch (name.type) {
              case "ComputedPropertyName":
                conditions.push([name.expression, " in ", ref]);
                subRef = [ref, name];
                break;
              case "Literal":
              case "StringLiteral":
              case "NumericLiteral":
                conditions.push([name, " in ", ref]);
                subRef = [ref, "[", name, "]"];
                break;
              default:
                conditions.push(["'", name, "' in ", ref]);
                subRef = [ref, ".", name];
            }
            if (value) {
              getPatternConditions(value, subRef, conditions);
            }
            break;
          }
        }
      });
      break;
    }
    case "ConditionFragment": {
      let { children } = pattern;
      if (children.length) {
        let [first, ...rest] = children;
        let [ws, ...op] = first;
        ws = [" "].concat(ws);
        first = [ws, ...op];
        children = [first, ...rest];
      }
      conditions.push(
        processBinaryOpExpression([ref, children])
      );
      break;
    }
    case "RegularExpressionLiteral": {
      conditions.push(
        ["typeof ", ref, " === 'string'"],
        [pattern, ".test(", ref, ")"]
      );
      break;
    }
    case "PinPattern":
      conditions.push([
        ref,
        " === ",
        pattern.expression
      ]);
      break;
    case "Literal":
      conditions.push([
        ref,
        " === ",
        pattern
      ]);
      break;
    default:
      break;
  }
  return conditions;
}
function getPatternBlockPrefix(pattern, ref, decl = "const ", suffix) {
  switch (pattern.type) {
    case "ArrayBindingPattern": {
      if (!pattern.length) {
        return;
      }
      ;
      break;
    }
    case "ObjectBindingPattern": {
      if (!pattern.properties.length) {
        return;
      }
      ;
      break;
    }
    case "Literal":
    case "RegularExpressionLiteral":
    case "PinPattern":
    case "ConditionFragment": {
      return;
    }
  }
  let [splices, thisAssignments] = gatherBindingCode(pattern);
  const patternBindings = nonMatcherBindings(pattern);
  splices = splices.map((s) => [", ", nonMatcherBindings(s)]);
  thisAssignments = thisAssignments.map(($6) => ["", $6, ";"]);
  const duplicateDeclarations = aggregateDuplicateBindings([patternBindings, splices]);
  return [
    ["", {
      type: "Declaration",
      children: [decl, patternBindings, suffix, " = ", ref, ...splices],
      names: [],
      bindings: []
      // avoid implicit return of any bindings
    }, ";"],
    ...thisAssignments,
    ...duplicateDeclarations.map(($7) => ["", $7, ";"])
  ];
}
function elideMatchersFromArrayBindings(elements) {
  return elements.map((el) => {
    if (el.type === "BindingRestElement") {
      return ["", el, void 0];
    }
    const { children: [ws, e, delim] } = el;
    switch (e.type) {
      case "Literal":
      case "RegularExpressionLiteral":
      case "StringLiteral":
      case "PinPattern":
        return delim;
      default:
        return [ws, nonMatcherBindings(e), delim];
    }
  });
}
function elideMatchersFromPropertyBindings(properties) {
  return properties.map((p) => {
    switch (p.type) {
      case "BindingProperty": {
        const { children, name, value } = p;
        const [ws] = children;
        switch (value && value.type) {
          case "ArrayBindingPattern":
          case "ObjectBindingPattern": {
            const bindings = nonMatcherBindings(value);
            return {
              ...p,
              children: [ws, name, bindings && ": ", bindings, p.delim]
            };
          }
          case "Identifier": {
            return p;
          }
          case "Literal":
          case "RegularExpressionLiteral":
          case "StringLiteral":
          default:
            return {
              ...p,
              children: [ws, name, p.delim]
            };
        }
      }
      case "PinProperty":
      case "BindingRestProperty":
      default:
        return p;
    }
  });
}
function nonMatcherBindings(pattern) {
  switch (pattern.type) {
    case "ArrayBindingPattern": {
      const elements = elideMatchersFromArrayBindings(pattern.elements);
      return {
        ...pattern,
        elements,
        children: pattern.children.map(($8) => $8 === pattern.elements ? elements : $8)
      };
    }
    case "PostRestBindingElements": {
      const els = elideMatchersFromArrayBindings(pattern.children[1]);
      return {
        ...pattern,
        children: [
          pattern.children[0],
          els,
          ...pattern.children.slice(2)
        ]
      };
    }
    case "ObjectBindingPattern": {
      const properties = elideMatchersFromPropertyBindings(pattern.properties);
      return {
        ...pattern,
        properties,
        children: pattern.children.map(($9) => $9 === pattern.properties ? properties : $9)
      };
    }
    default: {
      return pattern;
    }
  }
}
function aggregateDuplicateBindings(bindings) {
  const props = gatherRecursiveAll(bindings, ($10) => $10.type === "BindingProperty");
  const arrayBindings = gatherRecursiveAll(bindings, ($11) => $11.type === "ArrayBindingPattern");
  arrayBindings.forEach((a) => {
    const { elements } = a;
    return elements.forEach((element) => {
      if (Array.isArray(element)) {
        const [, e] = element;
        if (e.type === "Identifier") {
          return props.push(e);
        } else if (e.type === "BindingRestElement") {
          return props.push(e);
        }
        ;
        return;
      }
      ;
      return;
    });
  });
  const declarations = [];
  const propsGroupedByName = /* @__PURE__ */ new Map();
  for (const p of props) {
    const { name, value } = p;
    let m;
    if (m = value?.type, m === "ArrayBindingPattern" || m === "ObjectBindingPattern") {
      continue;
    }
    const key = value?.name || name?.name || name;
    if (propsGroupedByName.has(key)) {
      propsGroupedByName.get(key).push(p);
    } else {
      propsGroupedByName.set(key, [p]);
    }
  }
  propsGroupedByName.forEach((shared, key) => {
    if (!key) {
      return;
    }
    if (ReservedWord({ fail() {
    } }, {
      pos: 0,
      input: key
    })) {
      shared.forEach((p) => {
        return aliasBinding(p, makeRef(`_${key}`, key));
      });
      return;
    }
    if (shared.length === 1) {
      return;
    }
    const refs = shared.map((p) => {
      const ref = makeRef(key);
      aliasBinding(p, ref);
      return ref;
    });
    return declarations.push({
      type: "Declaration",
      children: [
        "const ",
        key,
        " = [",
        ...refs.map((r, i) => i === 0 ? r : [", ", r]),
        "]"
      ],
      names: [],
      bindings: []
    });
  });
  return declarations;
}
function aliasBinding(p, ref) {
  if (p.type === "Identifier") {
    p.children[0] = ref;
  } else if (p.type === "BindingRestElement") {
    aliasBinding(p.binding, ref);
  } else if (p.value?.type === "Identifier") {
    aliasBinding(p.value, ref);
  } else {
    p.value = ref;
    const index = p.children.indexOf(p.name);
    p.children.splice(index + 1, 0, ": ", ref);
  }
}

// source/parser/declaration.civet
function len2(arr, length) {
  return arr.length === length;
}
function processAssignmentDeclaration(decl, pattern, suffix, ws, assign, e) {
  decl = {
    ...decl,
    $loc: {
      pos: assign.$loc.pos - 1,
      length: assign.$loc.length + 1
    }
  };
  let [splices, assignments] = gatherBindingCode(pattern);
  splices = splices.map((s) => [", ", s]);
  const thisAssignments = assignments.map((a) => ["", a, ";"]);
  const initializer = makeNode({
    type: "Initializer",
    expression: e,
    children: [ws, assign, e]
  });
  const binding = makeNode({
    type: "Binding",
    pattern,
    initializer,
    splices,
    suffix,
    thisAssignments,
    children: [pattern, suffix, initializer]
  });
  const children = [decl, binding];
  return makeNode({
    type: "Declaration",
    names: pattern.names,
    decl,
    bindings: [binding],
    splices,
    thisAssignments,
    children
  });
}
function processDeclarations(statements) {
  gatherRecursiveAll(statements, ($) => $.type === "Declaration").forEach((statement) => {
    const { bindings } = statement;
    return bindings?.forEach((binding) => {
      const suffix = binding.suffix;
      if (suffix && suffix.optional && suffix.t) {
        convertOptionalType(suffix);
      }
      const { initializer } = binding;
      if (initializer) {
        return prependStatementExpressionBlock(initializer, statement);
      }
      ;
      return;
    });
  });
}
function prependStatementExpressionBlock(initializer, statement) {
  let { expression: exp } = initializer;
  let ws;
  if (Array.isArray(exp)) {
    ws = exp[0];
    exp = exp[1];
  }
  if (!(exp?.type === "StatementExpression")) {
    return;
  }
  const pre = [];
  const statementExp = exp.statement;
  const blockStatement = ["", statementExp];
  let ref;
  if (statementExp.type === "IterationExpression") {
    if (statementExp.async) {
      return;
    }
    const statement2 = statementExp.statement;
    blockStatement[1] = statement2;
    if (statement2.type === "ComptimeStatement") {
      return;
    }
    if (statement2.type === "DoStatement") {
      ref = initializer.expression = initializer.children[2] = makeRef();
      assignResults(blockStatement, (resultNode) => {
        return makeNode({
          type: "AssignmentExpression",
          children: [ref, " = ", resultNode],
          parent: statement2
        });
      });
      const refDec = {
        type: "Declaration",
        children: ["let ", ref, ";"]
      };
      pre.unshift(refDec);
    } else {
      wrapIterationReturningResults(statement2, { children: blockStatement }, function() {
      });
      ref = initializer.expression = initializer.children[2] = statement2.resultsRef;
    }
  } else {
    ref = initializer.expression = initializer.children[2] = makeRef();
    assignResults(blockStatement, (resultNode) => {
      return makeNode({
        type: "AssignmentExpression",
        children: [ref, " = ", resultNode],
        parent: statement
      });
    });
    const refDec = {
      type: "Declaration",
      children: ["let ", ref, ";"]
    };
    pre.unshift(refDec);
    if (ws) {
      pre.push(ws);
    }
  }
  statement.children.unshift(pre, blockStatement, ";");
  updateParentPointers(blockStatement, statement);
  return ref;
}
function processDeclarationCondition(condition, rootCondition, parent) {
  if (!(condition.type === "DeclarationCondition")) {
    return;
  }
  const { decl, bindings } = condition.declaration;
  const binding = bindings[0];
  let { pattern, suffix, initializer } = binding;
  const nullCheck = suffix?.optional && !suffix.t && !suffix.nonnull;
  if (!(initializer != null)) {
    condition.children = [
      {
        type: "Error",
        message: "Missing initializer in declaration condition"
      }
    ];
    return;
  }
  let ref = prependStatementExpressionBlock(initializer, parent);
  if (ref) {
    Object.assign(condition, {
      type: "AssignmentExpression",
      children: [ref],
      pattern,
      ref,
      statementDeclaration: true
    });
  } else {
    const { expression } = initializer;
    ref = maybeRef(expression);
    const simple = ref === expression;
    let children;
    if (simple) {
      ref = insertTrimmingSpace(ref, "");
      children = [ref];
    } else {
      children = [ref, initializer];
      const grandparent = condition.parent?.parent;
      if (pattern.type === "Identifier" && (grandparent?.type === "IfStatement" || grandparent?.type === "IterationStatement") && !nullCheck) {
        children.unshift("(");
        children.push(")");
      }
    }
    if (nullCheck) {
      children.unshift("(");
      children.push(") != null");
      suffix = void 0;
    }
    Object.assign(condition, {
      type: "AssignmentExpression",
      children,
      hoistDec: !simple ? {
        type: "Declaration",
        children: ["let ", ref, suffix],
        names: []
      } : void 0,
      pattern,
      ref
    });
  }
  updateParentPointers(condition, parent);
  rootCondition.blockPrefix = getPatternBlockPrefix(pattern, ref, decl, suffix);
}
function processDeclarationConditions(node) {
  gatherRecursiveAll(
    node,
    (n) => {
      return n.type === "IfStatement" || n.type === "IterationStatement" || n.type === "SwitchStatement";
    }
  ).forEach((s) => {
    return processDeclarationConditionStatement(s);
  });
}
function processDeclarationConditionStatement(s) {
  const { condition } = s;
  if (!condition?.expression) {
    return;
  }
  let { expression } = condition;
  if (expression && typeof expression === "object" && "type" in expression && expression.type === "UnaryExpression" && "children" in expression && Array.isArray(expression.children) && len2(expression.children, 2) && expression.children[0] === "!" && typeof expression.children[1] === "object" && expression.children[1] != null && "type" in expression.children[1] && expression.children[1].type === "ParenthesizedExpression" && "expression" in expression.children[1]) {
    const { type: type1, children: [, { type: type2, expression: expression2 }] } = expression;
    const type = [type1, type2];
    expression = expression2;
  }
  processDeclarationCondition(expression, condition.expression, s);
  const { ref, pattern } = expression;
  if (pattern) {
    const conditions = getPatternConditions(pattern, ref).filter((c) => {
      if (Array.isArray(c) && len2(c, 2) && c[0] === ref && c[1] === " != null") {
        const [,] = c;
        return false;
      } else {
        return true;
      }
    });
    if (conditions.length) {
      if (s.negated) {
        let m;
        if (!(m = condition.expression, typeof m === "object" && m != null && "type" in m && m.type === "UnaryExpression" && "children" in m && Array.isArray(m.children) && len2(m.children, 2) && m.children[0] === "!" && typeof m.children[1] === "object" && m.children[1] != null && "type" in m.children[1] && m.children[1].type === "ParenthesizedExpression")) {
          throw new Error("Unsupported negated condition");
        }
        const { children } = condition.expression.children[1];
        const close = children.pop();
        conditions.forEach((c) => {
          return children.push(" && ", c);
        });
        children.push(close);
      } else {
        condition.children.unshift("(");
        conditions.forEach((c) => {
          return condition.children.push(" && ", c);
        });
        condition.children.push(")");
      }
    }
  }
  const { blockPrefix } = condition.expression;
  if (s.negated && blockPrefix && (s.type === "IfStatement" && isExit(s.then) || s.type === "IterationStatement")) {
    const { ancestor, child } = findAncestor(
      s,
      (a) => a.type === "BlockStatement"
    );
    if (!(ancestor != null)) {
      throw new Error("Couldn't find block for postfix declaration");
    }
    const index = findChildIndex(ancestor.expressions, child);
    if (index < 0) {
      throw new Error("Couldn't find where in block to put postfix declaration");
    }
    ancestor.expressions.splice(index + 1, 0, ...blockPrefix);
    updateParentPointers(ancestor);
    braceBlock(ancestor);
    let ref1;
    switch (s.type) {
      case "IfStatement": {
        if (ref1 = s.else?.block) {
          const elseBlock = ref1;
          if (elseBlock.bare && !elseBlock.semicolon) {
            elseBlock.children.push(elseBlock.semicolon = ";");
          }
          ancestor.expressions.splice(index + 1 + blockPrefix.length, 0, ["", elseBlock]);
          s.children = s.children.filter((a1) => a1 !== s.else);
          s.else = void 0;
        }
        const block = s.then;
        if (block.bare && !block.semicolon) {
          block.children.push(block.semicolon = ";");
        }
        ;
        break;
      }
    }
    return;
  }
  switch (s.type) {
    case "IfStatement": {
      const { else: e } = s;
      if (s.negated) {
        if (e != null) {
          const block = blockWithPrefix(blockPrefix, e.block);
          e.children = e.children.map(($1) => $1 === e.block ? block : $1);
          e.block = block;
          updateParentPointers(e);
        }
      } else {
        const block = blockWithPrefix(blockPrefix, s.then);
        if (block.bare && e && !block.semicolon) {
          block.children.push(block.semicolon = ";");
        }
        s.children = s.children.map(($2) => $2 === s.then ? block : $2);
        s.then = block;
        updateParentPointers(s);
      }
      ;
      break;
    }
    case "IterationStatement": {
      if (!blockPrefix) {
        return;
      }
      const { children, block } = s;
      const newBlock = blockWithPrefix(blockPrefix, block);
      s.children = children.map(($3) => $3 === block ? newBlock : $3);
      updateParentPointers(s);
      break;
    }
    case "SwitchStatement": {
      const { ref: ref2, statementDeclaration } = condition.expression;
      if (!blockPrefix) {
        return;
      }
      const newCondition = {
        type: "ParenthesizedExpression",
        children: ["(", ref2, ")"],
        expression: ref2,
        parent: s
      };
      s.children = s.children.map(function(c) {
        if (c === s.condition) {
          return newCondition;
        } else {
          return c;
        }
      });
      s.condition = newCondition;
      updateParentPointers(s);
      if (statementDeclaration) {
        const block = makeEmptyBlock();
        replaceBlockExpression(s.parent, s, block);
        block.expressions.push(["", s]);
        s.children.splice(s.children.findIndex(($4) => $4.token === "switch"), 0, blockPrefix);
        s.parent = block;
      } else {
        const block = blockWithPrefix([["", [{
          type: "Declaration",
          children: ["let ", ...condition.expression.children]
        }], ";"], ...blockPrefix], makeEmptyBlock());
        updateParentPointers(block, s.parent);
        replaceBlockExpression(s.parent, s, block);
        block.expressions.push(["", s]);
        s.parent = block;
      }
      ;
      break;
    }
  }
}
function dynamizeFromClause(from) {
  from = from.slice(1);
  from = insertTrimmingSpace(from, "");
  if (from[from.length - 1]?.type === "ImportAssertion") {
    const assert2 = from.pop();
    from.push(", {", assert2.keyword, ":", assert2.object, "}");
  }
  return ["(", ...from, ")"];
}
function dynamizeImportDeclaration(decl) {
  const { imports } = decl;
  let { star, binding, specifiers } = imports;
  const justDefault = binding && !specifiers && !star;
  let ref2;
  {
    if (binding) {
      if (specifiers) {
        ref2 = makeRef();
      } else {
        ref2 = binding;
      }
    } else {
      ref2 = convertNamedImportsToObject(imports, true);
    }
  }
  ;
  const pattern = ref2;
  const c = "const";
  const expression = [
    justDefault ? "(" : void 0,
    { type: "Await", children: ["await"] },
    " ",
    decl.children[0],
    // import
    dynamizeFromClause(decl.from),
    justDefault ? ").default" : void 0
  ];
  const initializer = {
    type: "Initializer",
    expression,
    children: [" ", "= ", expression]
  };
  const bindings = [{
    type: "Binding",
    names: pattern.names,
    pattern,
    initializer,
    children: [pattern, initializer]
  }];
  if (binding && specifiers) {
    const pattern2 = binding;
    const exp2 = [
      pattern,
      ".default"
    ];
    const initializer2 = {
      type: "Initializer",
      expression,
      children: [" ", "= ", exp2]
    };
    bindings.push({
      type: "Binding",
      names: binding.names,
      pattern: pattern2,
      initializer: initializer2,
      children: [pattern2, initializer2]
    });
    const pattern3 = convertNamedImportsToObject(imports.children.at(-1), true);
    const initializer3 = {
      type: "Initializer",
      expression: pattern,
      children: [" ", "= ", pattern]
    };
    bindings.push({
      type: "Binding",
      names: specifiers.names,
      pattern: pattern3,
      initializer: initializer3,
      children: [pattern3, initializer3]
    });
  }
  return {
    type: "Declaration",
    names: imports.names,
    bindings,
    decl: c,
    children: [
      c,
      " ",
      bindings.flatMap((binding2, i) => i > 0 ? [", ", binding2] : [binding2])
    ]
  };
}
function dynamizeImportDeclarationExpression($0) {
  const [imp, ws1, named, ws2, from] = $0;
  const object = convertNamedImportsToObject(named);
  const dot = ".";
  return processCallMemberExpression({
    type: "CallExpression",
    children: [
      { type: "Await", children: "await" },
      " ",
      imp,
      insertTrimmingSpace(ws2, ""),
      dynamizeFromClause(from),
      {
        type: "PropertyGlob",
        dot,
        object,
        children: [ws1, dot, object],
        reversed: true
      }
    ]
  });
}
function convertWithClause(withClause, extendsClause) {
  let extendsToken, extendsTarget, ws;
  if (extendsClause) {
    [extendsToken, ws, extendsTarget] = extendsClause;
  } else {
    extendsToken = {
      type: "Extends",
      children: [" extends"]
    };
    ws = "";
    extendsTarget = "Object";
  }
  const wrapped = withClause.targets.reduce(
    (extendsTarget2, [wsNext, withTarget]) => {
      const args = [extendsTarget2];
      const exp = {
        type: "CallExpression",
        children: [
          makeLeftHandSideExpression(withTarget),
          {
            type: "Call",
            args,
            children: ["(", trimFirstSpace(ws), args, ")"]
          }
        ]
      };
      ws = wsNext;
      return exp;
    },
    extendsTarget
  );
  return [extendsToken, insertTrimmingSpace(ws, " "), wrapped];
}

// source/parser/unary.civet
function processUnaryExpression(pre, exp, post) {
  if (!(pre.length || post))
    return exp;
  if (post?.token === "?") {
    post = {
      $loc: post.$loc,
      token: " != null"
    };
    if (pre.length) {
      const lastPre = pre[pre.length - 1];
      if (lastPre.token === "!") {
        post.token = " == null";
        pre = pre.slice(0, -1);
      } else if (lastPre.length === 2 && lastPre[0].token === "!") {
        post.token = " == null";
        pre = pre.slice(0, -1);
      }
    }
    const existence = {
      type: "Existence",
      expression: exp,
      children: [exp, post],
      parent: void 0
    };
    exp = makeLeftHandSideExpression(existence);
    if (pre.length) {
      return {
        type: "UnaryExpression",
        children: [...pre, exp]
      };
    }
    return exp;
  }
  if (exp.type === "Literal") {
    if (pre.length === 1) {
      const { token } = pre[0];
      if (token === "-" || token === "+") {
        const children = [pre[0], ...exp.children];
        const literal = {
          type: "Literal",
          children,
          raw: `${token}${exp.raw}`
        };
        if (post) {
          return {
            type: "UnaryExpression",
            children: [literal, post]
          };
        }
        return literal;
      }
    }
  }
  let ref;
  while (ref = pre.length) {
    const l = ref;
    const last = pre[l - 1];
    if (last.type === "Await") {
      if (last.op) {
        if (exp.type !== "ParenthesizedExpression") {
          exp = ["(", exp, ")"];
        }
        exp = {
          type: "CallExpression",
          children: [...last.children, "Promise", last.op, exp]
        };
        pre = pre.slice(0, -1);
      } else {
        exp = {
          type: "AwaitExpression",
          children: [...last.children, exp]
        };
        pre = pre.slice(0, -1);
      }
    } else {
      break;
    }
  }
  return {
    type: "UnaryExpression",
    children: [...pre, exp, post]
  };
}

// source/parser/pipe.civet
function constructInvocation(fn, arg) {
  const fnArr = [fn.leadingComment, fn.expr, fn.trailingComment];
  let expr = fn.expr;
  while (expr.type === "ParenthesizedExpression") {
    expr = expr.expression;
  }
  if (expr.ampersandBlock) {
    const { ref, body } = expr;
    ref.type = "PipedExpression";
    ref.children = [makeLeftHandSideExpression(arg)];
    updateParentPointers(ref);
    return makeNode({
      type: "UnwrappedExpression",
      children: [skipIfOnlyWS(fn.leadingComment), body, skipIfOnlyWS(fn.trailingComment)]
    });
  }
  expr = fn.expr;
  const lhs = makeLeftHandSideExpression(expr);
  let comment = skipIfOnlyWS(fn.trailingComment);
  if (comment)
    lhs.children.splice(2, 0, comment);
  comment = skipIfOnlyWS(fn.leadingComment);
  if (comment)
    lhs.children.splice(1, 0, comment);
  switch (arg.type) {
    case "CommaExpression":
      arg = makeLeftHandSideExpression(arg);
      break;
  }
  return {
    type: "CallExpression",
    children: [lhs, "(", arg, ")"]
  };
}
function constructPipeStep(fn, arg, returning) {
  let children = [[fn.leadingComment, fn.expr, fn.trailingComment].map(skipIfOnlyWS), " ", arg];
  switch (fn.expr.token) {
    case "yield":
    case "await":
      if (fn.expr.op) {
        children = processUnaryExpression([fn.expr], arg, void 0);
      }
      if (returning) {
        return [
          children,
          returning
        ];
      }
      return [
        children,
        null
      ];
    case "return":
      return [{
        type: "ReturnStatement",
        children
      }, null];
  }
  if (returning) {
    return [
      constructInvocation(fn, arg),
      returning
    ];
  }
  return [constructInvocation(fn, arg), null];
}
function processPipelineExpressions(statements) {
  gatherRecursiveAll(statements, (n) => n.type === "PipelineExpression").forEach((s) => {
    const [ws, , body] = s.children;
    let [, arg] = s.children;
    let i = 0, l = body.length;
    const children = [ws];
    let usingRef = null;
    for (i = 0; i < l; i++) {
      const step = body[i];
      const [leadingComment, pipe, trailingComment, expr] = step;
      const returns = pipe.token === "||>";
      let ref, result, returning = returns ? arg : null;
      if (pipe.token === "|>=") {
        let initRef;
        if (i === 0) {
          outer:
            switch (arg.type) {
              case "MemberExpression":
                if (arg.children.length <= 2)
                  break;
              case "CallExpression":
                const access = arg.children.pop();
                switch (access.type) {
                  case "PropertyAccess":
                  case "SliceExpression":
                  case "Index":
                    break;
                  default:
                    children.unshift({
                      type: "Error",
                      $loc: pipe.token.$loc,
                      message: `Can't assign to ${access.type}`
                    });
                    arg.children.push(access);
                    break outer;
                }
                usingRef = makeRef();
                initRef = {
                  type: "AssignmentExpression",
                  children: [usingRef, " = ", arg, ","]
                };
                arg = {
                  type: "MemberExpression",
                  children: [usingRef, access]
                };
                break;
            }
          const lhs = [[
            [initRef],
            arg,
            [],
            { token: "=", children: [" = "] }
          ]];
          Object.assign(s, {
            type: "AssignmentExpression",
            children: [lhs, children],
            names: null,
            lhs,
            assigned: arg,
            expression: children
          });
          arg = clone(arg);
          removeHoistDecs(arg);
          if (arg.children[0].type === "Ref") {
            arg.children[0] = usingRef;
          }
        } else {
          children.unshift({
            type: "Error",
            $loc: pipe.token.$loc,
            message: "Can't use |>= in the middle of a pipeline"
          });
        }
      } else {
        if (i === 0)
          s.children = children;
      }
      if (returns && (ref = needsRef(arg))) {
        usingRef = usingRef || ref;
        arg = {
          type: "ParenthesizedExpression",
          children: ["(", {
            type: "AssignmentExpression",
            children: [usingRef, " = ", arg]
          }, ")"]
        };
        returning = usingRef;
      }
      ;
      [result, returning] = constructPipeStep(
        {
          leadingComment: skipIfOnlyWS(leadingComment),
          trailingComment: skipIfOnlyWS(trailingComment),
          expr
        },
        arg,
        returning
      );
      if (result.type === "ReturnStatement") {
        if (i < l - 1) {
          result.children.push({
            type: "Error",
            message: "Can't continue a pipeline after returning"
          });
        }
        arg = result;
        if (children[children.length - 1] === ",") {
          children.pop();
          children.push(";");
        }
        break;
      }
      if (returning) {
        arg = returning;
        children.push(result, ",");
      } else {
        arg = result;
      }
    }
    if (usingRef) {
      s.hoistDec = {
        type: "Declaration",
        children: ["let ", usingRef],
        names: []
      };
    }
    children.push(arg);
    if (!children.some(($) => $?.type === "ReturnStatement") && children.some(($1) => $1 === ",")) {
      const { parent } = s;
      const parenthesizedExpression = makeLeftHandSideExpression({ ...s });
      Object.assign(s, parenthesizedExpression, {
        parent,
        hoistDec: void 0
      });
    }
    return addParentPointers(s, s.parent);
  });
}

// source/parser/for.civet
function forRange(open, forDeclaration, range, stepExp, close) {
  const { start, end, inclusive } = range;
  const counterRef = makeRef("i");
  const infinite = end.type === "Identifier" && end.name === "Infinity";
  let stepRef;
  if (stepExp) {
    stepExp = insertTrimmingSpace(stepExp, "");
    stepRef = maybeRef(stepExp, "step");
  } else if (infinite) {
    stepExp = stepRef = "1";
  }
  let startRef = maybeRef(start, "start");
  let endRef = maybeRef(end, "end");
  const startRefDec = startRef !== start ? [startRef, " = ", start, ", "] : [];
  const endRefDec = endRef !== end ? [endRef, " = ", end, ", "] : [];
  let ascDec = [], ascRef, asc;
  if (stepRef) {
    if (!(stepRef === stepExp)) {
      ascDec = [", ", stepRef, " = ", stepExp];
    }
  } else if ("Literal" === start.type && start.type === end.type) {
    asc = literalValue(start) <= literalValue(end);
    if ("StringLiteral" === start.subtype && start.subtype === end.subtype) {
      startRef = literalValue(start).charCodeAt(0).toString();
      endRef = literalValue(end).charCodeAt(0).toString();
    }
  } else {
    ascRef = makeRef("asc");
    ascDec = [", ", ascRef, " = ", startRef, " <= ", endRef];
  }
  let varAssign = [], varLetAssign = varAssign, varLet = varAssign, blockPrefix;
  if (forDeclaration?.declare) {
    if (forDeclaration.declare.token === "let") {
      const varName = forDeclaration.children.splice(1);
      varAssign = [...insertTrimmingSpace(varName, ""), " = "];
      varLet = [",", ...varName, " = ", counterRef];
    } else {
      const value = "StringLiteral" === start.subtype ? ["String.fromCharCode(", counterRef, ")"] : counterRef;
      blockPrefix = [
        ["", [forDeclaration, " = ", value], ";"]
      ];
    }
  } else if (forDeclaration) {
    varAssign = varLetAssign = [forDeclaration, " = "];
  }
  const declaration = {
    type: "Declaration",
    children: ["let ", ...startRefDec, ...endRefDec, counterRef, " = ", ...varLetAssign, startRef, ...varLet, ...ascDec],
    names: forDeclaration?.names
  };
  const counterPart = inclusive ? [counterRef, " <= ", endRef, " : ", counterRef, " >= ", endRef] : [counterRef, " < ", endRef, " : ", counterRef, " > ", endRef];
  const condition = infinite ? [] : stepRef ? [stepRef, " !== 0 && (", stepRef, " > 0 ? ", ...counterPart, ")"] : ascRef ? [ascRef, " ? ", ...counterPart] : asc ? counterPart.slice(0, 3) : counterPart.slice(4);
  const increment = infinite ? [...varAssign, "++", counterRef] : stepRef ? [...varAssign, counterRef, " += ", stepRef] : ascRef ? [...varAssign, ascRef, " ? ++", counterRef, " : --", counterRef] : [...varAssign, asc ? "++" : "--", counterRef];
  return {
    declaration,
    children: [open, declaration, "; ", ...condition, "; ", ...increment, close],
    blockPrefix
  };
}
function processForInOf($0, getRef) {
  let [awaits, eachOwn, open, declaration, declaration2, ws, inOf, exp, step, close] = $0;
  if (exp.type === "RangeExpression" && inOf.token === "of" && !declaration2) {
    return forRange(open, declaration, exp, step, close);
  } else if (step) {
    throw new Error("for..of/in cannot use 'by' except with range literals");
  }
  let eachOwnError;
  let hoistDec, blockPrefix = [];
  if (eachOwn && eachOwn[0].token === "each") {
    if (inOf.token === "of") {
      const counterRef = makeRef("i");
      const lenRef = makeRef("len");
      const expRef2 = maybeRef(exp);
      const increment = "++";
      let assignmentNames = [...declaration.names];
      if (declaration2) {
        const [, , ws22, decl22] = declaration2;
        blockPrefix.push(["", [
          insertTrimmingSpace(ws22, ""),
          decl22,
          " = ",
          counterRef
        ], ";"]);
        assignmentNames.push(...decl22.names);
      }
      const expRefDec = expRef2 !== exp ? [insertTrimmingSpace(expRef2, " "), " = ", insertTrimmingSpace(exp, ""), ", "] : [];
      blockPrefix.push(["", {
        type: "Declaration",
        children: [declaration, " = ", insertTrimmingSpace(expRef2, ""), "[", counterRef, "]"],
        names: assignmentNames
      }, ";"]);
      declaration = {
        type: "Declaration",
        children: ["let ", ...expRefDec, counterRef, " = 0, ", lenRef, " = ", insertTrimmingSpace(expRef2, ""), ".length"],
        names: []
      };
      const condition = [counterRef, " < ", lenRef, "; "];
      const children = [open, declaration, "; ", condition, counterRef, increment, close];
      return { declaration, children, blockPrefix };
    } else {
      eachOwnError = {
        type: "Error",
        message: "'each' is only meaningful in for..of loops"
      };
    }
  }
  let own = eachOwn && eachOwn[0].token === "own";
  let expRef;
  if (own && inOf.token !== "in") {
    own = false;
    eachOwnError = {
      type: "Error",
      message: "'own' is only meaningful in for..in loops"
    };
  }
  if (!declaration2 && !own) {
    return {
      declaration,
      blockPrefix,
      children: [awaits, eachOwnError, open, declaration, ws, inOf, expRef ?? exp, step, close]
      // omit declaration2, replace eachOwn with eachOwnError, replace exp with expRef
    };
  }
  let ws2, decl2;
  if (declaration2)
    [, , ws2, decl2] = declaration2;
  switch (inOf.token) {
    case "of": {
      const counterRef = makeRef("i");
      hoistDec = {
        type: "Declaration",
        children: ["let ", counterRef, " = 0"],
        names: []
      };
      blockPrefix.push(["", {
        type: "Declaration",
        children: [insertTrimmingSpace(ws2, ""), decl2, " = ", counterRef, "++"],
        names: decl2.names
      }, ";"]);
      break;
    }
    case "in": {
      const expRef2 = maybeRef(exp);
      if (expRef2 !== exp) {
        hoistDec = {
          type: "Declaration",
          children: ["let ", expRef2],
          names: []
        };
        exp = {
          type: "AssignmentExpression",
          children: [" ", expRef2, " =", exp]
        };
      }
      let { binding } = declaration;
      if (binding?.type !== "Identifier") {
        const keyRef = makeRef("key");
        blockPrefix.push(["", [
          declaration,
          " = ",
          keyRef
        ], ";"]);
        declaration = {
          type: "ForDeclaration",
          binding: binding = keyRef,
          children: ["const ", keyRef],
          names: []
        };
      }
      if (own) {
        const hasPropRef = getRef("hasProp");
        blockPrefix.push(["", ["if (!", hasPropRef, "(", insertTrimmingSpace(expRef2, ""), ", ", insertTrimmingSpace(binding, ""), ")) continue"], ";"]);
      }
      if (decl2) {
        blockPrefix.push(["", {
          type: "Declaration",
          children: [insertTrimmingSpace(ws2, ""), decl2, " = ", insertTrimmingSpace(expRef2, ""), "[", insertTrimmingSpace(binding, ""), "]"],
          names: decl2.names
        }, ";"]);
      }
      break;
    }
    default:
      throw new Error(`for item, index must use 'of' or 'in' instead of '${inOf.token}'`);
  }
  return {
    declaration,
    children: [awaits, eachOwnError, open, declaration, ws, inOf, exp, step, close],
    // omit declaration2, replace each with eachOwnError
    blockPrefix,
    hoistDec
  };
}

// source/parser/auto-dec.civet
function findDecs(statements) {
  const predicate = ($) => $.type === "Declaration";
  const declarations = gatherNodes(statements, predicate);
  const declarationNames = declarations.flatMap((d) => d.names);
  return new Set(declarationNames);
}
function createConstLetDecs(statements, scopes, letOrConst) {
  function findVarDecs(statements2, decs) {
    const declarationNames = gatherRecursive(statements2, (node) => {
      return node.type === "Declaration" && node.children && node.children.length > 0 && node.children[0].token && node.children[0].token.startsWith("var") || node.type === "FunctionExpression";
    }).filter((node) => node.type === "Declaration").flatMap((node) => node.names);
    return new Set(declarationNames);
  }
  let declaredIdentifiers = findVarDecs(statements);
  function hasDec(name) {
    return declaredIdentifiers.has(name) || scopes.some((s) => s.has(name));
  }
  function gatherBlockOrOther(statement) {
    return gatherNodes(statement, (s) => s.type === "BlockStatement" || s.type === "AssignmentExpression" || s.type === "Declaration").flatMap((node) => {
      if (node.type == "BlockStatement") {
        return node.bare ? gatherBlockOrOther(node.expressions) : node;
      } else if (node.children && node.children.length) {
        return [...gatherBlockOrOther(node.children), node];
      } else {
        return [];
      }
    });
  }
  let currentScope = /* @__PURE__ */ new Set();
  scopes.push(currentScope);
  const fnNodes = gatherNodes(statements, isFunction);
  const forNodes = gatherNodes(statements, (s) => s.type === "ForStatement");
  let targetStatements = [];
  for (const statement of statements) {
    const nodes = gatherBlockOrOther(statement);
    let undeclaredIdentifiers = [];
    for (const node of nodes) {
      if (node.type == "BlockStatement") {
        let block = node;
        let fnNode = fnNodes.find((fnNode2) => fnNode2.block === block);
        let forNode = forNodes.find((forNode2) => forNode2.block === block);
        if (fnNode != null) {
          scopes.push(new Set(fnNode.parameters.names));
          createConstLetDecs(block.expressions, scopes, letOrConst);
          scopes.pop();
        } else if (forNode != null) {
          scopes.push(new Set(forNode.declaration.names));
          createConstLetDecs(block.expressions, scopes, letOrConst);
          scopes.pop();
        } else {
          createConstLetDecs(block.expressions, scopes, letOrConst);
        }
        continue;
      }
      if (node.names == null)
        continue;
      let names = node.names.filter((name) => !hasDec(name));
      if (node.type == "AssignmentExpression") {
        undeclaredIdentifiers.push(...names);
      }
      names.forEach((name) => currentScope.add(name));
    }
    if (undeclaredIdentifiers.length > 0) {
      let indent = statement[0];
      let firstIdentifier = gatherNodes(statement[1], (node) => node.type == "Identifier")[0];
      if (undeclaredIdentifiers.length == 1 && statement[1].type == "AssignmentExpression" && statement[1].names.length == 1 && statement[1].names[0] == undeclaredIdentifiers[0] && firstIdentifier && firstIdentifier.names == undeclaredIdentifiers[0] && gatherNodes(statement[1], (node) => node.type === "ObjectBindingPattern").length == 0) {
        statement[1].children.unshift([`${letOrConst} `]);
      } else {
        let tail = "\n";
        if (gatherNodes(indent, (node) => node.token && node.token.endsWith("\n")).length > 0) {
          tail = void 0;
        }
        targetStatements.push([indent, {
          type: "Declaration",
          children: ["let ", ...undeclaredIdentifiers.join(", ")],
          names: undeclaredIdentifiers
        }, tail]);
      }
    }
    targetStatements.push(statement);
  }
  scopes.pop();
  statements.splice(0, statements.length, ...targetStatements);
}
function createVarDecs(block, scopes, pushVar) {
  function hasDec(name) {
    return scopes.some((s) => s.has(name));
  }
  function findAssignments(statements2, decs2) {
    let assignmentStatements2 = gatherNodes(statements2, (node) => {
      return node.type === "AssignmentExpression";
    });
    if (assignmentStatements2.length) {
      assignmentStatements2 = assignmentStatements2.concat(findAssignments(assignmentStatements2.map((s) => s.children), decs2));
    }
    return assignmentStatements2;
  }
  if (!pushVar) {
    pushVar = function(name) {
      varIds.push(name);
      return decs.add(name);
    };
  }
  const { expressions: statements } = block;
  const decs = findDecs(statements);
  scopes.push(decs);
  const varIds = [];
  const assignmentStatements = findAssignments(statements, scopes);
  const undeclaredIdentifiers = assignmentStatements.flatMap(($1) => $1?.names || []);
  undeclaredIdentifiers.filter((x, i, a) => {
    if (!hasDec(x))
      return a.indexOf(x) === i;
    return;
  }).forEach(pushVar);
  const fnNodes = gatherNodes(statements, isFunction);
  const forNodes = gatherNodes(statements, (s) => s.type === "ForStatement");
  const blockNodes = new Set(gatherNodes(statements, (s) => s.type === "BlockStatement"));
  fnNodes.forEach(({ block: block2 }) => blockNodes.delete(block2));
  forNodes.forEach(({ block: block2 }) => blockNodes.delete(block2));
  blockNodes.forEach((block2) => {
    return createVarDecs(block2, scopes, pushVar);
  });
  forNodes.forEach(({ block: block2, declaration }) => {
    scopes.push(new Set(declaration.names));
    createVarDecs(block2, scopes, pushVar);
    return scopes.pop();
  });
  fnNodes.forEach(({ block: block2, parameters }) => {
    scopes.push(new Set(parameters.names));
    createVarDecs(block2, scopes);
    return scopes.pop();
  });
  if (varIds.length) {
    const indent = getIndent(statements[0]);
    let delimiter = ";";
    if (statements[0][1]?.parent?.root) {
      delimiter = ";\n";
    }
    braceBlock(block);
    statements.unshift([indent, {
      type: "Declaration",
      children: ["var ", varIds.join(", ")]
    }, delimiter]);
  }
  scopes.pop();
}

// source/parser/comptime.civet
var import_node_path = {};
var import_node_module = {};
var import_node_vm = {};

// source/generate.civet
function stringify(node) {
  try {
    return JSON.stringify(removeParentPointers(node));
  } catch (e) {
    return `${node}`;
  }
}
function gen(root, options) {
  const updateSourceMap = options?.sourceMap?.updateSourceMap;
  return recurse(root);
  function recurse(node) {
    if (!(node != null)) {
      return "";
    }
    if (typeof node === "string") {
      updateSourceMap?.(node);
      return node;
    }
    if (Array.isArray(node)) {
      return node.map(recurse).join("");
    }
    if (typeof node === "object") {
      if (options.js && node.ts) {
        return "";
      }
      if (!options.js && node.js) {
        return "";
      }
      if (node.type === "Error") {
        const filename2 = options?.filename ?? "unknown";
        let line = "?";
        let column = "?";
        let offset;
        if (options && typeof options === "object" && "sourceMap" in options) {
          const { sourceMap } = options;
          line = sourceMap.data.srcLine + 1;
          column = sourceMap.data.srcColumn + 1;
          offset = sourceMap.data.srcOffset;
        }
        options.errors ??= [];
        options.errors.push(new import_lib3.ParseError(
          node.message,
          void 0,
          // body
          filename2,
          line,
          column,
          offset
        ));
        return "";
      }
      if (node.$loc != null) {
        const { token, $loc } = node;
        updateSourceMap?.(token, $loc.pos);
        return token;
      }
      if (!node.children) {
        switch (node.type) {
          case "Ref": {
            throw new Error(`Unpopulated ref ${stringify(node)}`);
          }
        }
        throw new Error(`Unknown node ${stringify(node)}`);
      }
      return recurse(node.children);
    }
    throw new Error(`Unknown node ${stringify(node)}`);
  }
}
var generate_default = gen;
function prune(node) {
  if (!(node != null)) {
    return;
  }
  if (typeof node === "string" && node.length === 0) {
    return;
  }
  if (node.parent != null) {
    delete node.parent;
  }
  if (Array.isArray(node)) {
    const a = node.map(prune).filter(($) => $);
    if (a.length > 1) {
      return a;
    }
    if (a.length === 1) {
      return a[0];
    }
    return;
  }
  if (node.children != null) {
    node.children = prune(node.children) || [];
    return node;
  }
  return node;
}

// source/parser/comptime.civet
function processComptime(statements) {
  if (!getInitialConfig()?.comptime) {
    return;
  }
  const promises = runComptime(statements);
  if (getSync()) {
    return;
  } else {
    return (async () => {
      {
        await Promise.all(promises);
      }
    })();
  }
}
function runComptime(statements) {
  const sync2 = getSync();
  return gatherRecursive(
    statements,
    (node) => {
      return node.type === "ComptimeStatement" || node.type === "ComptimeExpression";
    }
  ).map((exp) => {
    let content = exp.type === "ComptimeStatement" ? exp.block : exp.expression;
    content = [
      ...extractPreludeFor(content),
      content
    ];
    const options = { js: true };
    let js = generate_default(prune(content), options);
    js = `"use strict";${js}`;
    if (options.errors != null) {
      return;
    }
    let output, context, contextGlobal;
    try {
      context = import_node_vm.default.createContext?.() ?? globalThis;
      const filename2 = context.__filename = (0, import_node_path.resolve)(getFilename() ?? "");
      context.__dirname = (0, import_node_path.dirname)(filename2);
      context.require = (0, import_node_module.createRequire)(filename2);
      if (import_node_vm.default.runInContext != null) {
        contextGlobal = import_node_vm.default.runInContext("globalThis", context);
        const builtins = new Set(Object.getOwnPropertyNames(contextGlobal));
        for (const name of Object.getOwnPropertyNames(globalThis)) {
          if (builtins.has(name)) {
            continue;
          }
          Object.defineProperty(contextGlobal, name, {
            __proto__: null,
            ...Object.getOwnPropertyDescriptor(globalThis, name)
          });
        }
        output = import_node_vm.default.runInContext(js, context, {
          filename: filename2,
          importModuleDynamically: import_node_vm.default.constants?.USE_MAIN_CONTEXT_DEFAULT_LOADER
        });
      } else {
        output = eval?.(js);
      }
    } catch (e) {
      exp.children = [
        {
          type: "Error",
          message: `comptime block failed to execute: ${e}
${js}`
        }
      ];
      return;
    }
    let promise;
    if (exp.type === "ComptimeExpression") {
      const finish = () => {
        let string;
        try {
          string = serialize(output, contextGlobal);
        } catch (e) {
          exp.children = [
            {
              type: "Error",
              message: `comptime result ${output} not serializable: ${e}`
            }
          ];
          return;
        }
        return exp.children = [string];
      };
      if (sync2) {
        finish();
      } else {
        promise = (async () => {
          {
            output = await output;
            return finish();
          }
        })();
      }
    } else {
      promise = output;
      exp.children = [];
    }
    return promise;
  });
}
function serialize(value, context) {
  const stack = /* @__PURE__ */ new Set();
  function recurse(val) {
    if (typeof val === "string") {
      return JSON.stringify(val);
    } else if (typeof val === "number") {
      if (Object.is(-0, val))
        return "-0";
      else
        return val.toString();
    } else if (typeof val === "boolean" || val == null) {
      return String(val);
    } else if (typeof val === "bigint") {
      return `${val}n`;
    } else if (typeof val === "function") {
      let string = Function.prototype.toString.call(val);
      if (/\{\s+\[native code]\s+\}$/.test(string)) {
        throw new TypeError("cannot serialize native function");
      }
      if (/^class[\s{]/u.test(string)) {
        return string;
      }
      if (stack.has(val)) {
        throw new Error("circular reference detected");
      }
      stack.add(val);
      if (/^(?:async\s*)?(?:\*\s*|[gs]et\s+)?\[/.test(string)) {
        throw new Error("cannot serialize method with computed name");
      }
      const protoHasProps = !(val.prototype === void 0 || Object.prototype === Object.getPrototypeOf(val.prototype) && Object.getOwnPropertyNames(val.prototype).length <= 1 && // constructor
      Object.getOwnPropertySymbols(val.prototype).length === 0 && [val, void 0].includes(val.prototype.constructor));
      const isGenerator = /^(?:async\s*)?(?:function\s*)?\*/u.test(string);
      if (protoHasProps && !isGenerator) {
        throw new TypeError("cannot serialize function with modified prototype");
      }
      if (!/^(?:async\s+)?(?:(function|class)(?!\p{ID_Continue})|\(|(?:\p{ID_Start}|[_$])(?:\p{ID_Continue}|[\u200C\u200D$])*\s*=>)/u.test(string)) {
        string = string.replace(/^(async\s+)?(?:[gs]et\s+(?=\p{ID_Start}))?/u, (_2, maybeAsync = "") => maybeAsync + "function ");
      }
      const defaultProps = ["length", "name", "arguments", "caller", "prototype"];
      const hasProps = !(Object.getOwnPropertyNames(val).every(($) => defaultProps.includes($)) && Object.getOwnPropertySymbols(val).length === 0);
      if (hasProps) {
        const props = Object.getOwnPropertyDescriptors(val);
        for (const prop of defaultProps) {
          delete props[prop];
        }
        string = `Object.defineProperties(${string},${recurse(props)})`;
      }
      stack.delete(val);
      return string;
    } else if (typeof val === "symbol") {
      let ref;
      if ((ref = Symbol.keyFor(val)) != null) {
        const key = ref;
        return `Symbol.for(${JSON.stringify(key)})`;
      }
      for (const name of Object.getOwnPropertyNames(Symbol)) {
        const sym = Symbol[name];
        if (val === sym) {
          return `Symbol.${name}`;
        }
      }
      throw new TypeError("cannot serialize unique symbol");
    } else if (typeof val === "object") {
      if (stack.has(val)) {
        throw new Error("circular reference detected");
      }
      stack.add(val);
      let ref1;
      switch (Object.getPrototypeOf(val)) {
        case RegExp.prototype:
        case context?.RegExp.prototype: {
          const re = val;
          ref1 = `/${re.source}/${re.flags}`;
          break;
        }
        case Date.prototype:
        case context?.Date.prototype: {
          ref1 = `new Date(${val.getTime()})`;
          break;
        }
        case Set.prototype:
        case context?.Set.prototype: {
          ref1 = "new Set([" + (() => {
            const results = [];
            for (const item of val) {
              results.push(recurse(item));
            }
            return results;
          })().join(",") + "])";
          break;
        }
        case Map.prototype:
        case context?.Map.prototype: {
          ref1 = "new Map([" + (() => {
            const results1 = [];
            for (const [key, value2] of val) {
              results1.push(`[${recurse(key)},${recurse(value2)}]`);
            }
            return results1;
          })().join(",") + "])";
          break;
        }
        case Object.prototype:
        case context?.Object.prototype: {
          let objStr = "{";
          let descStr = "";
          for (let ref2 = Object.getOwnPropertyNames(val).concat(Object.getOwnPropertySymbols(val)), i = 0, len3 = ref2.length; i < len3; i++) {
            const prop = ref2[i];
            const desc = Object.getOwnPropertyDescriptor(val, prop);
            if (desc.enumerable && desc.configurable && desc.writable) {
              if (typeof prop === "symbol") {
                objStr += `[${recurse(prop)}]`;
              } else {
                objStr += JSON.stringify(prop);
              }
              objStr += `:${recurse(desc.value)},`;
            } else {
              if (typeof prop === "symbol") {
                descStr += `[${recurse(prop)}]`;
              } else {
                descStr += JSON.stringify(prop);
              }
              descStr += `:${recurse(desc)},`;
            }
          }
          if (objStr.length > 1) {
            objStr = objStr.slice(0, -1);
          }
          objStr += "}";
          if (descStr !== "") {
            objStr = `Object.defineProperties(${objStr},{${descStr.slice(0, -1)}})`;
          }
          if (!Object.isExtensible(val)) {
            objStr = `Object.preventExtensions(${objStr})`;
          }
          ref1 = objStr;
          break;
        }
        case URL.prototype:
        case context?.URL.prototype: {
          ref1 = `new URL(${JSON.stringify(val.href)})`;
          break;
        }
        case null: {
          ref1 = `Object.create(null,${recurse(Object.getOwnPropertyDescriptors(val))})`;
          break;
        }
        case Int8Array.prototype:
        case Uint8Array.prototype:
        case Int16Array.prototype:
        case Uint16Array.prototype:
        case Int32Array.prototype:
        case Uint32Array.prototype:
        case Float32Array.prototype:
        case Float64Array.prototype:
        case Uint8ClampedArray.prototype:
        case context?.Int8Array.prototype:
        case context?.Uint8Array.prototype:
        case context?.Int16Array.prototype:
        case context?.Uint16Array.prototype:
        case context?.Int32Array.prototype:
        case context?.Uint32Array.prototype:
        case context?.Float32Array.prototype:
        case context?.Float64Array.prototype:
        case context?.Uint8ClampedArray.prototype: {
          ref1 = `new ${val.constructor.name}([${val.join(",")}])`;
          break;
        }
        case BigInt64Array.prototype:
        case BigUint64Array.prototype:
        case context?.BigInt64Array.prototype:
        case context?.BigUint64Array.prototype: {
          ref1 = `new ${val.constructor.name}([${Array.from(val, ($1) => `${$1}n`).join(",")}])`;
          break;
        }
        case globalThis.Buffer?.prototype:
        case context?.Buffer?.prototype: {
          ref1 = `Buffer.from([${val.join(",")}])`;
          break;
        }
        default: {
          if (Array.isArray(val)) {
            ref1 = `[${val.map(recurse).join(",")}]`;
          } else {
            throw new TypeError(`cannot serialize object with prototype ${val.constructor?.name ?? Object.getPrototypeOf(val)}`);
          }
        }
      }
      ;
      const str = ref1;
      stack.delete(val);
      return str;
    } else {
      throw new TypeError(`cannot serialize ${typeof val} value`);
    }
  }
  return recurse(value);
}

// source/parser/string.civet
function getIndentLevel(str, tab) {
  if (tab != null && tab != 1) {
    const tabs = str.match(/\t/g);
    const numTabs = tabs ? tabs.length : 0;
    return numTabs * tab + /*spaces*/
    (str.length - numTabs);
  } else {
    return str.length;
  }
}
function reduceIndentLevel(str, dedent, tab) {
  if (tab != null && tab != 1) {
    for (let i1 = 0, len3 = str.length; i1 < len3; i1++) {
      const i = i1;
      const char = str[i1];
      if (!dedent) {
        return str.slice(i);
      }
      if (char == "	") {
        dedent -= tab;
        if (dedent < 0) {
          return "".padStart(-dedent, " ") + str.slice(i + 1);
        }
      } else {
        dedent--;
      }
    }
    return "";
  } else {
    return str.slice(dedent);
  }
}
var indentRe = /\n([ \t]*)(?![ \t]|\r?\n|$)/g;
function getIndentOfBlockString(str, tab) {
  let minLevel = Infinity;
  let ref;
  while (ref = indentRe.exec(str)) {
    const match = ref;
    const level = getIndentLevel(match[1], tab);
    if (level < minLevel) {
      minLevel = level;
    }
  }
  if (minLevel === Infinity) {
    minLevel = 0;
  }
  return minLevel;
}
function dedentBlockString({ $loc, token: str }, tab, dedent, trimStart = true, trimEnd = true) {
  if (dedent == null && /^[ \t]*\r?\n/.test(str)) {
    dedent = getIndentOfBlockString(str, tab);
  }
  if (dedent) {
    str = str.replace(/(\n)([ \t]*)/g, (_2, newline, indent) => {
      return newline + reduceIndentLevel(indent, dedent, tab);
    });
  }
  if (trimStart) {
    str = str.replace(/^[ \t]*\r?\n/, "");
  }
  if (trimEnd) {
    str = str.replace(/(\r?\n|\n)[ \t]*$/, "");
  }
  str = str.replace(/(\\.|`|\$\{)/g, (s) => {
    if (s[0] === "\\") {
      return s;
    } else {
      return `\\${s}`;
    }
  });
  return { $loc, token: str };
}
function dedentBlockSubstitutions($0, tab) {
  const [s, strWithSubstitutions, e] = $0;
  if (!strWithSubstitutions.length) {
    return $0;
  }
  const stringPart = (() => {
    const results1 = [];
    for (let i2 = 0, len1 = strWithSubstitutions.length; i2 < len1; i2++) {
      const part = strWithSubstitutions[i2];
      results1.push(part.token ?? "s");
    }
    return results1;
  })().join("");
  let ref1;
  if (/^[ \t]*\r?\n/.test(stringPart)) {
    ref1 = getIndentOfBlockString(stringPart, tab);
  } else {
    ref1 = void 0;
  }
  ;
  const dedent = ref1;
  let results = [s];
  for (let i3 = 0, len22 = strWithSubstitutions.length; i3 < len22; i3++) {
    const i = i3;
    let part = strWithSubstitutions[i3];
    if (part.token != null) {
      part = dedentBlockString(
        part,
        tab,
        dedent,
        i === 0,
        i === strWithSubstitutions.length - 1
      );
    }
    results.push(part);
  }
  results.push(e);
  return {
    type: "TemplateLiteral",
    children: results
  };
}
function processCoffeeInterpolation(s, parts, e, $loc) {
  if (parts.length === 0) {
    return {
      type: "StringLiteral",
      token: '""',
      $loc
    };
  }
  if (parts.length === 1) {
    let ref2;
    if ((ref2 = parts[0]) && typeof ref2 === "object" && "token" in ref2) {
      const { token } = ref2;
      return {
        type: "StringLiteral",
        token: `"${modifyString(token)}"`,
        $loc
      };
    }
  }
  const results2 = [];
  for (let i4 = 0, len3 = parts.length; i4 < len3; i4++) {
    const part = parts[i4];
    if ("token" in part) {
      const token = modifyString(part.token.replace(/(`|\$\{)/g, "\\$1"));
      results2.push({
        ...part,
        token
      });
    } else {
      results2.push(part);
    }
  }
  ;
  parts = results2;
  s.token = e.token = "`";
  return {
    type: "TemplateLiteral",
    children: [s, parts, e]
  };
}
function modifyString(str) {
  return str.replace(/((?:\\.|[^\r\n])*)(\r\n|\n|\r)?/gsu, function(_2, chars, nl) {
    if (nl) {
      return chars + "\\n";
    } else {
      return chars;
    }
  });
}
function quoteString(str) {
  return JSON.stringify(str);
}

// source/parser/lib.civet
var xor = (a, b) => a ? !b && a : b;
function addPostfixStatement(statement, ws, post) {
  const expressions = [
    ...post.blockPrefix || [],
    ["", statement]
  ];
  const block = makeNode({
    type: "BlockStatement",
    children: [" { ", expressions, " }"],
    expressions
  });
  const children = [...post.children, block];
  if (!isWhitespaceOrEmpty(ws))
    children.push(ws);
  post = makeNode({ ...post, children, block });
  if (post.type === "IfStatement") {
    post.then = block;
  }
  return post;
}
function adjustIndexAccess(dot) {
  if (dot.optional) {
    return {
      ...dot,
      children: [...dot.children, "["]
    };
  } else {
    dot = replaceNodes(
      deepCopy(dot),
      (node) => node.token === ".",
      (node) => ({ ...node, token: "[" })
    );
  }
  return dot;
}
function negateCondition(condition) {
  let { expression } = condition;
  const children = condition.children.slice();
  const i = children.indexOf(expression);
  if (i < 0) {
    throw new Error(`Could not find expression in condition`);
  }
  children[i] = expression = {
    type: "UnaryExpression",
    children: [
      "!",
      makeLeftHandSideExpression(expression)
    ]
  };
  return { ...condition, expression, children };
}
function isExpression(node) {
  if (Array.isArray(node)) {
    return node.every(isExpression);
  }
  if (typeof node === "string") {
    return true;
  }
  switch (node?.type) {
    case "BlockStatement":
    case "DebuggerStatement":
    case "Declaration":
    case "ForStatement":
    case "IfStatement":
    case "IterationStatement":
    case "ReturnStatement":
    case "SwitchStatement":
    case "ThrowStatement":
    case "TryStatement":
      return false;
  }
  return true;
}
function expressionizeBlock(blockOrExpression) {
  if (blockOrExpression && typeof blockOrExpression === "object" && "expressions" in blockOrExpression) {
    const { expressions } = blockOrExpression;
    const l = expressions.length;
    const results = [];
    let i1 = 0;
    for (const [ws, s, _delim] of expressions) {
      const i = i1++;
      if (!isExpression(s))
        return;
      const wrapped = makeLeftHandSideExpression(s);
      if (i === l - 1) {
        results.push([ws, wrapped]);
      } else {
        results.push([ws, wrapped, ","]);
      }
    }
    if (results.length > 1) {
      return makeLeftHandSideExpression(results);
    } else if (results.length) {
      return results;
    } else {
      return ["void 0"];
    }
  } else {
    return blockOrExpression;
  }
}
function expressionizeIfStatement(statement) {
  const { condition, then: b, else: e } = statement;
  const [...condRest] = condition.children, [closeParen] = condRest.splice(-1);
  const expressionizedBlock = expressionizeBlock(b);
  if (!expressionizedBlock) {
    return wrapIIFE([["", statement]]);
  }
  const children = [
    ...condRest,
    "?",
    expressionizedBlock
  ];
  if (e) {
    const e2 = expressionizeBlock(e.block);
    if (!e2) {
      return wrapIIFE([["", statement]]);
    }
    children.push(e.children[0], ":", e2, ...e.children.slice(3));
  } else {
    children.push(":void 0");
  }
  children.push(closeParen);
  return makeNode({
    type: "IfExpression",
    children
  });
}
function expressionizeTypeIf([ws, ifOp, condition, t, e]) {
  const children = [
    ws,
    "(",
    insertTrimmingSpace(condition, ""),
    "?"
  ];
  if (!xor(ifOp.negated, condition.negated)) {
    children.push(t);
    if (e) {
      children.push(e[0], ":", ...e.slice(2));
    } else {
      children.push(":never");
    }
  } else {
    if (e) {
      children.push(...e.slice(2), e[0], ":");
    } else {
      children.push("never:");
    }
    children.push(t);
  }
  children.push(")");
  return children;
}
function handleThisPrivateShorthands(value) {
  if (value.privateShorthand) {
    value = value.children[1].children[1];
    return [value, false];
  }
  if (value.type === "MemberExpression" || value.type === "CallExpression") {
    let suppressPrefix = value.thisShorthand;
    value = {
      ...value,
      children: value.children.map((c, i) => {
        if (i === 0) {
          let s;
          [c, s] = handleThisPrivateShorthands(c);
          suppressPrefix ||= s;
        }
        return c;
      })
    };
    return [value, suppressPrefix];
  }
  return [value, value.thisShorthand];
}
function processTryBlock($0) {
  let [t, , b, c, e, f] = $0;
  if (!c && (e || !f)) {
    const emptyCatchBlock = makeEmptyBlock();
    c = {
      type: "CatchClause",
      children: [" ", "catch(e) ", emptyCatchBlock],
      block: emptyCatchBlock
    };
  }
  let hoistDec;
  if (e) {
    c = c;
    const ok = makeRef("ok");
    hoistDec = {
      type: "Declaration",
      children: ["let ", ok, " = true"],
      names: []
    };
    replaceNode(
      c.block,
      blockWithPrefix([["", "ok = false"]], c.block),
      c
    );
    const condition = {
      type: "ParenthesizedExpression",
      children: ["(", ok, ")"],
      expression: ok
    };
    const i = makeNode({
      type: "IfStatement",
      children: ["if", condition, e.block],
      condition,
      then: e.block,
      else: void 0
    });
    if (!f) {
      const emptyFinallyBlock = makeEmptyBlock();
      f = {
        type: "FinallyClause",
        children: [" ", "finally ", emptyFinallyBlock],
        block: emptyFinallyBlock
      };
    }
    replaceNode(
      f.block,
      blockWithPrefix([["", i]], f.block),
      f
    );
  }
  const blocks = [b];
  if (c) {
    blocks.push(c.block);
  }
  return {
    type: "TryStatement",
    blocks,
    children: [t, b, c, f],
    hoistDec
  };
}
function processCallMemberExpression(node) {
  const { children } = node;
  if (Array.isArray(children) && children.length >= 2 && typeof children[0] === "object" && children[0] != null && "parenthesizedOp" in children[0] && typeof children[0].parenthesizedOp === "object" && children[0].parenthesizedOp != null && "token" in children[0].parenthesizedOp && typeof children[1] === "object" && children[1] != null && "type" in children[1] && children[1].type === "Call") {
    const op = children[0].parenthesizedOp;
    let call = children[1];
    const args = [...call.args];
    call = {
      ...call,
      args,
      children: call.children.map((x) => x === call.args ? args : x)
    };
    if (isComma(args[args.length - 1])) {
      args[args.length - 1] = deepCopy(args[args.length - 1]);
      isComma(args[args.length - 1]).token = "";
    }
    let commaCount = 0;
    for (let i2 = 0, len1 = args.length; i2 < len1; i2++) {
      const i = i2;
      let arg = args[i2];
      if (isComma(arg)) {
        arg = args[i] = deepCopy(arg);
        isComma(arg).token = `)${op.token}(`;
        commaCount++;
      }
    }
    if (args.length) {
      children.splice(
        0,
        2,
        commaCount ? {
          type: "ParenthesizedExpression",
          children: ["(", ...call.children, ")"]
        } : { ...call, type: "ParenthesizedExpression" }
      );
    }
  }
  for (let i = 0; i < children.length; i++) {
    const glob = children[i];
    if (glob?.type === "PropertyGlob") {
      let prefix = children.slice(0, i);
      const parts = [];
      let hoistDec, refAssignmentComma;
      if (prefix.length > 1) {
        const ref = makeRef();
        ({ hoistDec, refAssignmentComma } = makeRefAssignment(ref, prefix));
        prefix = [ref];
      }
      prefix = prefix.concat(glob.dot);
      for (const part of glob.object.properties) {
        if (part.type === "Error") {
          parts.push(part);
          continue;
        }
        if (part.type === "MethodDefinition") {
          parts.push({
            type: "Error",
            message: "Glob pattern cannot have method definition"
          });
          continue;
        }
        if (part.value && !["CallExpression", "MemberExpression", "Identifier"].includes(part.value.type)) {
          parts.push({
            type: "Error",
            message: `Glob pattern must have call or member expression value, found ${JSON.stringify(part.value)}`
          });
          continue;
        }
        let suppressPrefix = false;
        let name = part.name;
        let value = part.value ?? name;
        const wValue = getTrimmingSpace(part.value);
        [value, suppressPrefix] = handleThisPrivateShorthands(value);
        if (glob.reversed) {
          [name, value] = [value, name];
        }
        if (!suppressPrefix) {
          value = prefix.concat(insertTrimmingSpace(value, ""));
        }
        if (wValue)
          value.unshift(wValue);
        if (part.type === "SpreadProperty") {
          parts.push({
            type: part.type,
            value,
            dots: part.dots,
            delim: part.delim,
            names: part.names,
            children: part.children.slice(0, 2).concat(value, part.delim)
          });
        } else {
          parts.push({
            type: part.type === "Identifier" ? "Property" : part.type,
            name,
            value,
            delim: part.delim,
            names: part.names,
            children: [
              isWhitespaceOrEmpty(part.children[0]) && part.children[0],
              name,
              isWhitespaceOrEmpty(part.children[2]) && part.children[2],
              part.children[3]?.token === ":" ? part.children[3] : ":",
              value,
              part.delim
              // comma delimiter
            ]
          });
        }
      }
      let ref1;
      let object = {
        type: "ObjectExpression",
        children: [
          glob.object.children[0],
          // {
          ...parts,
          (ref1 = glob.object.children)[ref1.length - 1]
          // whitespace and }
        ],
        properties: parts,
        hoistDec
      };
      if (refAssignmentComma) {
        object = {
          type: "ParenthesizedExpression",
          children: ["(", ...refAssignmentComma, object, ")"],
          expression: object
        };
      }
      if (i === children.length - 1)
        return object;
      return processCallMemberExpression({
        // in case there are more
        ...node,
        children: [object, ...children.slice(i + 1)]
      });
    } else if (glob?.type === "PropertyBind") {
      const prefix = children.slice(0, i);
      return processCallMemberExpression({
        // in case there are more
        ...node,
        children: [
          prefix,
          {
            ...glob,
            type: "PropertyAccess",
            children: [
              ...glob.children,
              ".bind(",
              prefix,
              ...glob.args.length > 0 ? [", "] : [],
              ...glob.args,
              ")"
            ]
          },
          ...children.slice(i + 1)
        ]
      });
    }
  }
  return node;
}
function replaceNode(node, newNode, parent) {
  parent ??= node.parent;
  if (!(parent != null)) {
    throw new Error("replaceNode failed: node has no parent");
  }
  function recurse(children) {
    for (let i3 = 0, len22 = children.length; i3 < len22; i3++) {
      const i = i3;
      const child = children[i3];
      if (child === node) {
        children[i] = newNode;
        return true;
      } else if (Array.isArray(child)) {
        if (recurse(child)) {
          return true;
        }
      }
    }
    return false;
  }
  if (!recurse(parent.children)) {
    throw new Error("replaceNode failed: didn't find child node in parent");
  }
  for (const key in parent) {
    const value = parent[key];
    if (value === node) {
      parent[key] = newNode;
    }
  }
  if (isASTNodeObject(newNode)) {
    newNode.parent = parent;
  }
}
function makeExpressionStatement(expression) {
  if (Array.isArray(expression) && expression[1]?.[0]?.[0]?.[1]?.token === ",") {
    return [
      makeExpressionStatement(expression[0]),
      expression[1].map(([comma, exp]) => {
        return [comma, makeExpressionStatement(exp)];
      })
    ];
  } else if (expression?.type === "ObjectExpression" || expression?.type === "FunctionExpression" && !expression.id) {
    return makeLeftHandSideExpression(expression);
  } else {
    return expression;
  }
}
function lastAccessInCallExpression(exp) {
  if (exp.type === "Identifier") {
    return exp;
  }
  let children, i;
  do {
    if (!(exp.children != null)) {
      return;
    }
    ;
    ({ children } = exp);
    i = children.length - 1;
    while (i >= 0 && (children[i].type === "Call" || children[i].type === "NonNullAssertion" || children[i].type === "Optional"))
      i--;
    if (i < 0)
      return;
  } while (children[i].type === "MemberExpression" && (exp = children[i]));
  return children[i];
}
function convertMethodToFunction(method) {
  const { signature, block } = method;
  let { modifier, optional } = signature;
  if (optional)
    return;
  if (modifier) {
    if (modifier.get || modifier.set) {
      return;
    } else if (modifier.async) {
      modifier = [modifier.children[0][0], " function ", ...modifier.children.slice(1)];
    } else {
      modifier = ["function ", ...modifier.children || []];
    }
  } else {
    modifier = "function ";
  }
  return {
    ...signature,
    id: signature.name,
    signature,
    type: "FunctionExpression",
    children: [
      [modifier, ...signature.children.slice(1)],
      block
    ],
    block
  };
}
function convertNamedImportsToObject(node, pattern) {
  const properties = node.specifiers.map((specifier) => {
    if (specifier.ts) {
      return { type: "Error", message: "cannot use `type` in dynamic import" };
    } else {
      const { source, binding } = specifier;
      let ref2;
      const delim = (ref2 = specifier.children)[ref2.length - 1];
      return {
        type: pattern ? "BindingProperty" : "Property",
        name: source,
        value: !(source === binding) ? binding : void 0,
        delim,
        children: source === binding ? [source, delim] : [source, ":", binding, delim]
      };
    }
  });
  let ref3;
  return {
    type: pattern ? "ObjectBindingPattern" : "ObjectExpression",
    names: node.names,
    properties,
    children: [
      node.children[0],
      // {
      properties,
      (ref3 = node.children)[ref3.length - 1]
      // }
    ]
  };
}
function convertObjectToJSXAttributes(obj) {
  const { properties } = obj;
  const parts = [];
  const rest = [];
  for (let i = 0; i < properties.length; i++) {
    if (i > 0)
      parts.push(" ");
    const part = properties[i];
    switch (part.type) {
      case "Identifier":
        parts.push([part.name, "={", part.name, "}"]);
        break;
      case "Property":
        if (part.name.type === "ComputedPropertyName") {
          rest.push(part);
        } else {
          parts.push([part.name, "={", insertTrimmingSpace(part.value, ""), "}"]);
        }
        break;
      case "SpreadProperty":
        parts.push(["{", part.dots, part.value, "}"]);
        break;
      case "MethodDefinition":
        const func = convertMethodToFunction(part);
        if (func) {
          parts.push([part.name, "={", convertMethodToFunction(part), "}"]);
        } else {
          rest.push(part);
        }
        break;
      default:
        throw new Error(`invalid object literal type in JSX attribute: ${part.type}`);
    }
  }
  if (rest.length) {
    parts.push(["{...{", ...rest, "}}"]);
  }
  return parts;
}
function makeGetterMethod(name, ws, value, returnType, block, kind = { token: "get" }, autoReturn = true) {
  const { token } = kind;
  ws = insertTrimmingSpace(ws, "");
  let setVal;
  const parameters = token === "get" ? {
    type: "Parameters",
    children: ["()"],
    names: [],
    implicit: true
  } : {
    type: "Parameters",
    children: ["(", setVal = makeRef("value"), ")"],
    names: [],
    implicit: false
  };
  let expressions;
  if (block) {
    block = duplicateBlock(block);
    expressions = block.expressions;
  } else {
    expressions = [];
    block = {
      type: "BlockStatement",
      expressions,
      children: ["{ ", expressions, " }"]
    };
  }
  if (autoReturn) {
    const finalStatement = token === "get" ? [[expressions[0]?.[0] || "", ws], wrapWithReturn(value)] : [[expressions[0]?.[0] || "", ws], [value, " = ", setVal]];
    expressions.push(finalStatement);
  }
  const children = [kind, " ", name, parameters, returnType, block];
  return {
    type: "MethodDefinition",
    children,
    name,
    signature: {
      type: "MethodSignature",
      modifier: {
        get: token === "get",
        set: token === "set",
        async: false
      },
      name,
      returnType
    },
    block,
    parameters
  };
}
function processBindingPatternLHS(lhs, tail) {
  adjustAtBindings(lhs, true);
  const [splices, thisAssignments] = gatherBindingCode(lhs);
  tail.push(...splices.map((s) => [", ", s]), ...thisAssignments.map((a) => [", ", a]));
}
function processAssignments(statements) {
  gatherRecursiveAll(statements, (n) => n.type === "AssignmentExpression" || n.type === "UpdateExpression").forEach((exp) => {
    function extractAssignment(lhs) {
      let expr = lhs;
      while (expr.type === "ParenthesizedExpression") {
        expr = expr.expression;
      }
      let m;
      if (m = expr.type, m === "AssignmentExpression" || m === "UpdateExpression") {
        if (expr.type === "UpdateExpression" && expr.children[0] === expr.assigned) {
          pre.push("(");
          post.push([", ", lhs, ")"]);
        } else {
          pre.push(["(", lhs, ", "]);
          post.push(")");
        }
        return expr.assigned;
      }
      ;
      return;
    }
    const pre = [], post = [];
    let ref4;
    switch (exp.type) {
      case "AssignmentExpression": {
        if (!exp.lhs)
          return;
        exp.lhs.forEach((lhsPart, i) => {
          let ref5;
          if (ref5 = extractAssignment(lhsPart[1])) {
            const newLhs = ref5;
            return lhsPart[1] = newLhs;
          }
          ;
          return;
        });
        break;
      }
      case "UpdateExpression": {
        if (ref4 = extractAssignment(exp.assigned)) {
          const newLhs = ref4;
          const i = exp.children.indexOf(exp.assigned);
          exp.assigned = exp.children[i] = newLhs;
        }
        ;
        break;
      }
    }
    if (pre.length)
      exp.children.unshift(...pre);
    if (post.length)
      exp.children.push(...post);
    if (exp.type === "UpdateExpression") {
      const { assigned } = exp;
      const ref = makeRef();
      const newMemberExp = unchainOptionalMemberExpression(assigned, ref, (children) => {
        return exp.children.map(($) => $ === assigned ? children : $);
      });
      if (newMemberExp !== assigned) {
        if (newMemberExp.usesRef) {
          newMemberExp.hoistDec = {
            type: "Declaration",
            children: ["let ", ref],
            names: []
          };
        }
        return replaceNode(exp, newMemberExp);
      }
      ;
      return;
    }
    ;
    return;
  });
  replaceNodesRecursive(
    statements,
    (n) => n.type === "AssignmentExpression" && n.names === null,
    (exp) => {
      let { lhs: $1, expression: $2 } = exp, tail = [], len3 = $1.length;
      let block;
      let ref6;
      if (exp.parent?.type === "BlockStatement" && !(ref6 = $1[$1.length - 1])?.[ref6.length - 1]?.special) {
        block = makeBlockFragment();
        let ref7;
        if (ref7 = prependStatementExpressionBlock(
          { type: "Initializer", expression: $2, children: [void 0, void 0, $2] },
          block
        )) {
          const ref = ref7;
          exp.children = exp.children.map(($3) => $3 === $2 ? ref : $3);
          $2 = ref;
        } else {
          block = void 0;
        }
      }
      let ref8;
      if ($1.some(($4) => (ref8 = $4)[ref8.length - 1].special)) {
        if ($1.length !== 1)
          throw new Error("Only one assignment with id= is allowed");
        const [, lhs, , op] = $1[0];
        const { call, omitLhs } = op;
        const index = exp.children.indexOf($2);
        if (index < 0)
          throw new Error("Assertion error: exp not in AssignmentExpression");
        exp.children.splice(
          index,
          1,
          exp.expression = $2 = [call, "(", lhs, ", ", $2, ")"]
        );
        if (omitLhs) {
          return $2;
        }
      }
      let wrapped = false;
      let i = 0;
      while (i < len3) {
        const lastAssignment = $1[i++];
        const [, lhs, , op] = lastAssignment;
        if (!(op.token === "=")) {
          continue;
        }
        let m1;
        if (m1 = lhs.type, m1 === "ObjectExpression" || m1 === "ObjectBindingPattern") {
          if (!wrapped) {
            wrapped = true;
            lhs.children.splice(0, 0, "(");
            tail.push(")");
          }
        }
      }
      const refsToDeclare = /* @__PURE__ */ new Set();
      i = len3 - 1;
      while (i >= 0) {
        const lastAssignment = $1[i];
        if (lastAssignment[3].token === "=") {
          const lhs = lastAssignment[1];
          let m2;
          if (lhs.type === "MemberExpression") {
            const members = lhs.children;
            const lastMember = members[members.length - 1];
            if (lastMember.type === "SliceExpression") {
              const { start, end, children: c } = lastMember;
              c[0].token = ".splice(";
              c[1] = start;
              c[2] = ", ";
              if (end) {
                c[3] = [end, " - ", start];
              } else {
                c[3] = ["1/0"];
              }
              c[4] = [", ...", $2];
              c[5] = ")";
              lastAssignment.pop();
              if (isWhitespaceOrEmpty(lastAssignment[2]))
                lastAssignment.pop();
              if ($1.length > 1) {
                throw new Error("Not implemented yet! TODO: Handle multiple splice assignments");
              }
              exp.children = [$1];
              exp.names = [];
              break;
            }
          } else if (m2 = lhs.type, m2 === "ObjectBindingPattern" || m2 === "ArrayBindingPattern") {
            processBindingPatternLHS(lhs, tail);
            gatherRecursiveAll(lhs, ($5) => $5.type === "Ref").forEach(refsToDeclare.add.bind(refsToDeclare));
          }
        }
        i--;
      }
      i = len3 - 1;
      const optionalChainRef = makeRef();
      while (i >= 0) {
        const assignment = $1[i];
        const [ws1, lhs, ws2, op] = assignment;
        if (lhs.type === "MemberExpression" || lhs.type === "CallExpression") {
          const newMemberExp = unchainOptionalMemberExpression(lhs, optionalChainRef, (children) => {
            const assigns = $1.splice(i + 1, len3 - 1 - i);
            $1.pop();
            return [ws1, ...children, ws2, op, ...assigns, $2];
          });
          if (newMemberExp !== lhs) {
            if (newMemberExp.usesRef) {
              exp.hoistDec = {
                type: "Declaration",
                children: ["let ", optionalChainRef],
                names: []
              };
            }
            replaceNode($2, newMemberExp);
            newMemberExp.parent = exp;
            $2 = newMemberExp;
          }
        }
        i--;
      }
      if (refsToDeclare.size) {
        if (exp.hoistDec) {
          exp.hoistDec.children.push([...refsToDeclare].map(($6) => [",", $6]));
        } else {
          exp.hoistDec = {
            type: "Declaration",
            children: ["let ", [...refsToDeclare].map((r, i2) => i2 ? [",", r] : r)],
            names: []
          };
        }
      }
      exp.names = $1.flatMap(([, l]) => l.names || []);
      if (tail.length) {
        const index = exp.children.indexOf($2);
        if (index < 0)
          throw new Error("Assertion error: exp not in AssignmentExpression");
        exp.children.splice(index + 1, 0, ...tail);
      }
      if (block) {
        block.parent = exp.parent;
        block.expressions.push(["", exp]);
        exp.parent = block;
        return block;
      }
      return exp;
    }
  );
}
function unchainOptionalMemberExpression(exp, ref, innerExp) {
  let j = 0;
  const { children } = exp;
  let usesRef = false;
  const conditions = [];
  while (j < children.length) {
    const child = children[j];
    const type = child?.type;
    let hasOptional = false;
    switch (type) {
      case "PropertyAccess": {
        if (child.dot?.optional) {
          hasOptional = true;
          child.dot.children.shift();
          child.dot.optional = false;
        }
        ;
        break;
      }
      case "Call":
      case "Index": {
        if (child.optional) {
          hasOptional = true;
          child.children.shift();
          child.optional = void 0;
        }
        ;
        break;
      }
    }
    if (hasOptional) {
      let base;
      if (j > 1 || needsRef(children[0])) {
        usesRef = true;
        base = makeLeftHandSideExpression({
          type: "AssignmentExpression",
          children: [ref, " = ", children.splice(0, j)]
        });
        base.parent = child;
        children.unshift(ref);
        j = 0;
      } else {
        base = children[0];
      }
      conditions.push([base, " != null"]);
    }
    j++;
  }
  let ref9;
  if (ref9 = conditions.length) {
    const l = ref9;
    const cs = flatJoin(conditions, " && ");
    return {
      ...exp,
      children: [...cs, " ? ", innerExp(children), " : void 0"],
      usesRef
    };
  } else {
    return exp;
  }
}
function attachPostfixStatementAsExpression(exp, post) {
  const postfixStatement = post[1];
  switch (postfixStatement.type) {
    case "ForStatement":
    case "IterationStatement":
    case "DoStatement": {
      const statement = addPostfixStatement(exp, ...post);
      return {
        type: "IterationExpression",
        children: [statement],
        block: statement.block,
        statement
      };
    }
    case "IfStatement": {
      return expressionizeIfStatement({ ...postfixStatement, then: exp });
    }
    default: {
      throw new Error("Unknown postfix statement");
    }
  }
}
function processTypes(node) {
  return gatherRecursiveAll(node, (n) => n.type === "UnaryType").forEach((unary) => {
    let last;
    let count = 0;
    let ref10;
    while (unary.suffix.length && (ref10 = unary.suffix)[ref10.length - 1]?.token === "?") {
      last = unary.suffix.pop();
      count++;
    }
    if (!count) {
      return;
    }
    if (unary.parent?.type === "TypeTuple") {
      if (count === 1) {
        unary.suffix.push(last);
        return;
      }
      replaceNode(unary, [
        getTrimmingSpace(unary),
        "(",
        parenthesizeType(insertTrimmingSpace(unary, "")),
        " | null)",
        last
      ]);
    } else {
      replaceNode(unary, [
        getTrimmingSpace(unary),
        "(",
        parenthesizeType(insertTrimmingSpace(unary, "")),
        count === 1 ? " | undefined" : " | undefined | null",
        ")"
      ]);
    }
  });
}
function processStatementExpressions(statements) {
  gatherRecursiveAll(statements, ($7) => $7.type === "StatementExpression").forEach((_exp) => {
    const exp = _exp;
    const { statement } = exp;
    let ref11;
    switch (statement.type) {
      case "IfStatement": {
        if (ref11 = expressionizeIfStatement(statement)) {
          const expression = ref11;
          return replaceNode(statement, expression, exp);
        } else {
          return replaceNode(statement, wrapIIFE([["", statement]]), exp);
        }
      }
      case "IterationExpression": {
        if (statement.subtype === "ComptimeStatement") {
          const { expressions } = statement.statement.block;
          const expression = wrapIIFE(expressions, hasAwait(expressions));
          return replaceNode(statement, makeNode({
            type: "ComptimeExpression",
            expression,
            children: [expression]
          }), exp);
        }
        ;
        return;
      }
      default: {
        return replaceNode(statement, wrapIIFE([["", statement]]), exp);
      }
    }
  });
}
function processNegativeIndexAccess(statements) {
  gatherRecursiveAll(statements, (n) => n.type === "NegativeIndex").forEach((exp) => {
    const { children } = exp.parent;
    let start = 0;
    while (start < children.length && isWhitespaceOrEmpty(children[start])) {
      start++;
    }
    const index = children.indexOf(exp);
    let ref, subexp;
    if (index === start + 1) {
      const child = children[start];
      ref = maybeRef(child);
      if (ref !== child) {
        subexp = children.splice(start, 1);
      }
    } else if (index > start + 1) {
      ref = makeRef();
      subexp = children.splice(start, index - start);
    } else {
      throw new Error("Invalid parse tree for negative index access");
    }
    if (subexp) {
      const { hoistDec, refAssignment } = makeRefAssignment(ref, subexp);
      exp.hoistDec = hoistDec;
      children.splice(start, 0, makeLeftHandSideExpression(refAssignment));
    }
    exp.len.children = [
      ref,
      ".length"
    ];
  });
}
function processProgram(root) {
  const state2 = getState();
  const config2 = getConfig();
  assert.equal(state2.forbidBracedApplication.length, 1, "forbidBracedApplication");
  assert.equal(state2.forbidClassImplicitCall.length, 1, "forbidClassImplicitCall");
  assert.equal(state2.forbidIndentedApplication.length, 1, "forbidIndentedApplication");
  assert.equal(state2.forbidNewlineBinaryOp.length, 1, "forbidNewlineBinaryOp");
  assert.equal(state2.forbidTrailingMemberProperty.length, 1, "forbidTrailingMemberProperty");
  assert.equal(state2.JSXTagStack.length, 1, "JSXTagStack");
  addParentPointers(root);
  const { expressions: statements } = root;
  processPlaceholders(statements);
  processNegativeIndexAccess(statements);
  processTypes(statements);
  processDeclarationConditions(statements);
  processPipelineExpressions(statements);
  processDeclarations(statements);
  processAssignments(statements);
  processStatementExpressions(statements);
  processPatternMatching(statements);
  gatherRecursiveAll(statements, (n) => n.type === "IterationExpression").forEach((e) => expressionizeIteration(e));
  hoistRefDecs(statements);
  processFunctions(statements, config2);
  statements.unshift(...state2.prelude);
  if (config2.autoLet) {
    createConstLetDecs(statements, [], "let");
  } else if (config2.autoConst) {
    createConstLetDecs(statements, [], "const");
  } else if (config2.autoVar) {
    createVarDecs(root, []);
  }
  processBlocks(statements);
  populateRefs(statements);
  adjustAtBindings(statements);
  if (getSync()) {
    processComptime(statements);
  }
}
async function processProgramAsync(root) {
  const { expressions: statements } = root;
  await processComptime(statements);
}
function populateRefs(statements) {
  const refNodes = gatherRecursive(statements, ({ type }) => type === "Ref");
  if (refNodes.length) {
    const ids = gatherRecursive(statements, (s) => s.type === "Identifier");
    const names = new Set(ids.flatMap(({ names: names2 }) => names2 || []));
    refNodes.forEach((ref) => {
      const { type, base } = ref;
      if (type !== "Ref")
        return;
      ref.type = "Identifier";
      let n = 0;
      let name = base;
      while (names.has(name)) {
        n++;
        name = `${base}${n}`;
      }
      names.add(name);
      return ref.children = ref.names = [name];
    });
  }
}
function processPlaceholders(statements) {
  const placeholderMap = /* @__PURE__ */ new Map();
  const liftedIfs = /* @__PURE__ */ new Set();
  gatherRecursiveAll(statements, ($8) => $8.type === "Placeholder").forEach((_exp) => {
    const exp = _exp;
    let ancestor;
    if (exp.subtype === ".") {
      ({ ancestor } = findAncestor(exp, ($9) => $9.type === "Call"));
      ancestor = ancestor?.parent;
      while (ancestor?.parent?.type === "UnaryExpression" || ancestor?.parent?.type === "NewExpression") {
        ancestor = ancestor.parent;
      }
      if (!ancestor) {
        replaceNode(exp, {
          type: "Error",
          message: "Partial placeholder . outside of call expression"
        });
        return;
      }
    } else {
      let child;
      ({ ancestor, child } = findAncestor(exp, (ancestor2, child2) => {
        const { type } = ancestor2;
        if (type === "IfStatement") {
          liftedIfs.add(ancestor2);
        }
        let m3;
        let m4;
        return type === "Call" || // Block, except for if/else blocks when condition already lifted
        type === "BlockStatement" && !((m3 = ancestor2.parent, typeof m3 === "object" && m3 != null && "type" in m3 && m3.type === "IfStatement") && liftedIfs.has(ancestor2.parent)) && !((m4 = ancestor2.parent, typeof m4 === "object" && m4 != null && "type" in m4 && m4.type === "ElseClause" && "parent" in m4 && typeof m4.parent === "object" && m4.parent != null && "type" in m4.parent && m4.parent.type === "IfStatement") && liftedIfs.has(ancestor2.parent.parent)) || type === "PipelineExpression" || // Declaration
        type === "Initializer" || // Right-hand side of assignment
        type === "AssignmentExpression" && findChildIndex(ancestor2, child2) === ancestor2.children.indexOf(ancestor2.expression) || type === "ReturnStatement" || type === "YieldExpression";
      }));
      switch (ancestor?.type) {
        case "Call": {
          const i = findChildIndex(ancestor.args, child);
          if (i >= 0) {
            ancestor.args[i] = maybeWrap(ancestor.args[i], ancestor);
            ancestor = ancestor.args[i];
          } else {
            ancestor = void 0;
          }
          ;
          break;
        }
        case "BlockStatement": {
          const i = findChildIndex(ancestor.expressions, child);
          if (i >= 0) {
            ancestor.expressions[i][1] = maybeWrap(ancestor.expressions[i][1], ancestor);
            ancestor = ancestor.expressions[i][1];
          } else {
            ancestor = void 0;
          }
          ;
          break;
        }
        case "PipelineExpression": {
          const i = findChildIndex(ancestor, child);
          if (i === 1) {
            ancestor = ancestor;
          } else if (i === 2) {
            const j = findChildIndex(ancestor.children[i], child);
            ancestor.children[i][j][3] = maybeWrap(ancestor.children[i][j][3], ancestor);
            ancestor = ancestor.children[i][j][3];
          } else {
            ancestor = void 0;
          }
          ;
          break;
        }
        case "AssignmentExpression":
        case "Initializer":
        case "ReturnStatement":
        case "YieldExpression": {
          const i = findChildIndex(ancestor, child);
          if (i >= 0 && ancestor.expression === ancestor.children[i]) {
            ancestor.expression = ancestor.children[i] = maybeWrap(ancestor.expression, ancestor);
            ancestor = ancestor.expression;
          } else {
            ancestor = void 0;
          }
          ;
          break;
        }
      }
      if (!ancestor) {
        replaceNode(exp, {
          type: "Error",
          message: "Ampersand placeholder & outside of block"
        });
      }
    }
    if (ancestor != null) {
      if (placeholderMap.has(ancestor)) {
        return placeholderMap.get(ancestor).push(exp);
      } else {
        return placeholderMap.set(ancestor, [exp]);
      }
    }
    ;
    return;
  });
  for (const [ancestor, placeholders] of placeholderMap) {
    let ref = makeRef("$");
    let typeSuffix;
    for (let i4 = 0, len3 = placeholders.length; i4 < len3; i4++) {
      const placeholder = placeholders[i4];
      typeSuffix ??= placeholder.typeSuffix;
      let ref12;
      replaceNode((ref12 = placeholder.children)[ref12.length - 1], ref);
    }
    const { parent } = ancestor;
    const body = maybeUnwrap(ancestor);
    let fnExp = makeAmpersandFunction({ ref, typeSuffix, body });
    let outer;
    switch (parent?.type) {
      case "Call": {
        outer = ancestor === parent.args[findChildIndex(parent.args, ancestor)];
        break;
      }
      case "BlockStatement": {
        outer = ancestor === parent.expressions[findChildIndex(parent.expressions, ancestor)][1];
        break;
      }
      case "PipelineExpression": {
        outer = ancestor === parent.children[2][findChildIndex(parent.children[2], ancestor)][3];
        break;
      }
      case "AssignmentExpression":
      case "Initializer":
      case "ReturnStatement":
      case "YieldExpression": {
        outer = ancestor === parent.expression;
        break;
      }
    }
    if (!outer) {
      fnExp = makeLeftHandSideExpression(fnExp);
    }
    replaceNode(ancestor, fnExp, parent);
    let ref13;
    if (ref13 = getTrimmingSpace(body)) {
      const ws = ref13;
      inplaceInsertTrimmingSpace(body, "");
      inplacePrepend(ws, fnExp);
    }
  }
  return;
}
function reorderBindingRestProperty(props) {
  const names = props.flatMap((p) => p.names);
  let restIndex = -1;
  let restCount = 0;
  props.forEach(({ type }, i) => {
    if (type === "BindingRestProperty") {
      if (restIndex < 0)
        restIndex = i;
      return restCount++;
    }
    ;
    return;
  });
  if (restCount === 0) {
    return {
      children: props,
      names
    };
  }
  let after = props.slice(restIndex + 1);
  let rest = props[restIndex];
  props = props.slice(0, restIndex);
  if (after.length) {
    const { delim: restDelim } = rest, lastAfterProp = after[after.length - 1], { delim: lastDelim, children: lastAfterChildren } = lastAfterProp;
    rest = {
      ...rest,
      delim: lastDelim,
      children: [...rest.children.slice(0, -1), lastDelim]
    };
    after = [
      ...after.slice(0, -1),
      {
        ...lastAfterProp,
        delim: restDelim,
        children: [...lastAfterChildren.slice(0, -1), restDelim]
      }
    ];
  }
  let ref14;
  if (Array.isArray(rest.delim) && (ref14 = rest.delim)[ref14.length - 1]?.token === ",") {
    rest.delim = rest.delim.slice(0, -1);
    rest.children = [...rest.children.slice(0, -1), rest.delim];
  }
  const children = [...props, ...after, rest];
  if (restCount > 1) {
    children.push({
      type: "Error",
      message: "Multiple rest properties in object pattern"
    });
  }
  return { children, names };
}
function replaceNodes(root, predicate, replacer) {
  if (!(root != null)) {
    return root;
  }
  const array = Array.isArray(root) ? root : root.children;
  if (!array) {
    if (predicate(root)) {
      return replacer(root, root);
    } else {
      return root;
    }
  }
  for (let i5 = 0, len4 = array.length; i5 < len4; i5++) {
    const i = i5;
    const node = array[i5];
    if (!(node != null)) {
      return;
    }
    if (predicate(node)) {
      array[i] = replacer(node, root);
    } else {
      replaceNodes(node, predicate, replacer);
    }
  }
  return root;
}
function replaceNodesRecursive(root, predicate, replacer) {
  if (!(root != null)) {
    return root;
  }
  const array = Array.isArray(root) ? root : root.children;
  if (!array) {
    if (predicate(root)) {
      return replacer(root, root);
    } else {
      return root;
    }
  }
  for (let i6 = 0, len5 = array.length; i6 < len5; i6++) {
    const i = i6;
    const node = array[i6];
    if (!(node != null)) {
      continue;
    }
    if (predicate(node)) {
      const ret = replacer(node, root);
      replaceNodesRecursive(ret, predicate, replacer);
      array[i] = ret;
    } else {
      replaceNodesRecursive(node, predicate, replacer);
    }
  }
  return root;
}
function typeOfJSX(node, config2) {
  switch (node.type) {
    case "JSXElement":
      return typeOfJSXElement(node, config2);
    case "JSXFragment":
      return typeOfJSXFragment(node, config2);
  }
}
function typeOfJSXElement(node, config2) {
  if (config2.solid) {
    if (config2.server && !config2.client) {
      return ["string"];
    }
    let { tag } = node;
    const clientType = tag[0] === tag[0].toLowerCase() ? [getHelperRef("IntrinsicElements"), '<"', tag, '">'] : ["ReturnType<typeof ", tag, ">"];
    if (config2.server) {
      return ["string", " | ", clientType];
    } else {
      return clientType;
    }
  }
  ;
  return;
}
function typeOfJSXFragment(node, config2) {
  if (config2.solid) {
    let type = [];
    let lastType;
    for (let child of node.jsxChildren) {
      switch (child.type) {
        case "JSXText":
          if (lastType !== "JSXText") {
            type.push("string");
          }
          break;
        case "JSXElement":
          type.push(typeOfJSXElement(child, config2));
          break;
        case "JSXFragment":
          type.push(...typeOfJSXFragment(child, config2));
          break;
        case "JSXChildExpression":
          if (child.expression) {
            type.push(["typeof ", child.expression]);
          }
          break;
        default:
          throw new Error(`unknown child in JSXFragment: ${JSON.stringify(child)}`);
      }
      lastType = child.type;
    }
    if (type.length === 1) {
      return type[0];
    } else {
      type = type.flatMap((t) => [t, ", "]);
      type.pop();
      return ["[", type, "]"];
    }
  }
  ;
  return;
}

// source/parser.hera
var grammar = {
  Program,
  TopLevelStatements,
  NestedTopLevelStatements,
  TopLevelSingleLineStatements,
  TopLevelStatement,
  ExtendedCommaExpression,
  ExtendedExpression,
  SingleLineExtendedExpression,
  NonPipelineExtendedExpression,
  NonAssignmentExtendedExpression,
  NestedNonAssignmentExtendedExpression,
  ExpressionizedStatementWithTrailingCallExpressions,
  ExpressionizedStatement,
  StatementExpression,
  CommaExpression,
  Arguments,
  ImplicitArguments,
  ExplicitArguments,
  ApplicationStart,
  ForbiddenImplicitCalls,
  ReservedBinary,
  ArgumentsWithTrailingMemberExpressions,
  TrailingMemberExpressions,
  AllowedTrailingMemberExpressions,
  TrailingCallExpressions,
  AllowedTrailingCallExpressions,
  CommaDelimiter,
  ArgumentList,
  NonPipelineArgumentList,
  NestedArgumentList,
  NestedArgument,
  SingleLineArgumentExpressions,
  ArgumentPart,
  NonPipelineArgumentPart,
  BinaryOpExpression,
  BinaryOpRHS,
  IsLike,
  WRHS,
  SingleLineBinaryOpRHS,
  RHS,
  UnaryExpression,
  UnaryWithoutParenthesizedAssignment,
  UnaryBody,
  UnaryWithoutParenthesizedAssignmentBody,
  ParenthesizedAssignment,
  UnaryPostfix,
  TypePostfix,
  Tuple,
  NWTypePostfix,
  UpdateExpression,
  UpdateExpressionSymbol,
  AssignmentExpression,
  NonPipelineAssignmentExpression,
  SingleLineAssignmentExpression,
  NonPipelineSingleLineAssignmentExpression,
  AssignmentExpressionTail,
  NonPipelineAssignmentExpressionTail,
  ActualAssignment,
  NonPipelineActualAssignment,
  YieldExpression,
  ArrowFunction,
  FatArrow,
  FatArrowToken,
  TrailingDeclaration,
  TrailingPipe,
  FatArrowBody,
  ConditionalExpression,
  TernaryRest,
  NestedTernaryRest,
  ShortCircuitExpression,
  PipelineExpression,
  PipelineHeadItem,
  PipelineTailItem,
  PrimaryExpression,
  ParenthesizedExpression,
  Placeholder,
  PlaceholderTypeSuffix,
  ClassDeclaration,
  ClassExpression,
  ClassBinding,
  ClassHeritage,
  ExtendsClause,
  WithClause,
  ExtendsToken,
  ExtendsShorthand,
  NotExtendsToken,
  OmittedNegation,
  ExtendsTarget,
  ImplementsClause,
  ImplementsToken,
  ImplementsShorthand,
  ImplementsTarget,
  ClassBody,
  NestedClassElements,
  NestedClassElement,
  ClassElement,
  ClassElementDefinition,
  ClassSignature,
  ClassSignatureBody,
  NestedClassSignatureElements,
  NestedClassSignatureElement,
  ClassSignatureElement,
  AccessModifier,
  FieldDefinition,
  ThisLiteral,
  HashThis,
  LengthShorthand,
  AtThis,
  LeftHandSideExpression,
  CallExpression,
  CallExpressionRest,
  OptionalShorthand,
  OptionalDot,
  NonNullAssertion,
  MemberExpression,
  ActualMemberExpression,
  MemberBase,
  MemberExpressionRest,
  MemberExpressionRestBody,
  MemberBracketContent,
  SliceParameters,
  AccessStart,
  ImplicitAccessStart,
  PropertyAccessModifier,
  PropertyAccess,
  PropertyGlob,
  PropertyBind,
  SuperProperty,
  MetaProperty,
  ReturnValue,
  AfterReturnShorthand,
  Parameters,
  ShortArrowParameters,
  ArrowParameters,
  NonEmptyParameters,
  ParameterList,
  NestedParameterList,
  NestedParameter,
  Parameter,
  FunctionRestParameter,
  ParameterElement,
  ParameterElementDelimiter,
  BindingIdentifier,
  NWBindingIdentifier,
  AtIdentifierRef,
  PinPattern,
  BindingPattern,
  ObjectBindingPattern,
  ObjectBindingPatternContent,
  BindingPropertyList,
  ArrayBindingPattern,
  ArrayBindingPatternContent,
  BindingElementList,
  NestedBindingElementList,
  Elision,
  NestedBindingProperties,
  NestedBindingPropertyList,
  BindingProperty,
  BindingRestProperty,
  NestedBindingElements,
  BindingElement,
  BindingRestElement,
  EmptyBindingPattern,
  FunctionDeclaration,
  FunctionSignature,
  FunctionExpression,
  OperatorDeclaration,
  OperatorSignature,
  OperatorBehavior,
  OperatorPrecedence,
  OperatorAssociativity,
  ThinArrowFunction,
  Arrow,
  ExplicitBlock,
  ImplicitNestedBlock,
  Block,
  BareNestedBlock,
  BareBlock,
  ThenClause,
  BracedThenClause,
  BracedOrEmptyBlock,
  NoCommaBracedOrEmptyBlock,
  NoPostfixBracedOrEmptyBlock,
  EmptyBlock,
  BlockOrEmptyStatement,
  BlockOrEmpty,
  EmptyStatementBareBlock,
  EmptyBareBlock,
  NoBlock,
  BracedBlock,
  NoPostfixBracedBlock,
  NoCommaBracedBlock,
  NonSingleBracedBlock,
  DeclarationOrStatement,
  SingleLineStatements,
  PostfixedSingleLineStatements,
  PostfixedSingleLineNoCommaStatements,
  BracedContent,
  NestedBlockStatements,
  NestedBlockStatement,
  BlockStatementPart,
  Literal,
  LiteralContent,
  NullLiteral,
  BooleanLiteral,
  _BooleanLiteral,
  CoffeeScriptBooleanLiteral,
  Identifier,
  IdentifierName,
  IdentifierReference,
  UpcomingAssignment,
  ArrayLiteral,
  _ArrayLiteral,
  RangeExpression,
  ArrayLiteralContent,
  NestedElementList,
  NestedElement,
  ArrayElementDelimiter,
  ElementListWithIndentedApplicationForbidden,
  ElementList,
  ElementListRest,
  ArrayElementExpression,
  ObjectLiteral,
  BracedObjectLiteral,
  BracedObjectLiteralContent,
  NestedImplicitObjectLiteral,
  NestedImplicitPropertyDefinitions,
  NestedImplicitPropertyDefinition,
  NestedPropertyDefinitions,
  NestedPropertyDefinition,
  ImplicitObjectLiteral,
  ImplicitObjectPropertyDelimiter,
  InlineObjectLiteral,
  InlineObjectPropertyDelimiter,
  ObjectPropertyDelimiter,
  PropertyDefinition,
  NamedProperty,
  SnugNamedProperty,
  PropertyName,
  ComputedPropertyName,
  Decorator,
  Decorators,
  MethodDefinition,
  MethodModifier,
  MethodSignature,
  ClassElementName,
  PrivateIdentifier,
  WAssignmentOp,
  AssignmentOp,
  OperatorAssignmentOp,
  AssignmentOpSymbol,
  CoffeeWordAssignmentOp,
  NotDedentedBinaryOp,
  IdentifierBinaryOp,
  BinaryOp,
  _BinaryOp,
  BinaryOpSymbol,
  ActualIn,
  CoffeeOfOp,
  NotOp,
  Xor,
  Xnor,
  UnaryOp,
  AwaitOp,
  ModuleItem,
  StatementListItem,
  PostfixedStatement,
  NoCommaStatementListItem,
  PostfixedNoCommaStatement,
  PostfixedExpression,
  PostfixedCommaExpression,
  NonPipelinePostfixedExpression,
  PostfixStatement,
  _PostfixStatement,
  Statement,
  NoCommaStatement,
  EmptyStatement,
  InsertEmptyStatement,
  BlockStatement,
  LabelledStatement,
  Label,
  LabelledItem,
  IfStatement,
  ElseClause,
  IfClause,
  IterationStatement,
  _IterationStatement,
  IterationExpression,
  LoopStatement,
  LoopClause,
  DoWhileStatement,
  DoStatement,
  ComptimeStatement,
  WhileStatement,
  WhileClause,
  ForStatement,
  ForClause,
  ForStatementControl,
  WhenCondition,
  CoffeeForStatementParameters,
  CoffeeForIndex,
  CoffeeForDeclaration,
  ForStatementParameters,
  ForRangeParameters,
  ForInOfDeclaration,
  ForDeclaration,
  ForBinding,
  SwitchStatement,
  EmptyCondition,
  CaseBlock,
  NestedCaseClauses,
  NestedCaseClause,
  CaseClause,
  PatternExpressionList,
  PatternExpression,
  CaseExpressionList,
  CaseExpression,
  ImpliedColon,
  IgnoreColon,
  TryStatement,
  CatchClause,
  CatchBind,
  FinallyClause,
  CatchParameter,
  Condition,
  DeclarationCondition,
  ExpressionWithIndentedApplicationForbidden,
  SingleLineExpressionWithIndentedApplicationForbidden,
  ExpressionWithObjectApplicationForbidden,
  LeftHandSideExpressionWithObjectApplicationForbidden,
  ForbidClassImplicitCall,
  AllowClassImplicitCall,
  RestoreClassImplicitCall,
  ClassImplicitCallForbidden,
  ForbidBracedApplication,
  AllowBracedApplication,
  RestoreBracedApplication,
  BracedApplicationAllowed,
  ForbidIndentedApplication,
  AllowIndentedApplication,
  RestoreIndentedApplication,
  IndentedApplicationAllowed,
  ForbidTrailingMemberProperty,
  AllowTrailingMemberProperty,
  RestoreTrailingMemberProperty,
  TrailingMemberPropertyAllowed,
  AllowNewlineBinaryOp,
  ForbidNewlineBinaryOp,
  RestoreNewlineBinaryOp,
  NewlineBinaryOpAllowed,
  AllowAll,
  RestoreAll,
  CommaExpressionStatement,
  ExpressionStatement,
  KeywordStatement,
  DebuggerStatement,
  ThrowStatement,
  Break,
  Continue,
  Debugger,
  MaybeNestedNonPipelineExtendedExpression,
  MaybeNestedPostfixedExpression,
  MaybeNestedExtendedExpression,
  MaybeParenNestedExtendedExpression,
  ImportDeclaration,
  ImpliedImport,
  ImportClause,
  NameSpaceImport,
  NamedImports,
  FromClause,
  ImportAssertion,
  TypeAndImportSpecifier,
  ImportSpecifier,
  OperatorImportSpecifier,
  ImportAsToken,
  ModuleExportName,
  ModuleSpecifier,
  UnprocessedModuleSpecifier,
  UnquotedSpecifier,
  ImportedBinding,
  ExportDeclaration,
  ExportVarDec,
  ExportFromClause,
  TypeAndNamedExports,
  NamedExports,
  ExportSpecifier,
  ImplicitExportSpecifier,
  Declaration,
  HoistableDeclaration,
  LexicalDeclaration,
  ConstAssignment,
  LetAssignment,
  TypeAssignment,
  LexicalBinding,
  Initializer,
  VariableStatement,
  VariableDeclarationList,
  NumericLiteral,
  NumericLiteralKind,
  DecimalBigIntegerLiteral,
  DecimalLiteral,
  ExponentPart,
  BinaryIntegerLiteral,
  OctalIntegerLiteral,
  HexIntegerLiteral,
  IntegerLiteral,
  IntegerLiteralKind,
  DecimalIntegerLiteral,
  StringLiteral,
  DoubleStringCharacters,
  SingleStringCharacters,
  TripleDoubleStringCharacters,
  TripleSingleStringCharacters,
  CoffeeStringSubstitution,
  CoffeeInterpolatedDoubleQuotedString,
  CoffeeDoubleQuotedStringCharacters,
  RegularExpressionLiteral,
  RegularExpressionClass,
  RegularExpressionClassCharacters,
  HeregexLiteral,
  HeregexBody,
  HeregexPart,
  HeregexComment,
  RegularExpressionBody,
  RegExpPart,
  RegExpCharacter,
  RegularExpressionFlags,
  TemplateLiteral,
  _TemplateLiteral,
  TemplateSubstitution,
  TemplateCharacters,
  TemplateBlockCharacters,
  ReservedWord,
  Comment,
  _Comment,
  SingleLineComment,
  JSSingleLineComment,
  MultiLineComment,
  JSMultiLineComment,
  CoffeeSingleLineComment,
  CoffeeMultiLineComment,
  CoffeeHereCommentStart,
  InlineComment,
  RestOfLine,
  TrailingComment,
  _,
  NonNewlineWhitespace,
  Trimmed_,
  __,
  Whitespace,
  ExpressionDelimiter,
  SimpleStatementDelimiter,
  StatementDelimiter,
  SemicolonDelimiter,
  NonIdContinue,
  Loc,
  Abstract,
  Ampersand,
  As,
  At,
  AtAt,
  Async,
  Await,
  Backtick,
  By,
  Caret,
  Case,
  Catch,
  Class,
  CloseAngleBracket,
  CloseBrace,
  CloseBracket,
  CloseParen,
  CoffeeSubstitutionStart,
  Colon,
  Comma,
  Comptime,
  ConstructorShorthand,
  Declare,
  Default,
  Delete,
  Do,
  Dot,
  DotDot,
  DotDotDot,
  DoubleColon,
  DoubleQuote,
  Each,
  Else,
  Equals,
  ExclamationPoint,
  Export,
  Extends,
  Finally,
  For,
  From,
  Function: Function2,
  GetOrSet,
  Hash,
  If,
  Import,
  In,
  Infer,
  LetOrConst,
  Const,
  Is,
  LetOrConstOrVar,
  Like,
  Loop,
  New,
  Not,
  Of,
  OpenAngleBracket,
  OpenBrace,
  OpenBracket,
  OpenParen,
  Operator,
  Override,
  Own,
  Public,
  Private,
  Protected,
  Pipe,
  QuestionMark,
  Readonly,
  Return,
  Satisfies,
  Semicolon,
  SingleQuote,
  Star,
  Static,
  SubstitutionStart,
  Super,
  Switch,
  Target,
  Then,
  This,
  Throw,
  TripleDoubleQuote,
  TripleSingleQuote,
  TripleSlash,
  TripleTick,
  Try,
  Typeof,
  Undefined,
  Unless,
  Until,
  Using,
  Var,
  Void,
  When,
  While,
  With,
  Yield,
  JSXImplicitFragment,
  JSXTag,
  _JSXTag,
  JSXElement,
  JSXSelfClosingElement,
  PushJSXOpeningElement,
  PopJSXStack,
  JSXOpeningElement,
  JSXOptionalClosingElement,
  JSXClosingElement,
  JSXFragment,
  PushJSXOpeningFragment,
  JSXOptionalClosingFragment,
  JSXClosingFragment,
  JSXElementName,
  JSXIdentifierName,
  JSXAttributes,
  JSXAttribute,
  JSXAttributeSpace,
  JSXShorthandString,
  JSXAttributeName,
  JSXAttributeInitializer,
  JSXAttributeValue,
  InlineJSXAttributeValue,
  InlineJSXBinaryOpRHS,
  InlineJSXUnaryExpression,
  InlineJSXUnaryOp,
  InlineJSXUnaryPostfix,
  InlineJSXUpdateExpression,
  InlineJSXCallExpression,
  InlineJSXCallExpressionRest,
  InlineJSXMemberExpression,
  InlineJSXMemberExpressionRest,
  InlineJSXPrimaryExpression,
  JSXMixedChildren,
  JSXChildren,
  JSXNestedChildren,
  JSXEOS,
  JSXNested,
  JSXChild,
  JSXComment,
  JSXCommentContent,
  JSXText,
  JSXChildExpression,
  IndentedJSXChildExpression,
  NestedJSXChildExpression,
  UsingDeclaration,
  UsingBinding,
  UsingJSModeError,
  TypeDeclaration,
  TypeDeclarationRest,
  TypeAliasDeclaration,
  InterfaceDeclaration,
  NamespaceDeclaration,
  OptionalEquals,
  TypeLexicalDeclaration,
  TypeDeclarationBinding,
  InterfaceExtendsClause,
  InterfaceExtendsTarget,
  TypeKeyword,
  Enum,
  Interface,
  Global,
  Module,
  Namespace,
  InterfaceBlock,
  NestedInterfaceBlock,
  NestedInterfaceProperties,
  NestedInterfaceProperty,
  InterfaceProperty,
  BasicInterfaceProperty,
  InterfacePropertyDelimiter,
  ModuleBlock,
  NestedModuleItems,
  NestedModuleItem,
  DeclareBlock,
  NestedDeclareElements,
  NestedDeclareElement,
  DeclareElement,
  EnumDeclaration,
  EnumBlock,
  NestedEnumProperties,
  NestedEnumPropertyLine,
  EnumProperty,
  TypeProperty,
  TypeIndexSignature,
  TypeIndex,
  TypeSuffix,
  MaybeNestedType,
  ReturnTypeSuffix,
  ReturnType,
  TypePredicate,
  Type,
  TypeBinary,
  TypeUnary,
  TypeUnarySuffix,
  TypeUnaryOp,
  TypeIndexedAccess,
  UnknownAlias,
  TypePrimary,
  ImportType,
  TypeTuple,
  TypeList,
  TypeElement,
  NestedTypeList,
  NestedType,
  TypeConditional,
  TypeCondition,
  TypeIfThenElse,
  TypeElse,
  TypeBlock,
  TypeTemplateSubstitution,
  TypeTemplateLiteral,
  CoffeeStringTypeSubstitution,
  CoffeeInterpolatedDoubleQuotedTypeLiteral,
  TypeLiteral,
  InlineInterfaceLiteral,
  InlineBasicInterfaceProperty,
  InlineInterfacePropertyDelimiter,
  TypeBinaryOp,
  TypeFunction,
  TypeArrowFunction,
  TypeArguments,
  TypeArgument,
  TypeArgumentDelimiter,
  TypeParameters,
  TypeParameter,
  TypeConstraint,
  TypeInitializer,
  TypeParameterDelimiter,
  ThisType,
  Shebang,
  CivetPrologue,
  CivetPrologueContent,
  CivetOption,
  UnknownPrologue,
  TripleSlashDirective,
  DirectivePrologue,
  PrologueString,
  EOS,
  EOL,
  DebugHere,
  InsertColon,
  InsertSemicolon,
  InsertOpenParen,
  InsertCloseParen,
  InsertOpenBrace,
  InsertInlineOpenBrace,
  InsertCloseBrace,
  InsertOpenBracket,
  InsertCloseBracket,
  InsertComma,
  InsertSpaceEquals,
  InsertConst,
  InsertLet,
  InsertReadonly,
  InsertNewline,
  InsertIndent,
  InsertSpace,
  InsertDot,
  InsertBreak,
  InsertVar,
  InsertType,
  CoffeeBinaryExistentialEnabled,
  CoffeeBooleansEnabled,
  CoffeeClassesEnabled,
  CoffeeCommentEnabled,
  CoffeeDoEnabled,
  CoffeeForLoopsEnabled,
  CoffeeInterpolationEnabled,
  CoffeeIsntEnabled,
  CoffeeJSXEnabled,
  CoffeeLineContinuationEnabled,
  CoffeeNotEnabled,
  CoffeeOfEnabled,
  CoffeePrototypeEnabled,
  ObjectIsEnabled,
  Reset,
  Init,
  Prologue,
  ProloguePrefix,
  Indent,
  TrackIndented,
  PushIndent,
  PopIndent,
  Nested,
  IndentedFurther,
  IndentedAtLeast,
  NotDedented,
  Dedented
};
var $L0 = (0, import_lib3.$L)("");
var $L1 = (0, import_lib3.$L)("{");
var $L2 = (0, import_lib3.$L)("/ ");
var $L3 = (0, import_lib3.$L)("=");
var $L4 = (0, import_lib3.$L)("(");
var $L5 = (0, import_lib3.$L)("... ");
var $L6 = (0, import_lib3.$L)("?");
var $L7 = (0, import_lib3.$L)(".");
var $L8 = (0, import_lib3.$L)("tuple");
var $L9 = (0, import_lib3.$L)("++");
var $L10 = (0, import_lib3.$L)("--");
var $L11 = (0, import_lib3.$L)("\u29FA");
var $L12 = (0, import_lib3.$L)("\u2014");
var $L13 = (0, import_lib3.$L)("=>");
var $L14 = (0, import_lib3.$L)("\u21D2");
var $L15 = (0, import_lib3.$L)("import");
var $L16 = (0, import_lib3.$L)(":");
var $L17 = (0, import_lib3.$L)(",");
var $L18 = (0, import_lib3.$L)(" ");
var $L19 = (0, import_lib3.$L)("<");
var $L20 = (0, import_lib3.$L)("implements");
var $L21 = (0, import_lib3.$L)("<:");
var $L22 = (0, import_lib3.$L)("^");
var $L23 = (0, import_lib3.$L)("-");
var $L24 = (0, import_lib3.$L)("import.meta");
var $L25 = (0, import_lib3.$L)("return.value");
var $L26 = (0, import_lib3.$L)("tighter");
var $L27 = (0, import_lib3.$L)("looser");
var $L28 = (0, import_lib3.$L)("same");
var $L29 = (0, import_lib3.$L)("left");
var $L30 = (0, import_lib3.$L)("right");
var $L31 = (0, import_lib3.$L)("non");
var $L32 = (0, import_lib3.$L)("relational");
var $L33 = (0, import_lib3.$L)("arguments");
var $L34 = (0, import_lib3.$L)("->");
var $L35 = (0, import_lib3.$L)("\u2192");
var $L36 = (0, import_lib3.$L)("}");
var $L37 = (0, import_lib3.$L)("null");
var $L38 = (0, import_lib3.$L)("true");
var $L39 = (0, import_lib3.$L)("false");
var $L40 = (0, import_lib3.$L)("yes");
var $L41 = (0, import_lib3.$L)("on");
var $L42 = (0, import_lib3.$L)("no");
var $L43 = (0, import_lib3.$L)("off");
var $L44 = (0, import_lib3.$L)(">");
var $L45 = (0, import_lib3.$L)("]");
var $L46 = (0, import_lib3.$L)("**=");
var $L47 = (0, import_lib3.$L)("*=");
var $L48 = (0, import_lib3.$L)("/=");
var $L49 = (0, import_lib3.$L)("%=");
var $L50 = (0, import_lib3.$L)("+=");
var $L51 = (0, import_lib3.$L)("-=");
var $L52 = (0, import_lib3.$L)("<<=");
var $L53 = (0, import_lib3.$L)(">>>=");
var $L54 = (0, import_lib3.$L)(">>=");
var $L55 = (0, import_lib3.$L)("&&=");
var $L56 = (0, import_lib3.$L)("&=");
var $L57 = (0, import_lib3.$L)("^=");
var $L58 = (0, import_lib3.$L)("||=");
var $L59 = (0, import_lib3.$L)("|=");
var $L60 = (0, import_lib3.$L)("??=");
var $L61 = (0, import_lib3.$L)("?=");
var $L62 = (0, import_lib3.$L)("and=");
var $L63 = (0, import_lib3.$L)("or=");
var $L64 = (0, import_lib3.$L)("*");
var $L65 = (0, import_lib3.$L)("**");
var $L66 = (0, import_lib3.$L)("/");
var $L67 = (0, import_lib3.$L)("%%");
var $L68 = (0, import_lib3.$L)("%");
var $L69 = (0, import_lib3.$L)("+");
var $L70 = (0, import_lib3.$L)("<=");
var $L71 = (0, import_lib3.$L)("\u2264");
var $L72 = (0, import_lib3.$L)(">=");
var $L73 = (0, import_lib3.$L)("\u2265");
var $L74 = (0, import_lib3.$L)("<?");
var $L75 = (0, import_lib3.$L)("!<?");
var $L76 = (0, import_lib3.$L)("<<");
var $L77 = (0, import_lib3.$L)("\xAB");
var $L78 = (0, import_lib3.$L)(">>>");
var $L79 = (0, import_lib3.$L)("\u22D9");
var $L80 = (0, import_lib3.$L)(">>");
var $L81 = (0, import_lib3.$L)("\xBB");
var $L82 = (0, import_lib3.$L)("!==");
var $L83 = (0, import_lib3.$L)("\u2262");
var $L84 = (0, import_lib3.$L)("!=");
var $L85 = (0, import_lib3.$L)("\u2260");
var $L86 = (0, import_lib3.$L)("isnt");
var $L87 = (0, import_lib3.$L)("===");
var $L88 = (0, import_lib3.$L)("\u2263");
var $L89 = (0, import_lib3.$L)("\u2A76");
var $L90 = (0, import_lib3.$L)("==");
var $L91 = (0, import_lib3.$L)("\u2261");
var $L92 = (0, import_lib3.$L)("\u2A75");
var $L93 = (0, import_lib3.$L)("and");
var $L94 = (0, import_lib3.$L)("&&");
var $L95 = (0, import_lib3.$L)("or");
var $L96 = (0, import_lib3.$L)("||");
var $L97 = (0, import_lib3.$L)("\u2016");
var $L98 = (0, import_lib3.$L)("^^");
var $L99 = (0, import_lib3.$L)("xor");
var $L100 = (0, import_lib3.$L)("xnor");
var $L101 = (0, import_lib3.$L)("??");
var $L102 = (0, import_lib3.$L)("\u2047");
var $L103 = (0, import_lib3.$L)("instanceof");
var $L104 = (0, import_lib3.$L)("\u2208");
var $L105 = (0, import_lib3.$L)("\u220B");
var $L106 = (0, import_lib3.$L)("\u220C");
var $L107 = (0, import_lib3.$L)("\u2209");
var $L108 = (0, import_lib3.$L)("&");
var $L109 = (0, import_lib3.$L)("|");
var $L110 = (0, import_lib3.$L)("$:");
var $L111 = (0, import_lib3.$L)(";");
var $L112 = (0, import_lib3.$L)("break");
var $L113 = (0, import_lib3.$L)("continue");
var $L114 = (0, import_lib3.$L)("debugger");
var $L115 = (0, import_lib3.$L)("require");
var $L116 = (0, import_lib3.$L)("with");
var $L117 = (0, import_lib3.$L)("assert");
var $L118 = (0, import_lib3.$L)(":=");
var $L119 = (0, import_lib3.$L)("\u2254");
var $L120 = (0, import_lib3.$L)(".=");
var $L121 = (0, import_lib3.$L)("::=");
var $L122 = (0, import_lib3.$L)("/*");
var $L123 = (0, import_lib3.$L)("*/");
var $L124 = (0, import_lib3.$L)("\\");
var $L125 = (0, import_lib3.$L)(")");
var $L126 = (0, import_lib3.$L)("abstract");
var $L127 = (0, import_lib3.$L)("as");
var $L128 = (0, import_lib3.$L)("@");
var $L129 = (0, import_lib3.$L)("@@");
var $L130 = (0, import_lib3.$L)("async");
var $L131 = (0, import_lib3.$L)("await");
var $L132 = (0, import_lib3.$L)("`");
var $L133 = (0, import_lib3.$L)("by");
var $L134 = (0, import_lib3.$L)("case");
var $L135 = (0, import_lib3.$L)("catch");
var $L136 = (0, import_lib3.$L)("class");
var $L137 = (0, import_lib3.$L)("\#{");
var $L138 = (0, import_lib3.$L)("comptime");
var $L139 = (0, import_lib3.$L)("declare");
var $L140 = (0, import_lib3.$L)("default");
var $L141 = (0, import_lib3.$L)("delete");
var $L142 = (0, import_lib3.$L)("do");
var $L143 = (0, import_lib3.$L)("..");
var $L144 = (0, import_lib3.$L)("\\u2025");
var $L145 = (0, import_lib3.$L)("...");
var $L146 = (0, import_lib3.$L)("\\u2026");
var $L147 = (0, import_lib3.$L)("::");
var $L148 = (0, import_lib3.$L)('\"');
var $L149 = (0, import_lib3.$L)("each");
var $L150 = (0, import_lib3.$L)("else");
var $L151 = (0, import_lib3.$L)("!");
var $L152 = (0, import_lib3.$L)("export");
var $L153 = (0, import_lib3.$L)("extends");
var $L154 = (0, import_lib3.$L)("finally");
var $L155 = (0, import_lib3.$L)("for");
var $L156 = (0, import_lib3.$L)("from");
var $L157 = (0, import_lib3.$L)("function");
var $L158 = (0, import_lib3.$L)("get");
var $L159 = (0, import_lib3.$L)("set");
var $L160 = (0, import_lib3.$L)("\#");
var $L161 = (0, import_lib3.$L)("if");
var $L162 = (0, import_lib3.$L)("in");
var $L163 = (0, import_lib3.$L)("infer");
var $L164 = (0, import_lib3.$L)("let");
var $L165 = (0, import_lib3.$L)("const");
var $L166 = (0, import_lib3.$L)("is");
var $L167 = (0, import_lib3.$L)("like");
var $L168 = (0, import_lib3.$L)("loop");
var $L169 = (0, import_lib3.$L)("new");
var $L170 = (0, import_lib3.$L)("not");
var $L171 = (0, import_lib3.$L)("of");
var $L172 = (0, import_lib3.$L)("[");
var $L173 = (0, import_lib3.$L)("operator");
var $L174 = (0, import_lib3.$L)("override");
var $L175 = (0, import_lib3.$L)("own");
var $L176 = (0, import_lib3.$L)("public");
var $L177 = (0, import_lib3.$L)("private");
var $L178 = (0, import_lib3.$L)("protected");
var $L179 = (0, import_lib3.$L)("||>");
var $L180 = (0, import_lib3.$L)("|\u25B7");
var $L181 = (0, import_lib3.$L)("|>=");
var $L182 = (0, import_lib3.$L)("\u25B7=");
var $L183 = (0, import_lib3.$L)("|>");
var $L184 = (0, import_lib3.$L)("\u25B7");
var $L185 = (0, import_lib3.$L)("readonly");
var $L186 = (0, import_lib3.$L)("return");
var $L187 = (0, import_lib3.$L)("satisfies");
var $L188 = (0, import_lib3.$L)("'");
var $L189 = (0, import_lib3.$L)("static");
var $L190 = (0, import_lib3.$L)("${");
var $L191 = (0, import_lib3.$L)("super");
var $L192 = (0, import_lib3.$L)("switch");
var $L193 = (0, import_lib3.$L)("target");
var $L194 = (0, import_lib3.$L)("then");
var $L195 = (0, import_lib3.$L)("this");
var $L196 = (0, import_lib3.$L)("throw");
var $L197 = (0, import_lib3.$L)('"""');
var $L198 = (0, import_lib3.$L)("'''");
var $L199 = (0, import_lib3.$L)("///");
var $L200 = (0, import_lib3.$L)("```");
var $L201 = (0, import_lib3.$L)("try");
var $L202 = (0, import_lib3.$L)("typeof");
var $L203 = (0, import_lib3.$L)("undefined");
var $L204 = (0, import_lib3.$L)("unless");
var $L205 = (0, import_lib3.$L)("until");
var $L206 = (0, import_lib3.$L)("using");
var $L207 = (0, import_lib3.$L)("var");
var $L208 = (0, import_lib3.$L)("void");
var $L209 = (0, import_lib3.$L)("when");
var $L210 = (0, import_lib3.$L)("while");
var $L211 = (0, import_lib3.$L)("yield");
var $L212 = (0, import_lib3.$L)("/>");
var $L213 = (0, import_lib3.$L)("</");
var $L214 = (0, import_lib3.$L)("<>");
var $L215 = (0, import_lib3.$L)("</>");
var $L216 = (0, import_lib3.$L)("<!--");
var $L217 = (0, import_lib3.$L)("-->");
var $L218 = (0, import_lib3.$L)("type");
var $L219 = (0, import_lib3.$L)("enum");
var $L220 = (0, import_lib3.$L)("interface");
var $L221 = (0, import_lib3.$L)("global");
var $L222 = (0, import_lib3.$L)("module");
var $L223 = (0, import_lib3.$L)("namespace");
var $L224 = (0, import_lib3.$L)("asserts");
var $L225 = (0, import_lib3.$L)("keyof");
var $L226 = (0, import_lib3.$L)("???");
var $L227 = (0, import_lib3.$L)("unique");
var $L228 = (0, import_lib3.$L)("symbol");
var $L229 = (0, import_lib3.$L)("[]");
var $L230 = (0, import_lib3.$L)("civet");
var $R0 = (0, import_lib3.$R)(new RegExp("(?=async|debugger|if|unless|comptime|do|for|loop|until|while|switch|throw|try)", "suy"));
var $R1 = (0, import_lib3.$R)(new RegExp("&(?=\\s)", "suy"));
var $R2 = (0, import_lib3.$R)(new RegExp("(as|of|satisfies|then|when|implements|xor|xnor)(?!\\p{ID_Continue}|[\\u200C\\u200D$])", "suy"));
var $R3 = (0, import_lib3.$R)(new RegExp("[0-9]", "suy"));
var $R4 = (0, import_lib3.$R)(new RegExp("(?!\\p{ID_Start}|[_$0-9(\\[{])", "suy"));
var $R5 = (0, import_lib3.$R)(new RegExp("[ \\t]", "suy"));
var $R6 = (0, import_lib3.$R)(new RegExp("(?:\\p{ID_Continue}|[\\u200C\\u200D$.#])", "suy"));
var $R7 = (0, import_lib3.$R)(new RegExp("[&=]", "suy"));
var $R8 = (0, import_lib3.$R)(new RegExp("(?=['\"`])", "suy"));
var $R9 = (0, import_lib3.$R)(new RegExp("(?=[\\/?])", "suy"));
var $R10 = (0, import_lib3.$R)(new RegExp("(?=[\\/\\[{?.!@#'\u2019:])", "suy"));
var $R11 = (0, import_lib3.$R)(new RegExp("[)}]", "suy"));
var $R12 = (0, import_lib3.$R)(new RegExp("[+-]", "suy"));
var $R13 = (0, import_lib3.$R)(new RegExp("\\+\\+|--|[\\+\\-&]\\S", "suy"));
var $R14 = (0, import_lib3.$R)(new RegExp(`(?=[0-9.'"tfyno])`, "suy"));
var $R15 = (0, import_lib3.$R)(new RegExp("(?=true|false|yes|no|on|off)", "suy"));
var $R16 = (0, import_lib3.$R)(new RegExp("(?=\\p{ID_Start}|[_$])", "suy"));
var $R17 = (0, import_lib3.$R)(new RegExp("(?:\\p{ID_Start}|[_$])(?:\\p{ID_Continue}|[\\u200C\\u200D$])*", "suy"));
var $R18 = (0, import_lib3.$R)(new RegExp("(?=\\[)", "suy"));
var $R19 = (0, import_lib3.$R)(new RegExp("[!+-]?", "suy"));
var $R20 = (0, import_lib3.$R)(new RegExp("(?=\\p{ID_Start}|[_$^\xAB\xBB\u22D9\u2264\u2265\u2208\u220B\u2209\u220C\u2263\u2261\u2262\u2260=\u2A76\u2A75\u2016\u2047&|*\\/!?%<>\u29FA+-])", "suy"));
var $R21 = (0, import_lib3.$R)(new RegExp("!\\^\\^?", "suy"));
var $R22 = (0, import_lib3.$R)(new RegExp("(?!\\+\\+|--)[!~+-](?!\\s)", "suy"));
var $R23 = (0, import_lib3.$R)(new RegExp("[:.]", "suy"));
var $R24 = (0, import_lib3.$R)(new RegExp("(?=for|if|loop|unless|until|while)", "suy"));
var $R25 = (0, import_lib3.$R)(new RegExp("(?=loop|comptime|do|for|until|while)", "suy"));
var $R26 = (0, import_lib3.$R)(new RegExp("(?=[\\s\\),])", "suy"));
var $R27 = (0, import_lib3.$R)(new RegExp('[^;"\\s]+', "suy"));
var $R28 = (0, import_lib3.$R)(new RegExp("(?=[0-9.])", "suy"));
var $R29 = (0, import_lib3.$R)(new RegExp("(?:0|[1-9](?:_[0-9]|[0-9])*)n", "suy"));
var $R30 = (0, import_lib3.$R)(new RegExp("(?:0|[1-9](?:_[0-9]|[0-9])*)(?=\\.(?:\\p{ID_Start}|[_$]))", "suy"));
var $R31 = (0, import_lib3.$R)(new RegExp("(?:0|[1-9](?:_[0-9]|[0-9])*)(?:\\.(?:[0-9](?:_[0-9]|[0-9])*))?", "suy"));
var $R32 = (0, import_lib3.$R)(new RegExp("(?:\\.[0-9](?:_[0-9]|[0-9])*)", "suy"));
var $R33 = (0, import_lib3.$R)(new RegExp("(?:[eE][+-]?[0-9]+(?:_[0-9]|[0-9])*)", "suy"));
var $R34 = (0, import_lib3.$R)(new RegExp("0[bB][01](?:[01]|_[01])*n?", "suy"));
var $R35 = (0, import_lib3.$R)(new RegExp("0[oO][0-7](?:[0-7]|_[0-7])*n?", "suy"));
var $R36 = (0, import_lib3.$R)(new RegExp("0[xX][0-9a-fA-F](?:[0-9a-fA-F]|_[0-9a-fA-F])*n?", "suy"));
var $R37 = (0, import_lib3.$R)(new RegExp("(?=[0-9])", "suy"));
var $R38 = (0, import_lib3.$R)(new RegExp("(?:0|[1-9](?:_[0-9]|[0-9])*)", "suy"));
var $R39 = (0, import_lib3.$R)(new RegExp('(?:\\\\.|[^"])*', "suy"));
var $R40 = (0, import_lib3.$R)(new RegExp("(?:\\\\.|[^'])*", "suy"));
var $R41 = (0, import_lib3.$R)(new RegExp('(?:"(?!"")|#(?!\\{)|\\\\.|[^#"])+', "suy"));
var $R42 = (0, import_lib3.$R)(new RegExp("(?:'(?!'')|\\\\.|[^'])*", "suy"));
var $R43 = (0, import_lib3.$R)(new RegExp('(?:\\\\.|#(?!\\{)|[^"\#])+', "suy"));
var $R44 = (0, import_lib3.$R)(new RegExp("(?:\\\\.|[^\\]])*", "suy"));
var $R45 = (0, import_lib3.$R)(new RegExp("(?:\\\\.)", "suy"));
var $R46 = (0, import_lib3.$R)(new RegExp("[\\s]+", "suy"));
var $R47 = (0, import_lib3.$R)(new RegExp("\\/(?!\\/\\/)", "suy"));
var $R48 = (0, import_lib3.$R)(new RegExp("[^[\\/\\s#\\\\]+", "suy"));
var $R49 = (0, import_lib3.$R)(new RegExp("[*\\/\\r\\n]", "suy"));
var $R50 = (0, import_lib3.$R)(new RegExp("(?:\\\\.|[^[\\/\\r\\n])+", "suy"));
var $R51 = (0, import_lib3.$R)(new RegExp("(?:\\p{ID_Continue}|[\\u200C\\u200D$])*", "suy"));
var $R52 = (0, import_lib3.$R)(new RegExp("(?=[`'\"])", "suy"));
var $R53 = (0, import_lib3.$R)(new RegExp("(?:\\$(?!\\{)|\\\\.|[^$`])+", "suy"));
var $R54 = (0, import_lib3.$R)(new RegExp("(?:\\$(?!\\{)|`(?!``)|\\\\.|[^$`])+", "suy"));
var $R55 = (0, import_lib3.$R)(new RegExp("(?:on|off|yes|no)(?!\\p{ID_Continue})", "suy"));
var $R56 = (0, import_lib3.$R)(new RegExp("(?:isnt)(?!\\p{ID_Continue})", "suy"));
var $R57 = (0, import_lib3.$R)(new RegExp("(?:by)(?!\\p{ID_Continue})", "suy"));
var $R58 = (0, import_lib3.$R)(new RegExp("(?:of)(?!\\p{ID_Continue})", "suy"));
var $R59 = (0, import_lib3.$R)(new RegExp("(?:and|await|break|case|catch|class|const|continue|debugger|default|delete|do|else|enum|export|extends|false|finally|for|function|if|import|in|instanceof|interface|is|let|loop|new|not|null|or|private|protected|public|return|static|super|switch|this|throw|true|try|typeof|unless|until|var|void|while|with|yield)(?!\\p{ID_Continue})", "suy"));
var $R60 = (0, import_lib3.$R)(new RegExp("(?=\\/|#)", "suy"));
var $R61 = (0, import_lib3.$R)(new RegExp("\\/\\/(?!\\/)[^\\r\\n]*", "suy"));
var $R62 = (0, import_lib3.$R)(new RegExp(".", "suy"));
var $R63 = (0, import_lib3.$R)(new RegExp("\#(?!##(?!#))([^\\r\\n]*)", "suy"));
var $R64 = (0, import_lib3.$R)(new RegExp("[^]*?###", "suy"));
var $R65 = (0, import_lib3.$R)(new RegExp("\###(?!#)", "suy"));
var $R66 = (0, import_lib3.$R)(new RegExp("\\/\\*(?:(?!\\*\\/)[^\\r\\n])*\\*\\/", "suy"));
var $R67 = (0, import_lib3.$R)(new RegExp("(?=[ \\t\\/\\\\])", "suy"));
var $R68 = (0, import_lib3.$R)(new RegExp("[ \\t]+", "suy"));
var $R69 = (0, import_lib3.$R)(new RegExp("(?=\\s|\\/|#)", "suy"));
var $R70 = (0, import_lib3.$R)(new RegExp("(?!\\p{ID_Continue})", "suy"));
var $R71 = (0, import_lib3.$R)(new RegExp("[=:]", "suy"));
var $R72 = (0, import_lib3.$R)(new RegExp("['\u2019]s", "suy"));
var $R73 = (0, import_lib3.$R)(new RegExp("\\s", "suy"));
var $R74 = (0, import_lib3.$R)(new RegExp("(?=[<])", "suy"));
var $R75 = (0, import_lib3.$R)(new RegExp("(?:\\p{ID_Start}|[_$])(?:\\p{ID_Continue}|[\\u200C\\u200D$-])*", "suy"));
var $R76 = (0, import_lib3.$R)(new RegExp("[!+-]", "suy"));
var $R77 = (0, import_lib3.$R)(new RegExp("[\\s>]|\\/>", "suy"));
var $R78 = (0, import_lib3.$R)(new RegExp("(?:[\\w\\-:]+|\\([^()]*\\)|\\[[^\\[\\]]*\\])+", "suy"));
var $R79 = (0, import_lib3.$R)(new RegExp(`"[^"]*"|'[^']*'`, "suy"));
var $R80 = (0, import_lib3.$R)(new RegExp("[<>]", "suy"));
var $R81 = (0, import_lib3.$R)(new RegExp("[!~+-](?!\\s|[!~+-]*&)", "suy"));
var $R82 = (0, import_lib3.$R)(new RegExp("(?:-[^-]|[^-]*)*", "suy"));
var $R83 = (0, import_lib3.$R)(new RegExp("[^{}<>\\r\\n]+", "suy"));
var $R84 = (0, import_lib3.$R)(new RegExp("[+-]?", "suy"));
var $R85 = (0, import_lib3.$R)(new RegExp("(?=if|unless)", "suy"));
var $R86 = (0, import_lib3.$R)(new RegExp("\#![^\\r\\n]*", "suy"));
var $R87 = (0, import_lib3.$R)(new RegExp("[\\t ]*", "suy"));
var $R88 = (0, import_lib3.$R)(new RegExp("[ \\t]*", "suy"));
var $R89 = (0, import_lib3.$R)(new RegExp("[\\s]*", "suy"));
var $R90 = (0, import_lib3.$R)(new RegExp("\\s+([+-]?)([a-zA-Z0-9-]+)(\\s*=\\s*([a-zA-Z0-9.+-]*))?", "suy"));
var $R91 = (0, import_lib3.$R)(new RegExp("\\/\\/\\/[^\\r\\n]*", "suy"));
var $R92 = (0, import_lib3.$R)(new RegExp("(?=[ \\t\\r\\n\\/#]|$)", "suy"));
var $R93 = (0, import_lib3.$R)(new RegExp("\\r\\n|\\n|\\r|$", "suy"));
var $R94 = (0, import_lib3.$R)(new RegExp("[^]*", "suy"));
var Program$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Reset, Init, (0, import_lib3.$E)(EOS), TopLevelStatements, __), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var reset = $1;
  var init = $2;
  var ws1 = $3;
  var statements = $4;
  var ws2 = $5;
  const program = {
    type: "BlockStatement",
    expressions: statements,
    children: [reset, init, ws1, statements, ws2],
    bare: true,
    root: true
  };
  processProgram(program);
  return program;
});
function Program(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Program", Program$0);
}
var TopLevelStatements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(TrackIndented, TopLevelSingleLineStatements, (0, import_lib3.$Q)(NestedTopLevelStatements), PopIndent), function($skip, $loc, $0, $1, $2, $3, $4) {
  var indent = $1;
  var first = $2;
  var rest = $3;
  return [
    [indent, ...first[0]],
    ...first.slice(1).map((s) => ["", ...s]),
    ...rest.flat()
  ];
});
var TopLevelStatements$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(TopLevelSingleLineStatements, (0, import_lib3.$Q)(NestedTopLevelStatements)), function($skip, $loc, $0, $1, $2) {
  var first = $1;
  var rest = $2;
  return [
    ...first.map((s) => ["", ...s]),
    ...rest.flat()
  ];
});
var TopLevelStatements$2 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L0, 'TopLevelStatements ""'), function(value) {
  return [];
});
var TopLevelStatements$$ = [TopLevelStatements$0, TopLevelStatements$1, TopLevelStatements$2];
function TopLevelStatements(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TopLevelStatements", TopLevelStatements$$);
}
var NestedTopLevelStatements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, TopLevelSingleLineStatements), function($skip, $loc, $0, $1, $2) {
  var nested = $1;
  var statements = $2;
  return [
    [nested, ...statements[0]],
    ...statements.slice(1).map((s) => ["", ...s])
  ];
});
function NestedTopLevelStatements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedTopLevelStatements", NestedTopLevelStatements$0);
}
var TopLevelSingleLineStatements$0 = (0, import_lib3.$P)(TopLevelStatement);
function TopLevelSingleLineStatements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TopLevelSingleLineStatements", TopLevelSingleLineStatements$0);
}
var TopLevelStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), (0, import_lib3.$E)(_), ModuleItem, StatementDelimiter), function($skip, $loc, $0, $1, $2, $3, $4) {
  var ws = $2;
  var statement = $3;
  var delimiter = $4;
  statement = prepend(ws, statement);
  return [statement, delimiter];
});
function TopLevelStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TopLevelStatement", TopLevelStatement$0);
}
var ExtendedCommaExpression$0 = NonAssignmentExtendedExpression;
var ExtendedCommaExpression$1 = CommaExpression;
var ExtendedCommaExpression$$ = [ExtendedCommaExpression$0, ExtendedCommaExpression$1];
function ExtendedCommaExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ExtendedCommaExpression", ExtendedCommaExpression$$);
}
var ExtendedExpression$0 = NonAssignmentExtendedExpression;
var ExtendedExpression$1 = AssignmentExpression;
var ExtendedExpression$$ = [ExtendedExpression$0, ExtendedExpression$1];
function ExtendedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ExtendedExpression", ExtendedExpression$$);
}
var SingleLineExtendedExpression$0 = NonAssignmentExtendedExpression;
var SingleLineExtendedExpression$1 = SingleLineAssignmentExpression;
var SingleLineExtendedExpression$$ = [SingleLineExtendedExpression$0, SingleLineExtendedExpression$1];
function SingleLineExtendedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "SingleLineExtendedExpression", SingleLineExtendedExpression$$);
}
var NonPipelineExtendedExpression$0 = NonAssignmentExtendedExpression;
var NonPipelineExtendedExpression$1 = NonPipelineAssignmentExpression;
var NonPipelineExtendedExpression$$ = [NonPipelineExtendedExpression$0, NonPipelineExtendedExpression$1];
function NonPipelineExtendedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NonPipelineExtendedExpression", NonPipelineExtendedExpression$$);
}
var NonAssignmentExtendedExpression$0 = NestedNonAssignmentExtendedExpression;
var NonAssignmentExtendedExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), ExpressionizedStatementWithTrailingCallExpressions), function($skip, $loc, $0, $1, $2) {
  return prepend($1, $2);
});
var NonAssignmentExtendedExpression$$ = [NonAssignmentExtendedExpression$0, NonAssignmentExtendedExpression$1];
function NonAssignmentExtendedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NonAssignmentExtendedExpression", NonAssignmentExtendedExpression$$);
}
var NestedNonAssignmentExtendedExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), PushIndent, (0, import_lib3.$E)((0, import_lib3.$S)(Nested, ExpressionizedStatementWithTrailingCallExpressions)), PopIndent, (0, import_lib3.$E)(AllowedTrailingCallExpressions)), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var expression = $3;
  var trailing = $5;
  if (!expression)
    return $skip;
  if (!trailing)
    return expression;
  return {
    type: "CallExpression",
    children: [expression, ...trailing.flat()]
  };
});
function NestedNonAssignmentExtendedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedNonAssignmentExtendedExpression", NestedNonAssignmentExtendedExpression$0);
}
var ExpressionizedStatementWithTrailingCallExpressions$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ExpressionizedStatement, (0, import_lib3.$E)(AllowedTrailingCallExpressions)), function($skip, $loc, $0, $1, $2) {
  if (!$2)
    return $1;
  return {
    type: "CallExpression",
    children: [
      makeLeftHandSideExpression($1),
      $2
    ]
  };
});
function ExpressionizedStatementWithTrailingCallExpressions(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExpressionizedStatementWithTrailingCallExpressions", ExpressionizedStatementWithTrailingCallExpressions$0);
}
var ExpressionizedStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R0, "ExpressionizedStatement /(?=async|debugger|if|unless|comptime|do|for|loop|until|while|switch|throw|try)/"), StatementExpression), function($skip, $loc, $0, $1, $2) {
  var statement = $2;
  return {
    type: "StatementExpression",
    statement,
    children: [statement]
  };
});
function ExpressionizedStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExpressionizedStatement", ExpressionizedStatement$0);
}
var StatementExpression$0 = DebuggerStatement;
var StatementExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(IfStatement), function($skip, $loc, $0, $1) {
  if (!$1.else && $1.then.implicit)
    return $skip;
  return $1;
});
var StatementExpression$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(IterationExpression), function($skip, $loc, $0, $1) {
  if ($1.block.implicit && $1.subtype !== "DoStatement" && $1.subtype !== "ComptimeStatement") {
    return $skip;
  }
  return $1;
});
var StatementExpression$3 = SwitchStatement;
var StatementExpression$4 = ThrowStatement;
var StatementExpression$5 = TryStatement;
var StatementExpression$$ = [StatementExpression$0, StatementExpression$1, StatementExpression$2, StatementExpression$3, StatementExpression$4, StatementExpression$5];
function StatementExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "StatementExpression", StatementExpression$$);
}
var CommaExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(AssignmentExpression, (0, import_lib3.$Q)((0, import_lib3.$S)(CommaDelimiter, AssignmentExpression))), function($skip, $loc, $0, $1, $2) {
  if ($2.length == 0)
    return $1;
  return $0;
});
function CommaExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CommaExpression", CommaExpression$0);
}
var Arguments$0 = ExplicitArguments;
var Arguments$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidTrailingMemberProperty, (0, import_lib3.$E)(ImplicitArguments), RestoreTrailingMemberProperty), function($skip, $loc, $0, $1, $2, $3) {
  var args = $2;
  if (args)
    return args;
  return $skip;
});
var Arguments$$ = [Arguments$0, Arguments$1];
function Arguments(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Arguments", Arguments$$);
}
var ImplicitArguments$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ApplicationStart, InsertOpenParen, (0, import_lib3.$E)(Trimmed_), NonPipelineArgumentList, InsertCloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var open = $2;
  var ws = $3;
  var args = $4;
  var close = $5;
  if (skipImplicitArguments(args))
    return $skip;
  return {
    type: "Call",
    args,
    children: [open, ws, args, close]
  };
});
function ImplicitArguments(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImplicitArguments", ImplicitArguments$0);
}
var ExplicitArguments$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenParen, (0, import_lib3.$E)((0, import_lib3.$S)(ArgumentList, (0, import_lib3.$E)((0, import_lib3.$S)(__, Comma)))), __, CloseParen), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $1;
  var args = $2;
  var ws = $3;
  var close = $4;
  if (args) {
    if (args[1]) {
      args = [...args[0], args[1]];
    } else {
      args = args[0];
    }
  } else {
    args = [];
  }
  return {
    type: "Call",
    args,
    children: [open, args, ws, close]
  };
});
function ExplicitArguments(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExplicitArguments", ExplicitArguments$0);
}
var ApplicationStart$0 = (0, import_lib3.$S)(IndentedApplicationAllowed, (0, import_lib3.$Y)((0, import_lib3.$S)(IndentedFurther, (0, import_lib3.$N)(IdentifierBinaryOp), (0, import_lib3.$N)(AccessStart))));
var ApplicationStart$1 = (0, import_lib3.$S)((0, import_lib3.$N)(EOS), (0, import_lib3.$Y)((0, import_lib3.$S)(_, (0, import_lib3.$C)(BracedApplicationAllowed, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($L1, 'ApplicationStart "{"'))), (0, import_lib3.$N)(ForbiddenImplicitCalls))));
var ApplicationStart$$ = [ApplicationStart$0, ApplicationStart$1];
function ApplicationStart(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ApplicationStart", ApplicationStart$$);
}
var ForbiddenImplicitCalls$0 = ReservedBinary;
var ForbiddenImplicitCalls$1 = (0, import_lib3.$EXPECT)($L2, 'ForbiddenImplicitCalls "/ "');
var ForbiddenImplicitCalls$2 = (0, import_lib3.$S)((0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R1, "ForbiddenImplicitCalls /&(?=\\s)/")), (0, import_lib3.$N)((0, import_lib3.$S)(NotDedented, (0, import_lib3.$C)(Ampersand, ReservedBinary))), (0, import_lib3.$C)(IndentedFurther, (0, import_lib3.$N)(EOS)))), BinaryOpRHS);
var ForbiddenImplicitCalls$3 = (0, import_lib3.$S)(ClassImplicitCallForbidden, (0, import_lib3.$C)(Class, AtAt));
var ForbiddenImplicitCalls$4 = (0, import_lib3.$S)(Identifier, (0, import_lib3.$EXPECT)($L3, 'ForbiddenImplicitCalls "="'), Whitespace);
var ForbiddenImplicitCalls$5 = (0, import_lib3.$TS)((0, import_lib3.$S)(Identifier, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($L4, 'ForbiddenImplicitCalls "("'))), function($skip, $loc, $0, $1, $2) {
  var id = $1;
  if (state.operators.has(id.name))
    return $0;
  return $skip;
});
var ForbiddenImplicitCalls$6 = (0, import_lib3.$TS)((0, import_lib3.$S)(OmittedNegation, (0, import_lib3.$E)(_), Identifier), function($skip, $loc, $0, $1, $2, $3) {
  var id = $3;
  if (state.operators.has(id.name))
    return $0;
  return $skip;
});
var ForbiddenImplicitCalls$7 = (0, import_lib3.$S)(PostfixStatement, NoBlock);
var ForbiddenImplicitCalls$8 = (0, import_lib3.$EXPECT)($L5, 'ForbiddenImplicitCalls "... "');
var ForbiddenImplicitCalls$$ = [ForbiddenImplicitCalls$0, ForbiddenImplicitCalls$1, ForbiddenImplicitCalls$2, ForbiddenImplicitCalls$3, ForbiddenImplicitCalls$4, ForbiddenImplicitCalls$5, ForbiddenImplicitCalls$6, ForbiddenImplicitCalls$7, ForbiddenImplicitCalls$8];
function ForbiddenImplicitCalls(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ForbiddenImplicitCalls", ForbiddenImplicitCalls$$);
}
var ReservedBinary$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R2, "ReservedBinary /(as|of|satisfies|then|when|implements|xor|xnor)(?!\\p{ID_Continue}|[\\u200C\\u200D$])/"));
function ReservedBinary(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ReservedBinary", ReservedBinary$0);
}
var ArgumentsWithTrailingMemberExpressions$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Arguments, AllowedTrailingMemberExpressions), function($skip, $loc, $0, $1, $2) {
  var args = $1;
  var trailing = $2;
  return [args, ...trailing];
});
function ArgumentsWithTrailingMemberExpressions(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ArgumentsWithTrailingMemberExpressions", ArgumentsWithTrailingMemberExpressions$0);
}
var TrailingMemberExpressions$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)(MemberExpressionRest), (0, import_lib3.$Q)((0, import_lib3.$S)(IndentedAtLeast, (0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$EXPECT)($L6, 'TrailingMemberExpressions "?"')), (0, import_lib3.$EXPECT)($L7, 'TrailingMemberExpressions "."'), (0, import_lib3.$N)((0, import_lib3.$EXPECT)($R3, "TrailingMemberExpressions /[0-9]/")))), MemberExpressionRest))), function($skip, $loc, $0, $1, $2) {
  return $1.concat($2.map(([ws, , memberExpressionRest]) => {
    if (Array.isArray(memberExpressionRest)) {
      return [ws, ...memberExpressionRest];
    }
    return {
      ...memberExpressionRest,
      children: [ws, ...memberExpressionRest.children]
    };
  }));
});
function TrailingMemberExpressions(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TrailingMemberExpressions", TrailingMemberExpressions$0);
}
var AllowedTrailingMemberExpressions$0 = (0, import_lib3.$T)((0, import_lib3.$S)(TrailingMemberPropertyAllowed, TrailingMemberExpressions), function(value) {
  return value[1];
});
var AllowedTrailingMemberExpressions$1 = (0, import_lib3.$Q)(MemberExpressionRest);
var AllowedTrailingMemberExpressions$$ = [AllowedTrailingMemberExpressions$0, AllowedTrailingMemberExpressions$1];
function AllowedTrailingMemberExpressions(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "AllowedTrailingMemberExpressions", AllowedTrailingMemberExpressions$$);
}
var TrailingCallExpressions$0 = (0, import_lib3.$P)((0, import_lib3.$S)(IndentedAtLeast, (0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$EXPECT)($L6, 'TrailingCallExpressions "?"')), (0, import_lib3.$EXPECT)($L7, 'TrailingCallExpressions "."'), (0, import_lib3.$N)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R3, "TrailingCallExpressions /[0-9]/"))))), (0, import_lib3.$P)(CallExpressionRest)));
function TrailingCallExpressions(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TrailingCallExpressions", TrailingCallExpressions$0);
}
var AllowedTrailingCallExpressions$0 = (0, import_lib3.$T)((0, import_lib3.$S)(TrailingMemberPropertyAllowed, TrailingCallExpressions), function(value) {
  return value[1];
});
function AllowedTrailingCallExpressions(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AllowedTrailingCallExpressions", AllowedTrailingCallExpressions$0);
}
var CommaDelimiter$0 = (0, import_lib3.$S)(NotDedented, Comma);
function CommaDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CommaDelimiter", CommaDelimiter$0);
}
var ArgumentList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ArgumentPart, (0, import_lib3.$Q)((0, import_lib3.$S)(CommaDelimiter, (0, import_lib3.$N)(EOS), (0, import_lib3.$E)(_), ArgumentPart)), (0, import_lib3.$P)((0, import_lib3.$S)(CommaDelimiter, (0, import_lib3.$C)(NestedImplicitObjectLiteral, NestedArgumentList)))), function($skip, $loc, $0, $1, $2, $3) {
  return [
    $1,
    ...$2.flatMap(([comma, eos, ws, arg]) => [comma, prepend(ws, arg)]),
    ...$3.flatMap(
      ([comma, args]) => Array.isArray(args) ? [comma, ...args] : [comma, args]
    )
  ];
});
var ArgumentList$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(NestedImplicitObjectLiteral), function($skip, $loc, $0, $1) {
  return [insertTrimmingSpace($1, "")];
});
var ArgumentList$2 = NestedArgumentList;
var ArgumentList$3 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), ArgumentPart, (0, import_lib3.$Q)((0, import_lib3.$S)(CommaDelimiter, (0, import_lib3.$E)(_), ArgumentPart))), function($skip, $loc, $0, $1, $2, $3) {
  return [
    prepend($1, $2),
    ...$3.flatMap(([comma, ws, arg]) => [comma, prepend(ws, arg)])
  ];
});
var ArgumentList$$ = [ArgumentList$0, ArgumentList$1, ArgumentList$2, ArgumentList$3];
function ArgumentList(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ArgumentList", ArgumentList$$);
}
var NonPipelineArgumentList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), NonPipelineArgumentPart, (0, import_lib3.$Q)((0, import_lib3.$S)(CommaDelimiter, (0, import_lib3.$N)(EOS), (0, import_lib3.$E)(_), NonPipelineArgumentPart)), (0, import_lib3.$P)((0, import_lib3.$S)(CommaDelimiter, (0, import_lib3.$C)(NestedImplicitObjectLiteral, NestedArgumentList)))), function($skip, $loc, $0, $1, $2, $3, $4) {
  return [
    $2,
    ...$3.flatMap(([comma, eos, ws, arg]) => [comma, prepend(ws, arg)]),
    ...$4.flatMap(
      ([comma, args]) => Array.isArray(args) ? [comma, ...args] : [comma, args]
    )
  ];
});
var NonPipelineArgumentList$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(NestedImplicitObjectLiteral, (0, import_lib3.$Q)((0, import_lib3.$S)(CommaDelimiter, (0, import_lib3.$C)(NestedImplicitObjectLiteral, NestedArgumentList)))), function($skip, $loc, $0, $1, $2) {
  return [
    insertTrimmingSpace($1, ""),
    ...$2.flatMap(
      ([comma, args]) => Array.isArray(args) ? [comma, ...args] : [comma, args]
    )
  ];
});
var NonPipelineArgumentList$2 = NestedArgumentList;
var NonPipelineArgumentList$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(NonPipelineArgumentPart, (0, import_lib3.$Q)((0, import_lib3.$S)(CommaDelimiter, (0, import_lib3.$E)(_), NonPipelineArgumentPart))), function($skip, $loc, $0, $1, $2) {
  return [
    $1,
    ...$2.flatMap(([comma, ws, arg]) => [comma, prepend(ws, arg)])
  ];
});
var NonPipelineArgumentList$$ = [NonPipelineArgumentList$0, NonPipelineArgumentList$1, NonPipelineArgumentList$2, NonPipelineArgumentList$3];
function NonPipelineArgumentList(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NonPipelineArgumentList", NonPipelineArgumentList$$);
}
var NestedArgumentList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedArgument), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var args = $2;
  if (!args.length)
    return $skip;
  return args.flat();
});
function NestedArgumentList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedArgumentList", NestedArgumentList$0);
}
var NestedArgument$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, SingleLineArgumentExpressions, ParameterElementDelimiter), function($skip, $loc, $0, $1, $2, $3) {
  var indent = $1;
  var args = $2;
  var comma = $3;
  let [arg0, ...rest] = args;
  arg0 = [indent, ...arg0];
  return [arg0, ...rest, comma];
});
function NestedArgument(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedArgument", NestedArgument$0);
}
var SingleLineArgumentExpressions$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), ArgumentPart), (0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), Comma), (0, import_lib3.$S)((0, import_lib3.$E)(_), ArgumentPart)))), function($skip, $loc, $0, $1, $2) {
  return [$1, ...$2.flat()];
});
function SingleLineArgumentExpressions(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SingleLineArgumentExpressions", SingleLineArgumentExpressions$0);
}
var ArgumentPart$0 = (0, import_lib3.$S)(DotDotDot, ExtendedExpression);
var ArgumentPart$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(ExtendedExpression, (0, import_lib3.$E)(DotDotDot)), function($skip, $loc, $0, $1, $2) {
  if ($2) {
    return [$2, $1];
  }
  return $1;
});
var ArgumentPart$$ = [ArgumentPart$0, ArgumentPart$1];
function ArgumentPart(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ArgumentPart", ArgumentPart$$);
}
var NonPipelineArgumentPart$0 = (0, import_lib3.$S)(DotDotDot, NonPipelineExtendedExpression);
var NonPipelineArgumentPart$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(NonPipelineExtendedExpression, (0, import_lib3.$E)(DotDotDot)), function($skip, $loc, $0, $1, $2) {
  if ($2) {
    return [$2, $1];
  }
  return $1;
});
var NonPipelineArgumentPart$$ = [NonPipelineArgumentPart$0, NonPipelineArgumentPart$1];
function NonPipelineArgumentPart(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NonPipelineArgumentPart", NonPipelineArgumentPart$$);
}
var BinaryOpExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(UnaryExpression, (0, import_lib3.$Q)(BinaryOpRHS)), function($skip, $loc, $0, $1, $2) {
  if (!$2.length)
    return $1;
  return processBinaryOpExpression($0);
});
function BinaryOpExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BinaryOpExpression", BinaryOpExpression$0);
}
var BinaryOpRHS$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(NotDedented, IsLike, (0, import_lib3.$E)(_), PatternExpressionList), function($skip, $loc, $0, $1, $2, $3, $4) {
  var ws1 = $1;
  var op = $2;
  var ws2 = $3;
  var patterns = $4;
  return [ws1, op, ws2, patterns];
});
var BinaryOpRHS$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(BinaryOp, RHS), function($skip, $loc, $0, $1, $2) {
  var op = $1;
  var rhs = $2;
  return [[], op, [], rhs];
});
var BinaryOpRHS$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(NewlineBinaryOpAllowed, NotDedentedBinaryOp, WRHS), function($skip, $loc, $0, $1, $2, $3) {
  var op = $2;
  var rhs = $3;
  return [...op, ...rhs];
});
var BinaryOpRHS$3 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(NewlineBinaryOpAllowed), SingleLineBinaryOpRHS), function(value) {
  return value[1];
});
var BinaryOpRHS$$ = [BinaryOpRHS$0, BinaryOpRHS$1, BinaryOpRHS$2, BinaryOpRHS$3];
function BinaryOpRHS(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BinaryOpRHS", BinaryOpRHS$$);
}
var IsLike$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Is, (0, import_lib3.$E)(_), (0, import_lib3.$E)((0, import_lib3.$S)(OmittedNegation, (0, import_lib3.$E)(_))), Like), function($skip, $loc, $0, $1, $2, $3, $4) {
  var not = $3;
  return {
    type: "PatternTest",
    children: $0,
    special: true,
    negated: !!not
  };
});
function IsLike(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IsLike", IsLike$0);
}
var WRHS$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$S)(Nested, (0, import_lib3.$E)(_)), RHS)), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var wrhs = $2;
  if (!wrhs)
    return $skip;
  return wrhs;
});
var WRHS$1 = (0, import_lib3.$S)((0, import_lib3.$C)((0, import_lib3.$S)(EOS, __), _), RHS);
var WRHS$$ = [WRHS$0, WRHS$1];
function WRHS(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "WRHS", WRHS$$);
}
var SingleLineBinaryOpRHS$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), BinaryOp, (0, import_lib3.$C)((0, import_lib3.$S)(EOS, __), _), RHS), function($skip, $loc, $0, $1, $2, $3, $4) {
  var ws1 = $1;
  var op = $2;
  var ws2 = $3;
  var rhs = $4;
  return [ws1 || [], op, ws2, rhs];
});
function SingleLineBinaryOpRHS(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SingleLineBinaryOpRHS", SingleLineBinaryOpRHS$0);
}
var RHS$0 = ExpressionizedStatementWithTrailingCallExpressions;
var RHS$1 = UnaryExpression;
var RHS$$ = [RHS$0, RHS$1];
function RHS(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "RHS", RHS$$);
}
var UnaryExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)(UnaryOp), UnaryBody, (0, import_lib3.$E)(UnaryPostfix)), function($skip, $loc, $0, $1, $2, $3) {
  var pre = $1;
  var exp = $2;
  var post = $3;
  return processUnaryExpression(pre, exp, post);
});
var UnaryExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeeDoEnabled, Do, __, (0, import_lib3.$C)(ArrowFunction, (0, import_lib3.$S)(LeftHandSideExpression, (0, import_lib3.$N)((0, import_lib3.$S)(__, AssignmentOpSymbol))), ExtendedExpression)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var ws = $3;
  var exp = $4;
  return processCoffeeDo(ws, exp);
});
var UnaryExpression$$ = [UnaryExpression$0, UnaryExpression$1];
function UnaryExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "UnaryExpression", UnaryExpression$$);
}
var UnaryWithoutParenthesizedAssignment$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)(UnaryOp), UnaryWithoutParenthesizedAssignmentBody, (0, import_lib3.$E)(UnaryPostfix)), function($skip, $loc, $0, $1, $2, $3) {
  var pre = $1;
  var exp = $2;
  var post = $3;
  return processUnaryExpression(pre, exp, post);
});
function UnaryWithoutParenthesizedAssignment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "UnaryWithoutParenthesizedAssignment", UnaryWithoutParenthesizedAssignment$0);
}
var UnaryBody$0 = ParenthesizedAssignment;
var UnaryBody$1 = UpdateExpression;
var UnaryBody$2 = ExpressionizedStatementWithTrailingCallExpressions;
var UnaryBody$3 = NestedNonAssignmentExtendedExpression;
var UnaryBody$$ = [UnaryBody$0, UnaryBody$1, UnaryBody$2, UnaryBody$3];
function UnaryBody(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "UnaryBody", UnaryBody$$);
}
var UnaryWithoutParenthesizedAssignmentBody$0 = UpdateExpression;
var UnaryWithoutParenthesizedAssignmentBody$1 = ExpressionizedStatementWithTrailingCallExpressions;
var UnaryWithoutParenthesizedAssignmentBody$2 = NestedNonAssignmentExtendedExpression;
var UnaryWithoutParenthesizedAssignmentBody$$ = [UnaryWithoutParenthesizedAssignmentBody$0, UnaryWithoutParenthesizedAssignmentBody$1, UnaryWithoutParenthesizedAssignmentBody$2];
function UnaryWithoutParenthesizedAssignmentBody(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "UnaryWithoutParenthesizedAssignmentBody", UnaryWithoutParenthesizedAssignmentBody$$);
}
var ParenthesizedAssignment$0 = (0, import_lib3.$S)(InsertOpenParen, (0, import_lib3.$C)(ActualAssignment, ArrowFunction), InsertCloseParen);
function ParenthesizedAssignment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ParenthesizedAssignment", ParenthesizedAssignment$0);
}
var UnaryPostfix$0 = QuestionMark;
var UnaryPostfix$1 = (0, import_lib3.$P)(TypePostfix);
var UnaryPostfix$$ = [UnaryPostfix$0, UnaryPostfix$1];
function UnaryPostfix(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "UnaryPostfix", UnaryPostfix$$);
}
var TypePostfix$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(_, NWTypePostfix), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var postfix = $2;
  return prepend(ws, postfix);
});
function TypePostfix(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypePostfix", TypePostfix$0);
}
var Tuple$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L8, 'Tuple "tuple"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return {
    $loc,
    token: "readonly unknown[] | []"
  };
});
function Tuple(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Tuple", Tuple$0);
}
var NWTypePostfix$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(As, _, Tuple), function($skip, $loc, $0, $1, $2, $3) {
  return {
    ts: true,
    children: [{ $loc: $1.$loc, token: "satisfies" }, $2, $3]
  };
});
var NWTypePostfix$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(As, (0, import_lib3.$E)(ExclamationPoint), Type), function($skip, $loc, $0, $1, $2, $3) {
  var as = $1;
  var ex = $2;
  var type = $3;
  let children;
  if (ex) {
    children = [{ $loc: ex.$loc, token: "as unknown " }, as, type];
  } else {
    children = [as, type];
  }
  return { ts: true, children };
});
var NWTypePostfix$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(Satisfies, Type), function($skip, $loc, $0, $1, $2) {
  return { ts: true, children: $0 };
});
var NWTypePostfix$$ = [NWTypePostfix$0, NWTypePostfix$1, NWTypePostfix$2];
function NWTypePostfix(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NWTypePostfix", NWTypePostfix$$);
}
var UpdateExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(UpdateExpressionSymbol, UnaryWithoutParenthesizedAssignment), function($skip, $loc, $0, $1, $2) {
  return {
    type: "UpdateExpression",
    assigned: $2,
    children: $0
  };
});
var UpdateExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(LeftHandSideExpression, (0, import_lib3.$E)((0, import_lib3.$S)(UpdateExpressionSymbol, (0, import_lib3.$EXPECT)($R4, "UpdateExpression /(?!\\p{ID_Start}|[_$0-9(\\[{])/")))), function($skip, $loc, $0, $1, $2) {
  if (!$2)
    return $1;
  return {
    type: "UpdateExpression",
    assigned: $1,
    children: [$1, $2[0]]
  };
});
var UpdateExpression$$ = [UpdateExpression$0, UpdateExpression$1];
function UpdateExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "UpdateExpression", UpdateExpression$$);
}
var UpdateExpressionSymbol$0 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L9, 'UpdateExpressionSymbol "++"'), (0, import_lib3.$EXPECT)($L10, 'UpdateExpressionSymbol "--"')), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
var UpdateExpressionSymbol$1 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L11, 'UpdateExpressionSymbol "\u29FA"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "++" };
});
var UpdateExpressionSymbol$2 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L12, 'UpdateExpressionSymbol "\u2014"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "--" };
});
var UpdateExpressionSymbol$$ = [UpdateExpressionSymbol$0, UpdateExpressionSymbol$1, UpdateExpressionSymbol$2];
function UpdateExpressionSymbol(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "UpdateExpressionSymbol", UpdateExpressionSymbol$$);
}
var AssignmentExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), ActualAssignment), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var assign = $2;
  return prepend(ws, assign);
});
var AssignmentExpression$1 = PipelineExpression;
var AssignmentExpression$2 = SingleLineAssignmentExpression;
var AssignmentExpression$3 = (0, import_lib3.$S)((0, import_lib3.$S)(EOS, (0, import_lib3.$E)(_)), AssignmentExpressionTail);
var AssignmentExpression$$ = [AssignmentExpression$0, AssignmentExpression$1, AssignmentExpression$2, AssignmentExpression$3];
function AssignmentExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "AssignmentExpression", AssignmentExpression$$);
}
var NonPipelineAssignmentExpression$0 = NonPipelineSingleLineAssignmentExpression;
var NonPipelineAssignmentExpression$1 = (0, import_lib3.$S)((0, import_lib3.$S)(EOS, (0, import_lib3.$E)(_)), NonPipelineAssignmentExpressionTail);
var NonPipelineAssignmentExpression$$ = [NonPipelineAssignmentExpression$0, NonPipelineAssignmentExpression$1];
function NonPipelineAssignmentExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NonPipelineAssignmentExpression", NonPipelineAssignmentExpression$$);
}
var SingleLineAssignmentExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), AssignmentExpressionTail), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var tail = $2;
  return prepend(ws, tail);
});
function SingleLineAssignmentExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SingleLineAssignmentExpression", SingleLineAssignmentExpression$0);
}
var NonPipelineSingleLineAssignmentExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), NonPipelineAssignmentExpressionTail), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var tail = $2;
  return prepend(ws, tail);
});
function NonPipelineSingleLineAssignmentExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NonPipelineSingleLineAssignmentExpression", NonPipelineSingleLineAssignmentExpression$0);
}
var AssignmentExpressionTail$0 = YieldExpression;
var AssignmentExpressionTail$1 = ArrowFunction;
var AssignmentExpressionTail$2 = ActualAssignment;
var AssignmentExpressionTail$3 = ConditionalExpression;
var AssignmentExpressionTail$$ = [AssignmentExpressionTail$0, AssignmentExpressionTail$1, AssignmentExpressionTail$2, AssignmentExpressionTail$3];
function AssignmentExpressionTail(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "AssignmentExpressionTail", AssignmentExpressionTail$$);
}
var NonPipelineAssignmentExpressionTail$0 = YieldExpression;
var NonPipelineAssignmentExpressionTail$1 = ArrowFunction;
var NonPipelineAssignmentExpressionTail$2 = NonPipelineActualAssignment;
var NonPipelineAssignmentExpressionTail$3 = ConditionalExpression;
var NonPipelineAssignmentExpressionTail$$ = [NonPipelineAssignmentExpressionTail$0, NonPipelineAssignmentExpressionTail$1, NonPipelineAssignmentExpressionTail$2, NonPipelineAssignmentExpressionTail$3];
function NonPipelineAssignmentExpressionTail(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NonPipelineAssignmentExpressionTail", NonPipelineAssignmentExpressionTail$$);
}
var ActualAssignment$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$P)((0, import_lib3.$S)(NotDedented, UpdateExpression, WAssignmentOp)), MaybeNestedExtendedExpression), function($skip, $loc, $0, $1, $2) {
  $1 = $1.map((x) => [x[0], x[1], ...x[2]]);
  $0 = [$1, $2];
  return {
    type: "AssignmentExpression",
    children: $0,
    // NOTE: This null marks the assignment for later processing to distinguish it
    // from fake assignments that only add a name to a scope
    names: null,
    lhs: $1,
    assigned: $1[0][1],
    expression: $2
  };
});
function ActualAssignment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ActualAssignment", ActualAssignment$0);
}
var NonPipelineActualAssignment$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$P)((0, import_lib3.$S)(NotDedented, UpdateExpression, WAssignmentOp)), MaybeNestedNonPipelineExtendedExpression), function($skip, $loc, $0, $1, $2) {
  $1 = $1.map((x) => [x[0], x[1], ...x[2]]);
  $0 = [$1, $2];
  return {
    type: "AssignmentExpression",
    children: $0,
    // NOTE: This null marks the assignment for later processing to distinguish it
    // from fake assignments that only add a name to a scope
    names: null,
    lhs: $1,
    assigned: $1[0][1],
    expression: $2
  };
});
function NonPipelineActualAssignment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NonPipelineActualAssignment", NonPipelineActualAssignment$0);
}
var YieldExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Yield, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), Star)), MaybeParenNestedExtendedExpression))), function($skip, $loc, $0, $1, $2) {
  if ($2) {
    const [star, expression] = $2;
    return {
      type: "YieldExpression",
      star,
      expression,
      children: [$1, star, expression]
    };
  }
  return {
    type: "YieldExpression",
    children: [$1]
  };
});
function YieldExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "YieldExpression", YieldExpression$0);
}
var ArrowFunction$0 = ThinArrowFunction;
var ArrowFunction$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Async, _)), ArrowParameters, (0, import_lib3.$E)(ReturnTypeSuffix), FatArrow, FatArrowBody), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var async = $1;
  var parameters = $2;
  var suffix = $3;
  var arrow = $4;
  var expOrBlock = $5;
  if (!async)
    async = [];
  return {
    type: "ArrowFunction",
    signature: {
      modifier: {
        async: !!async.length
      },
      returnType: suffix
    },
    parameters,
    returnType: suffix,
    async,
    block: expOrBlock,
    children: [async, parameters, suffix, arrow, expOrBlock]
  };
});
var ArrowFunction$$ = [ArrowFunction$0, ArrowFunction$1];
function ArrowFunction(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ArrowFunction", ArrowFunction$$);
}
var FatArrow$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), FatArrowToken), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var arrow = $2;
  if (!ws)
    ws = " ";
  return [ws, arrow];
});
function FatArrow(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "FatArrow", FatArrow$0);
}
var FatArrowToken$0 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L13, 'FatArrowToken "=>"'), (0, import_lib3.$EXPECT)($L14, 'FatArrowToken "\u21D2"')), function($skip, $loc, $0, $1) {
  return { $loc, token: "=>" };
});
function FatArrowToken(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "FatArrowToken", FatArrowToken$0);
}
var TrailingDeclaration$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)(ConstAssignment, LetAssignment));
function TrailingDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TrailingDeclaration", TrailingDeclaration$0);
}
var TrailingPipe$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), Pipe);
function TrailingPipe(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TrailingPipe", TrailingPipe$0);
}
var FatArrowBody$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), (0, import_lib3.$N)((0, import_lib3.$S)((0, import_lib3.$E)(_), ExpressionizedStatement)), NonPipelinePostfixedExpression, (0, import_lib3.$N)(TrailingDeclaration), (0, import_lib3.$N)(TrailingPipe), (0, import_lib3.$N)(SemicolonDelimiter)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var exp = $3;
  if (exp.type === "ObjectExpression") {
    exp = makeLeftHandSideExpression(exp);
  }
  const expressions = [["", exp]];
  return {
    type: "BlockStatement",
    bare: true,
    expressions,
    children: [expressions],
    implicitlyReturned: true
  };
});
var FatArrowBody$1 = NoCommaBracedOrEmptyBlock;
var FatArrowBody$$ = [FatArrowBody$0, FatArrowBody$1];
function FatArrowBody(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "FatArrowBody", FatArrowBody$$);
}
var ConditionalExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ShortCircuitExpression, (0, import_lib3.$E)(TernaryRest)), function($skip, $loc, $0, $1, $2) {
  if ($2) {
    return [$1, ...$2];
  }
  return $1;
});
function ConditionalExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ConditionalExpression", ConditionalExpression$0);
}
var TernaryRest$0 = NestedTernaryRest;
var TernaryRest$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(CoffeeBinaryExistentialEnabled), (0, import_lib3.$Y)((0, import_lib3.$EXPECT)($R5, "TernaryRest /[ \\t]/")), _, QuestionMark, MaybeNestedExtendedExpression, __, Colon, MaybeNestedExtendedExpression), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8) {
  return $0.slice(2);
});
var TernaryRest$$ = [TernaryRest$0, TernaryRest$1];
function TernaryRest(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TernaryRest", TernaryRest$$);
}
var NestedTernaryRest$0 = (0, import_lib3.$S)(Nested, QuestionMark, MaybeNestedExtendedExpression, Nested, Colon, MaybeNestedExtendedExpression);
var NestedTernaryRest$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$E)((0, import_lib3.$S)(Nested, QuestionMark, MaybeNestedExtendedExpression, Nested, Colon, MaybeNestedExtendedExpression)), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  if ($2)
    return $2;
  return $skip;
});
var NestedTernaryRest$$ = [NestedTernaryRest$0, NestedTernaryRest$1];
function NestedTernaryRest(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NestedTernaryRest", NestedTernaryRest$$);
}
var ShortCircuitExpression$0 = BinaryOpExpression;
function ShortCircuitExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ShortCircuitExpression", ShortCircuitExpression$0);
}
var PipelineExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), PipelineHeadItem, (0, import_lib3.$P)((0, import_lib3.$S)(NotDedented, Pipe, __, PipelineTailItem))), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var head = $2;
  var body = $3;
  if (head.type === "ArrowFunction" && head.ampersandBlock) {
    const expressions = [{
      type: "PipelineExpression",
      children: [ws, head.block.expressions[0], body]
    }];
    const block = { ...head.block, expressions, children: [expressions] };
    return {
      ...head,
      block,
      body: expressions,
      children: [...head.children.slice(0, -1), block]
    };
  }
  return {
    type: "PipelineExpression",
    children: [ws, head, body]
  };
});
function PipelineExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PipelineExpression", PipelineExpression$0);
}
var PipelineHeadItem$0 = NonPipelineExtendedExpression;
var PipelineHeadItem$1 = ParenthesizedExpression;
var PipelineHeadItem$$ = [PipelineHeadItem$0, PipelineHeadItem$1];
function PipelineHeadItem(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PipelineHeadItem", PipelineHeadItem$$);
}
var PipelineTailItem$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$C)(AwaitOp, Yield, Return), (0, import_lib3.$N)(AccessStart)), function(value) {
  return value[0];
});
var PipelineTailItem$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L15, 'PipelineTailItem "import"'), (0, import_lib3.$N)(AccessStart)), function($skip, $loc, $0, $1, $2) {
  return {
    type: "Identifier",
    children: [$1]
  };
});
var PipelineTailItem$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(NWTypePostfix, (0, import_lib3.$Q)(TypePostfix)), function($skip, $loc, $0, $1, $2) {
  return makeAmpersandFunction({
    body: [" ", $1, ...$2]
  });
});
var PipelineTailItem$3 = (0, import_lib3.$T)((0, import_lib3.$S)(PipelineHeadItem), function(value) {
  return value[0];
});
var PipelineTailItem$$ = [PipelineTailItem$0, PipelineTailItem$1, PipelineTailItem$2, PipelineTailItem$3];
function PipelineTailItem(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PipelineTailItem", PipelineTailItem$$);
}
var PrimaryExpression$0 = ObjectLiteral;
var PrimaryExpression$1 = ThisLiteral;
var PrimaryExpression$2 = TemplateLiteral;
var PrimaryExpression$3 = Literal;
var PrimaryExpression$4 = ArrayLiteral;
var PrimaryExpression$5 = FunctionExpression;
var PrimaryExpression$6 = IdentifierReference;
var PrimaryExpression$7 = ClassExpression;
var PrimaryExpression$8 = RegularExpressionLiteral;
var PrimaryExpression$9 = ParenthesizedExpression;
var PrimaryExpression$10 = Placeholder;
var PrimaryExpression$11 = JSXImplicitFragment;
var PrimaryExpression$$ = [PrimaryExpression$0, PrimaryExpression$1, PrimaryExpression$2, PrimaryExpression$3, PrimaryExpression$4, PrimaryExpression$5, PrimaryExpression$6, PrimaryExpression$7, PrimaryExpression$8, PrimaryExpression$9, PrimaryExpression$10, PrimaryExpression$11];
function PrimaryExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PrimaryExpression", PrimaryExpression$$);
}
var ParenthesizedExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenParen, AllowAll, (0, import_lib3.$E)((0, import_lib3.$S)(PostfixedCommaExpression, __, CloseParen)), RestoreAll), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $1;
  if (!$3)
    return $skip;
  const [exp, ws, close] = $3;
  switch (exp.type) {
    case "StatementExpression":
      if (exp.statement.type !== "IterationExpression")
        break;
    case "IterationExpression":
      return exp;
    case "ParenthesizedExpression":
      if (exp.implicit) {
        return {
          ...exp,
          children: [open, exp.expression, ws, close],
          implicit: false
        };
      }
      break;
  }
  return {
    type: "ParenthesizedExpression",
    children: [open, exp, ws, close],
    expression: exp
  };
});
function ParenthesizedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ParenthesizedExpression", ParenthesizedExpression$0);
}
var Placeholder$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Dot, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($R6, "Placeholder /(?:\\p{ID_Continue}|[\\u200C\\u200D$.#])/")), (0, import_lib3.$E)(PlaceholderTypeSuffix)), function($skip, $loc, $0, $1, $2, $3) {
  var dot = $1;
  var typeSuffix = $3;
  return {
    type: "Placeholder",
    subtype: ".",
    typeSuffix,
    children: [dot]
  };
});
var Placeholder$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Ampersand, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($R7, "Placeholder /[&=]/")), (0, import_lib3.$E)(PlaceholderTypeSuffix)), function($skip, $loc, $0, $1, $2, $3) {
  var amp = $1;
  var typeSuffix = $3;
  return {
    type: "Placeholder",
    subtype: "&",
    typeSuffix,
    children: [amp]
  };
});
var Placeholder$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Y)(AccessStart), (0, import_lib3.$Y)(PropertyAccess), (0, import_lib3.$N)(NumericLiteral)), function($skip, $loc, $0, $1, $2, $3) {
  return {
    type: "Placeholder",
    subtype: "&",
    children: [{ token: "&" }]
  };
});
var Placeholder$$ = [Placeholder$0, Placeholder$1, Placeholder$2];
function Placeholder(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Placeholder", Placeholder$$);
}
var PlaceholderTypeSuffix$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$E)(QuestionMark), Colon)), TypeSuffix), function(value) {
  return value[1];
});
function PlaceholderTypeSuffix(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PlaceholderTypeSuffix", PlaceholderTypeSuffix$0);
}
var ClassDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ClassExpression), function($skip, $loc, $0, $1) {
  if ($1.id)
    return $1;
  return makeLeftHandSideExpression($1);
});
function ClassDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ClassDeclaration", ClassDeclaration$0);
}
var ClassExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(Decorators), (0, import_lib3.$E)((0, import_lib3.$S)(Abstract, __)), Class, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($L16, 'ClassExpression ":"')), (0, import_lib3.$E)(ClassBinding), (0, import_lib3.$E)(ClassHeritage), ClassBody), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var decorators = $1;
  var abstract = $2;
  var binding = $5;
  var heritage = $6;
  var body = $7;
  return {
    decorators,
    abstract,
    binding,
    id: binding?.[0],
    heritage,
    body,
    children: $0
  };
});
function ClassExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ClassExpression", ClassExpression$0);
}
var ClassBinding$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), BindingIdentifier, (0, import_lib3.$E)(TypeParameters)), function(value) {
  return [value[1], value[2]];
});
function ClassBinding(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ClassBinding", ClassBinding$0);
}
var ClassHeritage$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ExtendsClause, (0, import_lib3.$E)(WithClause), (0, import_lib3.$E)(ImplementsClause)), function($skip, $loc, $0, $1, $2, $3) {
  var extendsClause = $1;
  var withClause = $2;
  var implementsClause = $3;
  if (withClause) {
    extendsClause = convertWithClause(withClause, extendsClause);
  }
  return [extendsClause, implementsClause];
});
var ClassHeritage$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(WithClause), (0, import_lib3.$E)(ImplementsClause)), function($skip, $loc, $0, $1, $2) {
  var withClause = $1;
  var implementsClause = $2;
  if (withClause)
    return [convertWithClause(withClause), implementsClause];
  if (implementsClause)
    return implementsClause;
  return $skip;
});
var ClassHeritage$$ = [ClassHeritage$0, ClassHeritage$1];
function ClassHeritage(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ClassHeritage", ClassHeritage$$);
}
var ExtendsClause$0 = (0, import_lib3.$S)(ExtendsToken, __, ExtendsTarget);
function ExtendsClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExtendsClause", ExtendsClause$0);
}
var WithClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, With, __, ExtendsTarget, (0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L17, 'WithClause ","'), __, ExtendsTarget))), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var ws = $3;
  var t = $4;
  var rest = $5;
  return {
    type: "WithClause",
    children: $0,
    targets: [[ws, t], ...rest.map(([_comma, ws2, target]) => [ws2, target])]
  };
});
function WithClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "WithClause", WithClause$0);
}
var ExtendsToken$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Loc, (0, import_lib3.$E)(_), ExtendsShorthand, (0, import_lib3.$E)((0, import_lib3.$EXPECT)($L18, 'ExtendsToken " "'))), function($skip, $loc, $0, $1, $2, $3, $4) {
  var l = $1;
  var ws = $2;
  var t = $3;
  return {
    type: "Extends",
    children: [
      ws || { $loc: l.$loc, token: " " },
      t
    ]
  };
});
var ExtendsToken$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), Extends), function($skip, $loc, $0, $1, $2) {
  return {
    type: "Extends",
    children: $0
  };
});
var ExtendsToken$$ = [ExtendsToken$0, ExtendsToken$1];
function ExtendsToken(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ExtendsToken", ExtendsToken$$);
}
var ExtendsShorthand$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L19, 'ExtendsShorthand "<"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "extends " };
});
function ExtendsShorthand(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExtendsShorthand", ExtendsShorthand$0);
}
var NotExtendsToken$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Loc, (0, import_lib3.$E)(_), OmittedNegation, ExtendsShorthand, (0, import_lib3.$E)((0, import_lib3.$EXPECT)($L18, 'NotExtendsToken " "'))), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var l = $1;
  var ws1 = $2;
  var ws2 = $3;
  var t = $4;
  const ws = ws1 && ws2 ? [ws1, ws2] : ws1 || ws2 || { $loc: l.$loc, token: " " };
  return {
    type: "Extends",
    negated: true,
    children: [ws, t]
  };
});
var NotExtendsToken$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), OmittedNegation, Extends), function($skip, $loc, $0, $1, $2, $3) {
  return {
    type: "Extends",
    negated: true,
    children: $0
  };
});
var NotExtendsToken$$ = [NotExtendsToken$0, NotExtendsToken$1];
function NotExtendsToken(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NotExtendsToken", NotExtendsToken$$);
}
var OmittedNegation$0 = (0, import_lib3.$T)((0, import_lib3.$S)(ExclamationPoint), function(value) {
  return "";
});
var OmittedNegation$1 = (0, import_lib3.$T)((0, import_lib3.$S)(Not, (0, import_lib3.$E)((0, import_lib3.$EXPECT)($L18, 'OmittedNegation " "')), (0, import_lib3.$E)(_)), function(value) {
  return value[2];
});
var OmittedNegation$$ = [OmittedNegation$0, OmittedNegation$1];
function OmittedNegation(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "OmittedNegation", OmittedNegation$$);
}
var ExtendsTarget$0 = (0, import_lib3.$TV)(LeftHandSideExpressionWithObjectApplicationForbidden, function($skip, $loc, $0, $1) {
  var exp = $0;
  return makeLeftHandSideExpression(exp);
});
function ExtendsTarget(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExtendsTarget", ExtendsTarget$0);
}
var ImplementsClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ImplementsToken, ImplementsTarget, (0, import_lib3.$Q)((0, import_lib3.$S)(Comma, ImplementsTarget))), function($skip, $loc, $0, $1, $2, $3) {
  return {
    ts: true,
    children: $0
  };
});
function ImplementsClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImplementsClause", ImplementsClause$0);
}
var ImplementsToken$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Loc, __, ImplementsShorthand, (0, import_lib3.$E)((0, import_lib3.$EXPECT)($L18, 'ImplementsToken " "'))), function($skip, $loc, $0, $1, $2, $3, $4) {
  var l = $1;
  var ws = $2;
  var token = $3;
  const children = [...ws, token];
  if (!ws.length) {
    children.unshift({ $loc: l.$loc, token: " " });
  }
  return { children };
});
var ImplementsToken$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L20, 'ImplementsToken "implements"'), NonIdContinue), function($skip, $loc, $0, $1, $2, $3) {
  $2 = { $loc, token: $2 };
  return [$1, $2];
});
var ImplementsToken$$ = [ImplementsToken$0, ImplementsToken$1];
function ImplementsToken(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ImplementsToken", ImplementsToken$$);
}
var ImplementsShorthand$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L21, 'ImplementsShorthand "<:"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "implements " };
});
function ImplementsShorthand(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImplementsShorthand", ImplementsShorthand$0);
}
var ImplementsTarget$0 = (0, import_lib3.$S)(__, IdentifierName, (0, import_lib3.$Q)((0, import_lib3.$S)(Dot, IdentifierName)), (0, import_lib3.$E)(TypeArguments));
function ImplementsTarget(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImplementsTarget", ImplementsTarget$0);
}
var ClassBody$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, OpenBrace, (0, import_lib3.$E)(NestedClassElements), __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var expressions = $3;
  if (!expressions)
    expressions = $0[2] = [];
  return {
    type: "BlockStatement",
    subtype: "ClassBody",
    children: $0,
    expressions
  };
});
var ClassBody$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBrace, (0, import_lib3.$E)(NestedClassElements), InsertNewline, InsertIndent, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var expressions = $2;
  if (!expressions)
    expressions = $0[1] = [];
  return {
    type: "BlockStatement",
    subtype: "ClassBody",
    children: $0,
    expressions
  };
});
var ClassBody$$ = [ClassBody$0, ClassBody$1];
function ClassBody(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ClassBody", ClassBody$$);
}
var NestedClassElements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedClassElement), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var elements = $2;
  if (!elements.length)
    return $skip;
  return elements;
});
function NestedClassElements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedClassElements", NestedClassElements$0);
}
var NestedClassElement$0 = (0, import_lib3.$S)(Nested, ClassElement, StatementDelimiter);
function NestedClassElement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedClassElement", NestedClassElement$0);
}
var ClassElement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(Decorators), (0, import_lib3.$E)(AccessModifier), (0, import_lib3.$E)((0, import_lib3.$S)(Static, (0, import_lib3.$E)(_))), (0, import_lib3.$E)((0, import_lib3.$S)(Override, (0, import_lib3.$E)(_))), ClassElementDefinition), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var decorators = $1;
  var definition = $5;
  if (definition.type === "MultiMethodDefinition") {
    return {
      ...definition,
      children: definition.children.map((c) => {
        return {
          ...c,
          children: [decorators, ...c.children]
        };
      })
    };
  }
  return {
    ...definition,
    children: [decorators, $2, $3, $4, ...definition.children]
  };
});
var ClassElement$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Static, BracedBlock), function($skip, $loc, $0, $1, $2) {
  return {
    type: "ClassStaticBlock",
    children: $0
  };
});
var ClassElement$2 = EmptyStatement;
var ClassElement$$ = [ClassElement$0, ClassElement$1, ClassElement$2];
function ClassElement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ClassElement", ClassElement$$);
}
var ClassElementDefinition$0 = MethodDefinition;
var ClassElementDefinition$1 = FieldDefinition;
var ClassElementDefinition$$ = [ClassElementDefinition$0, ClassElementDefinition$1];
function ClassElementDefinition(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ClassElementDefinition", ClassElementDefinition$$);
}
var ClassSignature$0 = (0, import_lib3.$S)((0, import_lib3.$E)(Decorators), (0, import_lib3.$E)((0, import_lib3.$S)(Abstract, __)), Class, (0, import_lib3.$E)(ClassBinding), (0, import_lib3.$E)(ClassHeritage), ClassSignatureBody);
function ClassSignature(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ClassSignature", ClassSignature$0);
}
var ClassSignatureBody$0 = (0, import_lib3.$S)(__, OpenBrace, (0, import_lib3.$E)(NestedClassSignatureElements), __, CloseBrace);
var ClassSignatureBody$1 = (0, import_lib3.$S)(InsertOpenBrace, (0, import_lib3.$E)(NestedClassSignatureElements), InsertNewline, InsertIndent, InsertCloseBrace);
var ClassSignatureBody$$ = [ClassSignatureBody$0, ClassSignatureBody$1];
function ClassSignatureBody(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ClassSignatureBody", ClassSignatureBody$$);
}
var NestedClassSignatureElements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedClassSignatureElement), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var elements = $2;
  if (!elements.length)
    return $skip;
  return elements;
});
function NestedClassSignatureElements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedClassSignatureElements", NestedClassSignatureElements$0);
}
var NestedClassSignatureElement$0 = (0, import_lib3.$S)(Nested, ClassSignatureElement, StatementDelimiter);
function NestedClassSignatureElement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedClassSignatureElement", NestedClassSignatureElement$0);
}
var ClassSignatureElement$0 = (0, import_lib3.$S)((0, import_lib3.$E)(Decorators), (0, import_lib3.$E)(AccessModifier), (0, import_lib3.$E)((0, import_lib3.$S)(Static, (0, import_lib3.$E)(_))), (0, import_lib3.$E)((0, import_lib3.$S)(Override, (0, import_lib3.$E)(_))), (0, import_lib3.$C)(MethodSignature, FieldDefinition));
var ClassSignatureElement$1 = (0, import_lib3.$S)(Static, ClassSignatureBody);
var ClassSignatureElement$$ = [ClassSignatureElement$0, ClassSignatureElement$1];
function ClassSignatureElement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ClassSignatureElement", ClassSignatureElement$$);
}
var AccessModifier$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$C)(Public, Private, Protected), NotDedented)), (0, import_lib3.$E)((0, import_lib3.$S)(Readonly, NotDedented))), function($skip, $loc, $0, $1, $2) {
  if (!($1 || $2))
    return $skip;
  return {
    ts: true,
    children: $0
  };
});
function AccessModifier(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AccessModifier", AccessModifier$0);
}
var FieldDefinition$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeeClassesEnabled, ClassElementName, (0, import_lib3.$E)(_), Colon, __, AssignmentExpression), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var id = $2;
  var exp = $6;
  switch (exp.type) {
    case "FunctionExpression":
      const fnTokenIndex = exp.children.findIndex((c) => c?.token?.startsWith("function"));
      const children = exp.children.slice();
      if (exp.generator) {
        children.splice(fnTokenIndex, 2, children[fnTokenIndex + 1], id);
      } else {
        children.splice(fnTokenIndex, 1, id);
      }
      return {
        ...exp,
        children
      };
    default:
      return {
        type: "FieldDefinition",
        children: [id, " = ", exp]
      };
  }
});
var FieldDefinition$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertReadonly, ClassElementName, (0, import_lib3.$E)(TypeSuffix), __, ConstAssignment, MaybeNestedExtendedExpression), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var r = $1;
  var ca = $5;
  r.children[0].$loc = {
    pos: ca.$loc.pos - 1,
    length: ca.$loc.length + 1
  };
  return {
    type: "FieldDefinition",
    children: $0
  };
});
var FieldDefinition$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Abstract, (0, import_lib3.$E)(_))), (0, import_lib3.$E)((0, import_lib3.$S)(Readonly, (0, import_lib3.$E)(_))), ClassElementName, (0, import_lib3.$E)(TypeSuffix), (0, import_lib3.$E)(Initializer)), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  if ($1)
    return { children: $0, ts: true };
  return {
    type: "FieldDefinition",
    children: $0
  };
});
var FieldDefinition$$ = [FieldDefinition$0, FieldDefinition$1, FieldDefinition$2];
function FieldDefinition(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "FieldDefinition", FieldDefinition$$);
}
var ThisLiteral$0 = This;
var ThisLiteral$1 = HashThis;
var ThisLiteral$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(AtThis, (0, import_lib3.$TEXT)((0, import_lib3.$S)((0, import_lib3.$E)(Hash), IdentifierName))), function($skip, $loc, $0, $1, $2) {
  var at = $1;
  var id = $2;
  return {
    type: "MemberExpression",
    children: [at, {
      type: "PropertyAccess",
      name: id,
      children: [".", {
        $loc: {
          pos: $loc.pos + 1,
          length: $loc.length - 1
        },
        token: id
      }]
    }],
    thisShorthand: true
  };
});
var ThisLiteral$3 = AtThis;
var ThisLiteral$$ = [ThisLiteral$0, ThisLiteral$1, ThisLiteral$2, ThisLiteral$3];
function ThisLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ThisLiteral", ThisLiteral$$);
}
var HashThis$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(AtThis), LengthShorthand, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$Y)((0, import_lib3.$S)(_, (0, import_lib3.$E)((0, import_lib3.$S)(Not, __)), ActualIn)), (0, import_lib3.$EXPECT)($L0, 'HashThis ""')))), function($skip, $loc, $0, $1, $2, $3) {
  var at = $1;
  var id = $2;
  var beforeIn = $3;
  if (beforeIn != null && at == null)
    return ['"', id.name, '"'];
  return {
    type: "MemberExpression",
    children: [at ?? "this", {
      type: "PropertyAccess",
      name: id.name,
      children: [".", id]
    }],
    thisShorthand: true
  };
});
var HashThis$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(PrivateIdentifier, (0, import_lib3.$Y)((0, import_lib3.$S)(_, (0, import_lib3.$E)((0, import_lib3.$S)(Not, __)), ActualIn))), function($skip, $loc, $0, $1, $2) {
  var id = $1;
  return id;
});
var HashThis$2 = (0, import_lib3.$TV)(PrivateIdentifier, function($skip, $loc, $0, $1) {
  var id = $0;
  return {
    type: "MemberExpression",
    children: ["this", {
      type: "PropertyAccess",
      name: id.name,
      children: [".", id]
    }],
    privateShorthand: true,
    privateId: id
  };
});
var HashThis$$ = [HashThis$0, HashThis$1, HashThis$2];
function HashThis(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "HashThis", HashThis$$);
}
var LengthShorthand$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Hash, NonIdContinue), function($skip, $loc, $0, $1, $2) {
  const id = "length";
  return {
    type: "Identifier",
    name: id,
    names: [id],
    children: [{
      $loc,
      token: id
    }]
  };
});
function LengthShorthand(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "LengthShorthand", LengthShorthand$0);
}
var AtThis$0 = (0, import_lib3.$TV)(At, function($skip, $loc, $0, $1) {
  var at = $0;
  return { ...at, token: "this" };
});
function AtThis(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AtThis", AtThis$0);
}
var LeftHandSideExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$P)((0, import_lib3.$S)(New, (0, import_lib3.$N)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L7, 'LeftHandSideExpression "."'), (0, import_lib3.$EXPECT)($L16, 'LeftHandSideExpression ":"'))), __)), CallExpression), function($skip, $loc, $0, $1, $2) {
  var expression = $2;
  return {
    type: "NewExpression",
    children: $0,
    expression
  };
});
var LeftHandSideExpression$1 = CallExpression;
var LeftHandSideExpression$$ = [LeftHandSideExpression$0, LeftHandSideExpression$1];
function LeftHandSideExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "LeftHandSideExpression", LeftHandSideExpression$$);
}
var CallExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Super, ArgumentsWithTrailingMemberExpressions, (0, import_lib3.$Q)(CallExpressionRest)), function($skip, $loc, $0, $1, $2, $3) {
  var rest = $3;
  return processCallMemberExpression({
    type: "CallExpression",
    children: [$1, ...$2, ...rest.flat()]
  });
});
var CallExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Import, _, NamedImports, __, FromClause), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  return dynamizeImportDeclarationExpression($0);
});
var CallExpression$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L15, 'CallExpression "import"'), ArgumentsWithTrailingMemberExpressions, (0, import_lib3.$Q)(CallExpressionRest)), function($skip, $loc, $0, $1, $2, $3) {
  var rest = $3;
  return processCallMemberExpression({
    type: "CallExpression",
    children: [$1, ...$2, ...rest.flat()]
  });
});
var CallExpression$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(MemberExpression, AllowedTrailingMemberExpressions, (0, import_lib3.$Q)(CallExpressionRest)), function($skip, $loc, $0, $1, $2, $3) {
  var member = $1;
  var trailing = $2;
  var rest = $3;
  if (rest.length || trailing.length) {
    rest = rest.flat();
    return processCallMemberExpression({
      type: "CallExpression",
      children: [member, ...trailing, ...rest]
    });
  }
  return member;
});
var CallExpression$$ = [CallExpression$0, CallExpression$1, CallExpression$2, CallExpression$3];
function CallExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CallExpression", CallExpression$$);
}
var CallExpressionRest$0 = MemberExpressionRest;
var CallExpressionRest$1 = (0, import_lib3.$T)((0, import_lib3.$S)(TypeArguments, (0, import_lib3.$N)((0, import_lib3.$C)(IdentifierName, NumericLiteral))), function(value) {
  return value[0];
});
var CallExpressionRest$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R8, "CallExpressionRest /(?=['\"`])/"), (0, import_lib3.$C)(TemplateLiteral, StringLiteral)), function($skip, $loc, $0, $1, $2) {
  var literal = $2;
  if (literal.type === "StringLiteral") {
    literal = "`" + literal.token.slice(1, -1).replace(/(`|\$\{)/g, "\\$1") + "`";
  }
  return literal;
});
var CallExpressionRest$3 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(OptionalShorthand), ArgumentsWithTrailingMemberExpressions), function($skip, $loc, $0, $1, $2) {
  var optional = $1;
  var argsWithTrailing = $2;
  if (!optional)
    return argsWithTrailing;
  const call = argsWithTrailing[0];
  return [{
    ...call,
    children: [optional, ...call.children],
    optional
  }, ...argsWithTrailing.slice(1)];
});
var CallExpressionRest$$ = [CallExpressionRest$0, CallExpressionRest$1, CallExpressionRest$2, CallExpressionRest$3];
function CallExpressionRest(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CallExpressionRest", CallExpressionRest$$);
}
var OptionalShorthand$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R9, "OptionalShorthand /(?=[\\/?])/"), (0, import_lib3.$Q)(InlineComment), QuestionMark, OptionalDot), function($skip, $loc, $0, $1, $2, $3, $4) {
  var comments = $2;
  var q = $3;
  var d = $4;
  return {
    type: "Optional",
    children: [...comments, q, d]
  };
});
function OptionalShorthand(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "OptionalShorthand", OptionalShorthand$0);
}
var OptionalDot$0 = (0, import_lib3.$S)((0, import_lib3.$Q)(InlineComment), Dot);
var OptionalDot$1 = InsertDot;
var OptionalDot$$ = [OptionalDot$0, OptionalDot$1];
function OptionalDot(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "OptionalDot", OptionalDot$$);
}
var NonNullAssertion$0 = (0, import_lib3.$T)((0, import_lib3.$S)(ExclamationPoint, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($L22, 'NonNullAssertion "^"'))), function(value) {
  return { "type": "NonNullAssertion", "ts": true, "children": [value[0]] };
});
function NonNullAssertion(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NonNullAssertion", NonNullAssertion$0);
}
var MemberExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(MemberBase, (0, import_lib3.$Q)(MemberExpressionRest)), function($skip, $loc, $0, $1, $2) {
  var rest = $2;
  if (rest.length || Array.isArray($1)) {
    return processCallMemberExpression({
      type: "MemberExpression",
      children: [$1, ...rest].flat()
    });
  }
  return $1;
});
function MemberExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "MemberExpression", MemberExpression$0);
}
var ActualMemberExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(MemberBase, (0, import_lib3.$P)(MemberExpressionRest)), function($skip, $loc, $0, $1, $2) {
  var rest = $2;
  return processCallMemberExpression({
    type: "MemberExpression",
    children: [$1, ...rest].flat()
  });
});
function ActualMemberExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ActualMemberExpression", ActualMemberExpression$0);
}
var MemberBase$0 = PrimaryExpression;
var MemberBase$1 = SuperProperty;
var MemberBase$2 = MetaProperty;
var MemberBase$$ = [MemberBase$0, MemberBase$1, MemberBase$2];
function MemberBase(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MemberBase", MemberBase$$);
}
var MemberExpressionRest$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R10, "MemberExpressionRest /(?=[\\/\\[{?.!@#'\u2019:])/"), (0, import_lib3.$Q)(InlineComment), MemberExpressionRestBody), function($skip, $loc, $0, $1, $2, $3) {
  var comments = $2;
  var body = $3;
  if (Array.isArray(body))
    return [...comments, ...body];
  return {
    ...body,
    children: [...comments, ...body.children]
  };
});
function MemberExpressionRest(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "MemberExpressionRest", MemberExpressionRest$0);
}
var MemberExpressionRestBody$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(OptionalShorthand), (0, import_lib3.$Q)(InlineComment), MemberBracketContent), function($skip, $loc, $0, $1, $2, $3) {
  var dot = $1;
  var comments = $2;
  var content = $3;
  if (!dot && !comments.length)
    return content;
  if (dot) {
    if (dot.type === "Optional" && content.type === "SliceExpression") {
      return [...dot.children.slice(0, -1), ...comments, content];
    }
    return {
      ...content,
      children: [dot, ...comments, ...content.children],
      optional: dot
    };
  }
  return [...comments, content];
});
var MemberExpressionRestBody$1 = PropertyAccess;
var MemberExpressionRestBody$2 = PropertyGlob;
var MemberExpressionRestBody$3 = PropertyBind;
var MemberExpressionRestBody$4 = NonNullAssertion;
var MemberExpressionRestBody$$ = [MemberExpressionRestBody$0, MemberExpressionRestBody$1, MemberExpressionRestBody$2, MemberExpressionRestBody$3, MemberExpressionRestBody$4];
function MemberExpressionRestBody(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MemberExpressionRestBody", MemberExpressionRestBody$$);
}
var MemberBracketContent$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenBracket, (0, import_lib3.$C)(SliceParameters, PostfixedExpression), __, CloseBracket), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $1;
  var expression = $2;
  var ws = $3;
  var close = $4;
  if (expression.type === "SliceParameters") {
    const { start, end, children } = expression;
    return {
      type: "SliceExpression",
      start,
      end,
      children: [
        { ...open, token: ".slice(" },
        ...children,
        [...ws, { ...close, token: ")" }]
      ]
    };
  }
  return {
    type: "Index",
    children: $0,
    expression
  };
});
function MemberBracketContent(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "MemberBracketContent", MemberBracketContent$0);
}
var SliceParameters$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ExtendedExpression, __, (0, import_lib3.$C)(DotDotDot, DotDot), (0, import_lib3.$E)(ExtendedExpression)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var start = $1;
  var ws = $2;
  var sep = $3;
  var end = $4;
  const inclusive = sep.token === "..";
  let children;
  if (end) {
    const inc = [];
    if (inclusive) {
      end = ["1 + ", end];
      inc.push(" || 1/0");
    }
    children = [start, [...ws, { ...sep, token: ", " }], [end, ...inc]];
  } else {
    children = [start, ws];
  }
  return {
    type: "SliceParameters",
    start,
    end,
    children
  };
});
var SliceParameters$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Loc, __, (0, import_lib3.$C)(DotDotDot, DotDot), ExtendedExpression), function($skip, $loc, $0, $1, $2, $3, $4) {
  var l = $1;
  var ws = $2;
  var sep = $3;
  var end = $4;
  const inclusive = sep.token === "..";
  const inc = [];
  if (inclusive) {
    end = ["1 + ", end];
    inc.push(" || 1/0");
  }
  const start = {
    $loc: l.$loc,
    token: "0"
  };
  return {
    type: "SliceParameters",
    start,
    end,
    children: [start, [...ws, { ...sep, token: ", " }], [end, ...inc]]
  };
});
var SliceParameters$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(Loc, __, (0, import_lib3.$C)(DotDot, DotDotDot), (0, import_lib3.$Y)((0, import_lib3.$S)(__, CloseBracket))), function($skip, $loc, $0, $1, $2, $3, $4) {
  var l = $1;
  var ws = $2;
  const start = {
    $loc: l.$loc,
    token: "0"
  };
  return {
    type: "SliceParameters",
    start,
    end: void 0,
    children: [start, ws]
  };
});
var SliceParameters$$ = [SliceParameters$0, SliceParameters$1, SliceParameters$2];
function SliceParameters(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "SliceParameters", SliceParameters$$);
}
var AccessStart$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(PropertyAccessModifier), Dot, (0, import_lib3.$N)(Dot)), function($skip, $loc, $0, $1, $2, $3) {
  var modifier = $1;
  var dot = $2;
  return {
    type: "AccessStart",
    children: modifier ? [modifier, dot] : [dot],
    optional: modifier?.token === "?"
  };
});
function AccessStart(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AccessStart", AccessStart$0);
}
var ImplicitAccessStart$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(PropertyAccessModifier), InsertDot, (0, import_lib3.$N)(Dot)), function($skip, $loc, $0, $1, $2, $3) {
  var modifier = $1;
  var dot = $2;
  return {
    type: "AccessStart",
    children: modifier ? [modifier, dot] : [dot],
    optional: modifier?.token === "?"
  };
});
function ImplicitAccessStart(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImplicitAccessStart", ImplicitAccessStart$0);
}
var PropertyAccessModifier$0 = QuestionMark;
var PropertyAccessModifier$1 = NonNullAssertion;
var PropertyAccessModifier$$ = [PropertyAccessModifier$0, PropertyAccessModifier$1];
function PropertyAccessModifier(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PropertyAccessModifier", PropertyAccessModifier$$);
}
var PropertyAccess$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(AccessStart, (0, import_lib3.$C)(TemplateLiteral, StringLiteral, IntegerLiteral)), function($skip, $loc, $0, $1, $2) {
  var dot = $1;
  var literal = $2;
  return {
    type: "Index",
    children: [
      adjustIndexAccess(dot),
      literal,
      "]"
    ]
  };
});
var PropertyAccess$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(AccessStart, (0, import_lib3.$EXPECT)($L23, 'PropertyAccess "-"'), IntegerLiteral), function($skip, $loc, $0, $1, $2, $3) {
  var dot = $1;
  var neg = $2;
  var num = $3;
  const len3 = {
    children: []
  }, children = [
    adjustIndexAccess(dot),
    len3,
    neg,
    num,
    "]"
  ];
  return {
    type: "NegativeIndex",
    children,
    len: len3
  };
});
var PropertyAccess$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(AccessStart, (0, import_lib3.$Q)(InlineComment), (0, import_lib3.$C)(IdentifierName, PrivateIdentifier, LengthShorthand)), function($skip, $loc, $0, $1, $2, $3) {
  var dot = $1;
  var comments = $2;
  var id = $3;
  return {
    type: "PropertyAccess",
    name: id.name,
    dot,
    children: [dot, ...comments, ...id.children]
  };
});
var PropertyAccess$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(ImplicitAccessStart, (0, import_lib3.$C)(PrivateIdentifier, LengthShorthand)), function($skip, $loc, $0, $1, $2) {
  var dot = $1;
  var id = $2;
  return {
    type: "PropertyAccess",
    name: id.name,
    dot,
    children: [dot, ...id.children]
  };
});
var PropertyAccess$4 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeePrototypeEnabled, (0, import_lib3.$E)(PropertyAccessModifier), DoubleColon, (0, import_lib3.$E)(IdentifierName)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var modifier = $2;
  var p = $3;
  var id = $4;
  const dot = { token: ".", $loc: p.$loc };
  const start = {
    type: "AccessStart",
    children: modifier ? [modifier, dot] : [dot],
    optional: modifier?.token === "?"
  };
  if (id) {
    return {
      type: "PropertyAccess",
      name: id.name,
      dot: start,
      children: [start, "prototype.", id]
    };
  } else {
    return {
      type: "PropertyAccess",
      name: "prototype",
      dot: start,
      children: [start, "prototype"]
    };
  }
});
var PropertyAccess$$ = [PropertyAccess$0, PropertyAccess$1, PropertyAccess$2, PropertyAccess$3, PropertyAccess$4];
function PropertyAccess(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PropertyAccess", PropertyAccess$$);
}
var PropertyGlob$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(PropertyAccessModifier), OptionalDot), (0, import_lib3.$Q)(InlineComment), BracedObjectLiteral), function($skip, $loc, $0, $1, $2, $3) {
  var dot = $1;
  var object = $3;
  return {
    type: "PropertyGlob",
    dot,
    object,
    children: $0
  };
});
function PropertyGlob(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PropertyGlob", PropertyGlob$0);
}
var PropertyBind$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(PropertyAccessModifier), At, OptionalDot, (0, import_lib3.$C)(IdentifierName, PrivateIdentifier), (0, import_lib3.$E)(Arguments)), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var modifier = $1;
  var dot = $3;
  var id = $4;
  var args = $5;
  return {
    type: "PropertyBind",
    name: id.name,
    children: [modifier, dot, id],
    // omit `@` from children
    args: args?.children.slice(1, -1) ?? []
    // remove the parens from the arg list, or give an empty list
  };
});
function PropertyBind(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PropertyBind", PropertyBind$0);
}
var SuperProperty$0 = (0, import_lib3.$S)(Super, MemberBracketContent);
var SuperProperty$1 = (0, import_lib3.$S)(Super, (0, import_lib3.$N)(PropertyAccessModifier), PropertyAccess);
var SuperProperty$$ = [SuperProperty$0, SuperProperty$1];
function SuperProperty(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "SuperProperty", SuperProperty$$);
}
var MetaProperty$0 = (0, import_lib3.$S)(New, Dot, Target);
var MetaProperty$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L24, 'MetaProperty "import.meta"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
var MetaProperty$2 = ReturnValue;
var MetaProperty$$ = [MetaProperty$0, MetaProperty$1, MetaProperty$2];
function MetaProperty(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MetaProperty", MetaProperty$$);
}
var ReturnValue$0 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L25, 'ReturnValue "return.value"'), NonIdContinue), (0, import_lib3.$S)(Return, (0, import_lib3.$Y)(AfterReturnShorthand))), function($skip, $loc, $0, $1) {
  return { type: "ReturnValue", children: [$1[0]] };
});
function ReturnValue(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ReturnValue", ReturnValue$0);
}
var AfterReturnShorthand$0 = WAssignmentOp;
var AfterReturnShorthand$1 = UpdateExpressionSymbol;
var AfterReturnShorthand$2 = TypeSuffix;
var AfterReturnShorthand$3 = (0, import_lib3.$S)(__, LetAssignment);
var AfterReturnShorthand$4 = (0, import_lib3.$S)(__, ConstAssignment);
var AfterReturnShorthand$$ = [AfterReturnShorthand$0, AfterReturnShorthand$1, AfterReturnShorthand$2, AfterReturnShorthand$3, AfterReturnShorthand$4];
function AfterReturnShorthand(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "AfterReturnShorthand", AfterReturnShorthand$$);
}
var Parameters$0 = NonEmptyParameters;
var Parameters$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(TypeParameters), Loc), function($skip, $loc, $0, $1, $2) {
  var tp = $1;
  var p = $2;
  return {
    type: "Parameters",
    children: [tp, { $loc: p.$loc, token: "()" }],
    tp,
    names: [],
    implicit: true
  };
});
var Parameters$$ = [Parameters$0, Parameters$1];
function Parameters(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Parameters", Parameters$$);
}
var ShortArrowParameters$0 = ObjectBindingPattern;
var ShortArrowParameters$1 = ArrayBindingPattern;
var ShortArrowParameters$$ = [ShortArrowParameters$0, ShortArrowParameters$1];
function ShortArrowParameters(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ShortArrowParameters", ShortArrowParameters$$);
}
var ArrowParameters$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ShortArrowParameters), function($skip, $loc, $0, $1) {
  return {
    type: "Parameters",
    children: ["(", $0, ")"],
    names: $0.names
  };
});
var ArrowParameters$1 = Parameters;
var ArrowParameters$$ = [ArrowParameters$0, ArrowParameters$1];
function ArrowParameters(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ArrowParameters", ArrowParameters$$);
}
var NonEmptyParameters$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(TypeParameters), OpenParen, ParameterList, (0, import_lib3.$S)(__, CloseParen)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var tp = $1;
  var open = $2;
  var params = $3;
  var close = $4;
  let tt, before = [], rest, after = [], errors = [];
  function append(p) {
    (rest ? after : before).push(p);
  }
  for (const param of params) {
    switch (param.type) {
      case "ThisType":
        if (tt) {
          append({
            type: "Error",
            message: "Only one typed this parameter is allowed"
          });
          append(param);
        } else {
          tt = insertTrimmingSpace(param, "");
          if (before.length || rest) {
            let delim = tt.children.at(-1);
            if (Array.isArray(delim))
              delim = delim.at(-1);
            if (delim?.token !== ",") {
              tt = {
                ...tt,
                children: [...tt.children, ", "]
              };
            }
          }
        }
        break;
      case "FunctionRestParameter":
        if (rest) {
          append({
            type: "Error",
            message: "Only one rest parameter is allowed"
          });
          append(param);
        } else {
          rest = param;
        }
        break;
      default:
        append(param);
    }
  }
  const names = before.flatMap((p) => p.names);
  if (rest) {
    const restIdentifier = rest.binding.ref || rest.binding;
    names.push(...rest.names || []);
    let blockPrefix;
    if (after.length) {
      blockPrefix = {
        children: ["[", insertTrimmingSpace(after, ""), "] = ", restIdentifier, ".splice(-", after.length.toString(), ")"],
        names: after.flatMap((p) => p.names)
      };
    }
    return {
      type: "Parameters",
      children: [
        tp,
        open,
        tt,
        ...before,
        // Remove delimiter
        { ...rest, children: rest.children.slice(0, -1) },
        close
      ],
      tp,
      names,
      blockPrefix
    };
  }
  return {
    type: "Parameters",
    children: [tp, open, tt, ...before, close],
    names,
    tp
  };
});
function NonEmptyParameters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NonEmptyParameters", NonEmptyParameters$0);
}
var ParameterList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)(Parameter), NestedParameterList), function($skip, $loc, $0, $1, $2) {
  return [...$1, ...$2];
});
var ParameterList$1 = (0, import_lib3.$TV)((0, import_lib3.$Q)((0, import_lib3.$S)(__, Parameter)), function($skip, $loc, $0, $1) {
  return $1.map(([eos, p]) => ({
    ...p,
    children: [eos, ...p.children]
  }));
});
var ParameterList$$ = [ParameterList$0, ParameterList$1];
function ParameterList(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ParameterList", ParameterList$$);
}
var NestedParameterList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedParameter), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var params = $2;
  if (!params.length)
    return $skip;
  return params;
});
function NestedParameterList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedParameterList", NestedParameterList$0);
}
var NestedParameter$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, (0, import_lib3.$P)(Parameter)), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var params = $2;
  params = [...params];
  params[0] = prepend(ws, params[0]);
  return params;
});
function NestedParameter(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedParameter", NestedParameter$0);
}
var Parameter$0 = ThisType;
var Parameter$1 = ParameterElement;
var Parameter$2 = FunctionRestParameter;
var Parameter$$ = [Parameter$0, Parameter$1, Parameter$2];
function Parameter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Parameter", Parameter$$);
}
var FunctionRestParameter$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), BindingRestElement, (0, import_lib3.$E)(TypeSuffix), ParameterElementDelimiter), function($skip, $loc, $0, $1, $2, $3, $4) {
  var id = $2;
  return {
    type: "FunctionRestParameter",
    children: $0.slice(1),
    names: id.names,
    binding: id.binding
  };
});
function FunctionRestParameter(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "FunctionRestParameter", FunctionRestParameter$0);
}
var ParameterElement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$E)(AccessModifier), (0, import_lib3.$E)(_), (0, import_lib3.$C)(NWBindingIdentifier, BindingPattern), (0, import_lib3.$E)(TypeSuffix), (0, import_lib3.$E)(Initializer), ParameterElementDelimiter), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var accessModifier = $2;
  var binding = $4;
  var typeSuffix = $5;
  var initializer = $6;
  var delim = $7;
  return {
    type: "Parameter",
    children: $0,
    names: binding.names,
    typeSuffix,
    accessModifier,
    initializer,
    delim
  };
});
function ParameterElement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ParameterElement", ParameterElement$0);
}
var ParameterElementDelimiter$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), Comma);
var ParameterElementDelimiter$1 = (0, import_lib3.$Y)((0, import_lib3.$S)(__, (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R11, "ParameterElementDelimiter /[)}]/"))));
var ParameterElementDelimiter$2 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), InsertComma), function(value) {
  return value[1];
});
var ParameterElementDelimiter$$ = [ParameterElementDelimiter$0, ParameterElementDelimiter$1, ParameterElementDelimiter$2];
function ParameterElementDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ParameterElementDelimiter", ParameterElementDelimiter$$);
}
var BindingIdentifier$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, NWBindingIdentifier), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var identifier = $2;
  return prepend(ws, identifier);
});
function BindingIdentifier(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BindingIdentifier", BindingIdentifier$0);
}
var NWBindingIdentifier$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(At, AtIdentifierRef), function($skip, $loc, $0, $1, $2) {
  var ref = $2;
  return {
    type: "AtBinding",
    children: [ref],
    ref
  };
});
var NWBindingIdentifier$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Hash, AtIdentifierRef), function($skip, $loc, $0, $1, $2) {
  var ref = $2;
  ref = { ...ref, id: `#${ref.id}` };
  return {
    type: "AtBinding",
    children: [ref],
    ref
  };
});
var NWBindingIdentifier$2 = Identifier;
var NWBindingIdentifier$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(ReturnValue), function($skip, $loc, $0, $1) {
  return { children: [$1], names: [] };
});
var NWBindingIdentifier$$ = [NWBindingIdentifier$0, NWBindingIdentifier$1, NWBindingIdentifier$2, NWBindingIdentifier$3];
function NWBindingIdentifier(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NWBindingIdentifier", NWBindingIdentifier$$);
}
var AtIdentifierRef$0 = (0, import_lib3.$TV)(ReservedWord, function($skip, $loc, $0, $1) {
  var r = $0;
  return makeRef(`_${r}`, r);
});
var AtIdentifierRef$1 = (0, import_lib3.$TV)(IdentifierName, function($skip, $loc, $0, $1) {
  var id = $0;
  return makeRef(id.name);
});
var AtIdentifierRef$$ = [AtIdentifierRef$0, AtIdentifierRef$1];
function AtIdentifierRef(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "AtIdentifierRef", AtIdentifierRef$$);
}
var PinPattern$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Caret, SingleLineExpressionWithIndentedApplicationForbidden), function($skip, $loc, $0, $1, $2) {
  var expression = $2;
  return {
    type: "PinPattern",
    children: $0,
    expression
  };
});
var PinPattern$1 = (0, import_lib3.$TV)(ActualMemberExpression, function($skip, $loc, $0, $1) {
  var expression = $0;
  return {
    type: "PinPattern",
    children: [expression],
    expression
  };
});
var PinPattern$2 = (0, import_lib3.$TV)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R12, "PinPattern /[+-]/"), NumericLiteral), function($skip, $loc, $0, $1) {
  var expression = $0;
  return {
    type: "PinPattern",
    children: [expression],
    expression
  };
});
var PinPattern$3 = (0, import_lib3.$TV)(Undefined, function($skip, $loc, $0, $1) {
  var expression = $0;
  return {
    type: "PinPattern",
    children: [expression],
    expression
  };
});
var PinPattern$$ = [PinPattern$0, PinPattern$1, PinPattern$2, PinPattern$3];
function PinPattern(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PinPattern", PinPattern$$);
}
var BindingPattern$0 = ObjectBindingPattern;
var BindingPattern$1 = ArrayBindingPattern;
var BindingPattern$2 = PinPattern;
var BindingPattern$3 = Literal;
var BindingPattern$4 = RegularExpressionLiteral;
var BindingPattern$$ = [BindingPattern$0, BindingPattern$1, BindingPattern$2, BindingPattern$3, BindingPattern$4];
function BindingPattern(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BindingPattern", BindingPattern$$);
}
var ObjectBindingPattern$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), OpenBrace, ObjectBindingPatternContent, __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var ws1 = $1;
  var open = $2;
  var c = $3;
  var ws2 = $4;
  var close = $5;
  return {
    type: "ObjectBindingPattern",
    children: [ws1, open, c.children, ws2, close],
    names: c.names,
    properties: c.children
  };
});
function ObjectBindingPattern(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ObjectBindingPattern", ObjectBindingPattern$0);
}
var ObjectBindingPatternContent$0 = NestedBindingProperties;
var ObjectBindingPatternContent$1 = (0, import_lib3.$TV)((0, import_lib3.$E)(BindingPropertyList), function($skip, $loc, $0, $1) {
  var props = $0;
  if (!props)
    return { children: [], names: [] };
  return reorderBindingRestProperty(props);
});
var ObjectBindingPatternContent$$ = [ObjectBindingPatternContent$0, ObjectBindingPatternContent$1];
function ObjectBindingPatternContent(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ObjectBindingPatternContent", ObjectBindingPatternContent$$);
}
var BindingPropertyList$0 = (0, import_lib3.$TV)((0, import_lib3.$P)((0, import_lib3.$S)(BindingProperty, ObjectPropertyDelimiter)), function($skip, $loc, $0, $1) {
  var props = $0;
  return props.map(([prop, delim]) => {
    return {
      ...prop,
      delim,
      children: [...prop.children, delim]
    };
  });
});
function BindingPropertyList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BindingPropertyList", BindingPropertyList$0);
}
var ArrayBindingPattern$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), OpenBracket, ArrayBindingPatternContent, __, CloseBracket), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var ws1 = $1;
  var open = $2;
  var c = $3;
  var ws2 = $4;
  var close = $5;
  return {
    ...c,
    // names, blockPrefix, length
    type: "ArrayBindingPattern",
    elements: c.children,
    children: [ws1, open, c.children, ws2, close]
  };
});
function ArrayBindingPattern(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ArrayBindingPattern", ArrayBindingPattern$0);
}
var ArrayBindingPatternContent$0 = NestedBindingElements;
var ArrayBindingPatternContent$1 = (0, import_lib3.$TV)((0, import_lib3.$E)(BindingElementList), function($skip, $loc, $0, $1) {
  var elements = $0;
  if (!elements)
    return { children: [], names: [], length: 0 };
  return adjustBindingElements(elements);
});
var ArrayBindingPatternContent$$ = [ArrayBindingPatternContent$0, ArrayBindingPatternContent$1];
function ArrayBindingPatternContent(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ArrayBindingPatternContent", ArrayBindingPatternContent$$);
}
var BindingElementList$0 = (0, import_lib3.$TV)((0, import_lib3.$P)((0, import_lib3.$S)(BindingElement, ArrayElementDelimiter)), function($skip, $loc, $0, $1) {
  var elements = $0;
  return elements.map(([element, delim]) => {
    return {
      ...element,
      // BindingElement.children is a tuple of the form [ws, element]
      children: [...element.children, delim]
    };
  });
});
function BindingElementList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BindingElementList", BindingElementList$0);
}
var NestedBindingElementList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, BindingElementList), function($skip, $loc, $0, $1, $2) {
  var indent = $1;
  var elements = $2;
  return elements.map((element, i) => {
    if (i > 0)
      return element;
    return {
      ...element,
      children: [indent, ...element.children.slice(1)]
      // replace ws wth indent
    };
  });
});
function NestedBindingElementList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedBindingElementList", NestedBindingElementList$0);
}
var Elision$0 = (0, import_lib3.$S)(__, Comma);
function Elision(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Elision", Elision$0);
}
var NestedBindingProperties$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedBindingPropertyList), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var props = $2;
  if (!props.length)
    return $skip;
  return reorderBindingRestProperty(props.flat());
});
function NestedBindingProperties(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedBindingProperties", NestedBindingProperties$0);
}
var NestedBindingPropertyList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, BindingPropertyList), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var props = $2;
  return props.map((prop, i) => {
    if (i > 0)
      return prop;
    return prepend(ws, prop);
  });
});
function NestedBindingPropertyList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedBindingPropertyList", NestedBindingPropertyList$0);
}
var BindingProperty$0 = BindingRestProperty;
var BindingProperty$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), PropertyName, (0, import_lib3.$E)(_), Colon, (0, import_lib3.$E)(_), (0, import_lib3.$C)(BindingIdentifier, BindingPattern), (0, import_lib3.$E)(Initializer)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var name = $2;
  var value = $6;
  var initializer = $7;
  return {
    type: "BindingProperty",
    children: $0,
    name,
    value,
    initializer,
    names: value.names
  };
});
var BindingProperty$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$E)(Caret), BindingIdentifier, (0, import_lib3.$E)(Initializer)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var ws = $1;
  var pin = $2;
  var binding = $3;
  var initializer = $4;
  if (binding.type === "AtBinding") {
    return {
      type: "AtBindingProperty",
      children: $0,
      binding,
      ref: binding.ref,
      initializer,
      names: []
    };
  }
  if (pin) {
    return {
      type: "PinProperty",
      children: [ws, binding],
      name: binding,
      value: {
        type: "PinPattern",
        expression: binding
      }
    };
  }
  return {
    type: "BindingProperty",
    children: $0,
    name: binding,
    value: void 0,
    initializer,
    names: binding.names,
    identifier: binding
  };
});
var BindingProperty$$ = [BindingProperty$0, BindingProperty$1, BindingProperty$2];
function BindingProperty(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BindingProperty", BindingProperty$$);
}
var BindingRestProperty$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), DotDotDot, BindingIdentifier), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var dots = $2;
  var id = $3;
  return {
    ...id,
    type: "BindingRestProperty",
    children: [...ws || [], dots, ...id.children]
  };
});
var BindingRestProperty$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), BindingIdentifier, DotDotDot), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var id = $2;
  var dots = $3;
  return {
    ...id,
    type: "BindingRestProperty",
    children: [...ws || [], dots, ...id.children]
  };
});
var BindingRestProperty$$ = [BindingRestProperty$0, BindingRestProperty$1];
function BindingRestProperty(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BindingRestProperty", BindingRestProperty$$);
}
var NestedBindingElements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedBindingElementList), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var elements = $2;
  if (!elements.length)
    return $skip;
  return adjustBindingElements(elements.flat());
});
function NestedBindingElements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedBindingElements", NestedBindingElements$0);
}
var BindingElement$0 = BindingRestElement;
var BindingElement$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)(BindingIdentifier, BindingPattern), (0, import_lib3.$E)(Initializer)), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var binding = $2;
  var initializer = $3;
  if (binding.children) {
    binding = {
      ...binding,
      initializer,
      children: [...binding.children, initializer]
    };
  }
  return {
    names: binding.names,
    children: [ws, binding]
  };
});
var BindingElement$2 = (0, import_lib3.$TV)((0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$EXPECT)($L17, 'BindingElement ","'))), function($skip, $loc, $0, $1) {
  return {
    children: [{
      type: "ElisionElement",
      children: [""]
    }],
    names: []
  };
});
var BindingElement$$ = [BindingElement$0, BindingElement$1, BindingElement$2];
function BindingElement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BindingElement", BindingElement$$);
}
var BindingRestElement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), DotDotDot, (0, import_lib3.$C)(BindingIdentifier, BindingPattern, EmptyBindingPattern)), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var dots = $2;
  var binding = $3;
  return {
    type: "BindingRestElement",
    children: [ws, [dots, binding]],
    binding,
    name: binding.name,
    names: binding.names,
    rest: true
  };
});
var BindingRestElement$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)(BindingIdentifier, BindingPattern), DotDotDot), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var binding = $2;
  var dots = $3;
  return {
    type: "BindingRestElement",
    children: [...ws || [], dots, binding],
    binding,
    name: binding.name,
    names: binding.names,
    rest: true
  };
});
var BindingRestElement$$ = [BindingRestElement$0, BindingRestElement$1];
function BindingRestElement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BindingRestElement", BindingRestElement$$);
}
var EmptyBindingPattern$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'EmptyBindingPattern ""'), function($skip, $loc, $0, $1) {
  const ref = makeRef();
  return {
    type: "EmptyBinding",
    children: [ref],
    names: [],
    ref
  };
});
function EmptyBindingPattern(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EmptyBindingPattern", EmptyBindingPattern$0);
}
var FunctionDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(FunctionExpression), function($skip, $loc, $0, $1) {
  if ($1.type !== "FunctionExpression")
    return $skip;
  if ($1.id)
    return $1;
  return makeLeftHandSideExpression($1);
});
function FunctionDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "FunctionDeclaration", FunctionDeclaration$0);
}
var FunctionSignature$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Async, _)), Function2, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), Star)), (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), NWBindingIdentifier)), (0, import_lib3.$E)(_), Parameters, (0, import_lib3.$E)(ReturnTypeSuffix)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var async = $1;
  var func = $2;
  var generator = $3;
  var wid = $4;
  var w = $5;
  var parameters = $6;
  var suffix = $7;
  if (!async)
    async = [];
  if (!generator)
    generator = [];
  const id = wid?.[1];
  return {
    type: "FunctionSignature",
    id,
    name: id?.name,
    parameters,
    returnType: suffix,
    async,
    generator,
    modifier: {
      async: !!async.length,
      generator: !!generator.length
    },
    block: null,
    children: !parameters.implicit ? [async, func, generator, wid, w, parameters, suffix] : [async, func, generator, wid, parameters, w, suffix]
    // move whitespace w to after implicit () in parameters
  };
});
function FunctionSignature(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "FunctionSignature", FunctionSignature$0);
}
var FunctionExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(FunctionSignature, (0, import_lib3.$E)(BracedBlock)), function($skip, $loc, $0, $1, $2) {
  var signature = $1;
  var block = $2;
  if (!block) {
    return {
      ...signature,
      type: "FunctionExpression",
      signature,
      ts: true
    };
  }
  return {
    ...signature,
    type: "FunctionExpression",
    signature,
    children: [...signature.children, block],
    block
  };
});
var FunctionExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(ArrowFunction), OpenParen, BinaryOp, CloseParen), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $2;
  var op = $3;
  var close = $4;
  if (op.special && op.call && !op.negated)
    return op.call;
  const refA = makeRef("a"), refB = makeRef("b"), body = processBinaryOpExpression([refA, [
    [[], op, [], refB]
    // BinaryOpRHS
  ]]);
  const parameters = {
    type: "Parameters",
    children: ["(", refA, ",", refB, ")"],
    names: []
  };
  const block = {
    expressions: [body]
  };
  return {
    type: "ArrowFunction",
    signature: {
      modifier: {}
    },
    children: [open, parameters, " => ", body, close],
    body,
    parenthesized: true,
    parenthesizedOp: op,
    block,
    parameters
  };
});
var FunctionExpression$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenParen, NonPipelineAssignmentExpression, __, BinaryOp, __, CloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var open = $1;
  var lhs = $2;
  var ws1 = $3;
  var op = $4;
  var ws2 = $5;
  var close = $6;
  const refB = makeRef("b");
  const fn = makeAmpersandFunction({
    ref: refB,
    body: processBinaryOpExpression([lhs, [
      [ws1, op, ws2, refB]
      // BinaryOpRHS
    ]])
  });
  return {
    type: "ParenthesizedExpression",
    children: [open, fn, close],
    expression: fn
  };
});
var FunctionExpression$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenParen, (0, import_lib3.$P)((0, import_lib3.$S)(NotDedented, UpdateExpression, WAssignmentOp)), __, CloseParen), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $1;
  var lhs = $2;
  var ws2 = $3;
  var close = $4;
  lhs = lhs.map((x) => [x[0], x[1], ...x[2]]);
  const refB = makeRef("b");
  const fn = makeAmpersandFunction({
    ref: refB,
    body: {
      type: "AssignmentExpression",
      children: [lhs, ws2, refB],
      names: null,
      lhs,
      assigned: lhs[0][1],
      expression: refB
    }
  });
  return {
    type: "ParenthesizedExpression",
    children: [open, fn, close],
    expression: fn
  };
});
var FunctionExpression$4 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenParen, __, IsLike, __, PatternExpressionList, CloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var open = $1;
  var ws1 = $2;
  var op = $3;
  var ws2 = $4;
  var rhs = $5;
  var close = $6;
  const refA = makeRef("a");
  const fn = makeAmpersandFunction({
    ref: refA,
    body: processBinaryOpExpression([refA, [
      [ws1, op, ws2, rhs]
      // BinaryOpRHS
    ]])
  });
  return {
    type: "ParenthesizedExpression",
    children: [open, fn, close],
    expression: fn
  };
});
var FunctionExpression$5 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenParen, __, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($R13, "FunctionExpression /\\+\\+|--|[\\+\\-&]\\S/")), BinaryOp, __, NonPipelineAssignmentExpression, CloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var open = $1;
  var ws1 = $2;
  var op = $4;
  var ws2 = $5;
  var rhs = $6;
  var close = $7;
  const refA = makeRef("a");
  const fn = makeAmpersandFunction({
    ref: refA,
    body: processBinaryOpExpression([refA, [
      [ws1, op, ws2, rhs]
      // BinaryOpRHS
    ]])
  });
  return {
    type: "ParenthesizedExpression",
    children: [open, fn, close],
    expression: fn
  };
});
var FunctionExpression$$ = [FunctionExpression$0, FunctionExpression$1, FunctionExpression$2, FunctionExpression$3, FunctionExpression$4, FunctionExpression$5];
function FunctionExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "FunctionExpression", FunctionExpression$$);
}
var OperatorDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Operator, (0, import_lib3.$E)(OperatorBehavior), _, LexicalDeclaration), function($skip, $loc, $0, $1, $2, $3, $4) {
  var op = $1;
  var behavior = $2;
  var w = $3;
  var decl = $4;
  decl.names.forEach((name) => state.operators.set(name, behavior));
  return {
    ...decl,
    children: [insertTrimmingSpace(w, ""), ...decl.children]
  };
});
var OperatorDeclaration$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(OperatorSignature, BracedBlock), function($skip, $loc, $0, $1, $2) {
  var signature = $1;
  var block = $2;
  state.operators.set(signature.id.name, signature.behavior);
  return {
    ...signature,
    type: "FunctionExpression",
    signature,
    children: [...signature.children, block],
    block,
    operator: true
  };
});
var OperatorDeclaration$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(Operator, _, Identifier, (0, import_lib3.$E)(OperatorBehavior), (0, import_lib3.$Q)((0, import_lib3.$S)(CommaDelimiter, (0, import_lib3.$E)(_), Identifier, (0, import_lib3.$E)(OperatorBehavior)))), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var op = $1;
  var w1 = $2;
  var id = $3;
  var behavior = $4;
  var ids = $5;
  state.operators.set(id.name, behavior);
  ids.forEach(([, , id2, behavior2]) => state.operators.set(id2.name, behavior2));
  return {
    id,
    children: []
  };
});
var OperatorDeclaration$$ = [OperatorDeclaration$0, OperatorDeclaration$1, OperatorDeclaration$2];
function OperatorDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "OperatorDeclaration", OperatorDeclaration$$);
}
var OperatorSignature$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Async, _)), Operator, (0, import_lib3.$E)((0, import_lib3.$S)(_, Function2)), (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), Star)), _, Identifier, (0, import_lib3.$E)(OperatorBehavior), (0, import_lib3.$E)(_), NonEmptyParameters, (0, import_lib3.$E)(ReturnTypeSuffix)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10) {
  var async = $1;
  var op = $2;
  var func = $3;
  var generator = $4;
  var w1 = $5;
  var id = $6;
  var behavior = $7;
  var w2 = $8;
  var parameters = $9;
  var suffix = $10;
  if (!async)
    async = [];
  if (!generator)
    generator = [];
  if (!func) {
    func = { $loc: op.$loc, token: "function" };
  } else {
    func = [insertTrimmingSpace(func[0], ""), func[1]];
  }
  return {
    type: "FunctionSignature",
    id,
    name: id.name,
    parameters,
    returnType: suffix,
    async,
    generator,
    modifier: {
      async: !!async.length,
      generator: !!generator.length
    },
    block: null,
    children: [async, func, generator, w1, id, w2, parameters, suffix],
    behavior
  };
});
function OperatorSignature(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "OperatorSignature", OperatorSignature$0);
}
var OperatorBehavior$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OperatorPrecedence, (0, import_lib3.$E)(OperatorAssociativity)), function($skip, $loc, $0, $1, $2) {
  return { ...$1, ...$2 };
});
var OperatorBehavior$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(OperatorAssociativity, (0, import_lib3.$E)(OperatorPrecedence)), function($skip, $loc, $0, $1, $2) {
  return { ...$1, ...$2 };
});
var OperatorBehavior$$ = [OperatorBehavior$0, OperatorBehavior$1];
function OperatorBehavior(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "OperatorBehavior", OperatorBehavior$$);
}
var OperatorPrecedence$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)((0, import_lib3.$EXPECT)($L26, 'OperatorPrecedence "tighter"'), (0, import_lib3.$EXPECT)($L27, 'OperatorPrecedence "looser"'), (0, import_lib3.$EXPECT)($L28, 'OperatorPrecedence "same"')), NonIdContinue, (0, import_lib3.$E)(_), (0, import_lib3.$C)(Identifier, (0, import_lib3.$S)(OpenParen, BinaryOp, CloseParen))), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var mod = $2;
  var op = $5;
  let prec = op.type === "Identifier" ? state.operators.get(op.name).prec : getPrecedence(op[1]);
  switch (mod) {
    case "tighter":
      prec += 1 / 64;
      break;
    case "looser":
      prec -= 1 / 64;
      break;
  }
  return { prec };
});
function OperatorPrecedence(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "OperatorPrecedence", OperatorPrecedence$0);
}
var OperatorAssociativity$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)((0, import_lib3.$EXPECT)($L29, 'OperatorAssociativity "left"'), (0, import_lib3.$EXPECT)($L30, 'OperatorAssociativity "right"'), (0, import_lib3.$EXPECT)($L31, 'OperatorAssociativity "non"'), (0, import_lib3.$EXPECT)($L32, 'OperatorAssociativity "relational"'), (0, import_lib3.$EXPECT)($L33, 'OperatorAssociativity "arguments"')), NonIdContinue), function($skip, $loc, $0, $1, $2, $3) {
  var assoc = $2;
  if (assoc === "relational") {
    return { relational: true, assoc: "non" };
  }
  return { assoc };
});
function OperatorAssociativity(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "OperatorAssociativity", OperatorAssociativity$0);
}
var ThinArrowFunction$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Async, _)), ArrowParameters, (0, import_lib3.$E)(ReturnTypeSuffix), (0, import_lib3.$E)(_), Arrow, NoCommaBracedOrEmptyBlock), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var async = $1;
  var parameters = $2;
  var suffix = $3;
  var arrow = $5;
  var block = $6;
  if (!async)
    async = [];
  const generator = [];
  return {
    type: "FunctionExpression",
    id: void 0,
    parameters,
    returnType: suffix,
    async,
    generator,
    block,
    signature: {
      name: void 0,
      async,
      generator,
      modifier: {
        async: !!async.length,
        generator: !!generator.length
      },
      returnType: suffix
    },
    children: [
      async,
      { $loc: arrow.$loc, token: "function" },
      generator,
      parameters,
      suffix,
      block
    ]
  };
});
function ThinArrowFunction(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ThinArrowFunction", ThinArrowFunction$0);
}
var Arrow$0 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L34, 'Arrow "->"'), (0, import_lib3.$EXPECT)($L35, 'Arrow "\u2192"')), function($skip, $loc, $0, $1) {
  return { $loc, token: "->" };
});
function Arrow(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Arrow", Arrow$0);
}
var ExplicitBlock$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, OpenBrace, __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4) {
  const expressions = [];
  return {
    type: "BlockStatement",
    expressions,
    children: [$1, $2, expressions, $3, $4],
    bare: false,
    empty: true
  };
});
var ExplicitBlock$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, OpenBrace, NestedBlockStatements, __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var block = $3;
  return {
    ...block,
    children: [$1, $2, ...block.children, $4, $5],
    bare: false
  };
});
var ExplicitBlock$$ = [ExplicitBlock$0, ExplicitBlock$1];
function ExplicitBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ExplicitBlock", ExplicitBlock$$);
}
var ImplicitNestedBlock$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), InsertOpenBrace, AllowAll, (0, import_lib3.$E)((0, import_lib3.$S)(NestedBlockStatements, InsertNewline, InsertIndent, InsertCloseBrace)), RestoreAll), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var open = $2;
  if (!$4)
    return $skip;
  const [block, ...tail] = $4;
  return {
    ...block,
    children: [open, ...block.children, ...tail],
    bare: false
  };
});
function ImplicitNestedBlock(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImplicitNestedBlock", ImplicitNestedBlock$0);
}
var Block$0 = ImplicitNestedBlock;
var Block$1 = ExplicitBlock;
var Block$2 = ThenClause;
var Block$3 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$N)(EOS), Statement), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var s = $3;
  const expressions = [[ws, s]];
  return {
    type: "BlockStatement",
    expressions,
    children: [expressions],
    bare: true
  };
});
var Block$$ = [Block$0, Block$1, Block$2, Block$3];
function Block(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Block", Block$$);
}
var BareNestedBlock$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), AllowAll, (0, import_lib3.$E)(NestedBlockStatements), RestoreAll), function($skip, $loc, $0, $1, $2, $3, $4) {
  if (!$3)
    return $skip;
  return $3;
});
function BareNestedBlock(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BareNestedBlock", BareNestedBlock$0);
}
var BareBlock$0 = BareNestedBlock;
var BareBlock$1 = ExplicitBlock;
var BareBlock$2 = ThenClause;
var BareBlock$3 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$N)(EOS), Statement), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var s = $3;
  const expressions = [[ws, s]];
  return {
    type: "BlockStatement",
    expressions,
    children: [expressions],
    bare: true
  };
});
var BareBlock$4 = EmptyBareBlock;
var BareBlock$$ = [BareBlock$0, BareBlock$1, BareBlock$2, BareBlock$3, BareBlock$4];
function BareBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BareBlock", BareBlock$$);
}
var ThenClause$0 = (0, import_lib3.$T)((0, import_lib3.$S)(Then, SingleLineStatements), function(value) {
  return value[1];
});
function ThenClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ThenClause", ThenClause$0);
}
var BracedThenClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Y)(Then), InsertOpenBrace, ThenClause, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $2;
  var exp = $3;
  var close = $4;
  const expressions = [exp];
  return {
    type: "BlockStatement",
    expressions,
    children: [open, expressions, " ", close],
    bare: false
  };
});
function BracedThenClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BracedThenClause", BracedThenClause$0);
}
var BracedOrEmptyBlock$0 = BracedBlock;
var BracedOrEmptyBlock$1 = EmptyBlock;
var BracedOrEmptyBlock$$ = [BracedOrEmptyBlock$0, BracedOrEmptyBlock$1];
function BracedOrEmptyBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BracedOrEmptyBlock", BracedOrEmptyBlock$$);
}
var NoCommaBracedOrEmptyBlock$0 = NoCommaBracedBlock;
var NoCommaBracedOrEmptyBlock$1 = EmptyBlock;
var NoCommaBracedOrEmptyBlock$$ = [NoCommaBracedOrEmptyBlock$0, NoCommaBracedOrEmptyBlock$1];
function NoCommaBracedOrEmptyBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NoCommaBracedOrEmptyBlock", NoCommaBracedOrEmptyBlock$$);
}
var NoPostfixBracedOrEmptyBlock$0 = NoPostfixBracedBlock;
var NoPostfixBracedOrEmptyBlock$1 = EmptyBlock;
var NoPostfixBracedOrEmptyBlock$$ = [NoPostfixBracedOrEmptyBlock$0, NoPostfixBracedOrEmptyBlock$1];
function NoPostfixBracedOrEmptyBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NoPostfixBracedOrEmptyBlock", NoPostfixBracedOrEmptyBlock$$);
}
var EmptyBlock$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBrace, InsertCloseBrace), function($skip, $loc, $0, $1, $2) {
  const expressions = [];
  return {
    type: "BlockStatement",
    expressions,
    children: [$1, expressions, $2],
    bare: false,
    empty: true,
    implicit: true
  };
});
function EmptyBlock(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EmptyBlock", EmptyBlock$0);
}
var BlockOrEmptyStatement$0 = Block;
var BlockOrEmptyStatement$1 = (0, import_lib3.$T)((0, import_lib3.$S)(NoBlock, EmptyStatementBareBlock), function(value) {
  return value[1];
});
var BlockOrEmptyStatement$$ = [BlockOrEmptyStatement$0, BlockOrEmptyStatement$1];
function BlockOrEmptyStatement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BlockOrEmptyStatement", BlockOrEmptyStatement$$);
}
var BlockOrEmpty$0 = Block;
var BlockOrEmpty$1 = (0, import_lib3.$T)((0, import_lib3.$S)(NoBlock, EmptyBlock), function(value) {
  return value[1];
});
var BlockOrEmpty$$ = [BlockOrEmpty$0, BlockOrEmpty$1];
function BlockOrEmpty(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BlockOrEmpty", BlockOrEmpty$$);
}
var EmptyStatementBareBlock$0 = (0, import_lib3.$TV)(InsertEmptyStatement, function($skip, $loc, $0, $1) {
  var s = $0;
  const expressions = [["", s]];
  return {
    type: "BlockStatement",
    expressions,
    children: [expressions],
    bare: true,
    empty: true,
    implicit: true,
    semicolon: s.children[0]
  };
});
function EmptyStatementBareBlock(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EmptyStatementBareBlock", EmptyStatementBareBlock$0);
}
var EmptyBareBlock$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'EmptyBareBlock ""'), function($skip, $loc, $0, $1) {
  const expressions = [];
  return {
    type: "BlockStatement",
    expressions,
    children: [expressions],
    bare: true,
    empty: true,
    implicit: true
  };
});
function EmptyBareBlock(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EmptyBareBlock", EmptyBareBlock$0);
}
var NoBlock$0 = (0, import_lib3.$S)((0, import_lib3.$Y)(EOS), (0, import_lib3.$N)(IndentedFurther));
function NoBlock(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NoBlock", NoBlock$0);
}
var BracedBlock$0 = NonSingleBracedBlock;
var BracedBlock$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBrace, (0, import_lib3.$N)(EOS), PostfixedSingleLineStatements, InsertSpace, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var o = $1;
  var s = $3;
  var ws = $4;
  var c = $5;
  if (!s.children.length)
    return $skip;
  return {
    type: "BlockStatement",
    expressions: s.expressions,
    // Remove !EOS assertion
    children: [o, s.children, ws, c]
  };
});
var BracedBlock$$ = [BracedBlock$0, BracedBlock$1];
function BracedBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BracedBlock", BracedBlock$$);
}
var NoPostfixBracedBlock$0 = NonSingleBracedBlock;
var NoPostfixBracedBlock$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBrace, (0, import_lib3.$N)(EOS), SingleLineStatements, InsertSpace, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var o = $1;
  var s = $3;
  var ws = $4;
  var c = $5;
  if (!s.expressions.length)
    return $skip;
  return {
    type: "BlockStatement",
    expressions: s.expressions,
    // Remove !EOS assertion
    children: [o, s.children, ws, c]
  };
});
var NoPostfixBracedBlock$$ = [NoPostfixBracedBlock$0, NoPostfixBracedBlock$1];
function NoPostfixBracedBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NoPostfixBracedBlock", NoPostfixBracedBlock$$);
}
var NoCommaBracedBlock$0 = NonSingleBracedBlock;
var NoCommaBracedBlock$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBrace, (0, import_lib3.$N)(EOS), PostfixedSingleLineNoCommaStatements, InsertSpace, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var o = $1;
  var s = $3;
  var ws = $4;
  var c = $5;
  if (!s.children.length)
    return $skip;
  return {
    type: "BlockStatement",
    expressions: s.expressions,
    // Remove !EOS assertion
    children: [o, s.children, ws, c]
  };
});
var NoCommaBracedBlock$$ = [NoCommaBracedBlock$0, NoCommaBracedBlock$1];
function NoCommaBracedBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NoCommaBracedBlock", NoCommaBracedBlock$$);
}
var NonSingleBracedBlock$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), OpenBrace, AllowAll, (0, import_lib3.$E)((0, import_lib3.$S)(BracedContent, __, CloseBrace)), RestoreAll), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var ws1 = $1;
  var open = $2;
  if (!$4)
    return $skip;
  const [block, ws2, close] = $4;
  return {
    type: "BlockStatement",
    expressions: block.expressions,
    children: [ws1, open, ...block.children, ws2, close],
    bare: false
  };
  return block;
});
var NonSingleBracedBlock$1 = ImplicitNestedBlock;
var NonSingleBracedBlock$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBrace, NestedImplicitObjectLiteral, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3) {
  var o = $1;
  var s = $2;
  var c = $3;
  const expressions = [s];
  return {
    type: "BlockStatement",
    expressions,
    children: [o, expressions, c]
  };
});
var NonSingleBracedBlock$$ = [NonSingleBracedBlock$0, NonSingleBracedBlock$1, NonSingleBracedBlock$2];
function NonSingleBracedBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NonSingleBracedBlock", NonSingleBracedBlock$$);
}
var DeclarationOrStatement$0 = Declaration;
var DeclarationOrStatement$1 = Statement;
var DeclarationOrStatement$$ = [DeclarationOrStatement$0, DeclarationOrStatement$1];
function DeclarationOrStatement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "DeclarationOrStatement", DeclarationOrStatement$$);
}
var SingleLineStatements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidNewlineBinaryOp, (0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$N)(EOS)), DeclarationOrStatement, SemicolonDelimiter)), (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$N)(EOS)), DeclarationOrStatement, (0, import_lib3.$E)(SemicolonDelimiter))), RestoreNewlineBinaryOp), function($skip, $loc, $0, $1, $2, $3, $4) {
  var stmts = $2;
  var last = $3;
  const expressions = [...stmts];
  if (last)
    expressions.push(last);
  const maybeComment = expressions.at(-1)?.[2]?.children?.[2]?.at(-1);
  let hasTrailingComment = false;
  if (maybeComment?.type === "Comment" && maybeComment.token.startsWith("//")) {
    hasTrailingComment = true;
  }
  const children = [expressions];
  if (hasTrailingComment) {
    children.push("\n");
  }
  return {
    type: "BlockStatement",
    expressions,
    children,
    bare: true
  };
});
function SingleLineStatements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SingleLineStatements", SingleLineStatements$0);
}
var PostfixedSingleLineStatements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$N)(EOS)), StatementListItem, SemicolonDelimiter)), (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$N)(EOS)), StatementListItem, (0, import_lib3.$E)(SemicolonDelimiter)))), function($skip, $loc, $0, $1, $2) {
  var stmts = $1;
  var last = $2;
  const children = [...stmts];
  if (last)
    children.push(last);
  return {
    type: "BlockStatement",
    expressions: children,
    children,
    bare: true
  };
});
function PostfixedSingleLineStatements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PostfixedSingleLineStatements", PostfixedSingleLineStatements$0);
}
var PostfixedSingleLineNoCommaStatements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$N)(EOS)), NoCommaStatementListItem, SemicolonDelimiter)), (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$N)(EOS)), NoCommaStatementListItem, (0, import_lib3.$E)(SemicolonDelimiter)))), function($skip, $loc, $0, $1, $2) {
  var stmts = $1;
  var last = $2;
  const children = [...stmts];
  if (last)
    children.push(last);
  return {
    type: "BlockStatement",
    expressions: children,
    children,
    bare: true
  };
});
function PostfixedSingleLineNoCommaStatements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PostfixedSingleLineNoCommaStatements", PostfixedSingleLineNoCommaStatements$0);
}
var BracedContent$0 = NestedBlockStatements;
var BracedContent$1 = SingleLineStatements;
var BracedContent$2 = (0, import_lib3.$TV)((0, import_lib3.$Y)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L36, 'BracedContent "}"'))), function($skip, $loc, $0, $1) {
  const expressions = [];
  return {
    type: "BlockStatement",
    expressions,
    children: [expressions]
  };
});
var BracedContent$$ = [BracedContent$0, BracedContent$1, BracedContent$2];
function BracedContent(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BracedContent", BracedContent$$);
}
var NestedBlockStatements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedBlockStatement), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var statements = $2;
  if (!statements.length)
    return $skip;
  statements = statements.flat();
  return {
    type: "BlockStatement",
    expressions: statements,
    children: [statements],
    bare: true
  };
});
function NestedBlockStatements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedBlockStatements", NestedBlockStatements$0);
}
var NestedBlockStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, (0, import_lib3.$P)(BlockStatementPart)), function($skip, $loc, $0, $1, $2) {
  var nested = $1;
  var statements = $2;
  return [
    [nested, ...statements[0]],
    ...statements.slice(1).map((s) => ["", ...s])
  ];
});
function NestedBlockStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedBlockStatement", NestedBlockStatement$0);
}
var BlockStatementPart$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), (0, import_lib3.$E)(_), StatementListItem, StatementDelimiter), function($skip, $loc, $0, $1, $2, $3, $4) {
  var ws = $2;
  var statement = $3;
  var delimiter = $4;
  if (ws)
    statement = prepend(ws, statement);
  return [statement, delimiter];
});
function BlockStatementPart(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BlockStatementPart", BlockStatementPart$0);
}
var Literal$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R14, `Literal /(?=[0-9.'"tfyno])/`), LiteralContent), function($skip, $loc, $0, $1, $2) {
  var literal = $2;
  return {
    type: "Literal",
    subtype: literal.type,
    children: [literal],
    raw: literal.token
  };
});
function Literal(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Literal", Literal$0);
}
var LiteralContent$0 = NullLiteral;
var LiteralContent$1 = BooleanLiteral;
var LiteralContent$2 = NumericLiteral;
var LiteralContent$3 = StringLiteral;
var LiteralContent$$ = [LiteralContent$0, LiteralContent$1, LiteralContent$2, LiteralContent$3];
function LiteralContent(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "LiteralContent", LiteralContent$$);
}
var NullLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L37, 'NullLiteral "null"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function NullLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NullLiteral", NullLiteral$0);
}
var BooleanLiteral$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R15, "BooleanLiteral /(?=true|false|yes|no|on|off)/"), _BooleanLiteral), function(value) {
  return value[1];
});
function BooleanLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BooleanLiteral", BooleanLiteral$0);
}
var _BooleanLiteral$0 = (0, import_lib3.$T)((0, import_lib3.$S)(CoffeeBooleansEnabled, CoffeeScriptBooleanLiteral), function(value) {
  return value[1];
});
var _BooleanLiteral$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L38, '_BooleanLiteral "true"'), (0, import_lib3.$EXPECT)($L39, '_BooleanLiteral "false"')), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
var _BooleanLiteral$$ = [_BooleanLiteral$0, _BooleanLiteral$1];
function _BooleanLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "_BooleanLiteral", _BooleanLiteral$$);
}
var CoffeeScriptBooleanLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L40, 'CoffeeScriptBooleanLiteral "yes"'), (0, import_lib3.$EXPECT)($L41, 'CoffeeScriptBooleanLiteral "on"')), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: "true" };
});
var CoffeeScriptBooleanLiteral$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L42, 'CoffeeScriptBooleanLiteral "no"'), (0, import_lib3.$EXPECT)($L43, 'CoffeeScriptBooleanLiteral "off"')), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: "false" };
});
var CoffeeScriptBooleanLiteral$$ = [CoffeeScriptBooleanLiteral$0, CoffeeScriptBooleanLiteral$1];
function CoffeeScriptBooleanLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CoffeeScriptBooleanLiteral", CoffeeScriptBooleanLiteral$$);
}
var Identifier$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R16, "Identifier /(?=\\p{ID_Start}|[_$])/"), (0, import_lib3.$N)(ReservedWord), IdentifierName), function(value) {
  var id = value[2];
  return id;
});
function Identifier(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Identifier", Identifier$0);
}
var IdentifierName$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R17, "IdentifierName /(?:\\p{ID_Start}|[_$])(?:\\p{ID_Continue}|[\\u200C\\u200D$])*/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return {
    type: "Identifier",
    name: $0,
    names: [$0],
    children: [{
      $loc,
      token: $0
    }]
  };
});
function IdentifierName(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IdentifierName", IdentifierName$0);
}
var IdentifierReference$0 = Identifier;
function IdentifierReference(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IdentifierReference", IdentifierReference$0);
}
var UpcomingAssignment$0 = (0, import_lib3.$Y)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L3, 'UpcomingAssignment "="'), (0, import_lib3.$N)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L3, 'UpcomingAssignment "="'), (0, import_lib3.$EXPECT)($L44, 'UpcomingAssignment ">"')))));
function UpcomingAssignment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "UpcomingAssignment", UpcomingAssignment$0);
}
var ArrayLiteral$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R18, "ArrayLiteral /(?=\\[)/"), _ArrayLiteral), function(value) {
  return value[1];
});
function ArrayLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ArrayLiteral", ArrayLiteral$0);
}
var _ArrayLiteral$0 = (0, import_lib3.$T)((0, import_lib3.$S)(ArrayBindingPattern, UpcomingAssignment), function(value) {
  return value[0];
});
var _ArrayLiteral$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenBracket, AllowAll, (0, import_lib3.$E)((0, import_lib3.$S)(ArrayLiteralContent, __, CloseBracket)), RestoreAll), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $1;
  if (!$3)
    return $skip;
  const [content, ws, close] = $3;
  if (content.type === "RangeExpression") {
    return prepend(ws, content);
  }
  let children;
  if (Array.isArray(content)) {
    children = [open, ...content, ...ws, close];
  } else {
    children = [open, content, ...ws, close];
  }
  const names = children.flatMap((c) => c?.names || []);
  return {
    type: "ArrayExpression",
    children,
    names
  };
});
var _ArrayLiteral$$ = [_ArrayLiteral$0, _ArrayLiteral$1];
function _ArrayLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "_ArrayLiteral", _ArrayLiteral$$);
}
var RangeExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ExtendedExpression, __, (0, import_lib3.$C)(DotDotDot, DotDot), ExtendedExpression), function($skip, $loc, $0, $1, $2, $3, $4) {
  var s = $1;
  var ws = $2;
  var range = $3;
  var e = $4;
  const inclusive = range.token === "..";
  range.token = ",";
  if (s.type === "Literal" && e.type === "Literal") {
    const start = literalValue(s);
    const end = literalValue(e);
    if (typeof start !== typeof end) {
      throw new Error("Range start and end must be of the same type");
    }
    if (typeof start === "string") {
      if (start.length !== 1 || end.length !== 1) {
        throw new Error("String range start and end must be a single character");
      }
      const startCode = start.charCodeAt(0);
      const endCode = end.charCodeAt(0);
      const step = startCode < endCode ? 1 : -1;
      const length = Math.abs(endCode - startCode) + (inclusive ? 1 : 0);
      if (length <= 26) {
        return {
          type: "RangeExpression",
          children: ["[", Array.from({ length }, (_2, i) => JSON.stringify(String.fromCharCode(startCode + i * step))).join(", "), "]"],
          inclusive,
          start: s,
          end: e
        };
      } else {
        const inclusiveAdjust2 = inclusive ? " + 1" : "";
        const children2 = ["((s, e) => {let step = e > s ? 1 : -1; return Array.from({length: Math.abs(e - s)", inclusiveAdjust2, "}, (_, i) => String.fromCharCode(s + i * step))})(", startCode.toString(), ws, range, endCode.toString(), ")"];
        return {
          type: "RangeExpression",
          children: children2,
          inclusive,
          start: s,
          end: e
        };
      }
    } else if (typeof start === "number") {
      const step = end > start ? 1 : -1;
      const length = Math.abs(end - start) + (inclusive ? 1 : 0);
      if (length <= 20) {
        return {
          type: "RangeExpression",
          children: ["[", Array.from({ length }, (_2, i) => start + i * step).join(", "), "]"],
          inclusive,
          start: s,
          end: e
        };
      }
    }
  }
  const inclusiveAdjust = inclusive ? " + 1" : "";
  const children = ["((s, e) => {let step = e > s ? 1 : -1; return Array.from({length: Math.abs(e - s)", inclusiveAdjust, "}, (_, i) => s + i * step)})(", s, ws, range, e, ")"];
  return {
    type: "RangeExpression",
    children,
    inclusive,
    start: s,
    end: e
  };
});
var RangeExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(ExtendedExpression, __, DotDot, (0, import_lib3.$Y)((0, import_lib3.$S)(__, CloseBracket))), function($skip, $loc, $0, $1, $2, $3, $4) {
  var s = $1;
  var ws = $2;
  return {
    type: "RangeExpression",
    children: ["[]", {
      type: "Error",
      message: "Infinite range [x..] is only valid in for loops"
    }],
    start: s,
    end: {
      type: "Identifier",
      name: "Infinity",
      children: ["Infinity"]
    }
  };
});
var RangeExpression$$ = [RangeExpression$0, RangeExpression$1];
function RangeExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "RangeExpression", RangeExpression$$);
}
var ArrayLiteralContent$0 = RangeExpression;
var ArrayLiteralContent$1 = (0, import_lib3.$S)(NestedElementList, (0, import_lib3.$Y)((0, import_lib3.$S)(__, CloseBracket)));
var ArrayLiteralContent$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(ElementListWithIndentedApplicationForbidden, ArrayElementDelimiter, (0, import_lib3.$E)(NestedElementList), (0, import_lib3.$Y)((0, import_lib3.$S)(__, CloseBracket))), function($skip, $loc, $0, $1, $2, $3, $4) {
  var list = $1;
  var delimiter = $2;
  var nested = $3;
  if (!nested)
    return list;
  return [...list, delimiter, ...nested];
});
var ArrayLiteralContent$3 = (0, import_lib3.$TV)((0, import_lib3.$Q)((0, import_lib3.$S)(__, ElementListWithIndentedApplicationForbidden, ArrayElementDelimiter)), function($skip, $loc, $0, $1) {
  return $1.flat();
});
var ArrayLiteralContent$$ = [ArrayLiteralContent$0, ArrayLiteralContent$1, ArrayLiteralContent$2, ArrayLiteralContent$3];
function ArrayLiteralContent(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ArrayLiteralContent", ArrayLiteralContent$$);
}
var NestedElementList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedElement), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var elements = $2;
  if (elements.length)
    return elements.flat();
  return $skip;
});
function NestedElementList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedElementList", NestedElementList$0);
}
var NestedElement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, ElementList, ArrayElementDelimiter), function($skip, $loc, $0, $1, $2, $3) {
  var indent = $1;
  var list = $2;
  var delimiter = $3;
  const { length } = list;
  if (length) {
    return list.map((e, i) => {
      if (i === 0 && i === length - 1) {
        return { ...e, children: [indent, ...e.children, delimiter] };
      }
      if (i === 0) {
        return { ...e, children: [indent, ...e.children] };
      }
      if (i === length - 1) {
        return { ...e, children: [...e.children, delimiter] };
      }
      return e;
    });
  }
});
function NestedElement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedElement", NestedElement$0);
}
var ArrayElementDelimiter$0 = (0, import_lib3.$S)(__, Comma);
var ArrayElementDelimiter$1 = (0, import_lib3.$Y)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L45, 'ArrayElementDelimiter "]"')));
var ArrayElementDelimiter$2 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), InsertComma), function(value) {
  return value[1];
});
var ArrayElementDelimiter$$ = [ArrayElementDelimiter$0, ArrayElementDelimiter$1, ArrayElementDelimiter$2];
function ArrayElementDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ArrayElementDelimiter", ArrayElementDelimiter$$);
}
var ElementListWithIndentedApplicationForbidden$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidIndentedApplication, (0, import_lib3.$E)(ElementList), RestoreIndentedApplication), function($skip, $loc, $0, $1, $2, $3) {
  if ($2)
    return $2;
  return $skip;
});
function ElementListWithIndentedApplicationForbidden(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ElementListWithIndentedApplicationForbidden", ElementListWithIndentedApplicationForbidden$0);
}
var ElementList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), ArrayElementExpression, (0, import_lib3.$Q)(ElementListRest)), function($skip, $loc, $0, $1, $2, $3) {
  var first = $2;
  var rest = $3;
  if (rest.length) {
    return [{
      ...first,
      children: [...first.children, rest[0][0]]
    }].concat(rest.map(([_2, e], i) => {
      const delim = rest[i + 1]?.[0];
      return {
        ...e,
        children: [...e.children, delim]
      };
    }));
  }
  return [first];
});
function ElementList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ElementList", ElementList$0);
}
var ElementListRest$0 = (0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), Comma, (0, import_lib3.$N)(EOS)), ArrayElementExpression);
function ElementListRest(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ElementListRest", ElementListRest$0);
}
var ArrayElementExpression$0 = JSXTag;
var ArrayElementExpression$1 = (0, import_lib3.$T)((0, import_lib3.$S)(ImplicitObjectLiteral, (0, import_lib3.$Y)(ArrayElementDelimiter)), function(value) {
  return value[0];
});
var ArrayElementExpression$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(ExtendedExpression, (0, import_lib3.$E)(_), DotDotDot, (0, import_lib3.$Y)(ArrayElementDelimiter)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var exp = $1;
  var ws = $2;
  var dots = $3;
  if (!exp) {
    exp = { ...makeRef(), names: [] };
  }
  return {
    type: "SpreadElement",
    children: [ws, dots, exp],
    names: exp.names
  };
});
var ArrayElementExpression$3 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(__, DotDotDot, __)), PostfixedExpression)), (0, import_lib3.$Y)(ArrayElementDelimiter)), function($skip, $loc, $0, $1, $2) {
  var expMaybeSpread = $1;
  if (expMaybeSpread) {
    const [spread, exp] = expMaybeSpread;
    if (!spread) {
      return {
        type: "ArrayElement",
        children: [exp],
        names: exp.names
      };
    } else {
      return {
        type: "SpreadElement",
        children: [...spread, exp],
        names: exp.names
      };
    }
  }
  return {
    type: "ElisionElement",
    children: []
  };
});
var ArrayElementExpression$$ = [ArrayElementExpression$0, ArrayElementExpression$1, ArrayElementExpression$2, ArrayElementExpression$3];
function ArrayElementExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ArrayElementExpression", ArrayElementExpression$$);
}
var ObjectLiteral$0 = (0, import_lib3.$T)((0, import_lib3.$S)(ObjectBindingPattern, UpcomingAssignment), function(value) {
  return value[0];
});
var ObjectLiteral$1 = BracedObjectLiteral;
var ObjectLiteral$2 = NestedImplicitObjectLiteral;
var ObjectLiteral$3 = InlineObjectLiteral;
var ObjectLiteral$$ = [ObjectLiteral$0, ObjectLiteral$1, ObjectLiteral$2, ObjectLiteral$3];
function ObjectLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ObjectLiteral", ObjectLiteral$$);
}
var BracedObjectLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenBrace, AllowAll, (0, import_lib3.$E)((0, import_lib3.$S)(BracedObjectLiteralContent, __, CloseBrace)), RestoreAll), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $1;
  if (!$3)
    return $skip;
  const [properties, ...close] = $3;
  return {
    type: "ObjectExpression",
    children: [open, properties, close],
    names: properties.flatMap((c) => c.names || []),
    properties
  };
});
function BracedObjectLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BracedObjectLiteral", BracedObjectLiteral$0);
}
var BracedObjectLiteralContent$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)((0, import_lib3.$S)(PropertyDefinition, ObjectPropertyDelimiter)), (0, import_lib3.$E)(NestedPropertyDefinitions)), function($skip, $loc, $0, $1, $2) {
  var line = $1;
  var nested = $2;
  line = line.flatMap(([prop, delim]) => {
    prop = Array.isArray(prop) ? prop : [prop];
    let last = prop[prop.length - 1];
    if (!last)
      return [];
    last = {
      ...last,
      delim,
      children: [...last.children, delim]
    };
    return [...prop.slice(0, prop.length - 1), last];
  });
  return line.concat(nested || []);
});
var BracedObjectLiteralContent$1 = (0, import_lib3.$TV)((0, import_lib3.$P)((0, import_lib3.$S)(__, PropertyDefinition, ObjectPropertyDelimiter)), function($skip, $loc, $0, $1) {
  return $0.flatMap(([ws, prop, delim]) => {
    prop = Array.isArray(prop) ? prop : [prop];
    let last = prop[prop.length - 1];
    last = {
      ...last,
      delim,
      // __ will consume all whitespace that _? in PropertyDefinition could,
      // so replace _? (via slice) with __
      children: [ws, ...last.children.slice(1), delim]
    };
    return [...prop.slice(0, prop.length - 1), last];
  });
});
var BracedObjectLiteralContent$$ = [BracedObjectLiteralContent$0, BracedObjectLiteralContent$1];
function BracedObjectLiteralContent(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BracedObjectLiteralContent", BracedObjectLiteralContent$$);
}
var NestedImplicitObjectLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBrace, PushIndent, (0, import_lib3.$E)(NestedImplicitPropertyDefinitions), PopIndent, InsertNewline, InsertIndent, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var properties = $3;
  if (!properties)
    return $skip;
  return {
    type: "ObjectExpression",
    properties,
    children: $0
  };
});
function NestedImplicitObjectLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedImplicitObjectLiteral", NestedImplicitObjectLiteral$0);
}
var NestedImplicitPropertyDefinitions$0 = (0, import_lib3.$TV)((0, import_lib3.$P)(NestedImplicitPropertyDefinition), function($skip, $loc, $0, $1) {
  var defs = $0;
  return defs.flat();
});
function NestedImplicitPropertyDefinitions(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedImplicitPropertyDefinitions", NestedImplicitPropertyDefinitions$0);
}
var NestedImplicitPropertyDefinition$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, (0, import_lib3.$P)((0, import_lib3.$S)((0, import_lib3.$E)(_), NamedProperty, ObjectPropertyDelimiter))), function($skip, $loc, $0, $1, $2) {
  var indent = $1;
  var props = $2;
  return props.map(([ws, prop, delimiter], i) => ({
    ...prop,
    children: [
      ...i === 0 ? [indent, ws] : [ws],
      ...prop.children,
      delimiter
    ]
  }));
});
function NestedImplicitPropertyDefinition(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedImplicitPropertyDefinition", NestedImplicitPropertyDefinition$0);
}
var NestedPropertyDefinitions$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedPropertyDefinition), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var defs = $2;
  if (!defs.length)
    return $skip;
  return defs.flat();
});
function NestedPropertyDefinitions(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedPropertyDefinitions", NestedPropertyDefinitions$0);
}
var NestedPropertyDefinition$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, (0, import_lib3.$P)((0, import_lib3.$S)(PropertyDefinition, ObjectPropertyDelimiter))), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var inlineProps = $2;
  return inlineProps.flatMap(([prop, delim], i) => {
    if (!Array.isArray(prop))
      prop = [prop];
    if (i === 0) {
      const [first, ...rest] = prop;
      prop = [prepend(ws, first), ...rest];
    }
    const last = prop[prop.length - 1];
    prop = [
      ...prop.slice(0, prop.length - 1),
      {
        ...last,
        delim,
        children: [...last.children, delim]
      }
    ];
    return prop;
  });
});
function NestedPropertyDefinition(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedPropertyDefinition", NestedPropertyDefinition$0);
}
var ImplicitObjectLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertInlineOpenBrace, SnugNamedProperty, (0, import_lib3.$Q)((0, import_lib3.$S)(ImplicitObjectPropertyDelimiter, NamedProperty)), (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), Comma)), InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var open = $1;
  var first = $2;
  var rest = $3;
  var trailing = $4;
  var close = $5;
  return {
    type: "ObjectExpression",
    children: [open, first, ...rest, trailing, close]
  };
});
function ImplicitObjectLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImplicitObjectLiteral", ImplicitObjectLiteral$0);
}
var ImplicitObjectPropertyDelimiter$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), Comma, (0, import_lib3.$C)(NotDedented, (0, import_lib3.$E)(_)));
var ImplicitObjectPropertyDelimiter$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)((0, import_lib3.$S)(Nested, NamedProperty)), InsertComma, Nested), function(value) {
  return [value[1], value[2]];
});
var ImplicitObjectPropertyDelimiter$$ = [ImplicitObjectPropertyDelimiter$0, ImplicitObjectPropertyDelimiter$1];
function ImplicitObjectPropertyDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ImplicitObjectPropertyDelimiter", ImplicitObjectPropertyDelimiter$$);
}
var InlineObjectLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertInlineOpenBrace, SnugNamedProperty, (0, import_lib3.$Q)((0, import_lib3.$S)(InlineObjectPropertyDelimiter, NamedProperty)), (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), Comma, (0, import_lib3.$Y)(Dedented))), InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var open = $1;
  var first = $2;
  var rest = $3;
  var trailing = $4;
  var close = $5;
  return {
    type: "ObjectExpression",
    children: [open, first, ...rest, trailing, close]
  };
});
function InlineObjectLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineObjectLiteral", InlineObjectLiteral$0);
}
var InlineObjectPropertyDelimiter$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), Comma, (0, import_lib3.$C)(NotDedented, (0, import_lib3.$E)(_)));
function InlineObjectPropertyDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineObjectPropertyDelimiter", InlineObjectPropertyDelimiter$0);
}
var ObjectPropertyDelimiter$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), Comma);
var ObjectPropertyDelimiter$1 = (0, import_lib3.$Y)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L36, 'ObjectPropertyDelimiter "}"')));
var ObjectPropertyDelimiter$2 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), InsertComma), function(value) {
  return value[1];
});
var ObjectPropertyDelimiter$$ = [ObjectPropertyDelimiter$0, ObjectPropertyDelimiter$1, ObjectPropertyDelimiter$2];
function ObjectPropertyDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ObjectPropertyDelimiter", ObjectPropertyDelimiter$$);
}
var PropertyDefinition$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), NamedProperty), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var prop = $2;
  return prepend(ws, prop);
});
var PropertyDefinition$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$TEXT)((0, import_lib3.$EXPECT)($R19, "PropertyDefinition /[!+-]?/")), PropertyName, (0, import_lib3.$Y)(ObjectPropertyDelimiter)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var ws = $1;
  var toggle = $2;
  var id = $3;
  if (toggle) {
    const value = toggle === "+" ? "true" : "false";
    return {
      type: "Property",
      children: [ws, id, ": ", value],
      name: id,
      names: id.names,
      value
    };
  }
  return {
    type: "Property",
    children: [ws, id],
    name: id,
    names: id.names,
    value: id
  };
});
var PropertyDefinition$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), MethodDefinition), function($skip, $loc, $0, $1, $2) {
  var ws = $1;
  var def = $2;
  if (def.type === "MultiMethodDefinition") {
    return {
      children: def.children.flatMap((c, i) => i ? [",", c] : [c])
    };
  }
  if (!def.block || def.block.empty)
    return $skip;
  return prepend(ws, def);
});
var PropertyDefinition$3 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), DotDotDot, ExtendedExpression), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var dots = $2;
  var exp = $3;
  return {
    type: "SpreadProperty",
    children: [ws, dots, exp],
    names: exp.names,
    dots,
    value: exp
  };
});
var PropertyDefinition$4 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$N)((0, import_lib3.$C)(EOS, (0, import_lib3.$EXPECT)($L7, 'PropertyDefinition "."'))), (0, import_lib3.$Q)(UnaryOp), CallExpression, (0, import_lib3.$E)(UnaryPostfix)), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var ws = $1;
  var pre = $3;
  var value = $4;
  var post = $5;
  if (!pre.length && !post) {
    switch (value.type) {
      case "Identifier":
        return prepend(ws, value);
      case "ObjectExpression":
        let first = value.properties[0];
        if (first) {
          first = {
            ...first,
            children: [ws, ...first.children],
            hoistDec: value.hoistDec
          };
        }
        return [first, ...value.properties.slice(1)];
    }
  }
  const last = lastAccessInCallExpression(value);
  if (!last)
    return $skip;
  let name, hoistDec, ref, refAssignment;
  const { expression, type } = last;
  if (type === "Index") {
    ({ ref, hoistDec, refAssignment } = maybeRefAssignment(expression));
    if (refAssignment) {
      name = {
        type: "ComputedPropertyName",
        children: [last.children[0], "(", refAssignment, ",", ref, ")", ...last.children.slice(-2)]
      };
      value = {
        ...value,
        children: value.children.map((c) => {
          if (c === last)
            return {
              type: "Index",
              children: ["[", ref, "]"]
            };
          return c;
        })
      };
    } else {
      name = {
        type: "ComputedPropertyName",
        children: last.children
      };
    }
  } else {
    ({ name } = last);
    if (!name)
      return $skip;
  }
  if (name[0] === "\#")
    name = name.slice(1);
  return {
    type: "Property",
    children: [ws, name, ": ", processUnaryExpression(pre, value, post)],
    name,
    names: [],
    value,
    hoistDec
  };
});
var PropertyDefinition$$ = [PropertyDefinition$0, PropertyDefinition$1, PropertyDefinition$2, PropertyDefinition$3, PropertyDefinition$4];
function PropertyDefinition(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PropertyDefinition", PropertyDefinition$$);
}
var NamedProperty$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PropertyName, (0, import_lib3.$E)(_), Colon, PostfixedExpression), function($skip, $loc, $0, $1, $2, $3, $4) {
  var name = $1;
  var exp = $4;
  return {
    type: "Property",
    children: $0,
    name,
    names: exp.names || [],
    value: exp
  };
});
function NamedProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NamedProperty", NamedProperty$0);
}
var SnugNamedProperty$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PropertyName, Colon, MaybeNestedExtendedExpression, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), PostfixStatement), (0, import_lib3.$Y)((0, import_lib3.$S)(Nested, NamedProperty))))), function($skip, $loc, $0, $1, $2, $3, $4) {
  var name = $1;
  var colon = $2;
  var expression = $3;
  var post = $4;
  if (post) {
    expression = attachPostfixStatementAsExpression(expression, post[0]);
  }
  return {
    type: "Property",
    children: [name, colon, expression],
    names: expression.names || []
  };
});
function SnugNamedProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SnugNamedProperty", SnugNamedProperty$0);
}
var PropertyName$0 = NumericLiteral;
var PropertyName$1 = ComputedPropertyName;
var PropertyName$2 = StringLiteral;
var PropertyName$3 = IdentifierName;
var PropertyName$$ = [PropertyName$0, PropertyName$1, PropertyName$2, PropertyName$3];
function PropertyName(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PropertyName", PropertyName$$);
}
var ComputedPropertyName$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenBracket, PostfixedExpression, __, CloseBracket), function($skip, $loc, $0, $1, $2, $3, $4) {
  var expression = $2;
  return {
    type: "ComputedPropertyName",
    children: $0,
    expression
  };
});
var ComputedPropertyName$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBracket, TemplateLiteral, InsertCloseBracket), function($skip, $loc, $0, $1, $2, $3) {
  var expression = $2;
  if ($2.type === "StringLiteral")
    return $2;
  return {
    type: "ComputedPropertyName",
    children: $0,
    expression,
    implicit: true
  };
});
var ComputedPropertyName$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBracket, (0, import_lib3.$EXPECT)($R12, "ComputedPropertyName /[+-]/"), NumericLiteral, InsertCloseBracket), function($skip, $loc, $0, $1, $2, $3, $4) {
  const expression = [$2, $3];
  return {
    type: "ComputedPropertyName",
    expression,
    children: [$1, expression, $4],
    implicit: true
  };
});
var ComputedPropertyName$$ = [ComputedPropertyName$0, ComputedPropertyName$1, ComputedPropertyName$2];
function ComputedPropertyName(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ComputedPropertyName", ComputedPropertyName$$);
}
var Decorator$0 = (0, import_lib3.$S)(AtAt, CallExpression);
function Decorator(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Decorator", Decorator$0);
}
var Decorators$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidClassImplicitCall, (0, import_lib3.$Q)((0, import_lib3.$S)(__, Decorator)), __, RestoreClassImplicitCall), function($skip, $loc, $0, $1, $2, $3, $4) {
  var decorators = $2;
  if (!decorators.length)
    return $skip;
  return $0;
});
function Decorators(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Decorators", Decorators$0);
}
var MethodDefinition$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Abstract, __, MethodSignature), function($skip, $loc, $0, $1, $2, $3) {
  var signature = $3;
  return {
    type: "MethodDefinition",
    children: $0,
    name: signature.name,
    abstract: true,
    signature,
    parameters: signature.parameters,
    ts: true
  };
});
var MethodDefinition$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(MethodSignature, (0, import_lib3.$N)((0, import_lib3.$C)(PropertyAccess, UnaryPostfix, NonNullAssertion)), (0, import_lib3.$E)(BracedBlock)), function($skip, $loc, $0, $1, $2, $3) {
  var signature = $1;
  var block = $3;
  let children = $0;
  let generatorPos = 0;
  let { modifier } = signature;
  if (hasAwait(block)) {
    generatorPos++;
    children = children.slice();
    if (modifier?.get || modifier?.set) {
      children.push({
        type: "Error",
        message: "Getters and setters cannot be async"
      });
    } else if (modifier?.async) {
    } else {
      children.unshift("async ");
      modifier = { ...modifier, async: true };
      signature = { ...signature, modifier };
    }
  }
  if (hasYield(block)) {
    if (children === $0)
      children = children.slice();
    if (modifier?.get || modifier?.set) {
      children.push({
        type: "Error",
        message: "Getters and setters cannot be generators"
      });
    } else if (modifier?.generator) {
    } else {
      children.splice(generatorPos, 0, "*");
      modifier = { ...modifier, generator: true };
      signature = { ...signature, modifier };
    }
  }
  return {
    type: "MethodDefinition",
    children,
    name: signature.name,
    signature,
    block,
    parameters: signature.parameters
  };
});
var MethodDefinition$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(GetOrSet, (0, import_lib3.$E)(_), ForbidIndentedApplication, (0, import_lib3.$E)((0, import_lib3.$S)(MemberBase, (0, import_lib3.$Q)(CallExpressionRest), (0, import_lib3.$E)(ReturnTypeSuffix))), RestoreIndentedApplication, (0, import_lib3.$E)(BracedBlock)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var kind = $1;
  var ws = $2;
  var content = $4;
  var block = $6;
  if (!content)
    return $skip;
  const [base, rest, returnType] = content;
  const value = [base, rest];
  if (!rest.length) {
    let name2;
    if (base.type === "MemberExpression") {
      const lastAccess2 = lastAccessInCallExpression(base);
      if (lastAccess2) {
        ({ name: name2 } = lastAccess2);
      }
    }
    if (!name2)
      ({ name: name2 } = base);
    if (!name2)
      return $skip;
    if (name2[0] === "\#")
      name2 = name2.slice(1);
    const autoReturn = !block || base.type !== "Identifier";
    return makeGetterMethod(name2, ws, base, returnType, block, kind, autoReturn);
  }
  let last = rest[rest.length - 1];
  while (Array.isArray(last)) {
    last = last[last.length - 1];
  }
  switch (last.type) {
    case "Call":
      return $skip;
    case "PropertyAccess":
      const { name: name2 } = last;
      return makeGetterMethod(name2, ws, value, returnType, block, kind);
    case "PropertyGlob":
      return {
        type: "MultiMethodDefinition",
        children: last.object.properties.map((p) => {
          const { name: name3, type } = p;
          let v;
          switch (type) {
            case "Identifier":
              v = insertTrimmingSpace(p, "");
              break;
            case "Property":
              const { value: value2 } = p;
              if (value2.privateShorthand) {
                v = value2.privateId;
              } else {
                v = insertTrimmingSpace(value2, "");
              }
              break;
          }
          const exp = processCallMemberExpression({
            type: "CallExpression",
            children: [base, ...rest.slice(0, -1), {
              type: "PropertyAccess",
              children: [last.dot, {
                ...v,
                children: [v.children.slice(0, 2)]
                // Remove potential delimiter
              }]
            }]
          });
          return makeGetterMethod(name3, ws, exp, returnType, block, kind);
        })
      };
  }
  const lastAccess = lastAccessInCallExpression({ children: rest });
  const { name } = lastAccess;
  return makeGetterMethod(name, ws, value, returnType, block, kind);
});
var MethodDefinition$$ = [MethodDefinition$0, MethodDefinition$1, MethodDefinition$2];
function MethodDefinition(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MethodDefinition", MethodDefinition$$);
}
var MethodModifier$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(GetOrSet, (0, import_lib3.$E)(_), (0, import_lib3.$Y)(ClassElementName)), function($skip, $loc, $0, $1, $2, $3) {
  var kind = $1;
  return {
    type: "MethodModifier",
    async: false,
    generator: false,
    get: kind.token === "get",
    set: kind.token === "set",
    children: $0
  };
});
var MethodModifier$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$S)(Async, __), (0, import_lib3.$E)((0, import_lib3.$S)(Star, __))), function($skip, $loc, $0, $1, $2) {
  return {
    type: "MethodModifier",
    async: true,
    get: false,
    set: false,
    generator: !!$2,
    children: $0
  };
});
var MethodModifier$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(Star, __), function($skip, $loc, $0, $1, $2) {
  return {
    type: "MethodModifier",
    async: false,
    get: false,
    set: false,
    generator: true,
    children: $0
  };
});
var MethodModifier$$ = [MethodModifier$0, MethodModifier$1, MethodModifier$2];
function MethodModifier(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MethodModifier", MethodModifier$$);
}
var MethodSignature$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ConstructorShorthand, NonEmptyParameters), function($skip, $loc, $0, $1, $2) {
  var parameters = $2;
  return {
    type: "MethodSignature",
    children: $0,
    name: $1.token,
    modifier: {},
    returnType: void 0,
    parameters
  };
});
var MethodSignature$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(MethodModifier), ClassElementName, (0, import_lib3.$E)(_), (0, import_lib3.$E)(QuestionMark), (0, import_lib3.$E)(_), NonEmptyParameters, (0, import_lib3.$E)(ReturnTypeSuffix)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var modifier = $1;
  var name = $2;
  var optional = $4;
  var parameters = $6;
  var returnType = $7;
  if (name.name) {
    name = name.name;
  } else if (name.token) {
    name = name.token.match(/^(?:"|')/) ? name.token.slice(1, -1) : name.token;
  }
  if (optional)
    $0[3] = optional = { ...optional, ts: true };
  modifier = modifier || {};
  return {
    type: "MethodSignature",
    children: $0,
    name,
    optional,
    modifier,
    // get/set/async/generator
    returnType,
    parameters
  };
});
var MethodSignature$$ = [MethodSignature$0, MethodSignature$1];
function MethodSignature(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MethodSignature", MethodSignature$$);
}
var ClassElementName$0 = PropertyName;
var ClassElementName$1 = LengthShorthand;
var ClassElementName$2 = PrivateIdentifier;
var ClassElementName$$ = [ClassElementName$0, ClassElementName$1, ClassElementName$2];
function ClassElementName(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ClassElementName", ClassElementName$$);
}
var PrivateIdentifier$0 = (0, import_lib3.$TV)((0, import_lib3.$TEXT)((0, import_lib3.$S)(Hash, IdentifierName)), function($skip, $loc, $0, $1) {
  var id = $0;
  return {
    type: "Identifier",
    name: id,
    names: [id],
    children: [{
      $loc,
      token: id
    }]
  };
});
function PrivateIdentifier(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PrivateIdentifier", PrivateIdentifier$0);
}
var WAssignmentOp$0 = (0, import_lib3.$S)(__, AssignmentOp);
var WAssignmentOp$1 = (0, import_lib3.$S)((0, import_lib3.$E)(_), OperatorAssignmentOp);
var WAssignmentOp$$ = [WAssignmentOp$0, WAssignmentOp$1];
function WAssignmentOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "WAssignmentOp", WAssignmentOp$$);
}
var AssignmentOp$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(AssignmentOpSymbol, (0, import_lib3.$E)(_)), function($skip, $loc, $0, $1, $2) {
  if ($2?.length) {
    if (typeof $1 !== "string") {
      return { ...$1, children: [...$1.children, $2] };
    }
    return {
      token: $1,
      children: [$1, ...$2]
    };
  }
  if (typeof $1 !== "string")
    return $1;
  return { $loc, token: $1 };
});
function AssignmentOp(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AssignmentOp", AssignmentOp$0);
}
var OperatorAssignmentOp$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Xor, (0, import_lib3.$EXPECT)($L3, 'OperatorAssignmentOp "="'), (0, import_lib3.$E)(_)), function($skip, $loc, $0, $1, $2, $3) {
  return {
    special: true,
    call: getHelperRef("xor"),
    children: [$2, ...$3 || []]
  };
});
var OperatorAssignmentOp$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Xnor, (0, import_lib3.$EXPECT)($L3, 'OperatorAssignmentOp "="'), (0, import_lib3.$E)(_)), function($skip, $loc, $0, $1, $2, $3) {
  return {
    special: true,
    call: getHelperRef("xnor"),
    children: [$2, ...$3 || []]
  };
});
var OperatorAssignmentOp$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(Identifier, (0, import_lib3.$EXPECT)($L3, 'OperatorAssignmentOp "="'), (0, import_lib3.$E)(_)), function($skip, $loc, $0, $1, $2, $3) {
  return {
    special: true,
    call: $1,
    children: [$2, ...$3 || []]
  };
});
var OperatorAssignmentOp$$ = [OperatorAssignmentOp$0, OperatorAssignmentOp$1, OperatorAssignmentOp$2];
function OperatorAssignmentOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "OperatorAssignmentOp", OperatorAssignmentOp$$);
}
var AssignmentOpSymbol$0 = (0, import_lib3.$EXPECT)($L46, 'AssignmentOpSymbol "**="');
var AssignmentOpSymbol$1 = (0, import_lib3.$EXPECT)($L47, 'AssignmentOpSymbol "*="');
var AssignmentOpSymbol$2 = (0, import_lib3.$EXPECT)($L48, 'AssignmentOpSymbol "/="');
var AssignmentOpSymbol$3 = (0, import_lib3.$EXPECT)($L49, 'AssignmentOpSymbol "%="');
var AssignmentOpSymbol$4 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L9, 'AssignmentOpSymbol "++"'), (0, import_lib3.$EXPECT)($L11, 'AssignmentOpSymbol "\u29FA"')), Equals), function($skip, $loc, $0, $1, $2) {
  return {
    special: true,
    call: getHelperRef("concatAssign"),
    omitLhs: true,
    children: [$2]
  };
});
var AssignmentOpSymbol$5 = (0, import_lib3.$EXPECT)($L50, 'AssignmentOpSymbol "+="');
var AssignmentOpSymbol$6 = (0, import_lib3.$EXPECT)($L51, 'AssignmentOpSymbol "-="');
var AssignmentOpSymbol$7 = (0, import_lib3.$EXPECT)($L52, 'AssignmentOpSymbol "<<="');
var AssignmentOpSymbol$8 = (0, import_lib3.$EXPECT)($L53, 'AssignmentOpSymbol ">>>="');
var AssignmentOpSymbol$9 = (0, import_lib3.$EXPECT)($L54, 'AssignmentOpSymbol ">>="');
var AssignmentOpSymbol$10 = (0, import_lib3.$EXPECT)($L55, 'AssignmentOpSymbol "&&="');
var AssignmentOpSymbol$11 = (0, import_lib3.$EXPECT)($L56, 'AssignmentOpSymbol "&="');
var AssignmentOpSymbol$12 = (0, import_lib3.$EXPECT)($L57, 'AssignmentOpSymbol "^="');
var AssignmentOpSymbol$13 = (0, import_lib3.$EXPECT)($L58, 'AssignmentOpSymbol "||="');
var AssignmentOpSymbol$14 = (0, import_lib3.$EXPECT)($L59, 'AssignmentOpSymbol "|="');
var AssignmentOpSymbol$15 = (0, import_lib3.$EXPECT)($L60, 'AssignmentOpSymbol "??="');
var AssignmentOpSymbol$16 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L61, 'AssignmentOpSymbol "?="'), function(value) {
  return "??=";
});
var AssignmentOpSymbol$17 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L3, 'AssignmentOpSymbol "="'), (0, import_lib3.$N)((0, import_lib3.$EXPECT)($L3, 'AssignmentOpSymbol "="'))), function(value) {
  return value[0];
});
var AssignmentOpSymbol$18 = (0, import_lib3.$T)((0, import_lib3.$S)(CoffeeWordAssignmentOp), function(value) {
  return value[0];
});
var AssignmentOpSymbol$$ = [AssignmentOpSymbol$0, AssignmentOpSymbol$1, AssignmentOpSymbol$2, AssignmentOpSymbol$3, AssignmentOpSymbol$4, AssignmentOpSymbol$5, AssignmentOpSymbol$6, AssignmentOpSymbol$7, AssignmentOpSymbol$8, AssignmentOpSymbol$9, AssignmentOpSymbol$10, AssignmentOpSymbol$11, AssignmentOpSymbol$12, AssignmentOpSymbol$13, AssignmentOpSymbol$14, AssignmentOpSymbol$15, AssignmentOpSymbol$16, AssignmentOpSymbol$17, AssignmentOpSymbol$18];
function AssignmentOpSymbol(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "AssignmentOpSymbol", AssignmentOpSymbol$$);
}
var CoffeeWordAssignmentOp$0 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L62, 'CoffeeWordAssignmentOp "and="'), function(value) {
  return "&&=";
});
var CoffeeWordAssignmentOp$1 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L63, 'CoffeeWordAssignmentOp "or="'), function(value) {
  return "||=";
});
var CoffeeWordAssignmentOp$$ = [CoffeeWordAssignmentOp$0, CoffeeWordAssignmentOp$1];
function CoffeeWordAssignmentOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CoffeeWordAssignmentOp", CoffeeWordAssignmentOp$$);
}
var NotDedentedBinaryOp$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(IndentedFurther), (0, import_lib3.$E)(_), BinaryOp), function($skip, $loc, $0, $1, $2, $3) {
  const ws = [];
  if ($1)
    ws.push(...$1);
  if ($2)
    ws.push(...$2);
  return [ws, $3];
});
var NotDedentedBinaryOp$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, (0, import_lib3.$E)(_), (0, import_lib3.$N)(Identifier), (0, import_lib3.$C)((0, import_lib3.$N)((0, import_lib3.$EXPECT)($L64, 'NotDedentedBinaryOp "*"')), (0, import_lib3.$N)(ImportDeclaration)), BinaryOp), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var op = $5;
  const ws = [...$1];
  if ($2)
    ws.push(...$2);
  return [ws, op];
});
var NotDedentedBinaryOp$$ = [NotDedentedBinaryOp$0, NotDedentedBinaryOp$1];
function NotDedentedBinaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NotDedentedBinaryOp", NotDedentedBinaryOp$$);
}
var IdentifierBinaryOp$0 = (0, import_lib3.$TV)(Identifier, function($skip, $loc, $0, $1) {
  var id = $0;
  if (state.operators.has(id.name))
    return id;
  return $skip;
});
function IdentifierBinaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IdentifierBinaryOp", IdentifierBinaryOp$0);
}
var BinaryOp$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R20, "BinaryOp /(?=\\p{ID_Start}|[_$^\xAB\xBB\u22D9\u2264\u2265\u2208\u220B\u2209\u220C\u2263\u2261\u2262\u2260=\u2A76\u2A75\u2016\u2047&|*\\/!?%<>\u29FA+-])/"), _BinaryOp), function(value) {
  var op = value[1];
  return op;
});
function BinaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BinaryOp", BinaryOp$0);
}
var _BinaryOp$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(BinaryOpSymbol), function($skip, $loc, $0, $1) {
  if (typeof $1 === "string")
    return { $loc, token: $1 };
  return $1;
});
var _BinaryOp$1 = (0, import_lib3.$TV)(Identifier, function($skip, $loc, $0, $1) {
  var id = $0;
  if (!state.operators.has(id.name))
    return $skip;
  return {
    token: id.name,
    call: id,
    special: true,
    ...state.operators.get(id.name)
  };
});
var _BinaryOp$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(OmittedNegation, __, Identifier), function($skip, $loc, $0, $1, $2, $3) {
  var id = $3;
  if (!state.operators.has(id.name))
    return $skip;
  return {
    token: id.name,
    call: id,
    special: true,
    negated: true,
    ...state.operators.get(id.name)
  };
});
var _BinaryOp$$ = [_BinaryOp$0, _BinaryOp$1, _BinaryOp$2];
function _BinaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "_BinaryOp", _BinaryOp$$);
}
var BinaryOpSymbol$0 = (0, import_lib3.$EXPECT)($L65, 'BinaryOpSymbol "**"');
var BinaryOpSymbol$1 = (0, import_lib3.$EXPECT)($L64, 'BinaryOpSymbol "*"');
var BinaryOpSymbol$2 = (0, import_lib3.$EXPECT)($L66, 'BinaryOpSymbol "/"');
var BinaryOpSymbol$3 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L67, 'BinaryOpSymbol "%%"'), function($skip, $loc, $0, $1) {
  return {
    call: getHelperRef("modulo"),
    special: true
  };
});
var BinaryOpSymbol$4 = (0, import_lib3.$EXPECT)($L68, 'BinaryOpSymbol "%"');
var BinaryOpSymbol$5 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L9, 'BinaryOpSymbol "++"'), (0, import_lib3.$EXPECT)($L11, 'BinaryOpSymbol "\u29FA"')), function($skip, $loc, $0, $1) {
  return {
    method: "concat",
    special: true
  };
});
var BinaryOpSymbol$6 = (0, import_lib3.$EXPECT)($L69, 'BinaryOpSymbol "+"');
var BinaryOpSymbol$7 = (0, import_lib3.$EXPECT)($L23, 'BinaryOpSymbol "-"');
var BinaryOpSymbol$8 = (0, import_lib3.$EXPECT)($L70, 'BinaryOpSymbol "<="');
var BinaryOpSymbol$9 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L71, 'BinaryOpSymbol "\u2264"'), function(value) {
  return "<=";
});
var BinaryOpSymbol$10 = (0, import_lib3.$EXPECT)($L72, 'BinaryOpSymbol ">="');
var BinaryOpSymbol$11 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L73, 'BinaryOpSymbol "\u2265"'), function(value) {
  return ">=";
});
var BinaryOpSymbol$12 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L74, 'BinaryOpSymbol "<?"'), function($skip, $loc, $0, $1) {
  return {
    $loc,
    token: "instanceof",
    relational: true,
    special: true
  };
});
var BinaryOpSymbol$13 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L75, 'BinaryOpSymbol "!<?"'), function($skip, $loc, $0, $1) {
  return {
    $loc,
    token: "instanceof",
    relational: true,
    special: true,
    negated: true
  };
});
var BinaryOpSymbol$14 = (0, import_lib3.$EXPECT)($L76, 'BinaryOpSymbol "<<"');
var BinaryOpSymbol$15 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L77, 'BinaryOpSymbol "\xAB"'), function(value) {
  return "<<";
});
var BinaryOpSymbol$16 = (0, import_lib3.$EXPECT)($L19, 'BinaryOpSymbol "<"');
var BinaryOpSymbol$17 = (0, import_lib3.$EXPECT)($L78, 'BinaryOpSymbol ">>>"');
var BinaryOpSymbol$18 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L79, 'BinaryOpSymbol "\u22D9"'), function(value) {
  return ">>>";
});
var BinaryOpSymbol$19 = (0, import_lib3.$EXPECT)($L80, 'BinaryOpSymbol ">>"');
var BinaryOpSymbol$20 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L81, 'BinaryOpSymbol "\xBB"'), function(value) {
  return ">>";
});
var BinaryOpSymbol$21 = (0, import_lib3.$EXPECT)($L44, 'BinaryOpSymbol ">"');
var BinaryOpSymbol$22 = (0, import_lib3.$EXPECT)($L82, 'BinaryOpSymbol "!=="');
var BinaryOpSymbol$23 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L83, 'BinaryOpSymbol "\u2262"'), function(value) {
  return "!==";
});
var BinaryOpSymbol$24 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L84, 'BinaryOpSymbol "!="'), (0, import_lib3.$EXPECT)($L85, 'BinaryOpSymbol "\u2260"')), function($skip, $loc, $0, $1) {
  if (config.coffeeEq)
    return "!==";
  return "!=";
});
var BinaryOpSymbol$25 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L86, 'BinaryOpSymbol "isnt"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  if (config.coffeeIsnt)
    return "!==";
  return $skip;
});
var BinaryOpSymbol$26 = (0, import_lib3.$EXPECT)($L87, 'BinaryOpSymbol "==="');
var BinaryOpSymbol$27 = (0, import_lib3.$T)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L88, 'BinaryOpSymbol "\u2263"'), (0, import_lib3.$EXPECT)($L89, 'BinaryOpSymbol "\u2A76"')), function(value) {
  return "===";
});
var BinaryOpSymbol$28 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L90, 'BinaryOpSymbol "=="'), (0, import_lib3.$EXPECT)($L91, 'BinaryOpSymbol "\u2261"'), (0, import_lib3.$EXPECT)($L92, 'BinaryOpSymbol "\u2A75"')), function($skip, $loc, $0, $1) {
  if (config.coffeeEq)
    return "===";
  return "==";
});
var BinaryOpSymbol$29 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L93, 'BinaryOpSymbol "and"'), NonIdContinue), function(value) {
  return "&&";
});
var BinaryOpSymbol$30 = (0, import_lib3.$EXPECT)($L94, 'BinaryOpSymbol "&&"');
var BinaryOpSymbol$31 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L95, 'BinaryOpSymbol "or"'), NonIdContinue), function(value) {
  return "||";
});
var BinaryOpSymbol$32 = (0, import_lib3.$EXPECT)($L96, 'BinaryOpSymbol "||"');
var BinaryOpSymbol$33 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L97, 'BinaryOpSymbol "\u2016"'), function(value) {
  return "||";
});
var BinaryOpSymbol$34 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L98, 'BinaryOpSymbol "^^"'), (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L99, 'BinaryOpSymbol "xor"'), NonIdContinue)), function($skip, $loc, $0, $1) {
  return {
    call: getHelperRef("xor"),
    special: true,
    prec: "^^"
  };
});
var BinaryOpSymbol$35 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($R21, "BinaryOpSymbol /!\\^\\^?/"), (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L100, 'BinaryOpSymbol "xnor"'), NonIdContinue)), function($skip, $loc, $0, $1) {
  return {
    call: getHelperRef("xnor"),
    special: true,
    prec: "^^"
  };
});
var BinaryOpSymbol$36 = (0, import_lib3.$EXPECT)($L101, 'BinaryOpSymbol "??"');
var BinaryOpSymbol$37 = (0, import_lib3.$T)((0, import_lib3.$EXPECT)($L102, 'BinaryOpSymbol "\u2047"'), function(value) {
  return "??";
});
var BinaryOpSymbol$38 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L6, 'BinaryOpSymbol "?"'), CoffeeBinaryExistentialEnabled), function(value) {
  return "??";
});
var BinaryOpSymbol$39 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L103, 'BinaryOpSymbol "instanceof"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return {
    $loc,
    token: $1,
    relational: true,
    special: true
    // for typeof shorthand
  };
});
var BinaryOpSymbol$40 = (0, import_lib3.$T)((0, import_lib3.$S)(CoffeeOfEnabled, CoffeeOfOp), function(value) {
  var op = value[1];
  return op;
});
var BinaryOpSymbol$41 = (0, import_lib3.$TS)((0, import_lib3.$S)(OmittedNegation, __, NotOp), function($skip, $loc, $0, $1, $2, $3) {
  var op = $3;
  return { ...op, $loc };
});
var BinaryOpSymbol$42 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$S)(Is, __, In), (0, import_lib3.$EXPECT)($L104, 'BinaryOpSymbol "\u2208"')), function($skip, $loc, $0, $1) {
  return {
    method: "includes",
    relational: true,
    reversed: true,
    special: true
  };
});
var BinaryOpSymbol$43 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L105, 'BinaryOpSymbol "\u220B"'), function($skip, $loc, $0, $1) {
  return {
    method: "includes",
    relational: true,
    special: true
  };
});
var BinaryOpSymbol$44 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L106, 'BinaryOpSymbol "\u220C"'), function($skip, $loc, $0, $1) {
  return {
    method: "includes",
    relational: true,
    special: true,
    negated: true
  };
});
var BinaryOpSymbol$45 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$S)(Is, __, OmittedNegation, __, In), (0, import_lib3.$EXPECT)($L107, 'BinaryOpSymbol "\u2209"')), function($skip, $loc, $0, $1) {
  return {
    method: "includes",
    relational: true,
    reversed: true,
    special: true,
    negated: true
  };
});
var BinaryOpSymbol$46 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(CoffeeNotEnabled), Is, __, Not), function($skip, $loc, $0, $1, $2, $3, $4) {
  if (config.objectIs) {
    return {
      call: getHelperRef("is"),
      relational: true,
      special: true,
      asConst: true,
      negated: true
    };
  }
  return "!==";
});
var BinaryOpSymbol$47 = (0, import_lib3.$TS)((0, import_lib3.$S)(Is), function($skip, $loc, $0, $1) {
  if (config.objectIs) {
    return {
      call: getHelperRef("is"),
      relational: true,
      special: true,
      asConst: true
    };
  }
  return "===";
});
var BinaryOpSymbol$48 = In;
var BinaryOpSymbol$49 = (0, import_lib3.$EXPECT)($L108, 'BinaryOpSymbol "&"');
var BinaryOpSymbol$50 = (0, import_lib3.$EXPECT)($L22, 'BinaryOpSymbol "^"');
var BinaryOpSymbol$51 = (0, import_lib3.$EXPECT)($L109, 'BinaryOpSymbol "|"');
var BinaryOpSymbol$$ = [BinaryOpSymbol$0, BinaryOpSymbol$1, BinaryOpSymbol$2, BinaryOpSymbol$3, BinaryOpSymbol$4, BinaryOpSymbol$5, BinaryOpSymbol$6, BinaryOpSymbol$7, BinaryOpSymbol$8, BinaryOpSymbol$9, BinaryOpSymbol$10, BinaryOpSymbol$11, BinaryOpSymbol$12, BinaryOpSymbol$13, BinaryOpSymbol$14, BinaryOpSymbol$15, BinaryOpSymbol$16, BinaryOpSymbol$17, BinaryOpSymbol$18, BinaryOpSymbol$19, BinaryOpSymbol$20, BinaryOpSymbol$21, BinaryOpSymbol$22, BinaryOpSymbol$23, BinaryOpSymbol$24, BinaryOpSymbol$25, BinaryOpSymbol$26, BinaryOpSymbol$27, BinaryOpSymbol$28, BinaryOpSymbol$29, BinaryOpSymbol$30, BinaryOpSymbol$31, BinaryOpSymbol$32, BinaryOpSymbol$33, BinaryOpSymbol$34, BinaryOpSymbol$35, BinaryOpSymbol$36, BinaryOpSymbol$37, BinaryOpSymbol$38, BinaryOpSymbol$39, BinaryOpSymbol$40, BinaryOpSymbol$41, BinaryOpSymbol$42, BinaryOpSymbol$43, BinaryOpSymbol$44, BinaryOpSymbol$45, BinaryOpSymbol$46, BinaryOpSymbol$47, BinaryOpSymbol$48, BinaryOpSymbol$49, BinaryOpSymbol$50, BinaryOpSymbol$51];
function BinaryOpSymbol(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "BinaryOpSymbol", BinaryOpSymbol$$);
}
var ActualIn$0 = (0, import_lib3.$T)((0, import_lib3.$S)(CoffeeOfEnabled, Of), function(value) {
  return value[1];
});
var ActualIn$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(CoffeeOfEnabled), In), function(value) {
  return value[1];
});
var ActualIn$$ = [ActualIn$0, ActualIn$1];
function ActualIn(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ActualIn", ActualIn$$);
}
var CoffeeOfOp$0 = (0, import_lib3.$T)((0, import_lib3.$S)(Of), function(value) {
  return "in";
});
var CoffeeOfOp$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(In), function($skip, $loc, $0, $1) {
  return {
    call: [getHelperRef("indexOf"), ".call"],
    relational: true,
    reversed: true,
    suffix: " >= 0",
    special: true
  };
});
var CoffeeOfOp$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(OmittedNegation, __, Of, NonIdContinue), function($skip, $loc, $0, $1, $2, $3, $4) {
  return {
    $loc,
    token: "in",
    special: true,
    negated: true
  };
});
var CoffeeOfOp$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(OmittedNegation, __, In), function($skip, $loc, $0, $1, $2, $3) {
  return {
    call: [getHelperRef("indexOf"), ".call"],
    relational: true,
    reversed: true,
    suffix: " < 0",
    special: true
  };
});
var CoffeeOfOp$$ = [CoffeeOfOp$0, CoffeeOfOp$1, CoffeeOfOp$2, CoffeeOfOp$3];
function CoffeeOfOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CoffeeOfOp", CoffeeOfOp$$);
}
var NotOp$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L103, 'NotOp "instanceof"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return {
    $loc,
    token: "instanceof",
    relational: true,
    special: true,
    negated: true
  };
});
var NotOp$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(In), function($skip, $loc, $0, $1) {
  return {
    $loc,
    token: "in",
    special: true,
    negated: true
  };
});
var NotOp$$ = [NotOp$0, NotOp$1];
function NotOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NotOp", NotOp$$);
}
var Xor$0 = (0, import_lib3.$EXPECT)($L98, 'Xor "^^"');
var Xor$1 = (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L99, 'Xor "xor"'), NonIdContinue);
var Xor$$ = [Xor$0, Xor$1];
function Xor(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Xor", Xor$$);
}
var Xnor$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R21, "Xnor /!\\^\\^?/"));
var Xnor$1 = (0, import_lib3.$EXPECT)($L100, 'Xnor "xnor"');
var Xnor$$ = [Xnor$0, Xnor$1];
function Xnor(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Xnor", Xnor$$);
}
var UnaryOp$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R22, "UnaryOp /(?!\\+\\+|--)[!~+-](?!\\s)/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
var UnaryOp$1 = AwaitOp;
var UnaryOp$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)(Delete, Void, Typeof), (0, import_lib3.$N)((0, import_lib3.$EXPECT)($R23, "UnaryOp /[:.]/")), (0, import_lib3.$E)(_)), function($skip, $loc, $0, $1, $2, $3) {
  var op = $1;
  var ws = $3;
  if (!ws)
    return [op, [" "]];
  return [op, ws];
});
var UnaryOp$3 = (0, import_lib3.$T)((0, import_lib3.$S)(Not, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($R23, "UnaryOp /[:.]/")), (0, import_lib3.$E)((0, import_lib3.$EXPECT)($L18, 'UnaryOp " "')), (0, import_lib3.$E)(_)), function(value) {
  return [value[0], value[3]];
});
var UnaryOp$$ = [UnaryOp$0, UnaryOp$1, UnaryOp$2, UnaryOp$3];
function UnaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "UnaryOp", UnaryOp$$);
}
var AwaitOp$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Await, (0, import_lib3.$E)((0, import_lib3.$S)(Dot, IdentifierName)), (0, import_lib3.$E)(_)), function($skip, $loc, $0, $1, $2, $3) {
  var a = $1;
  var op = $2;
  var ws = $3;
  return {
    ...a,
    op,
    children: [a, ...ws || [" "]]
  };
});
function AwaitOp(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AwaitOp", AwaitOp$0);
}
var ModuleItem$0 = ImportDeclaration;
var ModuleItem$1 = ExportDeclaration;
var ModuleItem$2 = StatementListItem;
var ModuleItem$$ = [ModuleItem$0, ModuleItem$1, ModuleItem$2];
function ModuleItem(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ModuleItem", ModuleItem$$);
}
var StatementListItem$0 = Declaration;
var StatementListItem$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)((0, import_lib3.$EXPECT)($L110, 'StatementListItem "$:"')), ImplicitObjectLiteral), function($skip, $loc, $0, $1, $2) {
  return makeLeftHandSideExpression($2);
});
var StatementListItem$2 = PostfixedStatement;
var StatementListItem$$ = [StatementListItem$0, StatementListItem$1, StatementListItem$2];
function StatementListItem(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "StatementListItem", StatementListItem$$);
}
var PostfixedStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Statement, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), PostfixStatement))), function($skip, $loc, $0, $1, $2) {
  var statement = $1;
  var post = $2;
  if (post)
    return addPostfixStatement(statement, ...post);
  return statement;
});
function PostfixedStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PostfixedStatement", PostfixedStatement$0);
}
var NoCommaStatementListItem$0 = Declaration;
var NoCommaStatementListItem$1 = PostfixedNoCommaStatement;
var NoCommaStatementListItem$$ = [NoCommaStatementListItem$0, NoCommaStatementListItem$1];
function NoCommaStatementListItem(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NoCommaStatementListItem", NoCommaStatementListItem$$);
}
var PostfixedNoCommaStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(NoCommaStatement, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), PostfixStatement))), function($skip, $loc, $0, $1, $2) {
  var statement = $1;
  var post = $2;
  if (post)
    return addPostfixStatement(statement, ...post);
  return statement;
});
function PostfixedNoCommaStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PostfixedNoCommaStatement", PostfixedNoCommaStatement$0);
}
var PostfixedExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ExtendedExpression, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), PostfixStatement))), function($skip, $loc, $0, $1, $2) {
  var expression = $1;
  var post = $2;
  if (post)
    return attachPostfixStatementAsExpression(expression, post);
  return expression;
});
function PostfixedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PostfixedExpression", PostfixedExpression$0);
}
var PostfixedCommaExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PostfixedExpression, (0, import_lib3.$C)((0, import_lib3.$S)((0, import_lib3.$E)(_), PostfixStatement), (0, import_lib3.$Q)((0, import_lib3.$S)(CommaDelimiter, AssignmentExpression)))), function($skip, $loc, $0, $1, $2) {
  var expression = $1;
  var post = $2;
  if (!post.length)
    return $1;
  if (post.length === 2 && !Array.isArray(post[1])) {
    return attachPostfixStatementAsExpression(expression, post);
  }
  return $0;
});
function PostfixedCommaExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PostfixedCommaExpression", PostfixedCommaExpression$0);
}
var NonPipelinePostfixedExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(NonPipelineExtendedExpression, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), PostfixStatement))), function($skip, $loc, $0, $1, $2) {
  var expression = $1;
  var post = $2;
  if (post)
    return attachPostfixStatementAsExpression(expression, post);
  return expression;
});
function NonPipelinePostfixedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NonPipelinePostfixedExpression", NonPipelinePostfixedExpression$0);
}
var PostfixStatement$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R24, "PostfixStatement /(?=for|if|loop|unless|until|while)/"), _PostfixStatement), function(value) {
  return value[1];
});
function PostfixStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PostfixStatement", PostfixStatement$0);
}
var _PostfixStatement$0 = ForClause;
var _PostfixStatement$1 = IfClause;
var _PostfixStatement$2 = LoopClause;
var _PostfixStatement$3 = WhileClause;
var _PostfixStatement$$ = [_PostfixStatement$0, _PostfixStatement$1, _PostfixStatement$2, _PostfixStatement$3];
function _PostfixStatement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "_PostfixStatement", _PostfixStatement$$);
}
var Statement$0 = KeywordStatement;
var Statement$1 = VariableStatement;
var Statement$2 = IfStatement;
var Statement$3 = IterationStatement;
var Statement$4 = SwitchStatement;
var Statement$5 = TryStatement;
var Statement$6 = EmptyStatement;
var Statement$7 = LabelledStatement;
var Statement$8 = CommaExpressionStatement;
var Statement$9 = BlockStatement;
var Statement$$ = [Statement$0, Statement$1, Statement$2, Statement$3, Statement$4, Statement$5, Statement$6, Statement$7, Statement$8, Statement$9];
function Statement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Statement", Statement$$);
}
var NoCommaStatement$0 = KeywordStatement;
var NoCommaStatement$1 = VariableStatement;
var NoCommaStatement$2 = IfStatement;
var NoCommaStatement$3 = IterationStatement;
var NoCommaStatement$4 = SwitchStatement;
var NoCommaStatement$5 = TryStatement;
var NoCommaStatement$6 = EmptyStatement;
var NoCommaStatement$7 = LabelledStatement;
var NoCommaStatement$8 = ExpressionStatement;
var NoCommaStatement$9 = BlockStatement;
var NoCommaStatement$$ = [NoCommaStatement$0, NoCommaStatement$1, NoCommaStatement$2, NoCommaStatement$3, NoCommaStatement$4, NoCommaStatement$5, NoCommaStatement$6, NoCommaStatement$7, NoCommaStatement$8, NoCommaStatement$9];
function NoCommaStatement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NoCommaStatement", NoCommaStatement$$);
}
var EmptyStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$Y)((0, import_lib3.$EXPECT)($L111, 'EmptyStatement ";"'))), function($skip, $loc, $0, $1, $2) {
  return { type: "EmptyStatement", children: $1 || [] };
});
function EmptyStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EmptyStatement", EmptyStatement$0);
}
var InsertEmptyStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertSemicolon), function($skip, $loc, $0, $1) {
  return { type: "EmptyStatement", children: [$1] };
});
function InsertEmptyStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertEmptyStatement", InsertEmptyStatement$0);
}
var BlockStatement$0 = (0, import_lib3.$T)((0, import_lib3.$S)(ExplicitBlock, (0, import_lib3.$N)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L3, 'BlockStatement "="')))), function(value) {
  return value[0];
});
function BlockStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BlockStatement", BlockStatement$0);
}
var LabelledStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Label, LabelledItem), function($skip, $loc, $0, $1, $2) {
  var label = $1;
  var statement = $2;
  return {
    type: "LabelledStatement",
    label,
    statement,
    children: $0
  };
});
function LabelledStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "LabelledStatement", LabelledStatement$0);
}
var Label$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Colon, Identifier, Whitespace), function($skip, $loc, $0, $1, $2, $3) {
  var colon = $1;
  var id = $2;
  var w = $3;
  return [id, colon, w];
});
var Label$1 = (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L110, 'Label "$:"'), Whitespace);
var Label$$ = [Label$0, Label$1];
function Label(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Label", Label$$);
}
var LabelledItem$0 = Statement;
var LabelledItem$1 = FunctionDeclaration;
var LabelledItem$$ = [LabelledItem$0, LabelledItem$1];
function LabelledItem(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "LabelledItem", LabelledItem$$);
}
var IfStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(IfClause, BlockOrEmpty, (0, import_lib3.$E)(ElseClause)), function($skip, $loc, $0, $1, $2, $3) {
  var clause = $1;
  var block = $2;
  var e = $3;
  return {
    type: "IfStatement",
    children: [...clause.children, block, e],
    condition: clause.condition,
    negated: clause.negated,
    then: block,
    else: e
  };
});
function IfStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IfStatement", IfStatement$0);
}
var ElseClause$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$C)(Nested, (0, import_lib3.$E)(_)), Else, BlockOrEmpty), function(value) {
  var block = value[2];
  return { "type": "ElseClause", "children": value, "block": block };
});
function ElseClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ElseClause", ElseClause$0);
}
var IfClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)(If, Unless), (0, import_lib3.$E)(_), Condition), function($skip, $loc, $0, $1, $2, $3) {
  var kind = $1;
  var ws = $2;
  var condition = $3;
  if (kind.negated) {
    kind = { ...kind, token: "if" };
    condition = negateCondition(condition);
  }
  return {
    type: "IfStatement",
    children: [kind, ws, condition],
    condition,
    negated: kind.negated
  };
});
function IfClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IfClause", IfClause$0);
}
var IterationStatement$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R25, "IterationStatement /(?=loop|comptime|do|for|until|while)/"), _IterationStatement), function(value) {
  return value[1];
});
function IterationStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IterationStatement", IterationStatement$0);
}
var _IterationStatement$0 = LoopStatement;
var _IterationStatement$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(CoffeeDoEnabled), DoWhileStatement), function(value) {
  return value[1];
});
var _IterationStatement$2 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(CoffeeDoEnabled), DoStatement), function(value) {
  return value[1];
});
var _IterationStatement$3 = ComptimeStatement;
var _IterationStatement$4 = WhileStatement;
var _IterationStatement$5 = ForStatement;
var _IterationStatement$$ = [_IterationStatement$0, _IterationStatement$1, _IterationStatement$2, _IterationStatement$3, _IterationStatement$4, _IterationStatement$5];
function _IterationStatement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "_IterationStatement", _IterationStatement$$);
}
var IterationExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Async, __)), IterationStatement), function($skip, $loc, $0, $1, $2) {
  var async = $1;
  var statement = $2;
  return {
    type: "IterationExpression",
    subtype: statement.type,
    children: [statement],
    block: statement.block,
    statement,
    async
  };
});
function IterationExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IterationExpression", IterationExpression$0);
}
var LoopStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(LoopClause, BlockOrEmptyStatement), function($skip, $loc, $0, $1, $2) {
  var clause = $1;
  var block = $2;
  return {
    ...clause,
    type: "IterationStatement",
    children: [...clause.children, block],
    block
  };
});
function LoopStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "LoopStatement", LoopStatement$0);
}
var LoopClause$0 = (0, import_lib3.$TV)(Loop, function($skip, $loc, $0, $1) {
  var kind = $0;
  const expression = {
    type: "Literal",
    children: ["true"],
    raw: "true"
  };
  const condition = {
    type: "ParenthesizedExpression",
    children: ["(", expression, ")"],
    expression
  };
  return {
    type: "IterationStatement",
    subtype: kind.token,
    children: [kind, condition],
    condition
  };
});
function LoopClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "LoopClause", LoopClause$0);
}
var DoWhileStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Do, NoPostfixBracedOrEmptyBlock, __, WhileClause), function($skip, $loc, $0, $1, $2, $3, $4) {
  var block = $2;
  var clause = $4;
  return {
    ...clause,
    type: "IterationStatement",
    subtype: "do-while",
    children: $0,
    block
  };
});
function DoWhileStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DoWhileStatement", DoWhileStatement$0);
}
var DoStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Do, NoPostfixBracedOrEmptyBlock), function($skip, $loc, $0, $1, $2) {
  var block = $2;
  block = insertTrimmingSpace(block, "");
  return {
    type: "DoStatement",
    children: [block],
    block
  };
});
function DoStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DoStatement", DoStatement$0);
}
var ComptimeStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Comptime, NoPostfixBracedOrEmptyBlock, (0, import_lib3.$E)(ElseClause)), function($skip, $loc, $0, $1, $2, $3) {
  var t = $2;
  var e = $3;
  let block = !initialConfig.comptime && e?.block || t;
  block = insertTrimmingSpace(block, "");
  return {
    type: "ComptimeStatement",
    children: [block],
    block,
    // In case we want access to the original blocks:
    then: t,
    else: e
  };
});
function ComptimeStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ComptimeStatement", ComptimeStatement$0);
}
var WhileStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(WhileClause, BlockOrEmptyStatement), function($skip, $loc, $0, $1, $2) {
  var clause = $1;
  var block = $2;
  return {
    ...clause,
    children: [...clause.children, block],
    block
  };
});
function WhileStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "WhileStatement", WhileStatement$0);
}
var WhileClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)(While, Until), (0, import_lib3.$E)(_), Condition), function($skip, $loc, $0, $1, $2, $3) {
  var kind = $1;
  var ws = $2;
  var condition = $3;
  if (kind.negated) {
    kind = { ...kind, token: "while" };
    condition = negateCondition(condition);
  }
  return {
    type: "IterationStatement",
    subtype: kind.token,
    children: [kind, ws, condition],
    condition,
    negated: kind.negated
  };
});
function WhileClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "WhileClause", WhileClause$0);
}
var ForStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForClause, BlockOrEmptyStatement), function($skip, $loc, $0, $1, $2) {
  var clause = $1;
  var block = $2;
  block = blockWithPrefix(clause.blockPrefix, block);
  return {
    ...clause,
    children: [...clause.children, block],
    block
  };
});
function ForStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ForStatement", ForStatement$0);
}
var ForClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(For, __, ForStatementControl), function($skip, $loc, $0, $1, $2, $3) {
  var c = $3;
  const { children, declaration } = c;
  return {
    type: "ForStatement",
    children: [$1, ...$2, ...children],
    declaration,
    block: null,
    blockPrefix: c.blockPrefix,
    hoistDec: c.hoistDec
  };
});
function ForClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ForClause", ForClause$0);
}
var ForStatementControl$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(CoffeeForLoopsEnabled), ForStatementParameters), function(value) {
  return value[1];
});
var ForStatementControl$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeeForLoopsEnabled, CoffeeForStatementParameters, (0, import_lib3.$E)(WhenCondition)), function($skip, $loc, $0, $1, $2, $3) {
  var condition = $3;
  if (condition) {
    const block = "continue";
    $2 = {
      ...$2,
      blockPrefix: [
        ...$2.blockPrefix,
        ["", {
          type: "IfStatement",
          then: block,
          children: ["if (!(", insertTrimmingSpace(condition, ""), ")) ", block]
        }, ";"]
      ]
    };
  }
  return $2;
});
var ForStatementControl$$ = [ForStatementControl$0, ForStatementControl$1];
function ForStatementControl(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ForStatementControl", ForStatementControl$$);
}
var WhenCondition$0 = (0, import_lib3.$T)((0, import_lib3.$S)(__, When, ExpressionWithObjectApplicationForbidden), function(value) {
  var exp = value[2];
  return exp;
});
function WhenCondition(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "WhenCondition", WhenCondition$0);
}
var CoffeeForStatementParameters$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Await, __)), InsertOpenParen, CoffeeForDeclaration, (0, import_lib3.$E)(CoffeeForIndex), __, (0, import_lib3.$C)(In, Of, From), ExpressionWithObjectApplicationForbidden, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), By, ExpressionWithObjectApplicationForbidden)), InsertCloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  var open = $2;
  var declaration = $3;
  var index = $4;
  var kind = $6;
  var exp = $7;
  var step = $8;
  var close = $9;
  let blockPrefix = [];
  exp = insertTrimmingSpace(exp, "");
  declaration = insertTrimmingSpace(declaration, "");
  if (kind.token === "from") {
    if (step) {
      throw new Error("Can't use 'by' with 'from' in CoffeeScript for loops");
    }
    kind = { ...kind, token: "of" };
  } else if (kind.token === "of") {
    if (step) {
      throw new Error("Can't use 'by' with 'of' in CoffeeScript for loops");
    }
    if (declaration.own) {
      const hasPropRef = getHelperRef("hasProp");
      blockPrefix.push(["", ["if (!", hasPropRef, "(", exp, ", ", declaration, ")) continue"], ";"]);
    }
    if (index) {
      blockPrefix.push(["", {
        type: "AssignmentExpression",
        children: [index, " = ", exp, "[", declaration, "]"],
        names: index.names
      }, ";"]);
    }
    kind.token = "in";
  } else if (kind.token === "in") {
    const counterRef = makeRef("i");
    const lenRef = makeRef("len");
    if (exp.type === "RangeExpression") {
      return forRange(open, declaration, exp, step?.[2], close);
    }
    const expRef = maybeRef(exp);
    const varRef = declaration;
    let increment = "++", indexAssignment, assignmentNames = [...varRef.names];
    if (index) {
      index = insertTrimmingSpace(index, "");
      indexAssignment = [index, "="];
      assignmentNames.push(...index.names);
    }
    const expRefDec = expRef !== exp ? [expRef, " = ", insertTrimmingSpace(exp, ""), ", "] : [];
    blockPrefix.push(["", {
      type: "AssignmentExpression",
      children: [varRef, " = ", expRef, "[", indexAssignment, counterRef, "]"],
      names: assignmentNames
    }, ";"]);
    declaration = {
      type: "Declaration",
      children: ["let ", ...expRefDec, counterRef, " = 0, ", lenRef, " = ", expRef, ".length"],
      names: []
    };
    let condition = [counterRef, " < ", lenRef, "; "];
    if (step) {
      let [stepWs, , stepExp] = step;
      stepWs = insertTrimmingSpace(stepWs, "");
      if (stepExp.type === "Literal") {
        increment = [" +=", ...stepWs, stepExp];
        if (stepExp.raw[0] === "-") {
          declaration = {
            type: "Declaration",
            children: ["let ", ...expRefDec, counterRef, " = ", expRef, ".length - 1"],
            names: []
          };
          condition = [counterRef, " >= 0; "];
        }
      } else {
        throw new Error("TODO: Support non-literal step in CoffeeScript for loops");
      }
      return {
        declaration,
        children: [$1, open, declaration, "; ", ...condition, counterRef, increment, close],
        blockPrefix
      };
    }
    return {
      declaration,
      children: [$1, open, declaration, "; ", ...condition, counterRef, increment, close],
      blockPrefix
    };
  }
  return {
    declaration,
    children: [$1, open, declaration, $5, kind, " ", exp, close],
    blockPrefix
  };
});
var CoffeeForStatementParameters$1 = ForRangeParameters;
var CoffeeForStatementParameters$$ = [CoffeeForStatementParameters$0, CoffeeForStatementParameters$1];
function CoffeeForStatementParameters(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CoffeeForStatementParameters", CoffeeForStatementParameters$$);
}
var CoffeeForIndex$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), Comma, (0, import_lib3.$E)(_), BindingIdentifier), function($skip, $loc, $0, $1, $2, $3, $4) {
  var ws1 = $1;
  var ws2 = $3;
  var id = $4;
  ws2 = insertTrimmingSpace(ws1, "");
  return {
    ...id,
    children: [...ws1 || [], ...ws2 || [], ...id.children]
  };
});
function CoffeeForIndex(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeForIndex", CoffeeForIndex$0);
}
var CoffeeForDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(__, Own)), ForBinding), function($skip, $loc, $0, $1, $2) {
  var own = $1;
  var binding = $2;
  return {
    type: "AssignmentExpression",
    own: Boolean(own),
    children: [$2],
    names: $2.names
  };
});
function CoffeeForDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeForDeclaration", CoffeeForDeclaration$0);
}
var ForStatementParameters$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenParen, __, (0, import_lib3.$C)(LexicalDeclaration, VariableStatement, (0, import_lib3.$E)(CommaExpression)), __, Semicolon, (0, import_lib3.$E)(CommaExpression), Semicolon, (0, import_lib3.$E)(CommaExpression), __, CloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10) {
  var declaration = $3;
  return {
    declaration,
    children: $0
  };
});
var ForStatementParameters$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenParen, __, (0, import_lib3.$C)(LexicalDeclaration, VariableStatement, (0, import_lib3.$E)(CommaExpression)), __, Semicolon, (0, import_lib3.$E)(CommaExpression), Semicolon, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), ExpressionWithIndentedApplicationForbidden)), InsertCloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  var declaration = $3;
  return {
    declaration,
    children: $0
  };
});
var ForStatementParameters$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Await, __)), (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$C)(Each, Own), __)), (0, import_lib3.$S)(OpenParen, __), ForInOfDeclaration, (0, import_lib3.$E)((0, import_lib3.$S)(__, Comma, __, ForInOfDeclaration)), __, (0, import_lib3.$C)(In, Of), ExpressionWithObjectApplicationForbidden, (0, import_lib3.$E)((0, import_lib3.$S)(__, By, ExpressionWithObjectApplicationForbidden)), (0, import_lib3.$S)(__, CloseParen)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10) {
  return processForInOf($0, getHelperRef);
});
var ForStatementParameters$3 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Await, __)), (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$C)(Each, Own), __)), InsertOpenParen, ForInOfDeclaration, (0, import_lib3.$E)((0, import_lib3.$S)(__, Comma, __, ForInOfDeclaration)), __, (0, import_lib3.$C)(In, Of), ExpressionWithObjectApplicationForbidden, (0, import_lib3.$E)((0, import_lib3.$S)(__, By, ExpressionWithObjectApplicationForbidden)), InsertCloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10) {
  return processForInOf($0, getHelperRef);
});
var ForStatementParameters$4 = ForRangeParameters;
var ForStatementParameters$$ = [ForStatementParameters$0, ForStatementParameters$1, ForStatementParameters$2, ForStatementParameters$3, ForStatementParameters$4];
function ForStatementParameters(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ForStatementParameters", ForStatementParameters$$);
}
var ForRangeParameters$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Await, __)), OpenParen, OpenBracket, RangeExpression, CloseBracket, (0, import_lib3.$E)((0, import_lib3.$S)(__, By, ExpressionWithObjectApplicationForbidden)), CloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var open = $2;
  var exp = $4;
  var step = $6;
  var close = $7;
  return forRange(open, null, exp, step, close);
});
var ForRangeParameters$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Await, __)), InsertOpenParen, OpenBracket, RangeExpression, CloseBracket, (0, import_lib3.$E)((0, import_lib3.$S)(__, By, ExpressionWithObjectApplicationForbidden)), InsertCloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var open = $2;
  var exp = $4;
  var step = $6;
  var close = $7;
  return forRange(open, null, exp, step, close);
});
var ForRangeParameters$$ = [ForRangeParameters$0, ForRangeParameters$1];
function ForRangeParameters(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ForRangeParameters", ForRangeParameters$$);
}
var ForInOfDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Var, ForBinding), function($skip, $loc, $0, $1, $2) {
  var binding = $2;
  return {
    type: "ForDeclaration",
    children: $0,
    declare: $1,
    binding,
    names: binding.names
  };
});
var ForInOfDeclaration$1 = ForDeclaration;
var ForInOfDeclaration$2 = LeftHandSideExpression;
var ForInOfDeclaration$$ = [ForInOfDeclaration$0, ForInOfDeclaration$1, ForInOfDeclaration$2];
function ForInOfDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ForInOfDeclaration", ForInOfDeclaration$$);
}
var ForDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(LetOrConst, ForBinding), function($skip, $loc, $0, $1, $2) {
  var c = $1;
  var binding = $2;
  return {
    type: "ForDeclaration",
    children: [c, binding],
    declare: c,
    binding,
    names: binding.names
  };
});
var ForDeclaration$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertConst, ForBinding, (0, import_lib3.$EXPECT)($R26, "ForDeclaration /(?=[\\s\\),])/")), function($skip, $loc, $0, $1, $2, $3) {
  var c = $1;
  var binding = $2;
  return {
    type: "ForDeclaration",
    children: [c, binding],
    declare: c,
    binding,
    names: binding.names
  };
});
var ForDeclaration$$ = [ForDeclaration$0, ForDeclaration$1];
function ForDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ForDeclaration", ForDeclaration$$);
}
var ForBinding$0 = BindingIdentifier;
var ForBinding$1 = BindingPattern;
var ForBinding$$ = [ForBinding$0, ForBinding$1];
function ForBinding(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ForBinding", ForBinding$$);
}
var SwitchStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Switch, (0, import_lib3.$C)(EmptyCondition, Condition), CaseBlock), function($skip, $loc, $0, $1, $2, $3) {
  var condition = $2;
  var caseBlock = $3;
  if (condition.type === "EmptyCondition") {
    caseBlock.clauses.forEach(({ cases }) => {
      if (cases) {
        cases.forEach((c) => {
          const exp = c[1];
          switch (exp.type) {
            case "Identifier":
            case "Literal":
              c.splice(1, 0, "!");
              break;
            default:
              c.splice(1, 1, "!(", exp, ")");
          }
        });
      }
    });
  }
  return {
    type: "SwitchStatement",
    children: $0,
    condition,
    caseBlock
  };
});
function SwitchStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SwitchStatement", SwitchStatement$0);
}
var EmptyCondition$0 = (0, import_lib3.$TV)((0, import_lib3.$Y)(EOS), function($skip, $loc, $0, $1) {
  return {
    type: "EmptyCondition",
    children: [{
      $loc,
      token: " (false)"
    }]
  };
});
function EmptyCondition(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EmptyCondition", EmptyCondition$0);
}
var CaseBlock$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$C)(Nested, _)), OpenBrace, NestedCaseClauses, __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var clauses = $3;
  return {
    type: "CaseBlock",
    clauses,
    children: $0
  };
});
var CaseBlock$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBrace, NestedCaseClauses, InsertNewline, InsertIndent, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var clauses = $2;
  return {
    type: "CaseBlock",
    clauses,
    children: $0
  };
});
var CaseBlock$$ = [CaseBlock$0, CaseBlock$1];
function CaseBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CaseBlock", CaseBlock$$);
}
var NestedCaseClauses$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedCaseClause), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var clauses = $2;
  if (clauses.length)
    return clauses;
  return $skip;
});
function NestedCaseClauses(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedCaseClauses", NestedCaseClauses$0);
}
var NestedCaseClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Nested, CaseClause), function($skip, $loc, $0, $1, $2) {
  var indent = $1;
  var clause = $2;
  return {
    ...clause,
    // Bring the indent into the clause
    children: [indent, ...clause.children]
  };
});
function NestedCaseClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedCaseClause", NestedCaseClause$0);
}
var CaseClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PatternExpressionList, (0, import_lib3.$C)(ThenClause, BareBlock)), function($skip, $loc, $0, $1, $2) {
  var patterns = $1;
  var block = $2;
  return {
    type: "PatternClause",
    children: $0,
    patterns,
    block
  };
});
var CaseClause$1 = (0, import_lib3.$T)((0, import_lib3.$S)(Case, CaseExpressionList, IgnoreColon, (0, import_lib3.$C)(ThenClause, BareBlock)), function(value) {
  var cases = value[1];
  var block = value[3];
  return { "type": "CaseClause", "children": value, "cases": cases, "block": block };
});
var CaseClause$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(When, CaseExpressionList, IgnoreColon, InsertOpenBrace, (0, import_lib3.$C)(ThenClause, BareBlock), InsertBreak, InsertNewline, InsertIndent, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  var cases = $2;
  var block = $5;
  var b = $6;
  return {
    type: "WhenClause",
    children: $0,
    cases,
    block,
    break: b
  };
});
var CaseClause$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(Default, ImpliedColon, (0, import_lib3.$C)(ThenClause, BareBlock)), function($skip, $loc, $0, $1, $2, $3) {
  var block = $3;
  return {
    type: "DefaultClause",
    block,
    children: $0
  };
});
var CaseClause$4 = (0, import_lib3.$TS)((0, import_lib3.$S)(Else, ImpliedColon, (0, import_lib3.$C)(ThenClause, BracedBlock, EmptyBlock)), function($skip, $loc, $0, $1, $2, $3) {
  var block = $3;
  $1.token = "default";
  return {
    type: "DefaultClause",
    block,
    children: $0
  };
});
var CaseClause$$ = [CaseClause$0, CaseClause$1, CaseClause$2, CaseClause$3, CaseClause$4];
function CaseClause(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CaseClause", CaseClause$$);
}
var PatternExpressionList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PatternExpression, (0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$E)(_), Comma, (0, import_lib3.$E)((0, import_lib3.$C)(Nested, _)), PatternExpression))), function($skip, $loc, $0, $1, $2) {
  var first = $1;
  var rest = $2;
  return [first, ...rest.map(([, , , p]) => p)];
});
function PatternExpressionList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PatternExpressionList", PatternExpressionList$0);
}
var PatternExpression$0 = BindingPattern;
var PatternExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidIndentedApplication, (0, import_lib3.$E)((0, import_lib3.$P)(SingleLineBinaryOpRHS)), RestoreIndentedApplication), function($skip, $loc, $0, $1, $2, $3) {
  var pattern = $2;
  if (!pattern)
    return $skip;
  return {
    type: "ConditionFragment",
    children: pattern
  };
});
var PatternExpression$$ = [PatternExpression$0, PatternExpression$1];
function PatternExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PatternExpression", PatternExpression$$);
}
var CaseExpressionList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$S)((0, import_lib3.$E)(_), CaseExpression, InsertColon), (0, import_lib3.$Q)((0, import_lib3.$S)(__, Comma, CaseExpression, InsertColon))), function($skip, $loc, $0, $1, $2) {
  var first = $1;
  var rest = $2;
  const result = rest.map(([ws, _comma, exp, col]) => {
    exp = insertTrimmingSpace(exp, "");
    if (ws.length)
      return [insertTrimmingSpace("case ", ws), exp, col];
    return ["case ", exp, col];
  });
  result.unshift(first);
  return result;
});
function CaseExpressionList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CaseExpressionList", CaseExpressionList$0);
}
var CaseExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PropertyName, (0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$E)(_), Colon))), function($skip, $loc, $0, $1, $2) {
  var value = $1;
  if (value.type === "ComputedPropertyName") {
    if (value.implicit)
      return value.expression;
    return { ...value, type: "ArrayExpression" };
  }
  return value;
});
var CaseExpression$1 = ExpressionWithObjectApplicationForbidden;
var CaseExpression$$ = [CaseExpression$0, CaseExpression$1];
function CaseExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CaseExpression", CaseExpression$$);
}
var ImpliedColon$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), Colon);
var ImpliedColon$1 = InsertColon;
var ImpliedColon$$ = [ImpliedColon$0, ImpliedColon$1];
function ImpliedColon(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ImpliedColon", ImpliedColon$$);
}
var IgnoreColon$0 = (0, import_lib3.$TV)((0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), Colon)), function($skip, $loc, $0, $1) {
  if ($1)
    return $1[0];
});
function IgnoreColon(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IgnoreColon", IgnoreColon$0);
}
var TryStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Try, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($L16, 'TryStatement ":"')), NoPostfixBracedOrEmptyBlock, (0, import_lib3.$E)(CatchClause), (0, import_lib3.$E)(ElseClause), (0, import_lib3.$E)(FinallyClause)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  return processTryBlock($0);
});
function TryStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TryStatement", TryStatement$0);
}
var CatchClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)(Nested, _), Catch, (0, import_lib3.$E)(CatchBind), (0, import_lib3.$C)(BracedThenClause, BracedOrEmptyBlock)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var block = $4;
  return {
    type: "CatchClause",
    children: $0,
    block
  };
});
function CatchClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CatchClause", CatchClause$0);
}
var CatchBind$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), OpenParen, __, CatchParameter, __, CloseParen);
var CatchBind$1 = (0, import_lib3.$S)(_, InsertOpenParen, (0, import_lib3.$N)(EOS), CatchParameter, InsertCloseParen);
var CatchBind$$ = [CatchBind$0, CatchBind$1];
function CatchBind(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CatchBind", CatchBind$$);
}
var FinallyClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)(Nested, _), Finally, (0, import_lib3.$C)(BracedThenClause, BracedOrEmptyBlock)), function($skip, $loc, $0, $1, $2, $3) {
  var block = $3;
  return {
    type: "FinallyClause",
    children: $0,
    block
  };
});
function FinallyClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "FinallyClause", FinallyClause$0);
}
var CatchParameter$0 = (0, import_lib3.$S)(BindingIdentifier, (0, import_lib3.$E)(TypeSuffix));
var CatchParameter$1 = (0, import_lib3.$S)(BindingPattern, (0, import_lib3.$E)(TypeSuffix));
var CatchParameter$$ = [CatchParameter$0, CatchParameter$1];
function CatchParameter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CatchParameter", CatchParameter$$);
}
var Condition$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenParen, (0, import_lib3.$E)(_), DeclarationCondition, CloseParen), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $1;
  var ws = $2;
  var expression = $3;
  var close = $4;
  return {
    type: "ParenthesizedExpression",
    children: [open, ws, expression, close],
    expression
  };
});
var Condition$1 = (0, import_lib3.$T)((0, import_lib3.$S)(ParenthesizedExpression, (0, import_lib3.$N)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)(BinaryOp, AssignmentOp, Dot, QuestionMark))), (0, import_lib3.$N)((0, import_lib3.$S)(_, OperatorAssignmentOp))), function(value) {
  return value[0];
});
var Condition$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenParen, DeclarationCondition, InsertCloseParen), function($skip, $loc, $0, $1, $2, $3) {
  var open = $1;
  var expression = $2;
  var close = $3;
  return {
    type: "ParenthesizedExpression",
    children: [open, expression, close],
    expression
  };
});
var Condition$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenParen, ExpressionWithObjectApplicationForbidden, InsertCloseParen), function($skip, $loc, $0, $1, $2, $3) {
  var open = $1;
  var expression = $2;
  var close = $3;
  if (expression.type === "ParenthesizedExpression")
    return expression;
  expression = insertTrimmingSpace(expression, "");
  return {
    type: "ParenthesizedExpression",
    children: [open, expression, close],
    expression
  };
});
var Condition$$ = [Condition$0, Condition$1, Condition$2, Condition$3];
function Condition(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Condition", Condition$$);
}
var DeclarationCondition$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidBracedApplication, ForbidIndentedApplication, ForbidNewlineBinaryOp, (0, import_lib3.$E)(LexicalDeclaration), RestoreNewlineBinaryOp, RestoreBracedApplication, RestoreIndentedApplication), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var declaration = $4;
  if (!declaration)
    return $skip;
  return {
    type: "DeclarationCondition",
    declaration,
    children: [declaration]
  };
});
function DeclarationCondition(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DeclarationCondition", DeclarationCondition$0);
}
var ExpressionWithIndentedApplicationForbidden$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidIndentedApplication, ForbidNewlineBinaryOp, (0, import_lib3.$E)(ExtendedExpression), RestoreNewlineBinaryOp, RestoreIndentedApplication), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var exp = $3;
  if (exp)
    return exp;
  return $skip;
});
function ExpressionWithIndentedApplicationForbidden(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExpressionWithIndentedApplicationForbidden", ExpressionWithIndentedApplicationForbidden$0);
}
var SingleLineExpressionWithIndentedApplicationForbidden$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidIndentedApplication, ForbidNewlineBinaryOp, (0, import_lib3.$E)(SingleLineAssignmentExpression), RestoreNewlineBinaryOp, RestoreIndentedApplication), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var exp = $3;
  if (exp)
    return exp;
  return $skip;
});
function SingleLineExpressionWithIndentedApplicationForbidden(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SingleLineExpressionWithIndentedApplicationForbidden", SingleLineExpressionWithIndentedApplicationForbidden$0);
}
var ExpressionWithObjectApplicationForbidden$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidBracedApplication, ForbidIndentedApplication, ForbidNewlineBinaryOp, (0, import_lib3.$E)(ExtendedExpression), RestoreNewlineBinaryOp, RestoreBracedApplication, RestoreIndentedApplication), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var exp = $4;
  if (exp)
    return exp;
  return $skip;
});
function ExpressionWithObjectApplicationForbidden(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExpressionWithObjectApplicationForbidden", ExpressionWithObjectApplicationForbidden$0);
}
var LeftHandSideExpressionWithObjectApplicationForbidden$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ForbidBracedApplication, ForbidIndentedApplication, ForbidNewlineBinaryOp, (0, import_lib3.$E)(LeftHandSideExpression), RestoreNewlineBinaryOp, RestoreBracedApplication, RestoreIndentedApplication), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var exp = $4;
  if (exp)
    return exp;
  return $skip;
});
function LeftHandSideExpressionWithObjectApplicationForbidden(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "LeftHandSideExpressionWithObjectApplicationForbidden", LeftHandSideExpressionWithObjectApplicationForbidden$0);
}
var ForbidClassImplicitCall$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'ForbidClassImplicitCall ""'), function($skip, $loc, $0, $1) {
  state.forbidClassImplicitCall.push(true);
});
function ForbidClassImplicitCall(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ForbidClassImplicitCall", ForbidClassImplicitCall$0);
}
var AllowClassImplicitCall$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'AllowClassImplicitCall ""'), function($skip, $loc, $0, $1) {
  state.forbidClassImplicitCall.push(false);
});
function AllowClassImplicitCall(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AllowClassImplicitCall", AllowClassImplicitCall$0);
}
var RestoreClassImplicitCall$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'RestoreClassImplicitCall ""'), function($skip, $loc, $0, $1) {
  state.forbidClassImplicitCall.pop();
});
function RestoreClassImplicitCall(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RestoreClassImplicitCall", RestoreClassImplicitCall$0);
}
var ClassImplicitCallForbidden$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'ClassImplicitCallForbidden ""'), function($skip, $loc, $0, $1) {
  if (!state.classImplicitCallForbidden)
    return $skip;
  return;
});
function ClassImplicitCallForbidden(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ClassImplicitCallForbidden", ClassImplicitCallForbidden$0);
}
var ForbidBracedApplication$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'ForbidBracedApplication ""'), function($skip, $loc, $0, $1) {
  state.forbidBracedApplication.push(true);
});
function ForbidBracedApplication(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ForbidBracedApplication", ForbidBracedApplication$0);
}
var AllowBracedApplication$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'AllowBracedApplication ""'), function($skip, $loc, $0, $1) {
  state.forbidBracedApplication.push(false);
});
function AllowBracedApplication(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AllowBracedApplication", AllowBracedApplication$0);
}
var RestoreBracedApplication$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'RestoreBracedApplication ""'), function($skip, $loc, $0, $1) {
  state.forbidBracedApplication.pop();
});
function RestoreBracedApplication(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RestoreBracedApplication", RestoreBracedApplication$0);
}
var BracedApplicationAllowed$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'BracedApplicationAllowed ""'), function($skip, $loc, $0, $1) {
  if (config.verbose) {
    console.log("forbidBracedApplication:", state.forbidBracedApplication);
  }
  if (state.bracedApplicationForbidden)
    return $skip;
  return;
});
function BracedApplicationAllowed(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BracedApplicationAllowed", BracedApplicationAllowed$0);
}
var ForbidIndentedApplication$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'ForbidIndentedApplication ""'), function($skip, $loc, $0, $1) {
  state.forbidIndentedApplication.push(true);
});
function ForbidIndentedApplication(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ForbidIndentedApplication", ForbidIndentedApplication$0);
}
var AllowIndentedApplication$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'AllowIndentedApplication ""'), function($skip, $loc, $0, $1) {
  state.forbidIndentedApplication.push(false);
});
function AllowIndentedApplication(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AllowIndentedApplication", AllowIndentedApplication$0);
}
var RestoreIndentedApplication$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'RestoreIndentedApplication ""'), function($skip, $loc, $0, $1) {
  state.forbidIndentedApplication.pop();
});
function RestoreIndentedApplication(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RestoreIndentedApplication", RestoreIndentedApplication$0);
}
var IndentedApplicationAllowed$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'IndentedApplicationAllowed ""'), function($skip, $loc, $0, $1) {
  if (config.verbose) {
    console.log("forbidIndentedApplication:", state.forbidIndentedApplication);
  }
  if (state.indentedApplicationForbidden)
    return $skip;
  return;
});
function IndentedApplicationAllowed(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IndentedApplicationAllowed", IndentedApplicationAllowed$0);
}
var ForbidTrailingMemberProperty$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'ForbidTrailingMemberProperty ""'), function($skip, $loc, $0, $1) {
  state.forbidTrailingMemberProperty.push(true);
});
function ForbidTrailingMemberProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ForbidTrailingMemberProperty", ForbidTrailingMemberProperty$0);
}
var AllowTrailingMemberProperty$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'AllowTrailingMemberProperty ""'), function($skip, $loc, $0, $1) {
  state.forbidTrailingMemberProperty.push(false);
});
function AllowTrailingMemberProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AllowTrailingMemberProperty", AllowTrailingMemberProperty$0);
}
var RestoreTrailingMemberProperty$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'RestoreTrailingMemberProperty ""'), function($skip, $loc, $0, $1) {
  state.forbidTrailingMemberProperty.pop();
});
function RestoreTrailingMemberProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RestoreTrailingMemberProperty", RestoreTrailingMemberProperty$0);
}
var TrailingMemberPropertyAllowed$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'TrailingMemberPropertyAllowed ""'), function($skip, $loc, $0, $1) {
  if (config.verbose) {
    console.log("forbidTrailingMemberProperty:", state.forbidTrailingMemberProperty);
  }
  if (state.trailingMemberPropertyForbidden)
    return $skip;
});
function TrailingMemberPropertyAllowed(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TrailingMemberPropertyAllowed", TrailingMemberPropertyAllowed$0);
}
var AllowNewlineBinaryOp$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'AllowNewlineBinaryOp ""'), function($skip, $loc, $0, $1) {
  state.forbidNewlineBinaryOp.push(false);
});
function AllowNewlineBinaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AllowNewlineBinaryOp", AllowNewlineBinaryOp$0);
}
var ForbidNewlineBinaryOp$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'ForbidNewlineBinaryOp ""'), function($skip, $loc, $0, $1) {
  state.forbidNewlineBinaryOp.push(true);
});
function ForbidNewlineBinaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ForbidNewlineBinaryOp", ForbidNewlineBinaryOp$0);
}
var RestoreNewlineBinaryOp$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'RestoreNewlineBinaryOp ""'), function($skip, $loc, $0, $1) {
  state.forbidNewlineBinaryOp.pop();
});
function RestoreNewlineBinaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RestoreNewlineBinaryOp", RestoreNewlineBinaryOp$0);
}
var NewlineBinaryOpAllowed$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'NewlineBinaryOpAllowed ""'), function($skip, $loc, $0, $1) {
  if (config.verbose) {
    console.log("forbidNewlineBinaryOp:", state.forbidNewlineBinaryOp);
  }
  if (state.newlineBinaryOpForbidden)
    return $skip;
});
function NewlineBinaryOpAllowed(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NewlineBinaryOpAllowed", NewlineBinaryOpAllowed$0);
}
var AllowAll$0 = (0, import_lib3.$S)(AllowTrailingMemberProperty, AllowBracedApplication, AllowIndentedApplication, AllowClassImplicitCall, AllowNewlineBinaryOp);
function AllowAll(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AllowAll", AllowAll$0);
}
var RestoreAll$0 = (0, import_lib3.$S)(RestoreTrailingMemberProperty, RestoreBracedApplication, RestoreIndentedApplication, RestoreClassImplicitCall, RestoreNewlineBinaryOp);
function RestoreAll(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RestoreAll", RestoreAll$0);
}
var CommaExpressionStatement$0 = IterationExpression;
var CommaExpressionStatement$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(CommaExpression), function($skip, $loc, $0, $1) {
  return makeExpressionStatement($1);
});
var CommaExpressionStatement$$ = [CommaExpressionStatement$0, CommaExpressionStatement$1];
function CommaExpressionStatement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CommaExpressionStatement", CommaExpressionStatement$$);
}
var ExpressionStatement$0 = IterationExpression;
var ExpressionStatement$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(AssignmentExpression), function($skip, $loc, $0, $1) {
  return makeExpressionStatement($1);
});
var ExpressionStatement$$ = [ExpressionStatement$0, ExpressionStatement$1];
function ExpressionStatement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ExpressionStatement", ExpressionStatement$$);
}
var KeywordStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Break, (0, import_lib3.$E)((0, import_lib3.$S)(_, (0, import_lib3.$E)(Colon), Identifier))), function($skip, $loc, $0, $1, $2) {
  return {
    type: "BreakStatement",
    children: $2 ? [$1, $2[0], $2[2]] : [$1]
    // omit colon
  };
});
var KeywordStatement$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Continue, (0, import_lib3.$E)((0, import_lib3.$S)(_, (0, import_lib3.$E)(Colon), Identifier))), function($skip, $loc, $0, $1, $2) {
  return {
    type: "ContinueStatement",
    children: $2 ? [$1, $2[0], $2[2]] : [$1]
    // omit colon
  };
});
var KeywordStatement$2 = DebuggerStatement;
var KeywordStatement$3 = (0, import_lib3.$T)((0, import_lib3.$S)(Return, (0, import_lib3.$N)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L16, 'KeywordStatement ":"'), (0, import_lib3.$EXPECT)($L7, 'KeywordStatement "."'), AfterReturnShorthand)), (0, import_lib3.$E)(MaybeParenNestedExtendedExpression)), function(value) {
  var expression = value[2];
  return { "type": "ReturnStatement", "expression": expression, "children": value };
});
var KeywordStatement$4 = ThrowStatement;
var KeywordStatement$$ = [KeywordStatement$0, KeywordStatement$1, KeywordStatement$2, KeywordStatement$3, KeywordStatement$4];
function KeywordStatement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "KeywordStatement", KeywordStatement$$);
}
var DebuggerStatement$0 = (0, import_lib3.$T)((0, import_lib3.$S)(Debugger), function(value) {
  return { "type": "DebuggerStatement", "children": value };
});
function DebuggerStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DebuggerStatement", DebuggerStatement$0);
}
var ThrowStatement$0 = (0, import_lib3.$T)((0, import_lib3.$S)(Throw, MaybeParenNestedExtendedExpression), function(value) {
  return { "type": "ThrowStatement", "children": value };
});
function ThrowStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ThrowStatement", ThrowStatement$0);
}
var Break$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L112, 'Break "break"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Break(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Break", Break$0);
}
var Continue$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L113, 'Continue "continue"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Continue(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Continue", Continue$0);
}
var Debugger$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L114, 'Debugger "debugger"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Debugger(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Debugger", Debugger$0);
}
var MaybeNestedNonPipelineExtendedExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$E)((0, import_lib3.$S)(Nested, NonPipelineExtendedExpression)), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  if ($3)
    return $3;
  return $skip;
});
var MaybeNestedNonPipelineExtendedExpression$1 = NonPipelineExtendedExpression;
var MaybeNestedNonPipelineExtendedExpression$$ = [MaybeNestedNonPipelineExtendedExpression$0, MaybeNestedNonPipelineExtendedExpression$1];
function MaybeNestedNonPipelineExtendedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MaybeNestedNonPipelineExtendedExpression", MaybeNestedNonPipelineExtendedExpression$$);
}
var MaybeNestedPostfixedExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$E)((0, import_lib3.$S)(Nested, PostfixedExpression)), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  if ($3)
    return $3;
  return $skip;
});
var MaybeNestedPostfixedExpression$1 = PostfixedExpression;
var MaybeNestedPostfixedExpression$$ = [MaybeNestedPostfixedExpression$0, MaybeNestedPostfixedExpression$1];
function MaybeNestedPostfixedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MaybeNestedPostfixedExpression", MaybeNestedPostfixedExpression$$);
}
var MaybeNestedExtendedExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$E)((0, import_lib3.$S)(Nested, ExtendedExpression)), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  if ($3)
    return $3;
  return $skip;
});
var MaybeNestedExtendedExpression$1 = ExtendedExpression;
var MaybeNestedExtendedExpression$$ = [MaybeNestedExtendedExpression$0, MaybeNestedExtendedExpression$1];
function MaybeNestedExtendedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MaybeNestedExtendedExpression", MaybeNestedExtendedExpression$$);
}
var MaybeParenNestedExtendedExpression$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), ExtendedExpression), function(value) {
  return value[1];
});
var MaybeParenNestedExtendedExpression$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), ObjectLiteral), function(value) {
  return value[1];
});
var MaybeParenNestedExtendedExpression$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), InsertSpace, InsertOpenParen, PushIndent, (0, import_lib3.$S)(Nested, ExtendedExpression), PopIndent, InsertNewline, InsertIndent, InsertCloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  var exp = $5;
  if (!exp)
    return $skip;
  return $0.slice(1);
});
var MaybeParenNestedExtendedExpression$$ = [MaybeParenNestedExtendedExpression$0, MaybeParenNestedExtendedExpression$1, MaybeParenNestedExtendedExpression$2];
function MaybeParenNestedExtendedExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MaybeParenNestedExtendedExpression", MaybeParenNestedExtendedExpression$$);
}
var ImportDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Import, _, Identifier, (0, import_lib3.$E)(_), Equals, __, (0, import_lib3.$EXPECT)($L115, 'ImportDeclaration "require"'), NonIdContinue, Arguments), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  const imp = [
    { ...$1, ts: true },
    { ...$1, token: "const", js: true }
  ];
  return {
    type: "ImportDeclaration",
    children: [imp, $0.slice(1)]
  };
});
var ImportDeclaration$1 = (0, import_lib3.$T)((0, import_lib3.$S)(Import, __, TypeKeyword, __, ImportClause, __, FromClause), function(value) {
  var imports = value[4];
  var from = value[6];
  return { "type": "ImportDeclaration", "ts": true, "children": value, "imports": imports, "from": from };
});
var ImportDeclaration$2 = (0, import_lib3.$T)((0, import_lib3.$S)(Import, __, ImportClause, __, FromClause), function(value) {
  var imports = value[2];
  var from = value[4];
  return { "type": "ImportDeclaration", "children": value, "imports": imports, "from": from };
});
var ImportDeclaration$3 = (0, import_lib3.$T)((0, import_lib3.$S)(Import, __, ModuleSpecifier), function(value) {
  var module2 = value[2];
  return { "type": "ImportDeclaration", "children": value, "module": module2 };
});
var ImportDeclaration$4 = (0, import_lib3.$TS)((0, import_lib3.$S)(ImpliedImport, (0, import_lib3.$E)((0, import_lib3.$S)(TypeKeyword, __)), ImportClause, __, FromClause), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var i = $1;
  var t = $2;
  var imports = $3;
  var w = $4;
  var from = $5;
  i.$loc = {
    pos: from[0].$loc.pos - 1,
    length: from[0].$loc.length + 1
  };
  const children = [i, t, imports, w, from];
  return { type: "ImportDeclaration", ts: !!t, children, imports, from };
});
var ImportDeclaration$$ = [ImportDeclaration$0, ImportDeclaration$1, ImportDeclaration$2, ImportDeclaration$3, ImportDeclaration$4];
function ImportDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ImportDeclaration", ImportDeclaration$$);
}
var ImpliedImport$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'ImpliedImport ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "import " };
});
function ImpliedImport(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImpliedImport", ImpliedImport$0);
}
var ImportClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(ImportedBinding, (0, import_lib3.$E)((0, import_lib3.$S)(__, Comma, __, (0, import_lib3.$C)(NameSpaceImport, NamedImports)))), function($skip, $loc, $0, $1, $2) {
  var binding = $1;
  var rest = $2;
  if (rest) {
    return {
      type: "Declaration",
      children: [binding, ...rest],
      names: [...binding.names, ...rest[3].names],
      binding,
      specifiers: rest[3].specifiers
    };
  }
  return {
    type: "Declaration",
    children: [binding],
    names: binding.names,
    binding
  };
});
var ImportClause$1 = NameSpaceImport;
var ImportClause$2 = NamedImports;
var ImportClause$$ = [ImportClause$0, ImportClause$1, ImportClause$2];
function ImportClause(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ImportClause", ImportClause$$);
}
var NameSpaceImport$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Star, ImportAsToken, __, ImportedBinding), function($skip, $loc, $0, $1, $2, $3, $4) {
  var star = $1;
  var binding = $4;
  return {
    type: "Declaration",
    children: $0,
    names: binding.names,
    binding,
    star
  };
});
function NameSpaceImport(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NameSpaceImport", NameSpaceImport$0);
}
var NamedImports$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenBrace, (0, import_lib3.$Q)(TypeAndImportSpecifier), (0, import_lib3.$E)((0, import_lib3.$S)(__, Comma)), __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var specifiers = $2;
  const names = specifiers.flatMap(({ binding }) => binding.names);
  return {
    type: "Declaration",
    children: $0,
    names,
    specifiers
  };
});
function NamedImports(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NamedImports", NamedImports$0);
}
var FromClause$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(From, __, ModuleSpecifier), function($skip, $loc, $0, $1, $2, $3) {
  var module2 = $3;
  if (!Array.isArray(module2))
    return $0;
  return [$1, $2, ...module2];
});
function FromClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "FromClause", FromClause$0);
}
var ImportAssertion$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)((0, import_lib3.$EXPECT)($L116, 'ImportAssertion "with"'), (0, import_lib3.$EXPECT)($L117, 'ImportAssertion "assert"')), NonIdContinue, (0, import_lib3.$E)(_), ObjectLiteral), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var keyword = $2;
  var object = $5;
  return {
    type: "ImportAssertion",
    keyword,
    object,
    children: $0
  };
});
function ImportAssertion(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImportAssertion", ImportAssertion$0);
}
var TypeAndImportSpecifier$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(__, TypeKeyword)), ImportSpecifier), function($skip, $loc, $0, $1, $2) {
  if (!$1)
    return $2;
  return { ts: true, children: $0, binding: $2.binding };
});
var TypeAndImportSpecifier$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, Operator, OperatorImportSpecifier), function($skip, $loc, $0, $1, $2, $3) {
  var ws = $1;
  var spec = $3;
  if (spec.binding.type !== "Identifier") {
    throw new Error("Expected identifier after `operator`");
  }
  state.operators.set(spec.binding.name, spec.behavior);
  return {
    ...spec,
    children: [
      ws,
      insertTrimmingSpace(spec[0], ""),
      spec.children.slice(1)
    ]
  };
});
var TypeAndImportSpecifier$$ = [TypeAndImportSpecifier$0, TypeAndImportSpecifier$1];
function TypeAndImportSpecifier(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeAndImportSpecifier", TypeAndImportSpecifier$$);
}
var ImportSpecifier$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, ModuleExportName, ImportAsToken, __, ImportedBinding, ObjectPropertyDelimiter), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var source = $2;
  var binding = $5;
  return {
    source,
    binding,
    children: $0
  };
});
var ImportSpecifier$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, ImportedBinding, ObjectPropertyDelimiter), function($skip, $loc, $0, $1, $2, $3) {
  var binding = $2;
  return {
    source: binding,
    binding,
    children: $0
  };
});
var ImportSpecifier$$ = [ImportSpecifier$0, ImportSpecifier$1];
function ImportSpecifier(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ImportSpecifier", ImportSpecifier$$);
}
var OperatorImportSpecifier$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, ModuleExportName, (0, import_lib3.$E)(OperatorBehavior), ImportAsToken, __, ImportedBinding, ObjectPropertyDelimiter), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var behavior = $3;
  var binding = $6;
  return {
    binding,
    behavior,
    children: [$1, $2, $4, $5, $6, $7]
  };
});
var OperatorImportSpecifier$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, ImportedBinding, (0, import_lib3.$E)(OperatorBehavior), ObjectPropertyDelimiter), function($skip, $loc, $0, $1, $2, $3, $4) {
  var binding = $2;
  var behavior = $3;
  return {
    binding,
    behavior,
    children: [$1, $2, $4]
  };
});
var OperatorImportSpecifier$$ = [OperatorImportSpecifier$0, OperatorImportSpecifier$1];
function OperatorImportSpecifier(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "OperatorImportSpecifier", OperatorImportSpecifier$$);
}
var ImportAsToken$0 = (0, import_lib3.$S)(__, As);
var ImportAsToken$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Loc, __, Colon, (0, import_lib3.$E)((0, import_lib3.$EXPECT)($L18, 'ImportAsToken " "'))), function($skip, $loc, $0, $1, $2, $3, $4) {
  var l = $1;
  var ws = $2;
  var c = $3;
  const children = [
    ...ws,
    { ...c, token: "as " }
  ];
  if (!ws.length) {
    children.unshift({ $loc: l.$loc, token: " " });
  }
  return {
    children
  };
});
var ImportAsToken$$ = [ImportAsToken$0, ImportAsToken$1];
function ImportAsToken(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ImportAsToken", ImportAsToken$$);
}
var ModuleExportName$0 = StringLiteral;
var ModuleExportName$1 = IdentifierName;
var ModuleExportName$$ = [ModuleExportName$0, ModuleExportName$1];
function ModuleExportName(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ModuleExportName", ModuleExportName$$);
}
var ModuleSpecifier$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(UnprocessedModuleSpecifier, (0, import_lib3.$E)(ImportAssertion)), function($skip, $loc, $0, $1, $2) {
  var a = $2;
  let { token } = $1;
  if (config.rewriteTsImports) {
    token = token.replace(/\.([mc])?ts(['"])$/, ".$1js$2");
  }
  if (config.rewriteCivetImports) {
    token = token.replace(
      /\.civet(['"])$/,
      `${config.rewriteCivetImports.replace(/\$/g, "$$")}$1`
    );
  }
  if (a)
    return [{ ...$1, token }, a];
  return { ...$1, token };
});
function ModuleSpecifier(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ModuleSpecifier", ModuleSpecifier$0);
}
var UnprocessedModuleSpecifier$0 = StringLiteral;
var UnprocessedModuleSpecifier$1 = UnquotedSpecifier;
var UnprocessedModuleSpecifier$$ = [UnprocessedModuleSpecifier$0, UnprocessedModuleSpecifier$1];
function UnprocessedModuleSpecifier(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "UnprocessedModuleSpecifier", UnprocessedModuleSpecifier$$);
}
var UnquotedSpecifier$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($R27, 'UnquotedSpecifier /[^;"\\s]+/'), function($skip, $loc, $0, $1) {
  var spec = $0;
  return { $loc, token: `"${spec}"` };
});
function UnquotedSpecifier(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "UnquotedSpecifier", UnquotedSpecifier$0);
}
var ImportedBinding$0 = BindingIdentifier;
function ImportedBinding(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImportedBinding", ImportedBinding$0);
}
var ExportDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Export, (0, import_lib3.$E)(_), Equals, MaybeNestedExtendedExpression), function($skip, $loc, $0, $1, $2, $3, $4) {
  const exp = [
    { ...$1, ts: true },
    { ...$1, token: "module.exports", js: true }
  ];
  return {
    type: "ExportDeclaration",
    children: [exp, $0.slice(1)]
  };
});
var ExportDeclaration$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(Decorators), Export, __, Default, __, (0, import_lib3.$N)(FunctionDeclaration), (0, import_lib3.$C)(LexicalDeclaration, VariableStatement, TypeAliasDeclaration, NamespaceDeclaration, EnumDeclaration, OperatorDeclaration)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7) {
  var declaration = $7;
  let id, error;
  if (declaration.id) {
    id = declaration.id;
  } else if (declaration.names) {
    if (declaration.names.length !== 1) {
      error = {
        type: "Error",
        message: `export default with ${declaration.names.length} variable declaration (should be 1)`
      };
    }
    id = declaration.names[0];
  } else {
    throw new Error("Could not find name of declaration in export default");
  }
  return [
    declaration,
    { children: [";"], ts: declaration.ts },
    error ?? {
      type: "ExportDeclaration",
      declaration: id,
      ts: declaration.ts,
      children: [...$0.slice(0, -2), id]
    }
  ];
});
var ExportDeclaration$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(Decorators), Export, __, Default, __, (0, import_lib3.$C)(HoistableDeclaration, ClassDeclaration, InterfaceDeclaration, MaybeNestedExtendedExpression)), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var declaration = $6;
  return { type: "ExportDeclaration", declaration, ts: declaration.ts, children: $0 };
});
var ExportDeclaration$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(Export, __, ExportFromClause, __, FromClause), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  return { type: "ExportDeclaration", ts: $3.ts, children: $0 };
});
var ExportDeclaration$4 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(Decorators), Export, __, (0, import_lib3.$C)(Declaration, VariableStatement, TypeAndNamedExports, ExportVarDec)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var declaration = $4;
  return { type: "ExportDeclaration", declaration, ts: declaration.ts, children: $0 };
});
var ExportDeclaration$$ = [ExportDeclaration$0, ExportDeclaration$1, ExportDeclaration$2, ExportDeclaration$3, ExportDeclaration$4];
function ExportDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ExportDeclaration", ExportDeclaration$$);
}
var ExportVarDec$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertVar, VariableDeclarationList), function($skip, $loc, $0, $1, $2) {
  return {
    ...$2,
    children: [$1, ...$2.children]
  };
});
function ExportVarDec(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExportVarDec", ExportVarDec$0);
}
var ExportFromClause$0 = (0, import_lib3.$S)(Star, (0, import_lib3.$E)((0, import_lib3.$S)(__, As, __, ModuleExportName)));
var ExportFromClause$1 = TypeAndNamedExports;
var ExportFromClause$$ = [ExportFromClause$0, ExportFromClause$1];
function ExportFromClause(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ExportFromClause", ExportFromClause$$);
}
var TypeAndNamedExports$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(TypeKeyword, __)), NamedExports), function($skip, $loc, $0, $1, $2) {
  if (!$1)
    return $2;
  return { ts: true, children: $0 };
});
function TypeAndNamedExports(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeAndNamedExports", TypeAndNamedExports$0);
}
var NamedExports$0 = (0, import_lib3.$S)(OpenBrace, (0, import_lib3.$Q)(ExportSpecifier), (0, import_lib3.$E)((0, import_lib3.$S)(__, Comma)), __, CloseBrace);
var NamedExports$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertInlineOpenBrace, ImplicitExportSpecifier, (0, import_lib3.$Q)((0, import_lib3.$S)(ImplicitObjectPropertyDelimiter, ImplicitExportSpecifier)), InsertCloseBrace, (0, import_lib3.$Y)((0, import_lib3.$C)(StatementDelimiter, (0, import_lib3.$S)(__, From)))), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var open = $1;
  var first = $2;
  var rest = $3;
  var close = $4;
  return [open, first, ...rest, close];
});
var NamedExports$$ = [NamedExports$0, NamedExports$1];
function NamedExports(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NamedExports", NamedExports$$);
}
var ExportSpecifier$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, (0, import_lib3.$E)((0, import_lib3.$S)(TypeKeyword, __)), ModuleExportName, (0, import_lib3.$E)((0, import_lib3.$S)(__, As, __, ModuleExportName)), ObjectPropertyDelimiter), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  if (!$2)
    return $0;
  return { ts: true, children: $0 };
});
function ExportSpecifier(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExportSpecifier", ExportSpecifier$0);
}
var ImplicitExportSpecifier$0 = (0, import_lib3.$S)((0, import_lib3.$N)(Default), ModuleExportName, (0, import_lib3.$E)((0, import_lib3.$S)(__, As, __, ModuleExportName)));
function ImplicitExportSpecifier(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ImplicitExportSpecifier", ImplicitExportSpecifier$0);
}
var Declaration$0 = (0, import_lib3.$TV)(ImportDeclaration, function($skip, $loc, $0, $1) {
  var decl = $0;
  if (decl.ts || decl.module || !decl.imports || !decl.from)
    return $skip;
  const { imports } = decl;
  if (!imports.binding && !imports.specifiers)
    return $skip;
  return dynamizeImportDeclaration(decl);
});
var Declaration$1 = HoistableDeclaration;
var Declaration$2 = ClassDeclaration;
var Declaration$3 = (0, import_lib3.$TV)(LexicalDeclaration, function($skip, $loc, $0, $1) {
  var d = $0;
  if (d.thisAssignments?.length)
    return {
      ...d,
      children: [...d.children, ...d.splices, ";", ...d.thisAssignments]
    };
  if (d.splices?.length)
    return {
      ...d,
      children: [...d.children, ...d.splices]
    };
  return d;
});
var Declaration$4 = TypeDeclaration;
var Declaration$5 = EnumDeclaration;
var Declaration$6 = OperatorDeclaration;
var Declaration$7 = UsingDeclaration;
var Declaration$$ = [Declaration$0, Declaration$1, Declaration$2, Declaration$3, Declaration$4, Declaration$5, Declaration$6, Declaration$7];
function Declaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Declaration", Declaration$$);
}
var HoistableDeclaration$0 = FunctionDeclaration;
function HoistableDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "HoistableDeclaration", HoistableDeclaration$0);
}
var LexicalDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(LetOrConst, LexicalBinding, (0, import_lib3.$Q)((0, import_lib3.$S)(__, Comma, __, LexicalBinding))), function($skip, $loc, $0, $1, $2, $3) {
  var decl = $1;
  var binding = $2;
  var tail = $3;
  const bindings = [binding].concat(tail.map(([, , , b]) => b));
  return {
    type: "Declaration",
    children: $0,
    names: bindings.flatMap((b) => b.names),
    bindings,
    decl,
    splices: bindings.flatMap((b) => b.splices),
    thisAssignments: bindings.flatMap((b) => b.thisAssignments)
  };
});
var LexicalDeclaration$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Loc, (0, import_lib3.$C)(BindingPattern, BindingIdentifier), (0, import_lib3.$E)(TypeSuffix), __, (0, import_lib3.$C)(ConstAssignment, LetAssignment), MaybeNestedPostfixedExpression), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var loc = $1;
  var assign = $5;
  return processAssignmentDeclaration(
    { $loc: loc, token: assign.decl },
    ...$0.slice(1)
  );
});
var LexicalDeclaration$$ = [LexicalDeclaration$0, LexicalDeclaration$1];
function LexicalDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "LexicalDeclaration", LexicalDeclaration$$);
}
var ConstAssignment$0 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L118, 'ConstAssignment ":="'), (0, import_lib3.$EXPECT)($L119, 'ConstAssignment "\u2254"')), function($skip, $loc, $0, $1) {
  return { $loc, token: "=", decl: "const " };
});
function ConstAssignment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ConstAssignment", ConstAssignment$0);
}
var LetAssignment$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L120, 'LetAssignment ".="'), function($skip, $loc, $0, $1) {
  return { $loc, token: "=", decl: "let " };
});
function LetAssignment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "LetAssignment", LetAssignment$0);
}
var TypeAssignment$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L121, 'TypeAssignment "::="'), function($skip, $loc, $0, $1) {
  return { $loc, token: "=" };
});
function TypeAssignment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeAssignment", TypeAssignment$0);
}
var LexicalBinding$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(BindingPattern, (0, import_lib3.$E)(TypeSuffix), Initializer), function($skip, $loc, $0, $1, $2, $3) {
  var pattern = $1;
  var suffix = $2;
  var initializer = $3;
  const [splices, thisAssignments] = gatherBindingCode(pattern);
  return {
    type: "Binding",
    children: $0,
    names: pattern.names,
    pattern,
    suffix,
    initializer,
    splices: splices.map((s) => [",", s]),
    thisAssignments: thisAssignments.map((s) => ["", s, ";"])
  };
});
var LexicalBinding$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(BindingIdentifier, (0, import_lib3.$E)(TypeSuffix), (0, import_lib3.$E)(Initializer)), function($skip, $loc, $0, $1, $2, $3) {
  var pattern = $1;
  var suffix = $2;
  var initializer = $3;
  return {
    type: "Binding",
    children: $0,
    names: pattern.names,
    pattern,
    suffix,
    initializer,
    splices: [],
    thisAssignments: []
  };
});
var LexicalBinding$$ = [LexicalBinding$0, LexicalBinding$1];
function LexicalBinding(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "LexicalBinding", LexicalBinding$$);
}
var Initializer$0 = (0, import_lib3.$T)((0, import_lib3.$S)(__, Equals, MaybeNestedExtendedExpression), function(value) {
  var expression = value[2];
  return { "type": "Initializer", "expression": expression, "children": value };
});
function Initializer(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Initializer", Initializer$0);
}
var VariableStatement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Var, __, VariableDeclarationList), function($skip, $loc, $0, $1, $2, $3) {
  return {
    ...$3,
    names: $3.names,
    children: [$1, ...$2, ...$3.children]
  };
});
function VariableStatement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "VariableStatement", VariableStatement$0);
}
var VariableDeclarationList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(LexicalBinding, (0, import_lib3.$Q)((0, import_lib3.$S)(__, Comma, __, LexicalBinding))), function($skip, $loc, $0, $1, $2) {
  var binding = $1;
  var tail = $2;
  const bindings = [binding].concat(tail.map(([, , , b]) => b));
  return {
    type: "Declaration",
    children: [binding, ...tail],
    bindings,
    names: bindings.flatMap((b) => b.names)
  };
});
function VariableDeclarationList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "VariableDeclarationList", VariableDeclarationList$0);
}
var NumericLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R28, "NumericLiteral /(?=[0-9.])/"), NumericLiteralKind), function($skip, $loc, $0, $1, $2) {
  var token = $2;
  return { type: "NumericLiteral", $loc, token };
});
function NumericLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NumericLiteral", NumericLiteral$0);
}
var NumericLiteralKind$0 = DecimalBigIntegerLiteral;
var NumericLiteralKind$1 = BinaryIntegerLiteral;
var NumericLiteralKind$2 = OctalIntegerLiteral;
var NumericLiteralKind$3 = HexIntegerLiteral;
var NumericLiteralKind$4 = DecimalLiteral;
var NumericLiteralKind$$ = [NumericLiteralKind$0, NumericLiteralKind$1, NumericLiteralKind$2, NumericLiteralKind$3, NumericLiteralKind$4];
function NumericLiteralKind(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NumericLiteralKind", NumericLiteralKind$$);
}
var DecimalBigIntegerLiteral$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R29, "DecimalBigIntegerLiteral /(?:0|[1-9](?:_[0-9]|[0-9])*)n/"));
function DecimalBigIntegerLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DecimalBigIntegerLiteral", DecimalBigIntegerLiteral$0);
}
var DecimalLiteral$0 = (0, import_lib3.$TV)((0, import_lib3.$TEXT)((0, import_lib3.$EXPECT)($R30, "DecimalLiteral /(?:0|[1-9](?:_[0-9]|[0-9])*)(?=\\.(?:\\p{ID_Start}|[_$]))/")), function($skip, $loc, $0, $1) {
  return $1 + ".";
});
var DecimalLiteral$1 = (0, import_lib3.$TEXT)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R31, "DecimalLiteral /(?:0|[1-9](?:_[0-9]|[0-9])*)(?:\\.(?:[0-9](?:_[0-9]|[0-9])*))?/"), (0, import_lib3.$E)(ExponentPart)));
var DecimalLiteral$2 = (0, import_lib3.$TEXT)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R32, "DecimalLiteral /(?:\\.[0-9](?:_[0-9]|[0-9])*)/"), (0, import_lib3.$E)(ExponentPart)));
var DecimalLiteral$$ = [DecimalLiteral$0, DecimalLiteral$1, DecimalLiteral$2];
function DecimalLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "DecimalLiteral", DecimalLiteral$$);
}
var ExponentPart$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R33, "ExponentPart /(?:[eE][+-]?[0-9]+(?:_[0-9]|[0-9])*)/"));
function ExponentPart(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExponentPart", ExponentPart$0);
}
var BinaryIntegerLiteral$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R34, "BinaryIntegerLiteral /0[bB][01](?:[01]|_[01])*n?/"));
function BinaryIntegerLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BinaryIntegerLiteral", BinaryIntegerLiteral$0);
}
var OctalIntegerLiteral$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R35, "OctalIntegerLiteral /0[oO][0-7](?:[0-7]|_[0-7])*n?/"));
function OctalIntegerLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "OctalIntegerLiteral", OctalIntegerLiteral$0);
}
var HexIntegerLiteral$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R36, "HexIntegerLiteral /0[xX][0-9a-fA-F](?:[0-9a-fA-F]|_[0-9a-fA-F])*n?/"));
function HexIntegerLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "HexIntegerLiteral", HexIntegerLiteral$0);
}
var IntegerLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R37, "IntegerLiteral /(?=[0-9])/"), IntegerLiteralKind), function($skip, $loc, $0, $1, $2) {
  var token = $2;
  return { $loc, token };
});
function IntegerLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IntegerLiteral", IntegerLiteral$0);
}
var IntegerLiteralKind$0 = DecimalBigIntegerLiteral;
var IntegerLiteralKind$1 = BinaryIntegerLiteral;
var IntegerLiteralKind$2 = OctalIntegerLiteral;
var IntegerLiteralKind$3 = HexIntegerLiteral;
var IntegerLiteralKind$4 = DecimalIntegerLiteral;
var IntegerLiteralKind$$ = [IntegerLiteralKind$0, IntegerLiteralKind$1, IntegerLiteralKind$2, IntegerLiteralKind$3, IntegerLiteralKind$4];
function IntegerLiteralKind(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "IntegerLiteralKind", IntegerLiteralKind$$);
}
var DecimalIntegerLiteral$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R38, "DecimalIntegerLiteral /(?:0|[1-9](?:_[0-9]|[0-9])*)/"));
function DecimalIntegerLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DecimalIntegerLiteral", DecimalIntegerLiteral$0);
}
var StringLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(DoubleQuote, DoubleStringCharacters, DoubleQuote), function($skip, $loc, $0, $1, $2, $3) {
  var str = $2;
  return {
    type: "StringLiteral",
    token: `"${modifyString(str.token)}"`,
    $loc
  };
});
var StringLiteral$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(SingleQuote, SingleStringCharacters, SingleQuote), function($skip, $loc, $0, $1, $2, $3) {
  var str = $2;
  return {
    type: "StringLiteral",
    token: `'${modifyString(str.token)}'`,
    $loc
  };
});
var StringLiteral$$ = [StringLiteral$0, StringLiteral$1];
function StringLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "StringLiteral", StringLiteral$$);
}
var DoubleStringCharacters$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R39, 'DoubleStringCharacters /(?:\\\\.|[^"])*/'), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function DoubleStringCharacters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DoubleStringCharacters", DoubleStringCharacters$0);
}
var SingleStringCharacters$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R40, "SingleStringCharacters /(?:\\\\.|[^'])*/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function SingleStringCharacters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SingleStringCharacters", SingleStringCharacters$0);
}
var TripleDoubleStringCharacters$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R41, 'TripleDoubleStringCharacters /(?:"(?!"")|#(?!\\{)|\\\\.|[^#"])+/'), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function TripleDoubleStringCharacters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TripleDoubleStringCharacters", TripleDoubleStringCharacters$0);
}
var TripleSingleStringCharacters$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R42, "TripleSingleStringCharacters /(?:'(?!'')|\\\\.|[^'])*/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function TripleSingleStringCharacters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TripleSingleStringCharacters", TripleSingleStringCharacters$0);
}
var CoffeeStringSubstitution$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeeSubstitutionStart, AllowAll, (0, import_lib3.$E)((0, import_lib3.$S)(PostfixedExpression, __, CloseBrace)), RestoreAll), function($skip, $loc, $0, $1, $2, $3, $4) {
  if (!$3)
    return $skip;
  return [$1, ...$3];
});
function CoffeeStringSubstitution(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeStringSubstitution", CoffeeStringSubstitution$0);
}
var CoffeeInterpolatedDoubleQuotedString$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeeInterpolationEnabled, DoubleQuote, (0, import_lib3.$Q)((0, import_lib3.$C)(CoffeeDoubleQuotedStringCharacters, CoffeeStringSubstitution)), DoubleQuote), function($skip, $loc, $0, $1, $2, $3, $4) {
  var s = $2;
  var parts = $3;
  var e = $4;
  return processCoffeeInterpolation(s, parts, e, $loc);
});
function CoffeeInterpolatedDoubleQuotedString(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeInterpolatedDoubleQuotedString", CoffeeInterpolatedDoubleQuotedString$0);
}
var CoffeeDoubleQuotedStringCharacters$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R43, 'CoffeeDoubleQuotedStringCharacters /(?:\\\\.|#(?!\\{)|[^"\#])+/'), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function CoffeeDoubleQuotedStringCharacters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeDoubleQuotedStringCharacters", CoffeeDoubleQuotedStringCharacters$0);
}
var RegularExpressionLiteral$0 = HeregexLiteral;
var RegularExpressionLiteral$1 = (0, import_lib3.$TV)((0, import_lib3.$TEXT)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L66, 'RegularExpressionLiteral "/"'), RegularExpressionBody, (0, import_lib3.$EXPECT)($L66, 'RegularExpressionLiteral "/"'), RegularExpressionFlags)), function($skip, $loc, $0, $1) {
  return { type: "RegularExpressionLiteral", $loc, token: $1 };
});
var RegularExpressionLiteral$$ = [RegularExpressionLiteral$0, RegularExpressionLiteral$1];
function RegularExpressionLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "RegularExpressionLiteral", RegularExpressionLiteral$$);
}
var RegularExpressionClass$0 = (0, import_lib3.$TV)((0, import_lib3.$TEXT)((0, import_lib3.$S)(OpenBracket, RegularExpressionClassCharacters, CloseBracket)), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function RegularExpressionClass(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RegularExpressionClass", RegularExpressionClass$0);
}
var RegularExpressionClassCharacters$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R44, "RegularExpressionClassCharacters /(?:\\\\.|[^\\]])*/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function RegularExpressionClassCharacters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RegularExpressionClassCharacters", RegularExpressionClassCharacters$0);
}
var HeregexLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(TripleSlash, HeregexBody, TripleSlash, RegularExpressionFlags), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $1;
  var body = $2;
  var close = $3;
  var flags = $4;
  let hasSubstitutions = body.some((part) => part.type === "Substitution");
  if (hasSubstitutions) {
    const children = [
      { ...open, token: "RegExp(`" },
      // Escape backticks, backslashes, and '$' in the body text
      body.map(
        (e) => e.type === "Substitution" ? e : {
          ...e,
          token: e.token.replace(/`|\\|\$/g, "\\$&")
        }
      ),
      "`"
    ];
    if (flags.length) {
      children.push(
        ", ",
        JSON.stringify(flags)
      );
    }
    children.push({ ...close, token: ")" });
    return {
      type: "RegularExpressionLiteral",
      children
    };
  }
  return {
    type: "RegularExpressionLiteral",
    children: $0
  };
});
function HeregexLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "HeregexLiteral", HeregexLiteral$0);
}
var HeregexBody$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(TripleSlash), (0, import_lib3.$Q)(HeregexPart)), function(value) {
  return value[1];
});
function HeregexBody(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "HeregexBody", HeregexBody$0);
}
var HeregexPart$0 = RegularExpressionClass;
var HeregexPart$1 = (0, import_lib3.$T)((0, import_lib3.$S)(CoffeeStringSubstitution), function(value) {
  return { "type": "Substitution", "children": value[0] };
});
var HeregexPart$2 = (0, import_lib3.$T)((0, import_lib3.$S)(TemplateSubstitution), function(value) {
  return { "type": "Substitution", "children": value[0] };
});
var HeregexPart$3 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R45, "HeregexPart /(?:\\\\.)/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  let token = $0;
  switch ($0[1]) {
    case "\n":
      token = "\\n";
      break;
    case "\r":
      token = "\\r";
      break;
    case " ":
      token = " ";
      break;
  }
  return { $loc, token };
});
var HeregexPart$4 = (0, import_lib3.$TS)((0, import_lib3.$S)(HeregexComment), function($skip, $loc, $0, $1) {
  return { $loc, token: "" };
});
var HeregexPart$5 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R46, "HeregexPart /[\\s]+/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: "" };
});
var HeregexPart$6 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R47, "HeregexPart /\\/(?!\\/\\/)/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: "\\/" };
});
var HeregexPart$7 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R48, "HeregexPart /[^[\\/\\s#\\\\]+/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
var HeregexPart$$ = [HeregexPart$0, HeregexPart$1, HeregexPart$2, HeregexPart$3, HeregexPart$4, HeregexPart$5, HeregexPart$6, HeregexPart$7];
function HeregexPart(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "HeregexPart", HeregexPart$$);
}
var HeregexComment$0 = JSSingleLineComment;
var HeregexComment$1 = CoffeeSingleLineComment;
var HeregexComment$$ = [HeregexComment$0, HeregexComment$1];
function HeregexComment(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "HeregexComment", HeregexComment$$);
}
var RegularExpressionBody$0 = (0, import_lib3.$S)((0, import_lib3.$N)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R49, "RegularExpressionBody /[*\\/\\r\\n]/"))), (0, import_lib3.$Q)(RegExpPart));
function RegularExpressionBody(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RegularExpressionBody", RegularExpressionBody$0);
}
var RegExpPart$0 = RegularExpressionClass;
var RegExpPart$1 = RegExpCharacter;
var RegExpPart$$ = [RegExpPart$0, RegExpPart$1];
function RegExpPart(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "RegExpPart", RegExpPart$$);
}
var RegExpCharacter$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R50, "RegExpCharacter /(?:\\\\.|[^[\\/\\r\\n])+/"));
function RegExpCharacter(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RegExpCharacter", RegExpCharacter$0);
}
var RegularExpressionFlags$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R51, "RegularExpressionFlags /(?:\\p{ID_Continue}|[\\u200C\\u200D$])*/"));
function RegularExpressionFlags(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RegularExpressionFlags", RegularExpressionFlags$0);
}
var TemplateLiteral$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R52, "TemplateLiteral /(?=[`'\"])/"), _TemplateLiteral), function(value) {
  return value[1];
});
function TemplateLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TemplateLiteral", TemplateLiteral$0);
}
var _TemplateLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(TripleTick, (0, import_lib3.$Q)((0, import_lib3.$C)(TemplateBlockCharacters, TemplateSubstitution)), TripleTick), function($skip, $loc, $0, $1, $2, $3) {
  return dedentBlockSubstitutions($0, config.tab);
});
var _TemplateLiteral$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Backtick, (0, import_lib3.$Q)((0, import_lib3.$C)(TemplateCharacters, TemplateSubstitution)), Backtick), function($skip, $loc, $0, $1, $2, $3) {
  return {
    type: "TemplateLiteral",
    children: $0
  };
});
var _TemplateLiteral$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(TripleDoubleQuote, (0, import_lib3.$Q)((0, import_lib3.$C)(TripleDoubleStringCharacters, CoffeeStringSubstitution)), TripleDoubleQuote), function($skip, $loc, $0, $1, $2, $3) {
  return dedentBlockSubstitutions($0, config.tab);
});
var _TemplateLiteral$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(TripleSingleQuote, TripleSingleStringCharacters, TripleSingleQuote), function($skip, $loc, $0, $1, $2, $3) {
  var s = $1;
  var str = $2;
  var e = $3;
  return {
    type: "TemplateLiteral",
    children: [s, dedentBlockString(str, config.tab), e]
  };
});
var _TemplateLiteral$4 = CoffeeInterpolatedDoubleQuotedString;
var _TemplateLiteral$$ = [_TemplateLiteral$0, _TemplateLiteral$1, _TemplateLiteral$2, _TemplateLiteral$3, _TemplateLiteral$4];
function _TemplateLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "_TemplateLiteral", _TemplateLiteral$$);
}
var TemplateSubstitution$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(SubstitutionStart, AllowAll, (0, import_lib3.$E)((0, import_lib3.$S)(PostfixedExpression, __, CloseBrace)), RestoreAll), function($skip, $loc, $0, $1, $2, $3, $4) {
  if (!$3)
    return $skip;
  return [$1, ...$3];
});
function TemplateSubstitution(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TemplateSubstitution", TemplateSubstitution$0);
}
var TemplateCharacters$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R53, "TemplateCharacters /(?:\\$(?!\\{)|\\\\.|[^$`])+/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function TemplateCharacters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TemplateCharacters", TemplateCharacters$0);
}
var TemplateBlockCharacters$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R54, "TemplateBlockCharacters /(?:\\$(?!\\{)|`(?!``)|\\\\.|[^$`])+/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function TemplateBlockCharacters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TemplateBlockCharacters", TemplateBlockCharacters$0);
}
var ReservedWord$0 = (0, import_lib3.$S)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R55, "ReservedWord /(?:on|off|yes|no)(?!\\p{ID_Continue})/")), CoffeeBooleansEnabled);
var ReservedWord$1 = (0, import_lib3.$S)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R56, "ReservedWord /(?:isnt)(?!\\p{ID_Continue})/")), CoffeeIsntEnabled);
var ReservedWord$2 = (0, import_lib3.$S)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R57, "ReservedWord /(?:by)(?!\\p{ID_Continue})/")), CoffeeForLoopsEnabled);
var ReservedWord$3 = (0, import_lib3.$S)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R58, "ReservedWord /(?:of)(?!\\p{ID_Continue})/")), CoffeeOfEnabled);
var ReservedWord$4 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R59, "ReservedWord /(?:and|await|break|case|catch|class|const|continue|debugger|default|delete|do|else|enum|export|extends|false|finally|for|function|if|import|in|instanceof|interface|is|let|loop|new|not|null|or|private|protected|public|return|static|super|switch|this|throw|true|try|typeof|unless|until|var|void|while|with|yield)(?!\\p{ID_Continue})/"));
var ReservedWord$$ = [ReservedWord$0, ReservedWord$1, ReservedWord$2, ReservedWord$3, ReservedWord$4];
function ReservedWord(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ReservedWord", ReservedWord$$);
}
var Comment$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R60, "Comment /(?=\\/|#)/"), _Comment), function(value) {
  return value[1];
});
function Comment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Comment", Comment$0);
}
var _Comment$0 = MultiLineComment;
var _Comment$1 = SingleLineComment;
var _Comment$$ = [_Comment$0, _Comment$1];
function _Comment(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "_Comment", _Comment$$);
}
var SingleLineComment$0 = JSSingleLineComment;
var SingleLineComment$1 = (0, import_lib3.$S)(CoffeeCommentEnabled, CoffeeSingleLineComment);
var SingleLineComment$$ = [SingleLineComment$0, SingleLineComment$1];
function SingleLineComment(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "SingleLineComment", SingleLineComment$$);
}
var JSSingleLineComment$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R61, "JSSingleLineComment /\\/\\/(?!\\/)[^\\r\\n]*/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { type: "Comment", $loc, token: $0 };
});
function JSSingleLineComment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSSingleLineComment", JSSingleLineComment$0);
}
var MultiLineComment$0 = JSMultiLineComment;
var MultiLineComment$1 = CoffeeMultiLineComment;
var MultiLineComment$$ = [MultiLineComment$0, MultiLineComment$1];
function MultiLineComment(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MultiLineComment", MultiLineComment$$);
}
var JSMultiLineComment$0 = (0, import_lib3.$TV)((0, import_lib3.$TEXT)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L122, 'JSMultiLineComment "/*"'), (0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$N)((0, import_lib3.$EXPECT)($L123, 'JSMultiLineComment "*/"')), (0, import_lib3.$EXPECT)($R62, "JSMultiLineComment /./"))), (0, import_lib3.$EXPECT)($L123, 'JSMultiLineComment "*/"'))), function($skip, $loc, $0, $1) {
  return { type: "Comment", $loc, token: $1 };
});
function JSMultiLineComment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSMultiLineComment", JSMultiLineComment$0);
}
var CoffeeSingleLineComment$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R63, "CoffeeSingleLineComment /#(?!##(?!#))([^\\r\\n]*)/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { type: "Comment", $loc, token: `//${$1}` };
});
function CoffeeSingleLineComment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeSingleLineComment", CoffeeSingleLineComment$0);
}
var CoffeeMultiLineComment$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeeHereCommentStart, (0, import_lib3.$TEXT)((0, import_lib3.$EXPECT)($R64, "CoffeeMultiLineComment /[^]*?###/"))), function($skip, $loc, $0, $1, $2) {
  $2 = $2.slice(0, $2.length - 3).replace(/\*\//g, "* /");
  return { type: "Comment", $loc, token: `/*${$2}*/` };
});
function CoffeeMultiLineComment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeMultiLineComment", CoffeeMultiLineComment$0);
}
var CoffeeHereCommentStart$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R65, "CoffeeHereCommentStart /###(?!#)/"));
function CoffeeHereCommentStart(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeHereCommentStart", CoffeeHereCommentStart$0);
}
var InlineComment$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R66, "InlineComment /\\/\\*(?:(?!\\*\\/)[^\\r\\n])*\\*\\//"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function InlineComment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineComment", InlineComment$0);
}
var RestOfLine$0 = (0, import_lib3.$S)((0, import_lib3.$Q)((0, import_lib3.$C)(NonNewlineWhitespace, Comment)), EOL);
function RestOfLine(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "RestOfLine", RestOfLine$0);
}
var TrailingComment$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$E)(SingleLineComment));
function TrailingComment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TrailingComment", TrailingComment$0);
}
var _$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R67, "_ /(?=[ \\t\\/\\\\])/"), (0, import_lib3.$P)((0, import_lib3.$C)(NonNewlineWhitespace, InlineComment))), function(value) {
  return value[1];
});
function _(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "_", _$0);
}
var NonNewlineWhitespace$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R68, "NonNewlineWhitespace /[ \\t]+/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
var NonNewlineWhitespace$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L124, 'NonNewlineWhitespace "\\\\\\\\"'), CoffeeLineContinuationEnabled, EOL), function(value) {
  return " ";
});
var NonNewlineWhitespace$$ = [NonNewlineWhitespace$0, NonNewlineWhitespace$1];
function NonNewlineWhitespace(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "NonNewlineWhitespace", NonNewlineWhitespace$$);
}
var Trimmed_$0 = (0, import_lib3.$TV)(_, function($skip, $loc, $0, $1) {
  var ws = $0;
  return insertTrimmingSpace(ws, "");
});
function Trimmed_(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Trimmed_", Trimmed_$0);
}
var __$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R69, "__ /(?=\\s|\\/|#)/"), (0, import_lib3.$Q)((0, import_lib3.$C)(Whitespace, Comment))), function(value) {
  return value[1];
});
var __$1 = (0, import_lib3.$EXPECT)($L0, '__ ""');
var __$$ = [__$0, __$1];
function __(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "__", __$$);
}
var Whitespace$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R46, "Whitespace /[\\s]+/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function Whitespace(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Whitespace", Whitespace$0);
}
var ExpressionDelimiter$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), Semicolon, InsertComma, TrailingComment), function($skip, $loc, $0, $1, $2, $3, $4) {
  return [$1, $3, $4];
});
var ExpressionDelimiter$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), InsertComma), function(value) {
  return value[1];
});
var ExpressionDelimiter$$ = [ExpressionDelimiter$0, ExpressionDelimiter$1];
function ExpressionDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ExpressionDelimiter", ExpressionDelimiter$$);
}
var SimpleStatementDelimiter$0 = (0, import_lib3.$Y)(EOS);
var SimpleStatementDelimiter$1 = SemicolonDelimiter;
var SimpleStatementDelimiter$$ = [SimpleStatementDelimiter$0, SimpleStatementDelimiter$1];
function SimpleStatementDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "SimpleStatementDelimiter", SimpleStatementDelimiter$$);
}
var StatementDelimiter$0 = (0, import_lib3.$Y)(EOS);
var StatementDelimiter$1 = SemicolonDelimiter;
var StatementDelimiter$2 = (0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)((0, import_lib3.$EXPECT)($L36, 'StatementDelimiter "}"'), (0, import_lib3.$EXPECT)($L125, 'StatementDelimiter ")"'), (0, import_lib3.$EXPECT)($L45, 'StatementDelimiter "]"'))));
var StatementDelimiter$$ = [StatementDelimiter$0, StatementDelimiter$1, StatementDelimiter$2];
function StatementDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "StatementDelimiter", StatementDelimiter$$);
}
var SemicolonDelimiter$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), Semicolon, TrailingComment), function($skip, $loc, $0, $1, $2, $3) {
  return {
    type: "SemicolonDelimiter",
    children: $0
  };
});
function SemicolonDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SemicolonDelimiter", SemicolonDelimiter$0);
}
var NonIdContinue$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R70, "NonIdContinue /(?!\\p{ID_Continue})/"));
function NonIdContinue(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NonIdContinue", NonIdContinue$0);
}
var Loc$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'Loc ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "" };
});
function Loc(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Loc", Loc$0);
}
var Abstract$0 = (0, import_lib3.$TV)((0, import_lib3.$TEXT)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L126, 'Abstract "abstract"'), NonIdContinue, (0, import_lib3.$E)((0, import_lib3.$EXPECT)($L18, 'Abstract " "')))), function($skip, $loc, $0, $1) {
  return { $loc, token: $1, ts: true };
});
function Abstract(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Abstract", Abstract$0);
}
var Ampersand$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L108, 'Ampersand "&"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function Ampersand(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Ampersand", Ampersand$0);
}
var As$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L127, 'As "as"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function As(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "As", As$0);
}
var At$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L128, 'At "@"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function At(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "At", At$0);
}
var AtAt$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L129, 'AtAt "@@"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "@" };
});
function AtAt(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "AtAt", AtAt$0);
}
var Async$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L130, 'Async "async"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1, type: "Async" };
});
function Async(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Async", Async$0);
}
var Await$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L131, 'Await "await"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1, type: "Await" };
});
function Await(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Await", Await$0);
}
var Backtick$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L132, 'Backtick "`"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function Backtick(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Backtick", Backtick$0);
}
var By$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L133, 'By "by"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function By(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "By", By$0);
}
var Caret$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L22, 'Caret "^"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function Caret(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Caret", Caret$0);
}
var Case$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L134, 'Case "case"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Case(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Case", Case$0);
}
var Catch$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L135, 'Catch "catch"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Catch(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Catch", Catch$0);
}
var Class$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L136, 'Class "class"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Class(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Class", Class$0);
}
var CloseAngleBracket$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L44, 'CloseAngleBracket ">"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function CloseAngleBracket(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CloseAngleBracket", CloseAngleBracket$0);
}
var CloseBrace$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L36, 'CloseBrace "}"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function CloseBrace(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CloseBrace", CloseBrace$0);
}
var CloseBracket$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L45, 'CloseBracket "]"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function CloseBracket(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CloseBracket", CloseBracket$0);
}
var CloseParen$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L125, 'CloseParen ")"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function CloseParen(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CloseParen", CloseParen$0);
}
var CoffeeSubstitutionStart$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L137, 'CoffeeSubstitutionStart "\#{"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "${" };
});
function CoffeeSubstitutionStart(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeSubstitutionStart", CoffeeSubstitutionStart$0);
}
var Colon$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L16, 'Colon ":"'), (0, import_lib3.$N)((0, import_lib3.$EXPECT)($R71, "Colon /[=:]/"))), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Colon(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Colon", Colon$0);
}
var Comma$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L17, 'Comma ","'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function Comma(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Comma", Comma$0);
}
var Comptime$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L138, 'Comptime "comptime"'), NonIdContinue, (0, import_lib3.$N)((0, import_lib3.$EXPECT)($L16, 'Comptime ":"'))), function($skip, $loc, $0, $1, $2, $3) {
  return { $loc, token: $1 };
});
function Comptime(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Comptime", Comptime$0);
}
var ConstructorShorthand$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L128, 'ConstructorShorthand "@"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "constructor" };
});
function ConstructorShorthand(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ConstructorShorthand", ConstructorShorthand$0);
}
var Declare$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L139, 'Declare "declare"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Declare(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Declare", Declare$0);
}
var Default$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L140, 'Default "default"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Default(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Default", Default$0);
}
var Delete$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L141, 'Delete "delete"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Delete(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Delete", Delete$0);
}
var Do$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L142, 'Do "do"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Do(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Do", Do$0);
}
var Dot$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L7, 'Dot "."'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
var Dot$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R72, "Dot /['\u2019]s/"), Trimmed_), function($skip, $loc, $0, $1, $2) {
  var ws = $2;
  return [
    { $loc, token: "." },
    ws
  ];
});
var Dot$$ = [Dot$0, Dot$1];
function Dot(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Dot", Dot$$);
}
var DotDot$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L143, 'DotDot ".."'), (0, import_lib3.$N)((0, import_lib3.$EXPECT)($L7, 'DotDot "."'))), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
var DotDot$1 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L144, 'DotDot "\u2025"'), function($skip, $loc, $0, $1) {
  return { $loc, token: ".." };
});
var DotDot$$ = [DotDot$0, DotDot$1];
function DotDot(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "DotDot", DotDot$$);
}
var DotDotDot$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L145, 'DotDotDot "..."'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
var DotDotDot$1 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L146, 'DotDotDot "\u2026"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "..." };
});
var DotDotDot$$ = [DotDotDot$0, DotDotDot$1];
function DotDotDot(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "DotDotDot", DotDotDot$$);
}
var DoubleColon$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L147, 'DoubleColon "::"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function DoubleColon(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DoubleColon", DoubleColon$0);
}
var DoubleQuote$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L148, 'DoubleQuote "\\\\\\""'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function DoubleQuote(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DoubleQuote", DoubleQuote$0);
}
var Each$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L149, 'Each "each"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Each(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Each", Each$0);
}
var Else$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L150, 'Else "else"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Else(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Else", Else$0);
}
var Equals$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L3, 'Equals "="'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function Equals(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Equals", Equals$0);
}
var ExclamationPoint$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L151, 'ExclamationPoint "!"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function ExclamationPoint(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ExclamationPoint", ExclamationPoint$0);
}
var Export$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L152, 'Export "export"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Export(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Export", Export$0);
}
var Extends$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L153, 'Extends "extends"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Extends(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Extends", Extends$0);
}
var Finally$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L154, 'Finally "finally"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Finally(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Finally", Finally$0);
}
var For$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L155, 'For "for"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function For(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "For", For$0);
}
var From$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L156, 'From "from"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function From(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "From", From$0);
}
var Function$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L157, 'Function "function"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Function2(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Function", Function$0);
}
var GetOrSet$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L158, 'GetOrSet "get"'), (0, import_lib3.$EXPECT)($L159, 'GetOrSet "set"')), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1, type: "GetOrSet" };
});
function GetOrSet(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "GetOrSet", GetOrSet$0);
}
var Hash$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L160, 'Hash "\#"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function Hash(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Hash", Hash$0);
}
var If$0 = (0, import_lib3.$TV)((0, import_lib3.$TEXT)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L161, 'If "if"'), NonIdContinue, (0, import_lib3.$E)((0, import_lib3.$EXPECT)($L18, 'If " "')))), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function If(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "If", If$0);
}
var Import$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L15, 'Import "import"'), (0, import_lib3.$Y)((0, import_lib3.$EXPECT)($R73, "Import /\\s/"))), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Import(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Import", Import$0);
}
var In$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L162, 'In "in"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function In(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "In", In$0);
}
var Infer$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L163, 'Infer "infer"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Infer(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Infer", Infer$0);
}
var LetOrConst$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L164, 'LetOrConst "let"'), (0, import_lib3.$EXPECT)($L165, 'LetOrConst "const"')), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function LetOrConst(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "LetOrConst", LetOrConst$0);
}
var Const$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L165, 'Const "const"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Const(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Const", Const$0);
}
var Is$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L166, 'Is "is"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Is(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Is", Is$0);
}
var LetOrConstOrVar$0 = LetOrConst;
var LetOrConstOrVar$1 = Var;
var LetOrConstOrVar$$ = [LetOrConstOrVar$0, LetOrConstOrVar$1];
function LetOrConstOrVar(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "LetOrConstOrVar", LetOrConstOrVar$$);
}
var Like$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L167, 'Like "like"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Like(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Like", Like$0);
}
var Loop$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L168, 'Loop "loop"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: "while" };
});
function Loop(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Loop", Loop$0);
}
var New$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L169, 'New "new"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function New(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "New", New$0);
}
var Not$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L170, 'Not "not"'), NonIdContinue, (0, import_lib3.$N)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$EXPECT)($L16, 'Not ":"')))), function($skip, $loc, $0, $1, $2, $3) {
  return { $loc, token: "!" };
});
function Not(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Not", Not$0);
}
var Of$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L171, 'Of "of"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Of(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Of", Of$0);
}
var OpenAngleBracket$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L19, 'OpenAngleBracket "<"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function OpenAngleBracket(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "OpenAngleBracket", OpenAngleBracket$0);
}
var OpenBrace$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L1, 'OpenBrace "{"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function OpenBrace(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "OpenBrace", OpenBrace$0);
}
var OpenBracket$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L172, 'OpenBracket "["'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function OpenBracket(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "OpenBracket", OpenBracket$0);
}
var OpenParen$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L4, 'OpenParen "("'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function OpenParen(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "OpenParen", OpenParen$0);
}
var Operator$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L173, 'Operator "operator"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Operator(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Operator", Operator$0);
}
var Override$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L174, 'Override "override"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1, ts: true };
});
function Override(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Override", Override$0);
}
var Own$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L175, 'Own "own"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Own(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Own", Own$0);
}
var Public$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L176, 'Public "public"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Public(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Public", Public$0);
}
var Private$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L177, 'Private "private"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Private(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Private", Private$0);
}
var Protected$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L178, 'Protected "protected"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Protected(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Protected", Protected$0);
}
var Pipe$0 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L179, 'Pipe "||>"'), (0, import_lib3.$EXPECT)($L180, 'Pipe "|\u25B7"')), function($skip, $loc, $0, $1) {
  return { $loc, token: "||>" };
});
var Pipe$1 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L181, 'Pipe "|>="'), (0, import_lib3.$EXPECT)($L182, 'Pipe "\u25B7="')), function($skip, $loc, $0, $1) {
  return { $loc, token: "|>=" };
});
var Pipe$2 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L183, 'Pipe "|>"'), (0, import_lib3.$EXPECT)($L184, 'Pipe "\u25B7"')), function($skip, $loc, $0, $1) {
  return { $loc, token: "|>" };
});
var Pipe$$ = [Pipe$0, Pipe$1, Pipe$2];
function Pipe(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Pipe", Pipe$$);
}
var QuestionMark$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L6, 'QuestionMark "?"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function QuestionMark(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "QuestionMark", QuestionMark$0);
}
var Readonly$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L185, 'Readonly "readonly"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1, ts: true };
});
function Readonly(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Readonly", Readonly$0);
}
var Return$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L186, 'Return "return"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Return(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Return", Return$0);
}
var Satisfies$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L187, 'Satisfies "satisfies"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Satisfies(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Satisfies", Satisfies$0);
}
var Semicolon$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L111, 'Semicolon ";"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function Semicolon(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Semicolon", Semicolon$0);
}
var SingleQuote$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L188, `SingleQuote "'"`), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function SingleQuote(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SingleQuote", SingleQuote$0);
}
var Star$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L64, 'Star "*"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function Star(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Star", Star$0);
}
var Static$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L189, 'Static "static"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
var Static$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L128, 'Static "@"'), (0, import_lib3.$N)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L4, 'Static "("'), (0, import_lib3.$EXPECT)($L128, 'Static "@"')))), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: "static " };
});
var Static$$ = [Static$0, Static$1];
function Static(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "Static", Static$$);
}
var SubstitutionStart$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L190, 'SubstitutionStart "${"'), function($skip, $loc, $0, $1) {
  return { $loc, token: $1 };
});
function SubstitutionStart(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "SubstitutionStart", SubstitutionStart$0);
}
var Super$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L191, 'Super "super"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Super(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Super", Super$0);
}
var Switch$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L192, 'Switch "switch"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Switch(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Switch", Switch$0);
}
var Target$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L193, 'Target "target"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Target(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Target", Target$0);
}
var Then$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L194, 'Then "then"'), NonIdContinue), function($skip, $loc, $0, $1, $2, $3) {
  return { $loc, token: "" };
});
function Then(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Then", Then$0);
}
var This$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L195, 'This "this"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function This(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "This", This$0);
}
var Throw$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L196, 'Throw "throw"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Throw(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Throw", Throw$0);
}
var TripleDoubleQuote$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L197, 'TripleDoubleQuote "\\\\\\"\\\\\\"\\\\\\""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "`" };
});
function TripleDoubleQuote(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TripleDoubleQuote", TripleDoubleQuote$0);
}
var TripleSingleQuote$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L198, `TripleSingleQuote "'''"`), function($skip, $loc, $0, $1) {
  return { $loc, token: "`" };
});
function TripleSingleQuote(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TripleSingleQuote", TripleSingleQuote$0);
}
var TripleSlash$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L199, 'TripleSlash "///"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "/" };
});
function TripleSlash(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TripleSlash", TripleSlash$0);
}
var TripleTick$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L200, 'TripleTick "```"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "`" };
});
function TripleTick(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TripleTick", TripleTick$0);
}
var Try$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L201, 'Try "try"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Try(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Try", Try$0);
}
var Typeof$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L202, 'Typeof "typeof"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Typeof(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Typeof", Typeof$0);
}
var Undefined$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L203, 'Undefined "undefined"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Undefined(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Undefined", Undefined$0);
}
var Unless$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L204, 'Unless "unless"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1, negated: true };
});
function Unless(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Unless", Unless$0);
}
var Until$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L205, 'Until "until"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1, negated: true };
});
function Until(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Until", Until$0);
}
var Using$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L206, 'Using "using"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Using(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Using", Using$0);
}
var Var$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L207, 'Var "var"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Var(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Var", Var$0);
}
var Void$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L208, 'Void "void"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Void(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Void", Void$0);
}
var When$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L209, 'When "when"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: "case" };
});
function When(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "When", When$0);
}
var While$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L210, 'While "while"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function While(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "While", While$0);
}
var With$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L116, 'With "with"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function With(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "With", With$0);
}
var Yield$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L211, 'Yield "yield"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1, type: "Yield" };
});
function Yield(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Yield", Yield$0);
}
var JSXImplicitFragment$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(JSXTag, (0, import_lib3.$Q)((0, import_lib3.$S)(Nested, JSXTag))), function($skip, $loc, $0, $1, $2) {
  const jsx = $2.length === 0 ? $1 : {
    type: "JSXFragment",
    children: [
      "<>\n",
      state.currentIndent.token,
      ...$0,
      "\n",
      state.currentIndent.token,
      "</>"
    ],
    jsxChildren: [$1].concat($2.map(([, tag]) => tag))
  };
  const type = typeOfJSX(jsx, config, getHelperRef);
  return type ? [
    { ts: true, children: ["("] },
    jsx,
    { ts: true, children: [" as any as ", type, ")"] }
  ] : jsx;
});
function JSXImplicitFragment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXImplicitFragment", JSXImplicitFragment$0);
}
var JSXTag$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R74, "JSXTag /(?=[<])/"), _JSXTag), function(value) {
  return value[1];
});
function JSXTag(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXTag", JSXTag$0);
}
var _JSXTag$0 = JSXElement;
var _JSXTag$1 = JSXFragment;
var _JSXTag$2 = JSXComment;
var _JSXTag$$ = [_JSXTag$0, _JSXTag$1, _JSXTag$2];
function _JSXTag(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "_JSXTag", _JSXTag$$);
}
var JSXElement$0 = JSXSelfClosingElement;
var JSXElement$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(CoffeeJSXEnabled), PushJSXOpeningElement, (0, import_lib3.$E)(JSXMixedChildren), JSXOptionalClosingElement, PopJSXStack), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var open = $2;
  var children = $3;
  var close = $4;
  if (!children)
    return $skip;
  let parts;
  $0 = $0.slice(1);
  if (close) {
    parts = $0;
  } else if (children.jsxChildren.length) {
    parts = [
      ...$0,
      "\n",
      // InsertNewline
      state.currentIndent.token,
      // InsertIndent
      ["</", open[1], ">"]
    ];
  } else {
    parts = [open.slice(0, -1), " />"];
  }
  return { type: "JSXElement", children: parts, tag: open[1] };
});
var JSXElement$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeeJSXEnabled, JSXOpeningElement, (0, import_lib3.$E)(JSXChildren), (0, import_lib3.$E)(Whitespace), JSXClosingElement), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var open = $2;
  var close = $5;
  $0 = $0.slice(1);
  if (open[1] !== close[2])
    return $skip;
  return { type: "JSXElement", children: $0, tag: open[1] };
});
var JSXElement$$ = [JSXElement$0, JSXElement$1, JSXElement$2];
function JSXElement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXElement", JSXElement$$);
}
var JSXSelfClosingElement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L19, 'JSXSelfClosingElement "<"'), JSXElementName, (0, import_lib3.$E)(TypeArguments), (0, import_lib3.$E)(JSXAttributes), (0, import_lib3.$E)(Whitespace), (0, import_lib3.$EXPECT)($L212, 'JSXSelfClosingElement "/>"')), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  return { type: "JSXElement", children: $0, tag: $2 };
});
function JSXSelfClosingElement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXSelfClosingElement", JSXSelfClosingElement$0);
}
var PushJSXOpeningElement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(JSXOpeningElement), function($skip, $loc, $0, $1) {
  state.JSXTagStack.push($1[1]);
  return $1;
});
function PushJSXOpeningElement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PushJSXOpeningElement", PushJSXOpeningElement$0);
}
var PopJSXStack$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'PopJSXStack ""'), function($skip, $loc, $0, $1) {
  state.JSXTagStack.pop();
});
function PopJSXStack(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PopJSXStack", PopJSXStack$0);
}
var JSXOpeningElement$0 = (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L19, 'JSXOpeningElement "<"'), JSXElementName, (0, import_lib3.$E)(TypeArguments), (0, import_lib3.$E)(JSXAttributes), (0, import_lib3.$E)(Whitespace), (0, import_lib3.$EXPECT)($L44, 'JSXOpeningElement ">"'));
function JSXOpeningElement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXOpeningElement", JSXOpeningElement$0);
}
var JSXOptionalClosingElement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(Whitespace), JSXClosingElement), function($skip, $loc, $0, $1, $2) {
  var close = $2;
  if (state.currentJSXTag !== close[2])
    return $skip;
  return $0;
});
var JSXOptionalClosingElement$1 = (0, import_lib3.$EXPECT)($L0, 'JSXOptionalClosingElement ""');
var JSXOptionalClosingElement$$ = [JSXOptionalClosingElement$0, JSXOptionalClosingElement$1];
function JSXOptionalClosingElement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXOptionalClosingElement", JSXOptionalClosingElement$$);
}
var JSXClosingElement$0 = (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L213, 'JSXClosingElement "</"'), (0, import_lib3.$E)(Whitespace), JSXElementName, (0, import_lib3.$E)(Whitespace), (0, import_lib3.$EXPECT)($L44, 'JSXClosingElement ">"'));
function JSXClosingElement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXClosingElement", JSXClosingElement$0);
}
var JSXFragment$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)(CoffeeJSXEnabled), PushJSXOpeningFragment, (0, import_lib3.$E)(JSXMixedChildren), JSXOptionalClosingFragment, PopJSXStack), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var open = $2;
  var children = $3;
  var close = $4;
  if (!children)
    return $skip;
  $0 = $0.slice(1);
  const parts = close ? $0 : [
    ...$0,
    "\n",
    // InsertNewline
    state.currentIndent.token,
    // InsertIndent
    "</>"
  ];
  return { type: "JSXFragment", children: parts, jsxChildren: children.jsxChildren };
});
var JSXFragment$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeeJSXEnabled, (0, import_lib3.$EXPECT)($L214, 'JSXFragment "<>"'), (0, import_lib3.$E)(JSXChildren), (0, import_lib3.$E)(Whitespace), JSXClosingFragment), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var children = $3;
  $0 = $0.slice(1);
  return {
    type: "JSXFragment",
    children: $0,
    jsxChildren: children ? children.jsxChildren : []
  };
});
var JSXFragment$$ = [JSXFragment$0, JSXFragment$1];
function JSXFragment(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXFragment", JSXFragment$$);
}
var PushJSXOpeningFragment$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L214, 'PushJSXOpeningFragment "<>"'), function($skip, $loc, $0, $1) {
  state.JSXTagStack.push("");
  return $1;
});
function PushJSXOpeningFragment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PushJSXOpeningFragment", PushJSXOpeningFragment$0);
}
var JSXOptionalClosingFragment$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(Whitespace), JSXClosingFragment), function($skip, $loc, $0, $1, $2) {
  if (state.currentJSXTag !== "")
    return $skip;
  return $0;
});
var JSXOptionalClosingFragment$1 = (0, import_lib3.$EXPECT)($L0, 'JSXOptionalClosingFragment ""');
var JSXOptionalClosingFragment$$ = [JSXOptionalClosingFragment$0, JSXOptionalClosingFragment$1];
function JSXOptionalClosingFragment(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXOptionalClosingFragment", JSXOptionalClosingFragment$$);
}
var JSXClosingFragment$0 = (0, import_lib3.$EXPECT)($L215, 'JSXClosingFragment "</>"');
function JSXClosingFragment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXClosingFragment", JSXClosingFragment$0);
}
var JSXElementName$0 = (0, import_lib3.$TV)((0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L160, 'JSXElementName "\#"'), Dot), JSXShorthandString)), function($skip, $loc, $0, $1) {
  return config.defaultElement;
});
var JSXElementName$1 = (0, import_lib3.$TEXT)((0, import_lib3.$S)(JSXIdentifierName, (0, import_lib3.$C)((0, import_lib3.$S)(Colon, JSXIdentifierName), (0, import_lib3.$Q)((0, import_lib3.$S)(Dot, JSXIdentifierName)))));
var JSXElementName$$ = [JSXElementName$0, JSXElementName$1];
function JSXElementName(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXElementName", JSXElementName$$);
}
var JSXIdentifierName$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R75, "JSXIdentifierName /(?:\\p{ID_Start}|[_$])(?:\\p{ID_Continue}|[\\u200C\\u200D$-])*/"));
function JSXIdentifierName(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXIdentifierName", JSXIdentifierName$0);
}
var JSXAttributes$0 = (0, import_lib3.$TV)((0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$E)(Whitespace), JSXAttribute)), function($skip, $loc, $0, $1) {
  const classes = [];
  let attrs = $0.filter((pair) => {
    const [, attr] = pair;
    if (attr.type === "JSXClass") {
      classes.push(attr.class);
      return false;
    }
    return true;
  });
  if (classes.length) {
    let isBraced2 = function(c) {
      return c[0] === "{" || c[0]?.token === "{";
    }, unbrace2 = function(c) {
      return c.slice(1, -1);
    }, parseClass2 = function(c) {
      c = c.token || c;
      if (c.startsWith("'")) {
        c = '"' + c.slice(1, -1).replace(/\\*"/g, (m) => m.length % 2 == 0 ? m : "\\" + m) + '"';
      }
      return JSON.parse(c);
    };
    var isBraced = isBraced2, unbrace = unbrace2, parseClass = parseClass2;
    let className = config.react ? "className" : "class";
    attrs = attrs.filter((pair) => {
      const [, attr] = pair;
      if ((attr[0][0] === "class" || attr[0][0] === "className") && !attr[0][1]) {
        className = attr[0][0];
        classes.push(attr[1][attr[1].length - 1]);
        return false;
      }
      return true;
    });
    const strings = [], exprs = [];
    classes.forEach((c) => {
      if (isBraced2(c)) {
        exprs.push(unbrace2(c));
        exprs.push(", ");
      } else {
        strings.push(parseClass2(c));
      }
    });
    const stringPart = strings.filter(Boolean).join(" ");
    let classValue;
    if (exprs.length) {
      exprs.pop();
      if (stringPart) {
        exprs.unshift(JSON.stringify(stringPart), ", ");
      }
      if (exprs.length === 1) {
        let root = exprs[0];
        while (root.length && isWhitespaceOrEmpty(root[root.length - 1])) {
          root = root.slice(0, -1);
        }
        while (root?.length === 1)
          root = root[0];
        if (root?.children)
          root = root.children;
        if (root?.[0]?.token === "`") {
          classValue = ["{", ...exprs, "}"];
        } else {
          classValue = ["{(", ...exprs, ') || ""}'];
        }
      } else {
        classValue = ["{[", ...exprs, '].filter(Boolean).join(" ")}'];
      }
    } else {
      if (!stringPart.includes("&") && !stringPart.includes('"')) {
        classValue = `"${stringPart}"`;
      } else if (!stringPart.includes("&") && !stringPart.includes("'")) {
        classValue = `'${stringPart}'`;
      } else {
        classValue = `{${JSON.stringify(stringPart)}}`;
      }
    }
    attrs.splice(0, 0, [" ", [className, ["=", classValue]]]);
  }
  return attrs.map((pair) => {
    const [space, attr] = pair;
    if (space && attr[0] === " ") {
      pair = [space, attr.slice(1)];
    }
    return pair;
  });
});
function JSXAttributes(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXAttributes", JSXAttributes$0);
}
var JSXAttribute$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(BracedObjectLiteral), function($skip, $loc, $0, $1) {
  return convertObjectToJSXAttributes($1);
});
var JSXAttribute$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(JSXAttributeName, (0, import_lib3.$C)(JSXAttributeInitializer, (0, import_lib3.$Y)(JSXAttributeSpace))), function($skip, $loc, $0, $1, $2) {
  var name = $1;
  var value = $2;
  if (name.type === "ComputedPropertyName") {
    if (value) {
      value = value[value.length - 1];
      if (value[0]?.token === "{" && value[value.length - 1]?.token === "}") {
        value = value.slice(1, -1);
      }
    } else {
      value = "true";
    }
    return ["{...{", name, ": ", value, "}}"];
  } else {
    return $0;
  }
});
var JSXAttribute$2 = (0, import_lib3.$S)(InsertInlineOpenBrace, DotDotDot, InlineJSXAttributeValue, InsertCloseBrace, (0, import_lib3.$Y)(JSXAttributeSpace));
var JSXAttribute$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(AtThis, (0, import_lib3.$E)(Identifier), (0, import_lib3.$Q)(InlineJSXCallExpressionRest), (0, import_lib3.$Y)(JSXAttributeSpace)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var at = $1;
  var id = $2;
  var rest = $3;
  const access = id && {
    type: "PropertyAccess",
    children: [".", id],
    name: id
  };
  const expr = processCallMemberExpression({
    type: "CallExpression",
    children: [at, access, ...rest.flat()]
  });
  const last = lastAccessInCallExpression(expr);
  if (!last)
    return $skip;
  let name;
  if (last.type === "Index") {
    return [
      "{...{",
      { ...last, type: "ComputedPropertyName" },
      ": ",
      expr,
      "}}"
    ];
  } else if (last.name) {
    return [last.name, "={", expr, "}"];
  }
  return $skip;
});
var JSXAttribute$4 = (0, import_lib3.$TS)((0, import_lib3.$S)(Identifier, (0, import_lib3.$P)(InlineJSXCallExpressionRest), (0, import_lib3.$Y)(JSXAttributeSpace)), function($skip, $loc, $0, $1, $2, $3) {
  var id = $1;
  var rest = $2;
  const expr = processCallMemberExpression({
    type: "CallExpression",
    children: [id, ...rest.flat()]
  });
  if (expr.type === "ObjectExpression") {
    return convertObjectToJSXAttributes(expr);
  }
  const last = lastAccessInCallExpression(expr);
  if (!last)
    return $skip;
  let name;
  if (last.type === "Index") {
    return [
      "{...{",
      { ...last, type: "ComputedPropertyName" },
      ": ",
      expr,
      "}}"
    ];
  } else if (last.name) {
    return [last.name, "={", expr, "}"];
  }
  return $skip;
});
var JSXAttribute$5 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L160, 'JSXAttribute "\#"'), JSXShorthandString), function($skip, $loc, $0, $1, $2) {
  return [" ", "id=", $2];
});
var JSXAttribute$6 = (0, import_lib3.$TS)((0, import_lib3.$S)(Dot, JSXShorthandString), function($skip, $loc, $0, $1, $2) {
  return {
    type: "JSXClass",
    class: $2
  };
});
var JSXAttribute$7 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$TEXT)((0, import_lib3.$EXPECT)($R76, "JSXAttribute /[!+-]/")), JSXAttributeName, (0, import_lib3.$Y)(JSXAttributeSpace)), function($skip, $loc, $0, $1, $2, $3) {
  var toggle = $1;
  var id = $2;
  const value = toggle === "+" ? "true" : "false";
  return [" ", id, "={", value, "}"];
});
var JSXAttribute$$ = [JSXAttribute$0, JSXAttribute$1, JSXAttribute$2, JSXAttribute$3, JSXAttribute$4, JSXAttribute$5, JSXAttribute$6, JSXAttribute$7];
function JSXAttribute(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXAttribute", JSXAttribute$$);
}
var JSXAttributeSpace$0 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R77, "JSXAttributeSpace /[\\s>]|\\/>/"));
function JSXAttributeSpace(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXAttributeSpace", JSXAttributeSpace$0);
}
var JSXShorthandString$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R78, "JSXShorthandString /(?:[\\w\\-:]+|\\([^()]*\\)|\\[[^\\[\\]]*\\])+/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return quoteString($0);
});
var JSXShorthandString$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(TemplateLiteral), function($skip, $loc, $0, $1) {
  return ["{", $1, "}"];
});
var JSXShorthandString$2 = StringLiteral;
var JSXShorthandString$3 = (0, import_lib3.$S)(OpenBrace, PostfixedExpression, (0, import_lib3.$E)(Whitespace), CloseBrace);
var JSXShorthandString$$ = [JSXShorthandString$0, JSXShorthandString$1, JSXShorthandString$2, JSXShorthandString$3];
function JSXShorthandString(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXShorthandString", JSXShorthandString$$);
}
var JSXAttributeName$0 = (0, import_lib3.$S)(JSXIdentifierName, (0, import_lib3.$E)((0, import_lib3.$S)(Colon, JSXIdentifierName)));
var JSXAttributeName$1 = ComputedPropertyName;
var JSXAttributeName$$ = [JSXAttributeName$0, JSXAttributeName$1];
function JSXAttributeName(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXAttributeName", JSXAttributeName$$);
}
var JSXAttributeInitializer$0 = (0, import_lib3.$S)((0, import_lib3.$E)(Whitespace), Equals, (0, import_lib3.$E)(Whitespace), JSXAttributeValue);
function JSXAttributeInitializer(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXAttributeInitializer", JSXAttributeInitializer$0);
}
var JSXAttributeValue$0 = (0, import_lib3.$S)(OpenBrace, PostfixedExpression, (0, import_lib3.$E)(Whitespace), CloseBrace);
var JSXAttributeValue$1 = JSXElement;
var JSXAttributeValue$2 = JSXFragment;
var JSXAttributeValue$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertInlineOpenBrace, InlineJSXAttributeValue, InsertCloseBrace, (0, import_lib3.$Y)(JSXAttributeSpace)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var open = $1;
  var value = $2;
  var close = $3;
  if (value.type === "StringLiteral") {
    return $skip;
  }
  return [open, value, close];
});
var JSXAttributeValue$4 = (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R79, `JSXAttributeValue /"[^"]*"|'[^']*'/`));
var JSXAttributeValue$$ = [JSXAttributeValue$0, JSXAttributeValue$1, JSXAttributeValue$2, JSXAttributeValue$3, JSXAttributeValue$4];
function JSXAttributeValue(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXAttributeValue", JSXAttributeValue$$);
}
var InlineJSXAttributeValue$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(InlineJSXUnaryExpression, (0, import_lib3.$Q)(InlineJSXBinaryOpRHS)), function($skip, $loc, $0, $1, $2) {
  if ($2.length)
    return processBinaryOpExpression($0);
  return $1;
});
function InlineJSXAttributeValue(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineJSXAttributeValue", InlineJSXAttributeValue$0);
}
var InlineJSXBinaryOpRHS$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$N)((0, import_lib3.$EXPECT)($R80, "InlineJSXBinaryOpRHS /[<>]/")), BinaryOp, InlineJSXUnaryExpression), function($skip, $loc, $0, $1, $2, $3) {
  var op = $2;
  var rhs = $3;
  return [[], op, [], rhs];
});
function InlineJSXBinaryOpRHS(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineJSXBinaryOpRHS", InlineJSXBinaryOpRHS$0);
}
var InlineJSXUnaryExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)(InlineJSXUnaryOp), InlineJSXUpdateExpression, (0, import_lib3.$E)(InlineJSXUnaryPostfix)), function($skip, $loc, $0, $1, $2, $3) {
  var pre = $1;
  var exp = $2;
  var post = $3;
  return processUnaryExpression(pre, exp, post);
});
function InlineJSXUnaryExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineJSXUnaryExpression", InlineJSXUnaryExpression$0);
}
var InlineJSXUnaryOp$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R81, "InlineJSXUnaryOp /[!~+-](?!\\s|[!~+-]*&)/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function InlineJSXUnaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineJSXUnaryOp", InlineJSXUnaryOp$0);
}
var InlineJSXUnaryPostfix$0 = QuestionMark;
function InlineJSXUnaryPostfix(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineJSXUnaryPostfix", InlineJSXUnaryPostfix$0);
}
var InlineJSXUpdateExpression$0 = (0, import_lib3.$S)(UpdateExpressionSymbol, UnaryExpression);
var InlineJSXUpdateExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InlineJSXCallExpression, (0, import_lib3.$E)(UpdateExpressionSymbol)), function($skip, $loc, $0, $1, $2) {
  if ($2)
    return $0;
  return $1;
});
var InlineJSXUpdateExpression$$ = [InlineJSXUpdateExpression$0, InlineJSXUpdateExpression$1];
function InlineJSXUpdateExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "InlineJSXUpdateExpression", InlineJSXUpdateExpression$$);
}
var InlineJSXCallExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Super, ExplicitArguments, (0, import_lib3.$Q)(InlineJSXCallExpressionRest)), function($skip, $loc, $0, $1, $2, $3) {
  var args = $2;
  var rest = $3;
  return processCallMemberExpression({
    type: "CallExpression",
    children: [
      $1,
      args,
      ...rest.flat()
    ]
  });
});
var InlineJSXCallExpression$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L15, 'InlineJSXCallExpression "import"'), ExplicitArguments, (0, import_lib3.$Q)(InlineJSXCallExpressionRest)), function($skip, $loc, $0, $1, $2, $3) {
  var args = $2;
  var rest = $3;
  return processCallMemberExpression({
    type: "CallExpression",
    children: [
      $1,
      args,
      ...rest.flat()
    ]
  });
});
var InlineJSXCallExpression$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(InlineJSXMemberExpression, (0, import_lib3.$Q)(InlineJSXCallExpressionRest)), function($skip, $loc, $0, $1, $2) {
  var member = $1;
  var rest = $2;
  if (rest.length) {
    rest = rest.flat();
    return processCallMemberExpression({
      type: "CallExpression",
      children: [member, ...rest]
    });
  }
  return member;
});
var InlineJSXCallExpression$$ = [InlineJSXCallExpression$0, InlineJSXCallExpression$1, InlineJSXCallExpression$2];
function InlineJSXCallExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "InlineJSXCallExpression", InlineJSXCallExpression$$);
}
var InlineJSXCallExpressionRest$0 = InlineJSXMemberExpressionRest;
var InlineJSXCallExpressionRest$1 = (0, import_lib3.$TV)((0, import_lib3.$C)(TemplateLiteral, StringLiteral), function($skip, $loc, $0, $1) {
  if ($1.type === "StringLiteral") {
    return "`" + $1.token.slice(1, -1).replace(/(`|\$\{)/g, "\\$1") + "`";
  }
  return $1;
});
var InlineJSXCallExpressionRest$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(OptionalShorthand), ExplicitArguments), function($skip, $loc, $0, $1, $2) {
  var args = $2;
  if (!$1)
    return args;
  return [$1, args];
});
var InlineJSXCallExpressionRest$$ = [InlineJSXCallExpressionRest$0, InlineJSXCallExpressionRest$1, InlineJSXCallExpressionRest$2];
function InlineJSXCallExpressionRest(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "InlineJSXCallExpressionRest", InlineJSXCallExpressionRest$$);
}
var InlineJSXMemberExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$C)(InlineJSXPrimaryExpression, SuperProperty, MetaProperty), (0, import_lib3.$Q)(InlineJSXMemberExpressionRest)), function($skip, $loc, $0, $1, $2) {
  var rest = $2;
  if (rest.length || Array.isArray($1)) {
    return processCallMemberExpression({
      type: "MemberExpression",
      children: [$1, ...rest].flat()
    });
  }
  return $1;
});
function InlineJSXMemberExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineJSXMemberExpression", InlineJSXMemberExpression$0);
}
var InlineJSXMemberExpressionRest$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(OptionalShorthand), (0, import_lib3.$Q)(InlineComment), MemberBracketContent), function($skip, $loc, $0, $1, $2, $3) {
  var dot = $1;
  var comments = $2;
  var content = $3;
  if (!dot && !comments.length)
    return content;
  if (dot) {
    if (dot.type === "Optional" && content.type === "SliceExpression") {
      return [...dot.children.slice(0, -1), ...comments, content];
    }
    return [dot, ...comments, content];
  }
  return [...comments, content];
});
var InlineJSXMemberExpressionRest$1 = PropertyAccess;
var InlineJSXMemberExpressionRest$2 = PropertyGlob;
var InlineJSXMemberExpressionRest$3 = PropertyBind;
var InlineJSXMemberExpressionRest$4 = NonNullAssertion;
var InlineJSXMemberExpressionRest$$ = [InlineJSXMemberExpressionRest$0, InlineJSXMemberExpressionRest$1, InlineJSXMemberExpressionRest$2, InlineJSXMemberExpressionRest$3, InlineJSXMemberExpressionRest$4];
function InlineJSXMemberExpressionRest(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "InlineJSXMemberExpressionRest", InlineJSXMemberExpressionRest$$);
}
var InlineJSXPrimaryExpression$0 = NullLiteral;
var InlineJSXPrimaryExpression$1 = BooleanLiteral;
var InlineJSXPrimaryExpression$2 = NumericLiteral;
var InlineJSXPrimaryExpression$3 = TemplateLiteral;
var InlineJSXPrimaryExpression$4 = ThisLiteral;
var InlineJSXPrimaryExpression$5 = ArrayLiteral;
var InlineJSXPrimaryExpression$6 = BracedObjectLiteral;
var InlineJSXPrimaryExpression$7 = IdentifierReference;
var InlineJSXPrimaryExpression$8 = RegularExpressionLiteral;
var InlineJSXPrimaryExpression$9 = ParenthesizedExpression;
var InlineJSXPrimaryExpression$$ = [InlineJSXPrimaryExpression$0, InlineJSXPrimaryExpression$1, InlineJSXPrimaryExpression$2, InlineJSXPrimaryExpression$3, InlineJSXPrimaryExpression$4, InlineJSXPrimaryExpression$5, InlineJSXPrimaryExpression$6, InlineJSXPrimaryExpression$7, InlineJSXPrimaryExpression$8, InlineJSXPrimaryExpression$9];
function InlineJSXPrimaryExpression(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "InlineJSXPrimaryExpression", InlineJSXPrimaryExpression$$);
}
var JSXMixedChildren$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)(JSXChild), JSXNestedChildren), function($skip, $loc, $0, $1, $2) {
  var c1 = $1;
  var c2 = $2;
  return {
    children: c1.concat(c2),
    jsxChildren: c1.concat(c2.jsxChildren)
  };
});
function JSXMixedChildren(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXMixedChildren", JSXMixedChildren$0);
}
var JSXChildren$0 = (0, import_lib3.$TV)((0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$E)(NonNewlineWhitespace), EOL, (0, import_lib3.$E)(NonNewlineWhitespace))), JSXChild)), function($skip, $loc, $0, $1) {
  return {
    children: $1,
    jsxChildren: $1.map((children) => children[1])
  };
});
function JSXChildren(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXChildren", JSXChildren$0);
}
var JSXNestedChildren$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)((0, import_lib3.$S)(JSXNested, (0, import_lib3.$P)(JSXChild))), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  if ($2.length) {
    return {
      children: $2,
      jsxChildren: [].concat(...$2.map((nestedChildren) => nestedChildren[1]))
    };
  }
  return $skip;
});
var JSXNestedChildren$1 = (0, import_lib3.$TV)((0, import_lib3.$Y)((0, import_lib3.$C)(JSXEOS, (0, import_lib3.$EXPECT)($L36, 'JSXNestedChildren "}"'), JSXClosingElement, JSXClosingFragment)), function($skip, $loc, $0, $1) {
  return { children: [], jsxChildren: [] };
});
var JSXNestedChildren$$ = [JSXNestedChildren$0, JSXNestedChildren$1];
function JSXNestedChildren(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXNestedChildren", JSXNestedChildren$$);
}
var JSXEOS$0 = (0, import_lib3.$P)((0, import_lib3.$S)((0, import_lib3.$E)(NonNewlineWhitespace), EOL));
function JSXEOS(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXEOS", JSXEOS$0);
}
var JSXNested$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(JSXEOS, Indent), function($skip, $loc, $0, $1, $2) {
  var eos = $1;
  var indent = $2;
  const { level } = indent;
  const currentIndent = state.currentIndent;
  if (level !== currentIndent.level)
    return $skip;
  return $0;
});
function JSXNested(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXNested", JSXNested$0);
}
var JSXChild$0 = JSXElement;
var JSXChild$1 = JSXFragment;
var JSXChild$2 = JSXComment;
var JSXChild$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenBrace, IndentedJSXChildExpression, __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4) {
  var expression = $2;
  return {
    type: "JSXChildExpression",
    children: $0,
    expression
  };
});
var JSXChild$4 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenBrace, (0, import_lib3.$E)(JSXChildExpression), __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4) {
  var expression = $2;
  return {
    type: "JSXChildExpression",
    children: $0,
    expression
  };
});
var JSXChild$5 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertInlineOpenBrace, ArrowFunction, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3) {
  var expression = $2;
  return {
    type: "JSXChildExpression",
    children: $0,
    expression
  };
});
var JSXChild$6 = JSXText;
var JSXChild$$ = [JSXChild$0, JSXChild$1, JSXChild$2, JSXChild$3, JSXChild$4, JSXChild$5, JSXChild$6];
function JSXChild(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "JSXChild", JSXChild$$);
}
var JSXComment$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L216, 'JSXComment "<!--"'), JSXCommentContent, (0, import_lib3.$EXPECT)($L217, 'JSXComment "-->"')), function($skip, $loc, $0, $1, $2, $3) {
  return ["{/*", $2, "*/}"];
});
function JSXComment(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXComment", JSXComment$0);
}
var JSXCommentContent$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R82, "JSXCommentContent /(?:-[^-]|[^-]*)*/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0.replace(/\*\//g, "* /") };
});
function JSXCommentContent(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXCommentContent", JSXCommentContent$0);
}
var JSXText$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R83, "JSXText /[^{}<>\\r\\n]+/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return {
    type: "JSXText",
    token: $0,
    $loc
  };
});
function JSXText(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXText", JSXText$0);
}
var JSXChildExpression$0 = (0, import_lib3.$S)((0, import_lib3.$E)(Whitespace), (0, import_lib3.$E)((0, import_lib3.$S)(DotDotDot, (0, import_lib3.$E)(Whitespace))), PostfixedExpression);
function JSXChildExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "JSXChildExpression", JSXChildExpression$0);
}
var IndentedJSXChildExpression$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$E)(NestedJSXChildExpression), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  if (!$2)
    return $skip;
  return $2;
});
function IndentedJSXChildExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IndentedJSXChildExpression", IndentedJSXChildExpression$0);
}
var NestedJSXChildExpression$0 = (0, import_lib3.$S)(JSXNested, JSXChildExpression);
function NestedJSXChildExpression(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedJSXChildExpression", NestedJSXChildExpression$0);
}
var UsingDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Using, (0, import_lib3.$E)(_), UsingBinding, (0, import_lib3.$Q)((0, import_lib3.$S)(__, Comma, __, UsingBinding)), UsingJSModeError), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var decl = $1;
  var binding = $3;
  var tail = $4;
  const bindings = [binding].concat(tail.map(([, , , b]) => b));
  return {
    type: "Declaration",
    children: $0,
    names: bindings.flatMap((b) => b.names),
    bindings,
    decl,
    splices: bindings.flatMap((b) => b.splices),
    thisAssignments: bindings.flatMap((b) => b.thisAssignments)
  };
});
function UsingDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "UsingDeclaration", UsingDeclaration$0);
}
var UsingBinding$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(BindingIdentifier, (0, import_lib3.$E)(TypeSuffix), Initializer), function($skip, $loc, $0, $1, $2, $3) {
  var pattern = $1;
  var suffix = $2;
  var initializer = $3;
  return {
    type: "Binding",
    children: $0,
    names: pattern.names,
    pattern,
    suffix,
    initializer,
    splices: [],
    thisAssignments: []
  };
});
function UsingBinding(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "UsingBinding", UsingBinding$0);
}
var UsingJSModeError$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'UsingJSModeError ""'), function($skip, $loc, $0, $1) {
  return {
    type: "Error",
    js: true,
    message: "`using` is not currently transpiled in JS mode."
  };
});
function UsingJSModeError(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "UsingJSModeError", UsingJSModeError$0);
}
var TypeDeclaration$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Export, (0, import_lib3.$E)(_))), (0, import_lib3.$S)(Declare, (0, import_lib3.$E)(_)), TypeLexicalDeclaration), function(value) {
  return { "ts": true, "children": value };
});
var TypeDeclaration$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Export, (0, import_lib3.$E)(_))), (0, import_lib3.$E)((0, import_lib3.$S)(Declare, (0, import_lib3.$E)(_))), TypeDeclarationRest), function($skip, $loc, $0, $1, $2, $3) {
  var export_ = $1;
  var declare = $2;
  var t = $3;
  return {
    ...t,
    ts: true,
    export: export_,
    declare,
    children: [export_, declare, ...t.children]
  };
});
var TypeDeclaration$$ = [TypeDeclaration$0, TypeDeclaration$1];
function TypeDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeDeclaration", TypeDeclaration$$);
}
var TypeDeclarationRest$0 = TypeAliasDeclaration;
var TypeDeclarationRest$1 = InterfaceDeclaration;
var TypeDeclarationRest$2 = NamespaceDeclaration;
var TypeDeclarationRest$3 = FunctionSignature;
var TypeDeclarationRest$$ = [TypeDeclarationRest$0, TypeDeclarationRest$1, TypeDeclarationRest$2, TypeDeclarationRest$3];
function TypeDeclarationRest(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeDeclarationRest", TypeDeclarationRest$$);
}
var TypeAliasDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(TypeKeyword, (0, import_lib3.$E)(_), IdentifierName, (0, import_lib3.$E)(TypeParameters), OptionalEquals, (0, import_lib3.$C)(MaybeNestedType, (0, import_lib3.$S)(__, Type))), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var id = $3;
  return {
    type: "TypeDeclaration",
    id,
    children: $0,
    ts: true
  };
});
var TypeAliasDeclaration$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertType, IdentifierName, (0, import_lib3.$E)(TypeParameters), __, TypeAssignment, (0, import_lib3.$C)(MaybeNestedType, (0, import_lib3.$S)(__, Type))), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var id = $2;
  return {
    type: "TypeDeclaration",
    id,
    children: $0,
    ts: true
  };
});
var TypeAliasDeclaration$$ = [TypeAliasDeclaration$0, TypeAliasDeclaration$1];
function TypeAliasDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeAliasDeclaration", TypeAliasDeclaration$$);
}
var InterfaceDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Interface, (0, import_lib3.$E)(_), IdentifierName, (0, import_lib3.$E)(TypeParameters), (0, import_lib3.$E)(InterfaceExtendsClause), InterfaceBlock), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var id = $3;
  return {
    type: "InterfaceDeclaration",
    id,
    children: $0,
    ts: true
  };
});
function InterfaceDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InterfaceDeclaration", InterfaceDeclaration$0);
}
var NamespaceDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Namespace, (0, import_lib3.$E)(_), IdentifierName, ModuleBlock), function($skip, $loc, $0, $1, $2, $3, $4) {
  var id = $3;
  return {
    type: "NamespaceDeclaration",
    id,
    children: $0,
    ts: true
  };
});
function NamespaceDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NamespaceDeclaration", NamespaceDeclaration$0);
}
var OptionalEquals$0 = (0, import_lib3.$S)(__, Equals);
var OptionalEquals$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)(IndentedFurther), InsertSpaceEquals), function(value) {
  return value[1];
});
var OptionalEquals$$ = [OptionalEquals$0, OptionalEquals$1];
function OptionalEquals(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "OptionalEquals", OptionalEquals$$);
}
var TypeLexicalDeclaration$0 = (0, import_lib3.$S)(__, LetOrConstOrVar, TypeDeclarationBinding, (0, import_lib3.$Q)((0, import_lib3.$S)(CommaDelimiter, __, TypeDeclarationBinding)));
var TypeLexicalDeclaration$1 = (0, import_lib3.$S)(__, EnumDeclaration);
var TypeLexicalDeclaration$2 = ClassSignature;
var TypeLexicalDeclaration$3 = (0, import_lib3.$S)(Namespace, (0, import_lib3.$E)(_), IdentifierName, DeclareBlock);
var TypeLexicalDeclaration$4 = (0, import_lib3.$S)(Module, _, StringLiteral, (0, import_lib3.$E)(DeclareBlock));
var TypeLexicalDeclaration$5 = (0, import_lib3.$S)(Global, (0, import_lib3.$E)(DeclareBlock));
var TypeLexicalDeclaration$$ = [TypeLexicalDeclaration$0, TypeLexicalDeclaration$1, TypeLexicalDeclaration$2, TypeLexicalDeclaration$3, TypeLexicalDeclaration$4, TypeLexicalDeclaration$5];
function TypeLexicalDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeLexicalDeclaration", TypeLexicalDeclaration$$);
}
var TypeDeclarationBinding$0 = (0, import_lib3.$S)((0, import_lib3.$C)(BindingPattern, BindingIdentifier), (0, import_lib3.$E)(TypeSuffix));
function TypeDeclarationBinding(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeDeclarationBinding", TypeDeclarationBinding$0);
}
var InterfaceExtendsClause$0 = (0, import_lib3.$S)(ExtendsToken, InterfaceExtendsTarget, (0, import_lib3.$Q)((0, import_lib3.$S)(Comma, InterfaceExtendsTarget)));
function InterfaceExtendsClause(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InterfaceExtendsClause", InterfaceExtendsClause$0);
}
var InterfaceExtendsTarget$0 = ImplementsTarget;
function InterfaceExtendsTarget(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InterfaceExtendsTarget", InterfaceExtendsTarget$0);
}
var TypeKeyword$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L218, 'TypeKeyword "type"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function TypeKeyword(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeKeyword", TypeKeyword$0);
}
var Enum$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L219, 'Enum "enum"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Enum(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Enum", Enum$0);
}
var Interface$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L220, 'Interface "interface"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Interface(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Interface", Interface$0);
}
var Global$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L221, 'Global "global"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Global(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Global", Global$0);
}
var Module$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L222, 'Module "module"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Module(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Module", Module$0);
}
var Namespace$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L223, 'Namespace "namespace"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { $loc, token: $1 };
});
function Namespace(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Namespace", Namespace$0);
}
var InterfaceBlock$0 = (0, import_lib3.$S)(__, OpenBrace, NestedInterfaceProperties, __, CloseBrace);
var InterfaceBlock$1 = (0, import_lib3.$S)(__, OpenBrace, (0, import_lib3.$Q)((0, import_lib3.$S)(__, InterfaceProperty)), __, CloseBrace);
var InterfaceBlock$2 = NestedInterfaceBlock;
var InterfaceBlock$$ = [InterfaceBlock$0, InterfaceBlock$1, InterfaceBlock$2];
function InterfaceBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "InterfaceBlock", InterfaceBlock$$);
}
var NestedInterfaceBlock$0 = (0, import_lib3.$S)(InsertOpenBrace, NestedInterfaceProperties, InsertNewline, InsertIndent, InsertCloseBrace);
function NestedInterfaceBlock(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedInterfaceBlock", NestedInterfaceBlock$0);
}
var NestedInterfaceProperties$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedInterfaceProperty), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var props = $2;
  if (props.length)
    return props;
  return $skip;
});
function NestedInterfaceProperties(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedInterfaceProperties", NestedInterfaceProperties$0);
}
var NestedInterfaceProperty$0 = (0, import_lib3.$S)(Nested, InterfaceProperty);
function NestedInterfaceProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedInterfaceProperty", NestedInterfaceProperty$0);
}
var InterfaceProperty$0 = BasicInterfaceProperty;
var InterfaceProperty$1 = (0, import_lib3.$S)(NonEmptyParameters, TypeSuffix, InterfacePropertyDelimiter);
var InterfaceProperty$2 = (0, import_lib3.$S)(MethodSignature, InterfacePropertyDelimiter);
var InterfaceProperty$$ = [InterfaceProperty$0, InterfaceProperty$1, InterfaceProperty$2];
function InterfaceProperty(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "InterfaceProperty", InterfaceProperty$$);
}
var BasicInterfaceProperty$0 = (0, import_lib3.$S)((0, import_lib3.$C)(TypeIndexSignature, TypeProperty), (0, import_lib3.$E)(_), TypeSuffix, InterfacePropertyDelimiter);
function BasicInterfaceProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "BasicInterfaceProperty", BasicInterfaceProperty$0);
}
var InterfacePropertyDelimiter$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)(Semicolon, Comma));
var InterfacePropertyDelimiter$1 = (0, import_lib3.$Y)((0, import_lib3.$S)(__, CloseBrace));
var InterfacePropertyDelimiter$2 = (0, import_lib3.$Y)(EOS);
var InterfacePropertyDelimiter$$ = [InterfacePropertyDelimiter$0, InterfacePropertyDelimiter$1, InterfacePropertyDelimiter$2];
function InterfacePropertyDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "InterfacePropertyDelimiter", InterfacePropertyDelimiter$$);
}
var ModuleBlock$0 = (0, import_lib3.$S)(__, OpenBrace, NestedModuleItems, __, CloseBrace);
var ModuleBlock$1 = (0, import_lib3.$S)(InsertOpenBrace, NestedModuleItems, InsertNewline, InsertIndent, InsertCloseBrace);
var ModuleBlock$$ = [ModuleBlock$0, ModuleBlock$1];
function ModuleBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ModuleBlock", ModuleBlock$$);
}
var NestedModuleItems$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedModuleItem), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var items = $2;
  if (items.length)
    return items;
  return $skip;
});
function NestedModuleItems(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedModuleItems", NestedModuleItems$0);
}
var NestedModuleItem$0 = (0, import_lib3.$S)(Nested, ModuleItem, StatementDelimiter);
function NestedModuleItem(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedModuleItem", NestedModuleItem$0);
}
var DeclareBlock$0 = (0, import_lib3.$S)(__, OpenBrace, NestedDeclareElements, __, CloseBrace);
var DeclareBlock$1 = (0, import_lib3.$S)(__, OpenBrace, (0, import_lib3.$Q)((0, import_lib3.$S)(__, DeclareElement, InterfacePropertyDelimiter)), __, CloseBrace);
var DeclareBlock$2 = (0, import_lib3.$S)(InsertOpenBrace, NestedDeclareElements, InsertNewline, InsertIndent, InsertCloseBrace);
var DeclareBlock$$ = [DeclareBlock$0, DeclareBlock$1, DeclareBlock$2];
function DeclareBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "DeclareBlock", DeclareBlock$$);
}
var NestedDeclareElements$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedDeclareElement), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var decs = $2;
  if (decs.length)
    return decs;
  return $skip;
});
function NestedDeclareElements(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedDeclareElements", NestedDeclareElements$0);
}
var NestedDeclareElement$0 = (0, import_lib3.$S)(Nested, DeclareElement, InterfacePropertyDelimiter);
function NestedDeclareElement(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedDeclareElement", NestedDeclareElement$0);
}
var DeclareElement$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$E)(Decorators), (0, import_lib3.$E)((0, import_lib3.$S)(Export, (0, import_lib3.$E)(_))), TypeLexicalDeclaration), function(value) {
  return { "ts": true, "children": value };
});
var DeclareElement$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Export, (0, import_lib3.$E)(_))), TypeDeclarationRest), function(value) {
  return { "ts": true, "children": value };
});
var DeclareElement$$ = [DeclareElement$0, DeclareElement$1];
function DeclareElement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "DeclareElement", DeclareElement$$);
}
var EnumDeclaration$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Const, _)), Enum, (0, import_lib3.$E)(_), IdentifierName, EnumBlock), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var isConst = $1;
  var id = $4;
  var block = $5;
  const ts2 = {
    ts: true,
    children: $0
  };
  const names = new Set(block.properties.map((p) => p.name.name));
  return {
    type: "EnumDeclaration",
    id,
    children: [ts2, {
      js: true,
      children: [
        ["let ", id, " = {};\n"],
        ...block.properties.map((property, i) => {
          let init, isString;
          if (property.initializer) {
            init = replaceNodes(
              deepCopy(property.initializer),
              (n) => n.type === "Identifier" && names.has(n.name),
              (n) => [id, '["', n.name, '"]']
            );
            const value = init[init.length - 1];
            isString = value.type === "TemplateLiteral" || value.type === "Literal" && value.subtype === "StringLiteral";
          } else {
            init = i === 0 ? " = 0" : [" = ", id, '["', block.properties[i - 1].name, '"] + 1'];
          }
          if (isString) {
            return [
              id,
              '["',
              property.name,
              '"]',
              init,
              ";\n"
            ];
          } else {
            return [
              id,
              "[",
              id,
              '["',
              property.name,
              '"]',
              init,
              '] = "',
              property.name,
              '";\n'
            ];
          }
        })
      ]
    }]
  };
});
function EnumDeclaration(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EnumDeclaration", EnumDeclaration$0);
}
var EnumBlock$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, OpenBrace, NestedEnumProperties, __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var props = $3;
  return {
    properties: props.properties,
    children: $0
  };
});
var EnumBlock$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, OpenBrace, (0, import_lib3.$Q)((0, import_lib3.$S)(__, EnumProperty)), __, CloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var props = $3;
  return {
    properties: props.map((p) => p[1]),
    children: $0
  };
});
var EnumBlock$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(InsertOpenBrace, NestedEnumProperties, InsertNewline, InsertIndent, InsertCloseBrace), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var props = $2;
  return {
    properties: props.properties,
    children: $0
  };
});
var EnumBlock$$ = [EnumBlock$0, EnumBlock$1, EnumBlock$2];
function EnumBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "EnumBlock", EnumBlock$$);
}
var NestedEnumProperties$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedEnumPropertyLine), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var props = $2;
  if (!props.length)
    return $skip;
  return {
    properties: props.flat().map((p) => p.property),
    children: $0
  };
});
function NestedEnumProperties(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedEnumProperties", NestedEnumProperties$0);
}
var NestedEnumPropertyLine$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$S)(Nested, EnumProperty), (0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$E)(_), EnumProperty))), function($skip, $loc, $0, $1, $2) {
  return [$1, ...$2].map((pair) => ({
    property: pair[1],
    children: pair
  }));
});
function NestedEnumPropertyLine(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedEnumPropertyLine", NestedEnumPropertyLine$0);
}
var EnumProperty$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Identifier, (0, import_lib3.$E)((0, import_lib3.$S)(__, Equals, MaybeNestedExtendedExpression)), ObjectPropertyDelimiter), function($skip, $loc, $0, $1, $2, $3) {
  var name = $1;
  var initializer = $2;
  return {
    type: "EnumProperty",
    name,
    initializer,
    children: $0
  };
});
function EnumProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EnumProperty", EnumProperty$0);
}
var TypeProperty$0 = (0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Readonly, NotDedented)), PropertyName);
function TypeProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeProperty", TypeProperty$0);
}
var TypeIndexSignature$0 = (0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R84, "TypeIndexSignature /[+-]?/")), Readonly, NotDedented)), OpenBracket, TypeIndex, CloseBracket, (0, import_lib3.$E)((0, import_lib3.$S)(__, (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R12, "TypeIndexSignature /[+-]/")), (0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$E)(_), QuestionMark)))));
function TypeIndexSignature(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeIndexSignature", TypeIndexSignature$0);
}
var TypeIndex$0 = (0, import_lib3.$S)(__, Identifier, TypeSuffix);
var TypeIndex$1 = (0, import_lib3.$S)(__, PropertyName, __, In, Type, (0, import_lib3.$E)((0, import_lib3.$S)(__, As, Type)));
var TypeIndex$$ = [TypeIndex$0, TypeIndex$1];
function TypeIndex(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeIndex", TypeIndex$$);
}
var TypeSuffix$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$E)(QuestionMark), (0, import_lib3.$E)(_), Colon, MaybeNestedType), function(value) {
  var optional = value[1];
  var t = value[4];
  return { "type": "TypeSuffix", "ts": true, "optional": optional, "t": t, "children": value };
});
var TypeSuffix$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$E)(_), QuestionMark, (0, import_lib3.$E)(_)), function(value) {
  var optional = value[1];
  return { "type": "TypeSuffix", "ts": true, "optional": optional, "children": value };
});
var TypeSuffix$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(NonNullAssertion, (0, import_lib3.$E)(_), (0, import_lib3.$E)((0, import_lib3.$S)(Colon, MaybeNestedType))), function($skip, $loc, $0, $1, $2, $3) {
  var nonnull = $1;
  var ct = $3;
  const [colon, t] = ct ?? [];
  return {
    type: "TypeSuffix",
    ts: true,
    nonnull,
    t,
    children: [$1, $2, colon, t]
  };
});
var TypeSuffix$$ = [TypeSuffix$0, TypeSuffix$1, TypeSuffix$2];
function TypeSuffix(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeSuffix", TypeSuffix$$);
}
var MaybeNestedType$0 = NestedInterfaceBlock;
var MaybeNestedType$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$E)((0, import_lib3.$S)(Nested, Type)), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  if (!$2)
    return $skip;
  return $2;
});
var MaybeNestedType$2 = Type;
var MaybeNestedType$$ = [MaybeNestedType$0, MaybeNestedType$1, MaybeNestedType$2];
function MaybeNestedType(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "MaybeNestedType", MaybeNestedType$$);
}
var ReturnTypeSuffix$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$E)(QuestionMark), (0, import_lib3.$E)(_), Colon, ReturnType), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var optional = $2;
  var t = $5;
  return {
    ...t,
    optional,
    children: [$1, optional, $3, $4, ...t.children]
  };
});
function ReturnTypeSuffix(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ReturnTypeSuffix", ReturnTypeSuffix$0);
}
var ReturnType$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L224, 'ReturnType "asserts"'), NonIdContinue)), TypePredicate), function($skip, $loc, $0, $1, $2) {
  var asserts = $1;
  var t = $2;
  if (asserts) {
    t = {
      type: "AssertsType",
      t,
      children: [asserts[0], asserts[1], t],
      ts: true
    };
  }
  return {
    type: "ReturnTypeAnnotation",
    children: [t],
    t,
    ts: true
  };
});
function ReturnType(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ReturnType", ReturnType$0);
}
var TypePredicate$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Type, (0, import_lib3.$E)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L166, 'TypePredicate "is"'), NonIdContinue, Type))), function($skip, $loc, $0, $1, $2) {
  var lhs = $1;
  var rhs = $2;
  if (!rhs)
    return lhs;
  return {
    type: "TypePredicate",
    lhs,
    rhs: rhs[3],
    children: [lhs, ...rhs]
  };
});
function TypePredicate(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypePredicate", TypePredicate$0);
}
var Type$0 = TypeConditional;
function Type(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Type", Type$0);
}
var TypeBinary$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(__, TypeBinaryOp, __)), TypeUnary, (0, import_lib3.$Q)((0, import_lib3.$S)(__, TypeBinaryOp, __, TypeUnary))), function($skip, $loc, $0, $1, $2, $3) {
  var optionalPrefix = $1;
  var t = $2;
  var ops = $3;
  if (!ops.length && !optionalPrefix)
    return t;
  if (!ops.length)
    return [optionalPrefix, t];
  if (!optionalPrefix)
    return [t, ...ops];
  return [optionalPrefix, t, ops];
});
function TypeBinary(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeBinary", TypeBinary$0);
}
var TypeUnary$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$Q)((0, import_lib3.$S)(__, TypeUnaryOp)), TypePrimary, (0, import_lib3.$Q)(TypeUnarySuffix)), function($skip, $loc, $0, $1, $2, $3) {
  var prefix = $1;
  var t = $2;
  var suffix = $3;
  if (!prefix.length && !suffix.length)
    return t;
  return {
    type: "UnaryType",
    prefix,
    suffix,
    t,
    // omit empty prefix for trimming space
    children: prefix.length ? $0 : [t, suffix]
  };
});
function TypeUnary(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeUnary", TypeUnary$0);
}
var TypeUnarySuffix$0 = TypeIndexedAccess;
var TypeUnarySuffix$1 = QuestionMark;
var TypeUnarySuffix$$ = [TypeUnarySuffix$0, TypeUnarySuffix$1];
function TypeUnarySuffix(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeUnarySuffix", TypeUnarySuffix$$);
}
var TypeUnaryOp$0 = (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L225, 'TypeUnaryOp "keyof"'), NonIdContinue);
var TypeUnaryOp$1 = (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L185, 'TypeUnaryOp "readonly"'), NonIdContinue);
var TypeUnaryOp$$ = [TypeUnaryOp$0, TypeUnaryOp$1];
function TypeUnaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeUnaryOp", TypeUnaryOp$$);
}
var TypeIndexedAccess$0 = (0, import_lib3.$S)(OpenBracket, (0, import_lib3.$E)(Type), __, CloseBracket);
var TypeIndexedAccess$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(Dot, (0, import_lib3.$C)(TemplateLiteral, StringLiteral, IntegerLiteral)), function($skip, $loc, $0, $1, $2) {
  var dot = $1;
  var literal = $2;
  const open = { ...dot, token: "[" };
  return [
    open,
    literal,
    "]"
  ];
});
var TypeIndexedAccess$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeePrototypeEnabled, DoubleColon, (0, import_lib3.$E)((0, import_lib3.$C)(IdentifierName, LengthShorthand))), function($skip, $loc, $0, $1, $2, $3) {
  var p = $2;
  var id = $3;
  const open = { ...p, token: '["' };
  return [
    open,
    id,
    '"]'
  ];
});
var TypeIndexedAccess$$ = [TypeIndexedAccess$0, TypeIndexedAccess$1, TypeIndexedAccess$2];
function TypeIndexedAccess(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeIndexedAccess", TypeIndexedAccess$$);
}
var UnknownAlias$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L226, 'UnknownAlias "???"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "unknown" };
});
function UnknownAlias(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "UnknownAlias", UnknownAlias$0);
}
var TypePrimary$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), Infer, (0, import_lib3.$E)(_), IdentifierName, (0, import_lib3.$E)((0, import_lib3.$S)(NotDedented, ExtendsToken, Type)));
var TypePrimary$1 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), Typeof, (0, import_lib3.$E)(_), UnaryExpression), function($skip, $loc, $0, $1, $2, $3, $4) {
  return {
    type: "TypeofType",
    children: $0
  };
});
var TypePrimary$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), TypeTuple), function($skip, $loc, $0, $1, $2) {
  return { ...$2, children: [$1, ...$2.children] };
});
var TypePrimary$3 = InterfaceBlock;
var TypePrimary$4 = (0, import_lib3.$S)((0, import_lib3.$E)(_), TypeFunction);
var TypePrimary$5 = (0, import_lib3.$S)((0, import_lib3.$E)(_), InlineInterfaceLiteral);
var TypePrimary$6 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), ImportType), function($skip, $loc, $0, $1, $2) {
  var t = $2;
  return {
    type: "ImportType",
    t,
    children: $0
  };
});
var TypePrimary$7 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), TypeLiteral), function($skip, $loc, $0, $1, $2) {
  var t = $2;
  return {
    type: "LiteralType",
    t,
    children: $0
  };
});
var TypePrimary$8 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), UnknownAlias), function($skip, $loc, $0, $1, $2) {
  return {
    type: "IdentifierType",
    children: $0,
    raw: $2.token,
    args: void 0
  };
});
var TypePrimary$9 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), IdentifierName, (0, import_lib3.$Q)((0, import_lib3.$S)(Dot, IdentifierName)), (0, import_lib3.$E)(TypeArguments)), function($skip, $loc, $0, $1, $2, $3, $4) {
  var args = $4;
  return {
    type: "IdentifierType",
    children: $0,
    raw: [$2.name, ...$3.map(([dot, id]) => dot.token + id.name)].join(""),
    args
  };
});
var TypePrimary$10 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), OpenParen, (0, import_lib3.$C)(Type, (0, import_lib3.$S)(EOS, Type)), __, CloseParen), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  return {
    type: "ParenthesizedType",
    children: $0
  };
});
var TypePrimary$$ = [TypePrimary$0, TypePrimary$1, TypePrimary$2, TypePrimary$3, TypePrimary$4, TypePrimary$5, TypePrimary$6, TypePrimary$7, TypePrimary$8, TypePrimary$9, TypePrimary$10];
function TypePrimary(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypePrimary", TypePrimary$$);
}
var ImportType$0 = (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L15, 'ImportType "import"'), OpenParen, __, StringLiteral, __, CloseParen, (0, import_lib3.$E)((0, import_lib3.$S)(Dot, IdentifierName)), (0, import_lib3.$E)(TypeArguments));
var ImportType$1 = (0, import_lib3.$S)((0, import_lib3.$EXPECT)($L15, 'ImportType "import"'), InsertOpenParen, (0, import_lib3.$E)(Trimmed_), StringLiteral, InsertCloseParen);
var ImportType$$ = [ImportType$0, ImportType$1];
function ImportType(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "ImportType", ImportType$$);
}
var TypeTuple$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenBracket, (0, import_lib3.$C)(NestedTypeList, (0, import_lib3.$E)(TypeList)), __, CloseBracket), function($skip, $loc, $0, $1, $2, $3, $4) {
  return {
    type: "TypeTuple",
    children: $0
  };
});
function TypeTuple(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeTuple", TypeTuple$0);
}
var TypeList$0 = (0, import_lib3.$S)(TypeElement, (0, import_lib3.$Q)((0, import_lib3.$S)(__, Comma, TypeElement)));
function TypeList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeList", TypeList$0);
}
var TypeElement$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(__, (0, import_lib3.$E)((0, import_lib3.$S)(DotDotDot, __)), IdentifierName, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), DotDotDot)), (0, import_lib3.$S)(__, (0, import_lib3.$E)((0, import_lib3.$S)(QuestionMark, (0, import_lib3.$E)(_))), Colon, __), Type), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6) {
  var ws = $1;
  var dots1 = $2;
  var name = $3;
  var dots2 = $4;
  var colon = $5;
  var type = $6;
  let dots = dots1 || dots2 && [dots2[1], dots2[0]];
  if (dots1 && dots2) {
    dots = [dots, {
      type: "Error",
      message: "... both before and after identifier"
    }];
  }
  return [ws, dots, name, colon, type];
});
var TypeElement$1 = (0, import_lib3.$S)(__, DotDotDot, __, Type);
var TypeElement$2 = (0, import_lib3.$TS)((0, import_lib3.$S)(Type, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)(_), DotDotDot))), function($skip, $loc, $0, $1, $2) {
  var type = $1;
  var spaceDots = $2;
  if (!spaceDots)
    return type;
  const [space, dots] = spaceDots;
  const ws = getTrimmingSpace(type);
  if (!ws)
    return [dots, space, type];
  return [ws, dots, space, insertTrimmingSpace(type, "")];
});
var TypeElement$$ = [TypeElement$0, TypeElement$1, TypeElement$2];
function TypeElement(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeElement", TypeElement$$);
}
var NestedTypeList$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$Q)(NestedType), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  var types = $2;
  if (types.length)
    return types;
  return $skip;
});
function NestedTypeList(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedTypeList", NestedTypeList$0);
}
var NestedType$0 = (0, import_lib3.$S)(Nested, TypeElement, ArrayElementDelimiter);
function NestedType(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NestedType", NestedType$0);
}
var TypeConditional$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$EXPECT)($R85, "TypeConditional /(?=if|unless)/"), TypeIfThenElse), function($skip, $loc, $0, $1, $2, $3) {
  return [$1, expressionizeTypeIf($3)];
});
var TypeConditional$1 = (0, import_lib3.$TS)((0, import_lib3.$S)(TypeCondition, NotDedented, QuestionMark, __, Type, __, Colon, __, Type), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  if ($1.negated)
    return [$1, $2, $3, $4, $9, $6, $7, $8, $5];
  return $0;
});
var TypeConditional$2 = TypeBinary;
var TypeConditional$$ = [TypeConditional$0, TypeConditional$1, TypeConditional$2];
function TypeConditional(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeConditional", TypeConditional$$);
}
var TypeCondition$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(TypeBinary, (0, import_lib3.$E)(IndentedFurther), (0, import_lib3.$C)(ExtendsToken, NotExtendsToken), Type), function($skip, $loc, $0, $1, $2, $3, $4) {
  return {
    type: "TypeCondition",
    negated: $3.negated,
    children: $0
  };
});
function TypeCondition(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeCondition", TypeCondition$0);
}
var TypeIfThenElse$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)(If, Unless), (0, import_lib3.$S)(OpenParen, TypeCondition, CloseParen), TypeBlock, (0, import_lib3.$E)(TypeElse)), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  return [$1, $2, $3[1], $4, $5];
});
var TypeIfThenElse$1 = (0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)(If, Unless), TypeCondition, TypeBlock, (0, import_lib3.$E)(TypeElse));
var TypeIfThenElse$$ = [TypeIfThenElse$0, TypeIfThenElse$1];
function TypeIfThenElse(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeIfThenElse", TypeIfThenElse$$);
}
var TypeElse$0 = (0, import_lib3.$S)(NotDedented, Else, TypeBlock);
function TypeElse(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeElse", TypeElse$0);
}
var TypeBlock$0 = (0, import_lib3.$T)((0, import_lib3.$S)(Then, Type), function(value) {
  return value[1];
});
var TypeBlock$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(EOS), Type), function(value) {
  return value[1];
});
var TypeBlock$2 = NestedInterfaceBlock;
var TypeBlock$3 = (0, import_lib3.$TS)((0, import_lib3.$S)(PushIndent, (0, import_lib3.$E)((0, import_lib3.$S)(Nested, Type)), PopIndent), function($skip, $loc, $0, $1, $2, $3) {
  if (!$2)
    return $skip;
  return $2;
});
var TypeBlock$$ = [TypeBlock$0, TypeBlock$1, TypeBlock$2, TypeBlock$3];
function TypeBlock(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeBlock", TypeBlock$$);
}
var TypeTemplateSubstitution$0 = (0, import_lib3.$S)(SubstitutionStart, Type, __, CloseBrace);
function TypeTemplateSubstitution(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeTemplateSubstitution", TypeTemplateSubstitution$0);
}
var TypeTemplateLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(Backtick, (0, import_lib3.$Q)((0, import_lib3.$C)(TemplateCharacters, TypeTemplateSubstitution)), Backtick), function($skip, $loc, $0, $1, $2, $3) {
  return {
    type: "TemplateLiteral",
    children: $0
  };
});
var TypeTemplateLiteral$1 = CoffeeInterpolatedDoubleQuotedTypeLiteral;
var TypeTemplateLiteral$$ = [TypeTemplateLiteral$0, TypeTemplateLiteral$1];
function TypeTemplateLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeTemplateLiteral", TypeTemplateLiteral$$);
}
var CoffeeStringTypeSubstitution$0 = (0, import_lib3.$S)(CoffeeSubstitutionStart, Type, __, CloseBrace);
function CoffeeStringTypeSubstitution(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeStringTypeSubstitution", CoffeeStringTypeSubstitution$0);
}
var CoffeeInterpolatedDoubleQuotedTypeLiteral$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(CoffeeInterpolationEnabled, DoubleQuote, (0, import_lib3.$Q)((0, import_lib3.$C)(CoffeeDoubleQuotedStringCharacters, CoffeeStringTypeSubstitution)), DoubleQuote), function($skip, $loc, $0, $1, $2, $3, $4) {
  var s = $2;
  var parts = $3;
  var e = $4;
  return processCoffeeInterpolation(s, parts, e, $loc);
});
function CoffeeInterpolatedDoubleQuotedTypeLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeInterpolatedDoubleQuotedTypeLiteral", CoffeeInterpolatedDoubleQuotedTypeLiteral$0);
}
var TypeLiteral$0 = TypeTemplateLiteral;
var TypeLiteral$1 = Literal;
var TypeLiteral$2 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R12, "TypeLiteral /[+-]/"), NumericLiteral), function($skip, $loc, $0, $1, $2) {
  var sign = $1;
  var num = $2;
  if (sign[0] === "+")
    return num;
  return $0;
});
var TypeLiteral$3 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L208, 'TypeLiteral "void"'), NonIdContinue), function($skip, $loc, $0, $1, $2) {
  return { type: "VoidType", $loc, token: $1 };
});
var TypeLiteral$4 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L227, 'TypeLiteral "unique"'), _, (0, import_lib3.$EXPECT)($L228, 'TypeLiteral "symbol"'), NonIdContinue), function($skip, $loc, $0, $1, $2, $3, $4) {
  return { type: "UniqueSymbolType", children: $0 };
});
var TypeLiteral$5 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L229, 'TypeLiteral "[]"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "[]" };
});
var TypeLiteral$$ = [TypeLiteral$0, TypeLiteral$1, TypeLiteral$2, TypeLiteral$3, TypeLiteral$4, TypeLiteral$5];
function TypeLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeLiteral", TypeLiteral$$);
}
var InlineInterfaceLiteral$0 = (0, import_lib3.$S)(InsertInlineOpenBrace, InlineBasicInterfaceProperty, (0, import_lib3.$Q)((0, import_lib3.$S)((0, import_lib3.$C)(IndentedFurther, (0, import_lib3.$E)(_)), InlineBasicInterfaceProperty)), InsertCloseBrace);
function InlineInterfaceLiteral(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineInterfaceLiteral", InlineInterfaceLiteral$0);
}
var InlineBasicInterfaceProperty$0 = (0, import_lib3.$S)((0, import_lib3.$C)(TypeIndexSignature, TypeProperty), (0, import_lib3.$E)(QuestionMark), Colon, Type, InlineInterfacePropertyDelimiter);
function InlineBasicInterfaceProperty(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InlineBasicInterfaceProperty", InlineBasicInterfaceProperty$0);
}
var InlineInterfacePropertyDelimiter$0 = (0, import_lib3.$C)((0, import_lib3.$S)((0, import_lib3.$E)(_), Semicolon), CommaDelimiter);
var InlineInterfacePropertyDelimiter$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)((0, import_lib3.$S)((0, import_lib3.$C)(IndentedFurther, (0, import_lib3.$E)(_)), InlineBasicInterfaceProperty)), InsertComma), function(value) {
  return value[1];
});
var InlineInterfacePropertyDelimiter$2 = (0, import_lib3.$Y)((0, import_lib3.$S)(__, (0, import_lib3.$C)((0, import_lib3.$EXPECT)($L16, 'InlineInterfacePropertyDelimiter ":"'), (0, import_lib3.$EXPECT)($L125, 'InlineInterfacePropertyDelimiter ")"'), (0, import_lib3.$EXPECT)($L45, 'InlineInterfacePropertyDelimiter "]"'), (0, import_lib3.$EXPECT)($L36, 'InlineInterfacePropertyDelimiter "}"'))));
var InlineInterfacePropertyDelimiter$3 = (0, import_lib3.$Y)(EOS);
var InlineInterfacePropertyDelimiter$$ = [InlineInterfacePropertyDelimiter$0, InlineInterfacePropertyDelimiter$1, InlineInterfacePropertyDelimiter$2, InlineInterfacePropertyDelimiter$3];
function InlineInterfacePropertyDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "InlineInterfacePropertyDelimiter", InlineInterfacePropertyDelimiter$$);
}
var TypeBinaryOp$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L109, 'TypeBinaryOp "|"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "|" };
});
var TypeBinaryOp$1 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L108, 'TypeBinaryOp "&"'), function($skip, $loc, $0, $1) {
  return { $loc, token: "&" };
});
var TypeBinaryOp$$ = [TypeBinaryOp$0, TypeBinaryOp$1];
function TypeBinaryOp(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeBinaryOp", TypeBinaryOp$$);
}
var TypeFunction$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$E)((0, import_lib3.$S)(Abstract, (0, import_lib3.$E)(_))), New, (0, import_lib3.$E)(_))), Parameters, __, TypeArrowFunction, (0, import_lib3.$E)(ReturnType)), function($skip, $loc, $0, $1, $2, $3, $4, $5) {
  var type = $5;
  if (type) {
    return $0;
  }
  return [...$0, "void"];
});
function TypeFunction(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeFunction", TypeFunction$0);
}
var TypeArrowFunction$0 = (0, import_lib3.$TV)((0, import_lib3.$C)((0, import_lib3.$EXPECT)($L13, 'TypeArrowFunction "=>"'), (0, import_lib3.$EXPECT)($L14, 'TypeArrowFunction "\u21D2"'), (0, import_lib3.$EXPECT)($L34, 'TypeArrowFunction "->"'), (0, import_lib3.$EXPECT)($L35, 'TypeArrowFunction "\u2192"')), function($skip, $loc, $0, $1) {
  return { $loc, token: "=>" };
});
function TypeArrowFunction(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeArrowFunction", TypeArrowFunction$0);
}
var TypeArguments$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenAngleBracket, (0, import_lib3.$P)(TypeArgument), __, CloseAngleBracket), function($skip, $loc, $0, $1, $2, $3, $4) {
  var args = $2;
  return {
    type: "TypeArguments",
    ts: true,
    types: args.map(([, t]) => t),
    children: $0
  };
});
function TypeArguments(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeArguments", TypeArguments$0);
}
var TypeArgument$0 = (0, import_lib3.$S)(__, Type, TypeArgumentDelimiter);
function TypeArgument(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeArgument", TypeArgument$0);
}
var TypeArgumentDelimiter$0 = TypeParameterDelimiter;
function TypeArgumentDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeArgumentDelimiter", TypeArgumentDelimiter$0);
}
var TypeParameters$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(OpenAngleBracket, (0, import_lib3.$P)(TypeParameter), __, CloseAngleBracket), function($skip, $loc, $0, $1, $2, $3, $4) {
  var parameters = $2;
  return {
    type: "TypeParameters",
    parameters,
    ts: true,
    children: $0
  };
});
function TypeParameters(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeParameters", TypeParameters$0);
}
var TypeParameter$0 = (0, import_lib3.$S)(__, (0, import_lib3.$E)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L165, 'TypeParameter "const"'), (0, import_lib3.$E)(_))), Identifier, (0, import_lib3.$E)(TypeConstraint), (0, import_lib3.$E)(TypeInitializer), TypeParameterDelimiter);
function TypeParameter(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeParameter", TypeParameter$0);
}
var TypeConstraint$0 = (0, import_lib3.$S)(__, ExtendsToken, Type);
function TypeConstraint(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeConstraint", TypeConstraint$0);
}
var TypeInitializer$0 = (0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L3, 'TypeInitializer "="'), Type);
function TypeInitializer(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TypeInitializer", TypeInitializer$0);
}
var TypeParameterDelimiter$0 = (0, import_lib3.$S)((0, import_lib3.$E)(_), Comma);
var TypeParameterDelimiter$1 = (0, import_lib3.$Y)((0, import_lib3.$S)(__, (0, import_lib3.$EXPECT)($L44, 'TypeParameterDelimiter ">"')));
var TypeParameterDelimiter$2 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$Y)(EOS), InsertComma), function(value) {
  return value[1];
});
var TypeParameterDelimiter$$ = [TypeParameterDelimiter$0, TypeParameterDelimiter$1, TypeParameterDelimiter$2];
function TypeParameterDelimiter(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "TypeParameterDelimiter", TypeParameterDelimiter$$);
}
var ThisType$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$E)(_), (0, import_lib3.$C)(This, AtThis), Colon, Type, ParameterElementDelimiter), function(value) {
  return { "type": "ThisType", "ts": true, "children": value };
});
function ThisType(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ThisType", ThisType$0);
}
var Shebang$0 = (0, import_lib3.$S)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R86, "Shebang /#![^\\r\\n]*/")), EOL);
function Shebang(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Shebang", Shebang$0);
}
var CivetPrologue$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R87, "CivetPrologue /[\\t ]*/"), DoubleQuote, CivetPrologueContent, DoubleQuote, SimpleStatementDelimiter, (0, import_lib3.$EXPECT)($R88, "CivetPrologue /[ \\t]*/"), (0, import_lib3.$C)(EOL, (0, import_lib3.$Y)(RestOfLine))), function(value) {
  var content = value[2];
  return content;
});
var CivetPrologue$1 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R87, "CivetPrologue /[\\t ]*/"), SingleQuote, CivetPrologueContent, SingleQuote, SimpleStatementDelimiter, (0, import_lib3.$EXPECT)($R88, "CivetPrologue /[ \\t]*/"), (0, import_lib3.$C)(EOL, (0, import_lib3.$Y)(RestOfLine))), function(value) {
  var content = value[2];
  return content;
});
var CivetPrologue$$ = [CivetPrologue$0, CivetPrologue$1];
function CivetPrologue(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "CivetPrologue", CivetPrologue$$);
}
var CivetPrologueContent$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($L230, 'CivetPrologueContent "civet"'), NonIdContinue, (0, import_lib3.$Q)(CivetOption), (0, import_lib3.$EXPECT)($R89, "CivetPrologueContent /[\\s]*/")), function($skip, $loc, $0, $1, $2, $3, $4) {
  var options = $3;
  return {
    type: "CivetPrologue",
    children: [],
    config: Object.fromEntries(options)
  };
});
function CivetPrologueContent(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CivetPrologueContent", CivetPrologueContent$0);
}
var CivetOption$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R90, "CivetOption /\\s+([+-]?)([a-zA-Z0-9-]+)(\\s*=\\s*([a-zA-Z0-9.+-]*))?/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  const optionName = $2.replace(/-+([a-z]?)/g, (_2, l) => {
    if (l)
      return l.toUpperCase();
    return "";
  });
  let value = $3 ? $4 : $1 === "-" ? false : true;
  if (optionName === "tab") {
    value = parseFloat(value);
    if (isNaN(value))
      value = 0;
  }
  return [optionName, value];
});
function CivetOption(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CivetOption", CivetOption$0);
}
var UnknownPrologue$0 = (0, import_lib3.$S)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R87, "UnknownPrologue /[\\t ]*/")), StringLiteral, (0, import_lib3.$TEXT)(SimpleStatementDelimiter), EOS);
function UnknownPrologue(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "UnknownPrologue", UnknownPrologue$0);
}
var TripleSlashDirective$0 = (0, import_lib3.$S)((0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R91, "TripleSlashDirective /\\/\\/\\/[^\\r\\n]*/")), (0, import_lib3.$E)(EOS));
function TripleSlashDirective(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TripleSlashDirective", TripleSlashDirective$0);
}
var DirectivePrologue$0 = (0, import_lib3.$T)((0, import_lib3.$S)(PrologueString, (0, import_lib3.$N)((0, import_lib3.$S)(__, (0, import_lib3.$C)(AccessStart, Pipe)))), function(value) {
  return value[0];
});
function DirectivePrologue(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DirectivePrologue", DirectivePrologue$0);
}
var PrologueString$0 = CivetPrologue;
var PrologueString$1 = UnknownPrologue;
var PrologueString$$ = [PrologueString$0, PrologueString$1];
function PrologueString(ctx, state2) {
  return (0, import_lib3.$EVENT_C)(ctx, state2, "PrologueString", PrologueString$$);
}
var EOS$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$EXPECT)($R92, "EOS /(?=[ \\t\\r\\n\\/#]|$)/"), (0, import_lib3.$P)(RestOfLine)), function(value) {
  return value[1];
});
function EOS(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EOS", EOS$0);
}
var EOL$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R93, "EOL /\\r\\n|\\n|\\r|$/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  return { $loc, token: $0 };
});
function EOL(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "EOL", EOL$0);
}
var DebugHere$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'DebugHere ""'), function($skip, $loc, $0, $1) {
  
});
function DebugHere(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "DebugHere", DebugHere$0);
}
var InsertColon$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertColon ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: ":" };
});
function InsertColon(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertColon", InsertColon$0);
}
var InsertSemicolon$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertSemicolon ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: ";" };
});
function InsertSemicolon(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertSemicolon", InsertSemicolon$0);
}
var InsertOpenParen$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertOpenParen ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "(" };
});
function InsertOpenParen(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertOpenParen", InsertOpenParen$0);
}
var InsertCloseParen$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertCloseParen ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: ")" };
});
function InsertCloseParen(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertCloseParen", InsertCloseParen$0);
}
var InsertOpenBrace$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertOpenBrace ""'), function($skip, $loc, $0, $1) {
  return [{ $loc, token: " " }, { $loc, token: "{" }];
});
function InsertOpenBrace(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertOpenBrace", InsertOpenBrace$0);
}
var InsertInlineOpenBrace$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertInlineOpenBrace ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "{" };
});
function InsertInlineOpenBrace(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertInlineOpenBrace", InsertInlineOpenBrace$0);
}
var InsertCloseBrace$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertCloseBrace ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "}" };
});
function InsertCloseBrace(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertCloseBrace", InsertCloseBrace$0);
}
var InsertOpenBracket$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertOpenBracket ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "[" };
});
function InsertOpenBracket(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertOpenBracket", InsertOpenBracket$0);
}
var InsertCloseBracket$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertCloseBracket ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "]" };
});
function InsertCloseBracket(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertCloseBracket", InsertCloseBracket$0);
}
var InsertComma$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertComma ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "," };
});
function InsertComma(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertComma", InsertComma$0);
}
var InsertSpaceEquals$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertSpaceEquals ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: " =" };
});
function InsertSpaceEquals(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertSpaceEquals", InsertSpaceEquals$0);
}
var InsertConst$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertConst ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "const " };
});
function InsertConst(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertConst", InsertConst$0);
}
var InsertLet$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertLet ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "let " };
});
function InsertLet(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertLet", InsertLet$0);
}
var InsertReadonly$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertReadonly ""'), function($skip, $loc, $0, $1) {
  return { ts: true, children: [{ $loc, token: "readonly " }] };
});
function InsertReadonly(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertReadonly", InsertReadonly$0);
}
var InsertNewline$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertNewline ""'), function($skip, $loc, $0, $1) {
  return "\n";
});
function InsertNewline(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertNewline", InsertNewline$0);
}
var InsertIndent$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertIndent ""'), function($skip, $loc, $0, $1) {
  return state.currentIndent.token;
});
function InsertIndent(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertIndent", InsertIndent$0);
}
var InsertSpace$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertSpace ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: " " };
});
function InsertSpace(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertSpace", InsertSpace$0);
}
var InsertDot$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertDot ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "." };
});
function InsertDot(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertDot", InsertDot$0);
}
var InsertBreak$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertBreak ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: ";break;" };
});
function InsertBreak(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertBreak", InsertBreak$0);
}
var InsertVar$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertVar ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "var " };
});
function InsertVar(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertVar", InsertVar$0);
}
var InsertType$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'InsertType ""'), function($skip, $loc, $0, $1) {
  return { $loc, token: "type " };
});
function InsertType(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "InsertType", InsertType$0);
}
var CoffeeBinaryExistentialEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeBinaryExistentialEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeBinaryExistential)
    return;
  return $skip;
});
function CoffeeBinaryExistentialEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeBinaryExistentialEnabled", CoffeeBinaryExistentialEnabled$0);
}
var CoffeeBooleansEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeBooleansEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeBooleans)
    return;
  return $skip;
});
function CoffeeBooleansEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeBooleansEnabled", CoffeeBooleansEnabled$0);
}
var CoffeeClassesEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeClassesEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeClasses)
    return;
  return $skip;
});
function CoffeeClassesEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeClassesEnabled", CoffeeClassesEnabled$0);
}
var CoffeeCommentEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeCommentEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeComment)
    return;
  return $skip;
});
function CoffeeCommentEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeCommentEnabled", CoffeeCommentEnabled$0);
}
var CoffeeDoEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeDoEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeDo)
    return;
  return $skip;
});
function CoffeeDoEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeDoEnabled", CoffeeDoEnabled$0);
}
var CoffeeForLoopsEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeForLoopsEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeForLoops)
    return;
  return $skip;
});
function CoffeeForLoopsEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeForLoopsEnabled", CoffeeForLoopsEnabled$0);
}
var CoffeeInterpolationEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeInterpolationEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeInterpolation)
    return;
  return $skip;
});
function CoffeeInterpolationEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeInterpolationEnabled", CoffeeInterpolationEnabled$0);
}
var CoffeeIsntEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeIsntEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeIsnt)
    return;
  return $skip;
});
function CoffeeIsntEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeIsntEnabled", CoffeeIsntEnabled$0);
}
var CoffeeJSXEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeJSXEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeJSX)
    return;
  return $skip;
});
function CoffeeJSXEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeJSXEnabled", CoffeeJSXEnabled$0);
}
var CoffeeLineContinuationEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeLineContinuationEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeLineContinuation)
    return;
  return $skip;
});
function CoffeeLineContinuationEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeLineContinuationEnabled", CoffeeLineContinuationEnabled$0);
}
var CoffeeNotEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeNotEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeNot)
    return;
  return $skip;
});
function CoffeeNotEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeNotEnabled", CoffeeNotEnabled$0);
}
var CoffeeOfEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeeOfEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeeOf)
    return;
  return $skip;
});
function CoffeeOfEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeeOfEnabled", CoffeeOfEnabled$0);
}
var CoffeePrototypeEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'CoffeePrototypeEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.coffeePrototype)
    return;
  return $skip;
});
function CoffeePrototypeEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "CoffeePrototypeEnabled", CoffeePrototypeEnabled$0);
}
var ObjectIsEnabled$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'ObjectIsEnabled ""'), function($skip, $loc, $0, $1) {
  if (config.objectIs)
    return;
  return $skip;
});
function ObjectIsEnabled(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ObjectIsEnabled", ObjectIsEnabled$0);
}
var Reset$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'Reset ""'), function($skip, $loc, $0, $1) {
  state.indentLevels = [{
    level: 0,
    token: ""
  }];
  state.forbidClassImplicitCall = [false];
  state.forbidIndentedApplication = [false];
  state.forbidBracedApplication = [false];
  state.forbidTrailingMemberProperty = [false];
  state.forbidNewlineBinaryOp = [false];
  state.JSXTagStack = [void 0];
  state.operators = /* @__PURE__ */ new Map();
  state.helperRefs = {};
  state.prelude = [];
  config = {
    autoConst: false,
    autoVar: false,
    autoLet: false,
    coffeeBinaryExistential: false,
    coffeeBooleans: false,
    coffeeClasses: false,
    coffeeComment: false,
    coffeeDo: false,
    coffeeEq: false,
    coffeeForLoops: false,
    coffeeInterpolation: false,
    coffeeIsnt: false,
    coffeeJSX: false,
    coffeeLineContinuation: false,
    coffeeNot: false,
    coffeeOf: false,
    coffeePrototype: false,
    defaultElement: "div",
    implicitReturns: true,
    objectIs: false,
    react: false,
    solid: false,
    client: false,
    // default behavior: client only
    rewriteTsImports: true,
    server: false,
    tab: void 0,
    // default behavior = same as space
    verbose: false
  };
  Object.defineProperty(config, "deno", {
    set(b) {
      config.rewriteTsImports = !b;
    }
  });
  config.deno = typeof Deno !== "undefined";
  Object.defineProperty(config, "coffeeCompat", {
    set(b) {
      for (const option of [
        "autoVar",
        "coffeeBinaryExistential",
        "coffeeBooleans",
        "coffeeClasses",
        "coffeeComment",
        "coffeeDo",
        "coffeeEq",
        "coffeeForLoops",
        "coffeeInterpolation",
        "coffeeIsnt",
        "coffeeJSX",
        "coffeeLineContinuation",
        "coffeeNot",
        "coffeeOf",
        "coffeePrototype"
      ]) {
        config[option] = b;
      }
      if (b) {
        config.objectIs = false;
      }
    }
  });
  Object.assign(config, initialConfig);
  return {
    type: "ParserMeta",
    children: [],
    getStateKey() {
      const stateInt = state.currentIndent.level % 256 << 8 | state.classImplicitCallForbidden << 7 | state.indentedApplicationForbidden << 6 | state.bracedApplicationForbidden << 5 | state.trailingMemberPropertyForbidden << 4 | state.newlineBinaryOpForbidden << 3 | // This is slightly different than the rest of the state,
      // since it is affected by the directive prologue and may be hit
      // by the EOL rule early in the parse. Later if we wanted to
      // allow block scoping of the compat directives we would need to
      // add them all here.
      config.coffeeComment << 2;
      return [stateInt, state.currentJSXTag];
    }
  };
});
function Reset(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Reset", Reset$0);
}
var Init$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(Shebang), Prologue), function($skip, $loc, $0, $1, $2) {
  var directives = $2;
  directives.forEach((directive) => {
    if (directive.type === "CivetPrologue") {
      Object.assign(config, directive.config);
    }
  });
  return $0;
});
function Init(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Init", Init$0);
}
var Prologue$0 = (0, import_lib3.$Q)((0, import_lib3.$C)(TripleSlashDirective, (0, import_lib3.$S)((0, import_lib3.$C)(JSSingleLineComment, JSMultiLineComment), EOS), DirectivePrologue));
function Prologue(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Prologue", Prologue$0);
}
var ProloguePrefix$0 = (0, import_lib3.$S)(Prologue, (0, import_lib3.$R$0)((0, import_lib3.$EXPECT)($R94, "ProloguePrefix /[^]*/")));
function ProloguePrefix(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "ProloguePrefix", ProloguePrefix$0);
}
var Indent$0 = (0, import_lib3.$TR)((0, import_lib3.$EXPECT)($R88, "Indent /[ \\t]*/"), function($skip, $loc, $0, $1, $2, $3, $4, $5, $6, $7, $8, $9) {
  const level = getIndentLevel($0, config.tab);
  return {
    $loc,
    token: $0,
    level
  };
});
function Indent(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Indent", Indent$0);
}
var TrackIndented$0 = (0, import_lib3.$TV)(Indent, function($skip, $loc, $0, $1) {
  var indent = $0;
  const { level } = indent;
  if (level <= state.currentIndent.level) {
    return $skip;
  }
  if (config.verbose) {
    console.log("pushing indent", indent);
  }
  state.indentLevels.push(indent);
  return $1;
});
function TrackIndented(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "TrackIndented", TrackIndented$0);
}
var PushIndent$0 = (0, import_lib3.$Y)((0, import_lib3.$S)(EOS, TrackIndented));
function PushIndent(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PushIndent", PushIndent$0);
}
var PopIndent$0 = (0, import_lib3.$TV)((0, import_lib3.$EXPECT)($L0, 'PopIndent ""'), function($skip, $loc, $0, $1) {
  if (config.verbose) {
    console.log("popping indent", state.indentLevels[state.indentLevels.length - 1], "->", state.indentLevels[state.indentLevels.length - 2]);
  }
  state.indentLevels.pop();
});
function PopIndent(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "PopIndent", PopIndent$0);
}
var Nested$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(EOS, Indent), function($skip, $loc, $0, $1, $2) {
  var indent = $2;
  if (indent.level === state.currentIndent.level)
    return $0;
  if (config.verbose) {
    console.log(`failing Nested: ${indent.level} does not match current indent level ${state.currentIndent.level}`);
  }
  return $skip;
});
function Nested(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Nested", Nested$0);
}
var IndentedFurther$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(EOS, Indent), function($skip, $loc, $0, $1, $2) {
  var indent = $2;
  if (indent.level > state.currentIndent.level)
    return $0;
  return $skip;
});
function IndentedFurther(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IndentedFurther", IndentedFurther$0);
}
var IndentedAtLeast$0 = (0, import_lib3.$TS)((0, import_lib3.$S)(EOS, Indent), function($skip, $loc, $0, $1, $2) {
  var indent = $2;
  if (indent.level >= state.currentIndent.level)
    return $0;
  return $skip;
});
function IndentedAtLeast(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "IndentedAtLeast", IndentedAtLeast$0);
}
var NotDedented$0 = (0, import_lib3.$TS)((0, import_lib3.$S)((0, import_lib3.$E)(IndentedAtLeast), (0, import_lib3.$E)(_)), function($skip, $loc, $0, $1, $2) {
  const ws = [];
  if ($1)
    ws.push(...$1);
  if ($2)
    ws.push(...$2);
  return ws.flat(Infinity).filter(Boolean);
});
function NotDedented(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "NotDedented", NotDedented$0);
}
var Dedented$0 = (0, import_lib3.$T)((0, import_lib3.$S)((0, import_lib3.$N)(IndentedAtLeast), EOS), function(value) {
  return value[1];
});
function Dedented(ctx, state2) {
  return (0, import_lib3.$EVENT)(ctx, state2, "Dedented", Dedented$0);
}
var parser = function() {
  const { fail, validate, reset } = (0, import_lib3.Validator)();
  let ctx = { expectation: "", fail };
  return {
    parse: (input, options = {}) => {
      if (typeof input !== "string")
        throw new Error("Input must be a string");
      const parser2 = options.startRule != null ? grammar[options.startRule] : Object.values(grammar)[0];
      if (!parser2)
        throw new Error(`Could not find rule with name '${options.startRule}'`);
      const filename2 = options.filename || "<anonymous>";
      reset();
      Object.assign(ctx, { ...options.events, tokenize: options.tokenize });
      return validate(input, parser2(ctx, {
        input,
        pos: 0
      }), {
        filename: filename2
      });
    }
  };
}();
var { parse } = parser;
var filename;
var initialConfig;
var config;
var sync;
var state = {};
var getState = () => state;
var getConfig = () => config;
var getInitialConfig = () => initialConfig;
var getFilename = () => filename;
var getSync = () => sync;
Object.defineProperties(state, {
  currentIndent: {
    get() {
      const { indentLevels: l } = state;
      return l[l.length - 1];
    }
  },
  classImplicitCallForbidden: {
    get() {
      const { forbidClassImplicitCall: s } = state;
      return s[s.length - 1];
    }
  },
  indentedApplicationForbidden: {
    get() {
      const { forbidIndentedApplication: s } = state;
      return s[s.length - 1];
    }
  },
  bracedApplicationForbidden: {
    get() {
      const { forbidBracedApplication: s } = state;
      return s[s.length - 1];
    }
  },
  trailingMemberPropertyForbidden: {
    get() {
      const { forbidTrailingMemberProperty: s } = state;
      return s[s.length - 1];
    }
  },
  newlineBinaryOpForbidden: {
    get() {
      const { forbidNewlineBinaryOp: s } = state;
      return s[s.length - 1];
    }
  },
  currentJSXTag: {
    get() {
      const { JSXTagStack: s } = state;
      return s[s.length - 1];
    }
  }
});
function parseProgram(input, options) {
  filename = options?.filename;
  initialConfig = options?.parseOptions;
  sync = options?.sync;
  const root = parse(input, options);
  if (sync) {
    filename = initialConfig = sync = null;
    return root;
  } else {
    return processProgramAsync(root).then(() => {
      filename = initialConfig = sync = null;
      return root;
    });
  }
}

// source/sourcemap.civet
var sourcemap_exports = {};
__export(sourcemap_exports, {
  SourceMap: () => SourceMap,
  base64Encode: () => base64Encode,
  locationTable: () => locationTable,
  lookupLineColumn: () => lookupLineColumn
});
var locationTable = function(input) {
  const linesRe = /([^\r\n]*)(\r\n|\r|\n|$)/y;
  const lines = [];
  let line = 0;
  let pos = 0;
  let ref;
  while (ref = linesRe.exec(input)) {
    const result = ref;
    pos += result[0].length;
    lines[line++] = pos;
    if (pos === input.length) {
      break;
    }
  }
  return lines;
};
var lookupLineColumn = function(table, pos) {
  let l = 0;
  let prevEnd = 0;
  while (table[l] <= pos) {
    prevEnd = table[l++];
  }
  return [l, pos - prevEnd];
};
var SourceMap = function(sourceString) {
  const srcTable = locationTable(sourceString);
  const sm = {
    lines: [[]],
    line: 0,
    colOffset: 0,
    // relative to previous entry
    srcLine: 0,
    srcColumn: 0,
    srcOffset: 0,
    srcTable
  };
  const EOL2 = /\r?\n|\r/;
  return {
    data: sm,
    source: function() {
      return sourceString;
    },
    renderMappings: function() {
      let lastSourceLine = 0;
      let lastSourceColumn = 0;
      return sm.lines.map((line) => {
        return line.map((entry) => {
          if (entry.length === 4) {
            let [colDelta, sourceFileIndex, srcLine, srcCol] = entry;
            const lineDelta = srcLine - lastSourceLine;
            colDelta = srcCol - lastSourceColumn;
            lastSourceLine = srcLine;
            lastSourceColumn = srcCol;
            return `${encodeVlq(entry[0])}${encodeVlq(sourceFileIndex)}${encodeVlq(lineDelta)}${encodeVlq(colDelta)}`;
          } else {
            return encodeVlq(entry[0]);
          }
        }).join(",");
      }).join(";");
    },
    json: function(srcFileName, outFileName) {
      return {
        version: 3,
        file: outFileName,
        sources: [srcFileName],
        mappings: this.renderMappings(),
        names: [],
        sourcesContent: [sourceString],
        toString: function() {
          return JSON.stringify(this);
        }
      };
    },
    updateSourceMap: function(outputStr, inputPos, colOffset = 0) {
      const outLines = outputStr.split(EOL2);
      let srcLine, srcCol;
      if (inputPos != null) {
        [srcLine, srcCol] = lookupLineColumn(srcTable, inputPos);
        srcCol += colOffset;
        sm.srcLine = srcLine;
        sm.srcColumn = srcCol;
        sm.srcOffset = inputPos + outputStr.length;
      }
      for (let i1 = 0, len3 = outLines.length; i1 < len3; i1++) {
        const i = i1;
        const line = outLines[i1];
        if (i > 0) {
          sm.line++;
          sm.srcLine++;
          sm.colOffset = 0;
          sm.lines[sm.line] = [];
          sm.srcColumn = srcCol = colOffset;
        }
        const l = sm.colOffset;
        sm.colOffset = line.length;
        sm.srcColumn += line.length;
        if (inputPos != null) {
          sm.lines[sm.line].push([l, 0, srcLine + i, srcCol]);
        } else if (l != 0) {
          sm.lines[sm.line].push([l]);
        }
      }
      return;
    }
  };
};
var smRegexp = /\n\/\/# sourceMappingURL=data:application\/json;charset=utf-8;base64,([+a-zA-Z0-9\/]*=?=?)$/;
var remap = function(codeWithSourceMap, upstreamMap, sourcePath, targetPath) {
  let sourceMapText;
  const codeWithoutSourceMap = codeWithSourceMap.replace(smRegexp, (match, sm) => {
    sourceMapText = sm;
    return "";
  });
  if (sourceMapText) {
    const parsed = parseWithLines(sourceMapText);
    const composedLines = composeLines(upstreamMap.data.lines, parsed.lines);
    upstreamMap.data.lines = composedLines;
  }
  const remappedSourceMapJSON = upstreamMap.json(sourcePath, targetPath);
  const newSourceMap = `${"sourceMapping"}URL=data:application/json;charset=utf-8;base64,${base64Encode(JSON.stringify(remappedSourceMapJSON))}`;
  const remappedCodeWithSourceMap = `${codeWithoutSourceMap}
//# ${newSourceMap}`;
  return remappedCodeWithSourceMap;
};
var composeLines = function(upstreamMapping, lines) {
  return lines.map((line) => {
    return line.map((entry) => {
      if (entry.length === 1) {
        return entry;
      }
      const [colDelta, sourceFileIndex, srcLine, srcCol] = entry;
      const srcPos = remapPosition([srcLine, srcCol], upstreamMapping);
      if (!srcPos) {
        return [entry[0]];
      }
      const [upstreamLine, upstreamCol] = srcPos;
      if (entry.length === 4) {
        return [colDelta, sourceFileIndex, upstreamLine, upstreamCol];
      }
      return [colDelta, sourceFileIndex, upstreamLine, upstreamCol, entry[4]];
    });
  });
};
var parseWithLines = function(base64encodedJSONstr) {
  const json = JSON.parse(Buffer.from(base64encodedJSONstr, "base64").toString("utf8"));
  let sourceLine = 0;
  let sourceColumn = 0;
  const lines = json.mappings.split(";").map((line) => {
    if (line.length === 0) {
      return [];
    }
    return line.split(",").map((entry) => {
      const result = decodeVLQ(entry);
      switch (result.length) {
        case 1: {
          return [result[0]];
        }
        case 4: {
          return [result[0], result[1], sourceLine += result[2], sourceColumn += result[3]];
        }
        case 5: {
          return [result[0], result[1], sourceLine += result[2], sourceColumn += result[3], result[4]];
        }
        default: {
          throw new Error("Unknown source map entry", result);
        }
      }
    });
  });
  json.lines = lines;
  return json;
};
Object.assign(SourceMap, { remap, parseWithLines, composeLines });
var VLQ_SHIFT = 5;
var VLQ_CONTINUATION_BIT = 1 << VLQ_SHIFT;
var VLQ_VALUE_MASK = VLQ_CONTINUATION_BIT - 1;
var encodeVlq = function(value) {
  let answer = "";
  let ref1;
  if (value < 0)
    ref1 = 1;
  else
    ref1 = 0;
  const signBit = ref1;
  let valueToEncode = (Math.abs(value) << 1) + signBit;
  while (valueToEncode || !answer) {
    let nextChunk = valueToEncode & VLQ_VALUE_MASK;
    valueToEncode = valueToEncode >> VLQ_SHIFT;
    if (valueToEncode) {
      nextChunk |= VLQ_CONTINUATION_BIT;
    }
    answer += encodeBase64(nextChunk);
  }
  return answer;
};
var BASE64_CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
var encodeBase64 = function(value) {
  return BASE64_CHARS[value] || (() => {
    throw new Error("Cannot Base64 encode value: ${value}");
  })();
};
var base64Encode = function(src) {
  return Buffer.from(src).toString("base64");
};
var vlqTable = new Uint8Array(128);
var vlqChars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
{
  let i = 0;
  let l = vlqTable.length;
  while (i < l) {
    vlqTable[i] = 255;
    i++;
  }
  i = 0;
  l = vlqChars.length;
  while (i < l) {
    vlqTable[vlqChars.charCodeAt(i)] = i;
    i++;
  }
}
var decodeError = function(message) {
  throw new Error(message);
};
var decodeVLQ = function(mapping) {
  let i = 0;
  let l = mapping.length;
  let result = [];
  while (i < l) {
    let shift = 0;
    let vlq = 0;
    let v = 0;
    while (true) {
      if (i >= l) {
        decodeError("Unexpected early end of mapping data");
      }
      const c = mapping.charCodeAt(i);
      if ((c & 127) != c) {
        decodeError("Invalid mapping character: ${JSON.stringify(String.fromCharCode(c))}");
      }
      const index = vlqTable[c & 127];
      if (index === 255) {
        decodeError("Invalid mapping character: ${JSON.stringify(String.fromCharCode(c))}");
      }
      i++;
      vlq |= (index & 31) << shift;
      shift += 5;
      if ((index & 32) === 0) {
        break;
      }
    }
    if (vlq & 1) {
      v = -(vlq >> 1);
    } else {
      v = vlq >> 1;
    }
    result.push(v);
  }
  return result;
};
var remapPosition = function(position, sourcemapLines) {
  const [line, character] = position;
  const textLine = sourcemapLines[line];
  if (!textLine?.length) {
    return void 0;
  }
  let i = 0;
  let p = 0;
  const l = textLine.length;
  let lastMapping = void 0;
  let lastMappingPosition = 0;
  while (i < l) {
    const mapping = textLine[i];
    p += mapping[0];
    if (mapping.length === 4) {
      lastMapping = mapping;
      lastMappingPosition = p;
    }
    if (p >= character) {
      break;
    }
    i++;
  }
  if (character - lastMappingPosition != 0) {
    return void 0;
  }
  if (lastMapping) {
    return [lastMapping[2], lastMapping[3]];
  } else {
    return void 0;
  }
};

// source/state-cache.civet
var StateCache = class {
  cache = /* @__PURE__ */ new Map();
  get(key) {
    return this.cache.get(key[0])?.get(key[1])?.get(key[2])?.get(key[3]);
  }
  /**
   * Check if this multi-layer cache has the given key.
   * Since the intermediate layers are always other maps we only need to check the last layer.
   */
  has(key) {
    return !!this.cache.get(key[0])?.get(key[1])?.get(key[2])?.has(key[3]);
  }
  set(key, value) {
    const cache0 = this.cache;
    let cache1;
    if (!cache0.has(key[0])) {
      cache1 = /* @__PURE__ */ new Map();
      this.cache.set(key[0], cache1);
    } else {
      cache1 = cache0.get(key[0]);
    }
    let cache2;
    if (!cache1?.has(key[1])) {
      cache2 = /* @__PURE__ */ new Map();
      cache1.set(key[1], cache2);
    } else {
      cache2 = cache1.get(key[1]);
    }
    let cache3;
    if (!cache2.has(key[2])) {
      cache3 = /* @__PURE__ */ new Map();
      cache2.set(key[2], cache3);
    } else {
      cache3 = cache2.get(key[2]);
    }
    cache3.set(key[3], value);
  }
};

// source/main.civet
var { SourceMap: SourceMap2 } = sourcemap_exports;
var ParseErrors = class extends Error {
  name = "ParseErrors";
  errors;
  constructor(errors) {
    const message = errors.map(($) => $.message).join("\n");
    super(errors.map(($1) => $1.message).join("\n"));
    this.message = message;
    this.errors = errors;
  }
};
var uncacheable = /* @__PURE__ */ new Set([
  // Meta
  "DebugHere",
  "Init",
  "Program",
  "Reset",
  // Indentation
  // We need to no-cache the state modifying rules up to the point where they
  // balance within a parent so PushIndent needs to be marked no-cache even
  // though it only calls TrackIndented which does the actual work.
  "PushIndent",
  "PopIndent",
  "TrackIndented",
  // JSX
  "PushJSXOpeningElement",
  "PushJSXOpeningFragment",
  "PopJSXStack",
  // State
  "AllowAll",
  "AllowClassImplicitCall",
  "AllowBracedApplication",
  "AllowIndentedApplication",
  "AllowMultiLineImplicitObjectLiteral",
  "AllowNewlineBinaryOp",
  "AllowTrailingMemberProperty",
  "ForbidClassImplicitCall",
  "ForbidBracedApplication",
  "ForbidIndentedApplication",
  "ForbidMultiLineImplicitObjectLiteral",
  "ForbidNewlineBinaryOp",
  "ForbidTrailingMemberProperty",
  "RestoreAll",
  "RestoreClassImplicitCall",
  "RestoreMultiLineImplicitObjectLiteral",
  "RestoreBracedApplication",
  "RestoreIndentedApplication",
  "RestoreTrailingMemberProperty",
  "RestoreNewlineBinaryOp"
]);
function compile(src, options) {
  if (!options) {
    options = {};
  } else {
    options = { ...options };
  }
  options.parseOptions = { ...options.parseOptions };
  const filename2 = options.filename || "unknown";
  if (filename2.endsWith(".coffee") && !/^(#![^\r\n]*(\r\n|\n|\r))?\s*['"]civet/.test(src)) {
    options.parseOptions.coffeeCompat = true;
  }
  const { hits, trace, noCache } = options;
  let events;
  if (!noCache) {
    events = makeCache({
      hits: !!hits,
      trace: !!trace
    });
  }
  let ast;
  try {
    ast = parseProgram(src, {
      parseOptions: options.parseOptions,
      sync: options.sync,
      filename: filename2,
      events
    });
  } finally {}
  const throwOnErrors = options.errors == null;
  function rest(ast2) {
    options = options;
    if (!(options.ast === "raw")) {
      ast2 = prune(ast2);
    }
    if (options.ast) {
      return ast2;
    }
    function checkErrors() {
      if (!throwOnErrors) {
        return;
      }
      options = options;
      if (options.errors?.length) {
        throw new ParseErrors(options.errors);
      }
      ;
      return;
    }
    if (options.sourceMap || options.inlineMap) {
      options.sourceMap = SourceMap2(src);
      const code = generate_default(ast2, options);
      checkErrors();
      if (options.inlineMap) {
        return SourceMap2.remap(code, options.sourceMap, filename2, filename2 + ".tsx");
      } else {
        return {
          code,
          sourceMap: options.sourceMap
        };
      }
    }
    const result = generate_default(ast2, options);
    if (options.errors?.length) {
      delete options.errors;
      options.sourceMap = SourceMap2(src);
      generate_default(ast2, options);
      checkErrors();
    }
    return result;
  }
  if (ast.then != null) {
    return ast.then(rest);
  } else {
    return rest(ast);
  }
}
var makeCache = function({ hits, trace } = {}) {
  const meta = {};
  let hitCount;
  if (hits) {
    hitCount = /* @__PURE__ */ new Map();
    meta.hits = hitCount;
  }
  let logs;
  if (trace) {
    logs = [];
    meta.logs = logs;
  }
  const stateCache = new StateCache();
  let getStateKey = null;
  const stack = [];
  const events = {
    meta,
    enter: function(ruleName, state2) {
      if (hits) {
        hitCount.set(ruleName, (hitCount.get(ruleName) || 0) + 1);
      }
      if (uncacheable.has(ruleName)) {
        return;
      }
      const [stateKey, tagKey] = getStateKey();
      const key = [tagKey, stateKey, state2.pos, ruleName];
      if (stateCache.has(key)) {
        if (trace) {
          logs.push("".padStart(stack.length * 2, " ") + ruleName + ":" + state2.pos + "\u{1F4B0}");
        }
        const result = stateCache.get(key);
        return {
          cache: result ? { ...result } : void 0
        };
      }
      if (trace) {
        logs.push("".padStart(stack.length * 2, " ") + ruleName + ":" + state2.pos + "\u2192");
        stack.push(ruleName);
      }
      return;
    },
    exit: function(ruleName, state2, result) {
      if (ruleName === "Reset") {
        ({ getStateKey } = result.value);
      }
      if (!uncacheable.has(ruleName)) {
        const [stateKey, tagKey] = getStateKey();
        const key = [tagKey, stateKey, state2.pos, ruleName];
        stateCache.set(key, result);
      }
      if (getConfig().verbose && result) {
        console.log(`Parsed ${JSON.stringify(state2.input.slice(state2.pos, result.pos))} [pos ${state2.pos}-${result.pos}] as ${ruleName}`);
      }
      if (trace) {
        stack.pop();
        logs.push("".padStart(stack.length * 2, " ") + ruleName + ":" + state2.pos + " " + (result ? "\u2705" : "\u274C"));
      }
      return;
    }
  };
  return events;
};
var isCompileError = function(err) {
  return err instanceof import_lib3.ParseError || err instanceof ParseErrors;
};
var main_default = { parse, parseProgram, ParseError: import_lib3.ParseError, ParseErrors, generate: generate_default, sourcemap: sourcemap_exports, SourceMap: SourceMap2, compile, isCompileError };
// Annotate the CommonJS export names for ESM import in node:

if(this.__to__compile__){
  compile(this.__to__compile__, { sync: true, ...(this.__compile__options || { parseOptions: { coffeeCompat: true } }) });
};
  "#)
}

