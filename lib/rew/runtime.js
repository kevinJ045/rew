(function () {
  const _log_out = console.log;
  const _err_out = console.error;

  const _createClass = (items) => {
    return {
      prototype: items
    }
  }

  delete globalThis.console;
  globalThis.rew = _createClass({
    io: _createClass({
      out: {
        ...globalThis.Deno.stdout,
        print(...a) {
          return _log_out(...a);
        },
        err(...a) {
          return _err_out(...a);
        }
      },
      "in": {
        ...globalThis.Deno.stdin
      }
    }),
    ops: {
      ...globalThis.Deno.core.ops
    }
  });
  delete globalThis.Deno;
  globalThis.inc = function (path) {
    return rew.prototype.ops.op_inc(path);
  };
})();