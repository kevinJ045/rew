
function augument(func){
  return function(...args){
    const c = {args: []};
    let foundKeys = false;
    
    args.forEach((o) => {
      if(typeof o == "object"){
        if(!foundKeys) {
          for(let i in o){
            c[i] = o[i];
          }
          foundKeys = true;
        } else {
          c.args.push(o);
        }
      } else {
        c.args.push(o);
      }
    });

    return func.call(c);
  }
}

module.exports = augument;