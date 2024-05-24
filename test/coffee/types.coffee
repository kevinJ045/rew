


str = typedef ''
num = typedef 5
bool = typedef true

complexObject = typedef { name: str, age: num }
anyObject = typedef {} 

class N
  constructor: () -> 
    @age = ''

class M extends N
  age = ''
  
f = { s: 'ss' }

classedType = typedef N

n = new N

print(typeis n, classedType)

fn = typedef () -> ''

fmm = () -> 'ss'

print(typeis fmm, fn)