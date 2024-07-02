const { run } = require("../main");

if(process.argv[2]){
  run(process.argv[2]);
} else {

  let data = '';
  process.stdin.setEncoding('utf8');

  process.stdin.on('data', (chunk) => {
    data += chunk;
  });

  process.stdin.on('end', () => {
    if(data.length) run('', {
      code: data
    });
  });

}
