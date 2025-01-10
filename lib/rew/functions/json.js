const jsYaml = require("js-yaml");
const { yamlFile } = require("../modules/yaml");



function json(thing){
  return JSON.parse(thing);
}

function jsons(thing){
  return JSON.stringify(thing);
}


function yaml(thing, ...schema){
  return yamlFile({
    content: thing,
    path: ''
  }, schema);
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