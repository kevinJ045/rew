import '#std'

passed = 0
failed = 0
taken = 0

using namespace std::ns ->
  define Main class Test

    @evaluate_value: (type, value) -> { type: type, value: value }

    @name: (name) -> @evaluate_value 'name', name
    @files: (...files) -> @evaluate_value 'files', files
    @expect: (expect) -> @evaluate_value 'expect', expect
    @inputFor: (query) -> {
      as: (value) => @evaluate_value 'inputFor', { query, value }
    }
    
    @test: (items) ->
      data = {
        description: ''
        name: ''
        expect: ''
        sin: {}
      }
      

      items.forEach (item) =>
        if item.type is 'name'
          data.description = item.value
        else if item.type is 'files'
          data.name = item.value[0].split('.coffee')[0]
        else if item.type is 'expect'
          data.expect = item.value
        else if item.type is 'inputFor'
          data.sin[item.value.query] = item.value.value
          
      return @testRaw data.description, data.name, data.expect, data.sin
    
    @testRaw: (description, name, expect = '', sin = {}) ->
      new Promise (resolve, reject) ->
        text = '%c--> [%b%!!%!%b] %bTest%! ' + description + '%! On Progress...'
        if text.length > std::out.cols
          text = text.slice(0, std::out.cols - 1)
        cb = (err, stdout) ->
          taken++
          result = stdout.split('\n').slice(3).join('\n').trim()
          printf '\r' + ' '.repeat(text.length)
          success = if expect then (
            if result.startsWith('^') then expect.trim().match(result.slice(1)) else result == expect.trim()) else true
          if err or not success
            print result
            printf '%c\r--> [%r%!X%!%r] %rTest%! ' + description + '%! failed\n'
            if err
              print err.toString().split('\n').map((i) => '\t' + i).join('\n')
            else if not success
              printf '%c%r\t -> Result not expected value'
            failed++
            reject()
          else
            printf '%c\r--> [%g%!-%!%g] %gTest%! ' + description + '%! done\n'
            passed++
            resolve()
        printf text
        proc = exec.background "npm run test #{name}", cb
        if Object.keys(sin).length
          proc.stdout.on 'data', (data) ->
            if sin[data]
              proc.stdin.write sin[data] + '\n'

    @main: (argv) ->

      wait @test [
        @name 'Print and Input'
        @files 'print.coffee'
        @expect 'num 1: num 2: 8'
        @inputFor('num 1: ').as('3')
        @inputFor('num 2: ').as('5')
      ]

      wait @test [
        @name 'Augumented Argumented Functions'
        @files 'aug.coffee'
        @expect 'aa'
      ]

      wait @test [
        @name 'Namespaces'
        @files 'f.coffee'
        @expect '1 find_out'
      ]

      wait @test [
        @name 'Conf Module'
        @files 'conf.coffee'
      ]

      wait @test [
        @name 'Shared Context'
        @files 'ctx.coffee', 'ctx2.coffee'
        @expect 'hi'
      ]

      wait @test [
        @name 'Phantom Syntax Declarators'
        @files 'dec.coffee'
        @expect "1122 1234\n{ name: '', something: '' }\ntrue"
      ]

      wait @test [
        @name 'Import Export'
        @files 'imp.coffee'
      ]

      wait @test [
        @name 'Map'
        @files 'map.coffee'
      ]

      wait @test [
        @name 'Match Statement'
        @files 'match.coffee'
        @expect 'huh\nkkk\nsss\nhello'
      ]

      wait @test [
        @name 'Multiple Rew Syntaxes'
        @files 'ns.coffee'
        @expect 'How much sugar(g)? Cup of coffee 32ml'
        @inputFor('How much sugar(g)? ').as('2')
      ]

      wait @test [
        @name 'Structs'
        @files 'other.coffee'
        @expect 'Sean is 60\nMakano is 19'
      ]

      wait @test [
        @name 'Type Functions'
        @files 'p.coffee'
        @expect 'a 2'
      ]

      wait @test [
        @name 'Native Require Functions'
        @files 'req.coffee'
      ]

      wait @test [
        @name 'Path Joining'
        @files 's.coffee'
        @expect '/helo'
      ]

      wait @test [
        @name 'Threads'
        @files 'toor.coffee'
        @expect "{ data: 'Hello' }\n{ data: 'smn' } back"
      ]

      wait @test [
        @name 'Types'
        @files 'types.coffee'
        @expect 'true\nfalse'
      ]

      wait @test [
        @name 'Yaml'
        @files 'yaml.coffee'
      ]

      wait @test [
        @name 'Wait Directive'
        @files 'directives.coffee'
        @expect [10..0].join('\n')
      ]

      print "%c\n\n  %gTests Passed: %!#{passed} / %b#{taken}%b%!\n  %rTests Failed: %!#{failed} / %y#{taken}%y%!"
