"no-compile"
if(!rew.extensions.has('yaml')) rew.extensions.add('yaml', (Deno, module) => rew.extensions.createClass({
  parse(string){
    return Deno.core.ops.op_string_to_yaml(string);
  },
  string(yaml){
    return Deno.core.ops.op_yaml_to_string(yaml);
  },
})); 