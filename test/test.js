const { run } = require("../lib/rew/main");
const path = require('path');

function test(){
  run(path.resolve(__dirname, './coffee/'+process.argv[2]+'.coffee'))
}

test();