
module.exports = () => ((styles) => {
  let css = "";
  const variables = {};
  const mixins = {};

  const parseValue = (v, self, name, vars) => {
    const vs = {...variables, ...vars, name, ...Object.fromEntries(Object.entries(self).map(i => ['prop_'+i[0], i[1]]))};
    return v.startsWith('@v') ? vs[v.split('@v')[1].trim()] : v
  .toString()
  .replace(/\$([A-Za-z0-9-_]+)/g, (_, name) => (vs)[name])
  }

  const declectProp = (value, name, selector, start, parent, self, vars = {}) => {
    let prop = `${name}: `;
    self.__parent = parent;
    if (typeof value == 'object') {
      prop += parseValue(value.default || 'unset', self, name, vars) + ';';
      prop += '}\n';
      for (let state in value) {
        if (state == 'default') continue;
        else if (state.startsWith('@media')) {
          prop += `${state} { ${start} ${name}: ${parseValue(value[state], self, name, vars)}; } }\n`
        } else if (state.startsWith(':')) {
          prop += `${selector}${state} { ${name}: ${parseValue(value[state], self, name, vars)}; }\n`
        }
      }
      prop += start;
    } else {
      let v = parseValue(value, self, name, vars);
      if(typeof v == 'object') return declectProp(v, name, selector, start, parent, self, vars);
      prop += v + ';';
    }
    return prop;
  }

  const declectNames = (names, $name, parent, values = {}) => {
    const start = $name ? $name + ' {' : '';
    let nameStyles = start;
    for (let name in names) {
      let selector = name.replace(/,/g, ', &').replace(/&/g, $name || '');
      if (name == '@variables') {
        for (let i in names[name]) {
          variables[i] = names[name][i];
        }
      } else if (name.startsWith('@mixin')) {
        const mame = name.split('@mixin')[1].trim();
        const mname = mame.split('(')[0];
        const args = mame.replace(mname, '').slice(1, -1);
        mixins[mname] = {
          args: args.split(','),
          value: names[name]
        }
      } else if (name.startsWith('@keyframes')) {
        nameStyles += $name ? '' : name + '{';

        for(let keyFrame in names[name]){
          nameStyles += declectNames(names[name][keyFrame], keyFrame, names[name], values);
        }

        if(!$name) nameStyles += '}\n';
      } else if ($name) {
        if (name.startsWith('&')) {
          nameStyles += '}\n';
          nameStyles += declectNames(names[name], selector, names, values);
          nameStyles += start;
        } else if(name.startsWith('@include')) {
          const mame = names[name];
          const mname = mame.split('(')[0];
          const args = mame.replace(mname, '').slice(1, -1).split(',');
          if(mixins[mname]){
            nameStyles += declectNames(mixins[mname].value, selector, names, {...values, ...Object.fromEntries(mixins[mname].args.map((n, i) => [n, args[i]]))}).trim().replace('@include ', '').slice(1, -1);
          }
        } else {
          nameStyles += declectProp(names[name], name, $name, start, parent || styles, names, values);
        }
      } else {
        nameStyles += declectNames(names[name], selector, names, values);
      }
    }
    if ($name) nameStyles += '}\n';
    return nameStyles;
  }

  css += declectNames(styles);

  return css.replace(/(.+) \{\}/g, '');
})