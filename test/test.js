const { run } = require("../src/main");
const path = require('path');

function test(){
  run(path.resolve(__dirname, './coffee/conf.coffee'))
}

test();