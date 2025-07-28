import "#std.types!";
using namespace rew::ns;

using JSX, (element, props, ...children) => typeof element == "function" ? element(props, ...children) : {name: element, ...props, children}

# Test code 
ssss = instantiate class
  sssss = "ssss"

print ssss

print 1 < 2
print 1 > 2
print 1 >= 2
print 1 <= 2
print 1 < 2

Example = (props) => <><div sksk="sss" {...props}></div></>
s = ['s', 's', 'd']

name = <><div jdj="sss">
  <p>sjsjsjs</p>
  <div>
    {s.map (i) => <p>{i}</p>}
  </div>
  <Example id="sss" {...ssss} />
</div></>

print name
