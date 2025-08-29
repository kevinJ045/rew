
using compiler::autoLet.disable
using public compiler::autoVar
using public JSX, () => {}

import "#std.types";

using namespace rew::ns

print rew::types::macro.child "sss::DDD"
print rew::types::macro.parent "sss.DDD"
print rew::types::macro.child "sss.DDD.ddd"
print rew::types::macro.parts "sss.DDD.ddd"

print private "s"

# this is a comment
# this is another comment

import "./jsx.test.coffee";

let f = {}

package ss;
print f.package

function macro(_, _fn)
  return (...args) ->
    fn = args.pop()
    full_args = args.length == 1 and args[0] == null ? [] : args
    return _fn fn, ...full_args

@{macro}
function macro_function(fn)
  # some comment
  return fn

export { macro }



@{macro_function}
function Something_else()
  @something = "a"


smn = typedef Something_else
@{proto.strict, [
  str.or(null) # param at index 0 can be
], smn}
function Something_else::addStrings(str1): Something_else
  @something += str1
  @


let someinstance = new Something_else()


function something()
  print 'ss'

function something.staticfn()
  print 'ss'

function something::new()
  print 'sss'

function something::smn(): sss
  print 'sss'

sss = <><div /></>

print f.export
export default f = 'gg'
export class f
export f = 'ss'

l = 'ss'

print someinstance.addStrings "stuff"
print someinstance.addStrings null

