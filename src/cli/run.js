const path = require('path');
const { run } = require('../main');
const { watch } = require('fs');


function exec(filePath){
  return run(filePath)
    .context.module.imports;
}


process.on('message', ({ filePath, watch }) => {
  const imports = exec(filePath);
  if(watch){
    process.send(imports);
    process.exit();
  } else {
    process.exit();
  }
});