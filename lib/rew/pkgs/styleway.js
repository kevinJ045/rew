
function normalizeCamelCase (camelCase) {
  let all = "";
  for(let i of camelCase){
    if(i.match(/[A-Z]/)){
      all += '-';
      i = i.toLowerCase();
    }
    all += i;
  }
  if(all.startsWith('-')) all = all.slice(1);
  return all;
}
const known_states = [
  'hover',          
  'active',         
  'focus',          
  'focus-within',   
  'focus-visible',  
  'visited',        
  'link',           
  'disabled',       
  'checked',        
  'unchecked',      
  'first-child',    
  'last-child',     
  'nth-child',      
  'nth-of-type',    
  'empty',          
  'not',
  'before',
  'after',
  'first-of-type',  
  'last-of-type',
  'only-child',    
  'only-of-type',   
  'valid',          
  'invalid',        
  'in-range',       
  'out-of-range',   
  'required',       
  'optional',       
  'disabled',       
  'enabled',        
  'enabled',        
  'default',        
  'selection',      
  'target',         
  'lang',           
  'dir',            
];


const parseValue = (O, v, self, name, vars) => {
  const vs = {...O.variables, ...vars, name, ...Object.fromEntries(Object.entries(self).map(i => ['prop_'+i[0], i[1]]))};
  return v.startsWith('@v') ? vs[v.split('@v')[1].trim()] : v
    .toString()
    .replace(/\$([A-Za-z0-9-_]+)/g, (_, name) => (vs)[name])
}

const declectProp = (O, value, name, selector, start, parent, self, vars = {}) => {
  let prop = "";
  self.__parent = parent;
  if (typeof value == 'object') {
    for (let state in value) {
      if (state == 'default') {
        prop += `\n${selector}{ ${name}: ${parseValue(O, value.default || 'unset', self, name, vars) + ';'} }`;
      } else if (state.startsWith('@media')) {
        prop += `\n${state} { ${selector}: ${parseValue(O, value[state], self, name, vars)}; }`
      } else if (state.startsWith(':') || known_states.includes(state)) {
        prop += `\n${selector}${!state.startsWith(':') ? ':' : ''}${state} { ${name}: ${parseValue(O, value[state], self, name, vars)}; }`
      }
    }
  } else {
    prop += `\n${selector}{ ${name}: `;
    let v = parseValue(O, value, self, name, vars);
    if(typeof v == 'object') return declectProp(v, name, selector, start, parent, self, vars);
    prop += v + ';';
    prop += '}\n';
  }
  return prop;
}

const declectNames = (O, names, $name, parent, values = {}, free = true) => {
  const start = $name && free ? $name + ' {' : '';
  let nameStyles = "";
  for (let name in names) {
    let selector = name.replace(/,/g, ', &').replace(/&/g, $name || '');
    if (name == '@variables') {
      for (let i in names[name]) {
        O.variables[i] = names[name][i];
      }
    } else if (name.startsWith('@mixin')) {
      const mame = name.split('@mixin')[1].trim();
      const mname = mame.split('(')[0];
      const args = mame.replace(mname, '').slice(1, -1);
      O.mixins[mname] = {
        args: args.split(','),
        value: names[name]
      }
    } else if (name.startsWith('@keyframes')) {
      nameStyles += $name ? '' : name + '{';

      for(let keyFrame in names[name]){
        nameStyles += declectNames(O, names[name][keyFrame], keyFrame, names[name], values);
      }

      if(!$name) nameStyles += '}\n';
    } else if ($name) {
      if (name.startsWith('&')) {
        nameStyles += '}\n';
        nameStyles += declectNames(O, names[name], selector, names, values);
        nameStyles += start;
      } else if(name.startsWith('@include')) {
        const mame = names[name];
        const mname = mame.split('(')[0];
        const args = mame.replace(mname, '').slice(1, -1).split(',');
        if(O.mixins[mname]){
          nameStyles += declectNames(O, O.mixins[mname].value, $name, parent, {...values, ...Object.fromEntries(O.mixins[mname].args.map((n, i) => [n, args[i]]))}, false).trim();
        }
      } else {
        nameStyles += declectProp(O, names[name], normalizeCamelCase(name), $name, start, parent || styles, names, values);
      }
    } else {
      nameStyles += declectNames(O, names[name], selector, names, values);
    }
  }
  return nameStyles;
}

const collectAndCleanUp = (css) => {
  const selectorMap = {};
  const cleanedCSS = [];

  const lines = css.split('\n');

  lines.forEach(line => {
    const match = line.match(/([^\{]+){(.+)}/);
    if (match) {
      const selector = match[1].trim();

      if (!selectorMap[selector]) {
        selectorMap[selector] = "";
      }

      selectorMap[selector] += match[2];
    }
  });

  for (const selector in selectorMap) {
    const selectorBlock = selectorMap[selector];
    cleanedCSS.push(`${selector} { ${selectorBlock} }`);
  }

  return cleanedCSS.join('\n');
};


module.exports = () => ((styles) => {
  let css = "";
  const variables = {};
  const mixins = {};

  css += declectNames({
    variables,
    mixins
  }, styles);

  return collectAndCleanUp(css);
})