const future = require("./future");
const fs = require('fs');

module.exports.curl = function curl(options, url){
  if(typeof options == 'string'){
    let newurl = options;
    options = typeof url == "object" ? url : {};
    url = newurl;
  }
  if(options.url && !url){
    url = options.url
  }
  const method = options.x || "GET";
  const f = future.promise(fetch(url, {
    ...options,
    method
  }).then(async r => {
    if(options.o) fs.writeFileSync(options.o, Buffer.from(await r.clone().arrayBuffer()));
    return r;
  }).then(r => options.json ? r.clone().json() : r));
  if(options.a) return f.wait();
  else return f;
}