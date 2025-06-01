
using compiler::autoLet.disable
using public compiler::autoVar
using public JSX, () => {}

print private "s"

import "./jsx.coffee";
using namespace rew::ns;

let f = {}

package ss;
print f.package

function something()
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
