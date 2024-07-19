import '#std'
using namespace std::ns ->
  define Main class
    @main: (argv) ->
      typef(str) brew = (coffee, sugar) ->
        coffee + sugar + 'ml'

      int @coffee = '30ml'

      sugar = input 'How much sugar(g)? ' |> int
      print 'Cup of coffee', brew @coffee, sugar