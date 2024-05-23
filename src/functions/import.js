const path = require('path');


module.exports.imp = function(runPath, context){
  return function(filename, options = {}){
    let type = 'coffee';
    let exports;

    // console.log(typeof runPath);

    if(type == 'coffee'){
      exports = runPath(path.resolve(path.dirname(context.module.filepath), filename), { ...options, useContext: true }, context).context.module.exports;
    } else if(type == 'js'){
      exports = runPath(path.resolve(path.dirname(context.module.filepath), filename), { ...options, useContext: true, compile: false }, context).context.module.exports;
    }

    if(options.save) {
      if(typeof options.save == 'string') context[options.save] = exports[i];
      else for(let i in exports){
        context[i] = exports[i];
      }
    }

    return exports;
  }
}