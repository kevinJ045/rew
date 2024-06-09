const jsYaml = require("js-yaml");



function json(thing){
  return JSON.parse(thing);
}

function jsons(thing){
  return JSON.stringify(thing);
}


function yaml(thing){
  return jsYaml.loadAll(thing)[0];
}

function yamls(thing){
  return jsYaml.dump(thing);
}

module.exports = {
  yaml,
  yamls,
  json,
  jsons
}