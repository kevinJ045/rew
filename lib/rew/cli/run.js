const { run } = require('../main');


function exec(filePath){
  return run(filePath)
    .context.module.imports;
}

const onmsg = ({ filePath, watch }) => {
  const imports = exec(filePath);
  if(watch){
    process.send(imports);
  }
  process.off('message', onmsg);
}

process.on('message', onmsg);