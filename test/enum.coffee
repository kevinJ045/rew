import "#std.types!";
using namespace rew::ns;

enum Animal {
  Cat
  Dog
  Capibara
}

isItCat = (a: Animal) -> match(a)
  .enum(Animal)
  .on 'Cat', -> print('It\'s a cat') ?? true
  .on Animal.Dog, -> print('It\'s a dog') ?? false
  .default -> print('Who knows') ?? false
  .end

isItCat Animal.Cat
isItCat Animal.Dog
isItCat Animal.Capibara