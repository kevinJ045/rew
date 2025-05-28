import "#std.types!";
using namespace rew::ns;

_person = struct {
  name: str,
  age: int
}

_not_person = struct {
  kk: str
}

sample = _person::new name: 'ss', age: 4

match(sample)
  .on _not_person, -> print('Matched person:', sample.name, sample.age)
  .default -> print 'nope'
  .end


person = instantiate class
  str name = input('Name: ')
  int age = input('Age: ')

print person.name, 'is', person.age, 'years old.'

match(person.name)
  .on 'Makano', -> print('Hello, Makano!')
  .on /jo/i, -> print('Hello, John!')
  .default -> print('Hello, stranger!')  
  .end
