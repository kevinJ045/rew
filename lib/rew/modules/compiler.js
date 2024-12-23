const { compile: compileCivet } = require('../../civet/main');
const { execOptions } = require('../const/opt');
const { findAppInfo } = require('../misc/findAppInfo');
const { from_qrew } = require('../qrew/compile');
const { getFile, file } = require('./fs');
const babel = require('@babel/core');
const path = require('path');
const babelReact = require('@babel/preset-react');
const { readFileSync, existsSync } = require('fs');
const { wait } = require('../functions/wait');
const { REW_FILE_TYPE } = require('../const/ext');
const { USING_DEFAULT } = require('../const/usage');
const { CONFIG_PATH } = require('../const/config_path');
const { straceLog } = require('../misc/strace');



function tokenizeCoffeeScript(code) {
  const tokens = [];
  let currentToken = '';
  let i = 0;

  while (i < code.length) {
    const char = code[i];
    const prevChar = code[i - 1];
    const nextChar = code[i + 1];
    const nextNextChar = code[i + 2];

    if (char === '#') {
      const commentEnd = code.indexOf('\n', i);
      const comment = code.substring(i, commentEnd < 0 ? code.length : commentEnd + 1);
      tokens.push({ type: 'COMMENT', value: comment });
      i += comment.length - 1;
    } else if (char === '"' && nextChar === '"' && nextNextChar === '"') {
      let string = '"""';
      i += 3;
      while (i < code.length && !(code[i] === '"' && code[i + 1] === '"' && code[i + 2] === '"')) {
        string += code[i];
        i++;
      }
      string += '"""';
      tokens.push({ type: 'TRIPLE_STRING', value: string });
      i += 2;
    } else if (char === '"' || char === "'") {
      let string = char;
      let escaped = false;
      i++;
      while (i < code.length && (code[i] !== char || escaped)) {
        string += code[i];
        if (code[i] === '\\' && !escaped) {
          escaped = true;
        } else {
          escaped = false;
        }
        i++;
      }
      string += char;
      tokens.push({ type: 'STRING', value: string });
    } else if (char === '/' && nextChar !== ' ' && nextChar !== '/' && nextChar !== '*' && prevChar !== '<') {
      let regex = char;
      i++;
      while (i < code.length && (code[i] !== '/' || regex.endsWith('\\'))) {
        regex += code[i];
        i++;
      }
      regex += '/';
      tokens.push({ type: 'REGEX', value: regex });
    } else if (/\s/.test(char)) {
      if (tokens[tokens.length - 1]?.type === 'WHITESPACE') {
        tokens[tokens.length - 1].value += char;
      } else {
        tokens.push({ type: 'WHITESPACE', value: char });
      }
    } else if (/[a-zA-Z_$@]/.test(char)) {
      let identifier = char;
      i++;
      while (i < code.length && /[a-zA-Z0-9_$]/.test(code[i])) {
        identifier += code[i];
        i++;
      }
      tokens.push({ type: 'IDENTIFIER', value: identifier });
      i--;
    } else if (/[a-f0-9.xn]/.test(char)) {
      let num = char;
      i++;
      while (i < code.length && /[a-f0-9.nx]/.test(code[i])) {
        num += code[i];
        i++;
      }
      tokens.push({ type: 'NUMBER', value: num });
      i--;
    } else {
      tokens.push({ type: 'OTHER', value: char });
    }
    i++;
  }

  return tokens;
}

const ValueIfy = (val) => {
  if(!isNaN(parseFloat(val)) || !isNaN(parseInt(val))){
    return isNaN(parseInt(val)) ? parseFloat(val) : parseInt(val);
  } if(val == 'true' || val == 'false') {
    return val == 'true' ? true : false;
  } else {
    return JSON.stringify(val);
  }
}

function insertTokenAt(array, index, value) {
	if (index < 0 || index > array.length) {
			throw new RangeError('Index out of bounds');
	}
	array.splice(index, 0, value);
}

const gnextToken = (i, n, tokens) => {
	return tokens[i + n] ? (tokens[i + n].type == 'WHITESPACE' ? gnextToken(i, n + 1, tokens) : { token: tokens[i + n], n, ti: i + n }) : null;
};

const fnextToken = (i, tokens, type, value) => {
	return tokens
		.map((t, ind) => {
			t.ti = ind;
			return t;
		})
		.slice(i, tokens.length - 1)
		.map((t, ind) => {
			t.ri = ind;
			t.index = ind - i;
			return t;
		})
		.find((t) => t.type == type && (value ? t.value == value : true));
};

function declareAlias(aliases, token) {
  const regex = /^#declare(\*)?\s+(\w+)\s+"([^"]+)"\s*=\s*([\s\S]*);$/;
  const match = token.value.trim().match(regex);
	straceLog('declareCase()');
	straceLog('==> WARN: Experimental feature detected');

  if (match) {
    const isPublic = !!match[1];
    const type = match[2] == "key" ? 'IDENTIFIER' : match[2];
    let name = match[3];
    let value = match[4].trim();
		straceLog('==> INFO !DECLARE', name, 'as', value);

    let aliasValue = value.startsWith('${')
      ? new Function('token', 'tokens', 'code', 'hooks', 'index', 'setIndex', value.slice(2, -1))
      : value;
		

		if(name.startsWith('=')){
			name = name.slice(1);
			let isDecOnly = false, isConstructor = false;
			if(name.endsWith('*')) {
				name = name.slice(0, -1);
				isDecOnly = true;
			}
			if(name.endsWith('!')) {
				name = name.slice(0, -1);
				isConstructor = true;
			}
			aliasValue = (token, tokens, code, hooks, index, setIndex) => {
				const nextToken = tokens[index+1]
				let nextToken2 = gnextToken(index, 3, tokens);
				if (nextToken?.value == '(' || tokens[index+2]?.value == '(') {
					let params = '';
					index += 2;
					let openBrackets = 1;
					while (index < tokens.length && openBrackets > 0) {
						const char = tokens[index].value;
						if (char === '(') openBrackets++;
						if (char === ')') openBrackets--;
						if (openBrackets > 0) params += char;
						index++;
					}
					const { token: nextToken2, n: n2, ti } = gnextToken(index, 1, tokens) || {};
					let offset = 1;
					if(tokens[ti+1].type == 'WHITESPACE') offset += 2;
					if(tokens[ti+3].type == 'WHITESPACE') offset += 1;

					let nextToken = gnextToken(index, offset+1, tokens);
					const args = nextToken.token.value;
					setIndex(ti + offset);
					return `${nextToken2.value} = ${isConstructor?'new ':''}${token.value} ${args && args !== '(' && args !== '{' && args !== '[' && args !== '-' && args !== '=' ? `${args},` : ''} ${params.trim() ? params.trim() + ', ' : ''}${args == '(' || args == '[' || args == '{' || args == '=' || args == '-' ? args : ''}`.replace(/,(| )$/, '');
				} else if(nextToken?.value == ' ' && (isDecOnly || nextToken2?.token.value == '=' || nextToken2?.token.value == ':')){
					nextToken.value = '';
					if(isDecOnly){
						nextToken2 = {
							token: { value: ':' },
							ti: index+2
						}
						value = '= ' + value + '()';
					}
					if(nextToken2.token.value == ':') nextToken2.token.value = '=';
					hooks.push({
						index: nextToken2.ti,
						value: ' ' + (isConstructor?'new ':'') + value
					})
					return "";
				}
				return token.value;
			}
		}

    aliases[type] = aliases[type] || {};
    aliases[type][name] = aliasValue;

		if(isPublic){
			straceLog('==>', 'INFO Declaration Globalized');
			execOptions._syntaxAliases[type] = execOptions._syntaxAliases[type] || {};
			execOptions._syntaxAliases[type][name] = aliasValue;
		}
  }
}

const stdTypes = (isPublic) => {
	let r = '';
	const dec = (name, fn, cons = 0) => {
		r += `#declare${isPublic?'*':''} key "=${name}${cons ? '!' : ''}" = ${fn || name};\n`
	}
	dec('int');
	dec('str');
	dec('float');
	dec('num');
	dec('bool');
	dec('typedef');
	dec('typef');
	dec('struct');
	dec('Usage', null, 1);
	return r;
};
const includeFile = (includeContent, options) => {
	straceLog('include()', includeContent);
	const dontInclude = includeContent.startsWith('*');
	if(dontInclude) {
		includeContent = includeContent.slice(1);
		straceLog('==> INFO ingoring output', includeContent);
	};
	const fp = path.resolve(path.dirname(options.filename || ''), includeContent);
	let packageName = options.filename ? (existsSync(fp) ? fp : includeContent) : includeContent;
	const file = packageName.startsWith('./') || packageName.startsWith('../');
	if(!(file) && packageName.match('/')) packageName = packageName.split('/').pop();
	if(file) packageName = path.extname(packageName) ? packageName.replace(path.extname(packageName), '.h.coffee') : packageName;
	if(file && !packageName.endsWith('.h.coffee')) packageName += '.h.coffee';
	if(includeContent == 'std') packageName = 'std';

	const _inc = (filename, c) => {
		options.included = true;
		options.filename = filename;
		return '\n'+ compileRewStuff(c ? filename : readFileSync(filename).toString(), options)+'\n'
	};
	let r = '';
	if(packageName == 'std'){
		r = _inc(stdTypes(dontInclude), true);
	} else if (existsSync(packageName)) {
		straceLog('==> includeFile(', '"'+packageName+'"', ')');
		r = _inc(packageName);
	} else {
		const packageName = includeContent.match('/') ? includeContent.split('/')[0] : includeContent;
		const headerFile = includeContent.match('/') ? includeContent.replace(packageName+'/', '') : 'main.h.coffee';
		const pathname = path.join(CONFIG_PATH, packageName, 'app', headerFile);
		straceLog('==> includePackage(', '"'+packageName+'"');
		if(existsSync(pathname)) r = _inc(pathname);
	}
	if(dontInclude){
		return 'intentional-nothing';
	}
	return r;
}

function useImp(token, options){
	if(token.type == 'STRING' && (
		token.value.startsWith('"#') ||
		token.value.startsWith("'#")
	)){
		straceLog('==> INFO imp() Uses HEADER');
		const dem = token.value.slice(0, 1);
		const value = token.value.slice(1, -1);
		let packageName = value.slice(1);
		token.value = dem+packageName+dem;
		straceLog('imp() with header for', `"${packageName}"`);
		return includeFile(packageName !== 'std' ? packageName : '*'+packageName, options); 
	}
	return '';
}

function mapTokensToStrings(tokens) {
  const result = [];
  let current = '';
  let inBrackets = false;

  tokens.forEach(token => {
    if (token.type === 'OTHER' && token.value === '{') {
      if (current) {
        result.push(current.trim());
        current = '';
      }
      inBrackets = true;
      current += token.value + ' ';
    } else if (token.type === 'OTHER' && token.value === '}') {
      current += token.value;
      result.push(current.trim());
      current = '';
      inBrackets = false;
    } else if(token.type === 'OTHER' && token.value === ',' && !inBrackets){
			return;
		} else {
      current += token.value;
      if (!inBrackets) {
        result.push(current.trim());
        current = '';
      } else {
        current += ' ';
      }
    }
  });

  return result;
}

function updateAliases(aliases, o = execOptions._syntaxAliases){
	for(let h in o){
		for(let i in o[h]){
			if(!aliases[h]) aliases[h] = {};
			aliases[h][i] = o[h][i]
		}
	}
	return aliases;
}

function insertAt(array, index, ...values) {
  if (index > array.length) {
    index = array.length;
  } else if (index < 0) {
    index = 0;
  }
  array.splice(index, 0, ...values);

  return array;
}

function compileRewStuff(content, options) {
	straceLog('tokeinze(currentFile)');
	const tokens = tokenizeCoffeeScript(content);
	let result = '';
  let multilineDeclareBuffer = [];
  let multilineDeclare = false;

	let hooks = [];


	let aliases = {
		...execOptions._syntaxAliases
	}

	for (let i = 0; i < tokens.length; i++) {
		const token = tokens[i];
		let { token: nextToken, n } = gnextToken(i, 1, tokens) || {};

		if(token.type == "COMMENT" && i < 2 && token.value.startsWith('#!')){
			continue;
		}

    if ((token.type === "COMMENT" && multilineDeclare) || (token.type !== "COMMENT" && multilineDeclare)) {
      if(token.type === "COMMENT"){
				multilineDeclareBuffer.push(token.value.startsWith('###') ? token.value.slice(3) : token.value.slice(1));
				if (token.value.trim().endsWith(';')) {
					multilineDeclare = false;
					const combinedDeclaration = multilineDeclareBuffer.join('\n');
					declareAlias(aliases, { ...token, value: combinedDeclaration });
					multilineDeclareBuffer = [];
				}
			} else {
				multilineDeclare = false;
				multilineDeclareBuffer = [];
			}
    }

		if (token.type === "COMMENT" && token.value.startsWith('#alias')) {
			let value = '#declare';
			if(token.value.match(/^#alias\*/)) value += '*';
			let subs;
			subs = token.value.replace(/^#alias/, '');
			if(token.value.endsWith('*')) subs.split('*')[1];
			
			value += ' key';
			value += ' ' + subs.replace(/([\S]+)\s*=\s*([\S]+)/, '"$1" = $2').trim();
			value += ';';
			declareAlias(aliases, {...token, value});
    }

		if (token.type === "COMMENT" && token.value.startsWith('#declare')) {
      if (token.value.includes(';')) {
        declareAlias(aliases, token);
      } else {
        multilineDeclare = true;
        multilineDeclareBuffer.push(token.value);
      }
    }

		if (token.type === 'IDENTIFIER' && token.value === 'opt.set') {
			const { token: nextNextToken } = gnextToken(i, 2, tokens) || {};
			if (nextNextToken && nextNextToken.value.slice(1).slice(0, -1) == 'jsxPragma') {
				const { token: nextLastToken } = gnextToken(i, 5, tokens) || {};
				execOptions.jsxPragma = nextLastToken.value.slice(1).slice(0, -1);
			}
		}
		
		if (token.type === 'COMMENT' && token.value.slice(1).trim().startsWith('@jsx')) {
			options.jsx = true;
			straceLog('jsx().with(comments)');
			if(token.value.split('@jsx')[1].trim()){
				options.jsxPragma = token.value.split('@jsx')[1].trim();
				straceLog('jsx().withPragma(', `"${options.jsxPragma}"`, ')');
			}
		}
		
    if (token.type === 'COMMENT' && token.value.slice(1).trim() === '@cls') {
			options.cls = true;
			straceLog('cliSyntax::enable()');
			straceLog('===> WARN: HIGHLY EXPERIMENTAL FEATURE DETECTED');
		}

    if (options.cls && token.type === 'OTHER' && token.value === '-' && nextToken.value == '-' && tokens[i-1]?.type == 'WHITESPACE') {
			// Argument case
      let offset = 0, writenext = false;
      const n = gnextToken(i, 2, tokens);
      let v = gnextToken(i, 3, tokens);
      if(v.token.type == 'IDENTIFIER' && v.token.value == '$'){
        writenext = true;
      }
      result +=  n.token.value + ': ' + (writenext ? '' : (v.token.value == ',' ? 'true, ' : v.token.type == "STRING" ? v.token.value : ValueIfy(v.token.value)));

      i = offset + tokens.indexOf(v.token);
      continue;
		}


		if (tokens[i-1]?.value !== '.' && token.type === 'IDENTIFIER' && token.value === 'export' && !options.keepImports) {
			token.value = 'pub';
			straceLog('INFO !TRANSLATE converting export to pub');
		}
		
		if (tokens[i-1]?.value !== '.' && token.type === 'IDENTIFIER' && token.value === 'package' && nextToken.type == 'STRING') {
			token.value = 'appPackage';
			straceLog('changeAppPackage()');
		}

		if (
			token.type === 'OTHER' && token.value === '-' &&
			nextToken.type == 'IDENTIFIER' && nextToken.value === 'wait'
		) {
			const { token: nextNextToken } = gnextToken(i, 2, tokens) || {};
			if(nextNextToken?.type == 'IDENTIFIER'){
				token.value = '';
				hooks.push({
					index: i + 3,
					value: ','
				});
			}
		}

		if (tokens[i-1]?.value !== '.' && token.type === 'IDENTIFIER' && token.value === 'using' && !options.disableUse) {
			straceLog('!+DIRECTIVE using()');
			const next = nextToken?.value;
			if(next in USING_DEFAULT) {
				const { use } = USING_DEFAULT[next];
				use?.(options);
				straceLog('==>', nextToken.value);
				nextToken.value = `"${nextToken.value}"`

				const { token: nextNextToken } = gnextToken(i, 3, tokens) || {};
				if(nextNextToken.value == "as") nextNextToken.value = ",";
			} else straceLog('==> !-UNKNOWN');
		}

		if (token.type === 'IDENTIFIER' && token.value === 'as' && !options.keepImports) {
			const isFrom = gnextToken(i, 3, tokens);
			const isInImport = tokens[i-2];
			if(isFrom?.token.value == 'from' && isInImport?.value !== '*'){
				insertTokenAt(tokens, i, { type: 'WHITESPACE', value: ' ' });
				insertTokenAt(tokens, i, { type: 'OTHER', value: '*' });
				insertTokenAt(tokens, i, { type: 'WHITESPACE', value: ' ' });
				insertTokenAt(tokens, i, { type: 'IDENTIFIER', value: 'import' });
				i -= 1;
				continue;
			}
		}

		if (tokens[i-1]?.value !== '.' && token.type === 'IDENTIFIER' && token.value === 'import' && !options.keepImports) {
			// console.log(nextToken.type);
			straceLog('import()');
			straceLog('==> WARN: Slows compilation');
			let ind = i + n + 2;
			let isAs = false;
			let usedDefault = false;

			let defaultName;
			if (nextToken.type === 'STRING') {
				straceLog('==> !SIMPLE');
				if(useImp(nextToken, options)) updateAliases(aliases);
				result += `inc ${nextToken.value}`;
				i += n;
			} else if (nextToken.type === 'OTHER' && nextToken.value == "(") {
				const closingBraceToken = fnextToken(ind, tokens, 'OTHER', ')');
				let lastindex = ind;
				if (closingBraceToken) {
					const namesTokens = tokens.slice(ind, closingBraceToken.ti);
					const next = gnextToken(closingBraceToken.ti, 1, tokens);
					let exports = [];
					if(next?.token?.value == 'as'){
						const ebraceOpen = gnextToken(next.token.ti, 1, tokens);
						const ebraceClose = fnextToken(ebraceOpen.ti, tokens, 'OTHER', ')');
						const exportsTokens = tokens.slice(ebraceOpen.ti+1, ebraceClose?.ti);
						exports = mapTokensToStrings(exportsTokens.filter(token => token.type !== 'WHITESPACE'));
						lastindex = ebraceClose.ti;
					} else lastindex = closingBraceToken.ti;
					const statements = namesTokens.filter(token => token.type !== 'WHITESPACE').map((token, index) => {
						return tokenizeCoffeeScript(`\nimport `+ (exports[index] ? `${exports[index].startsWith('@') ? '* as '+ exports[index].slice(1) : exports[index]} from ${token.value};` : `${token.value}`))
					});
					i = lastindex+1;
					insertAt(tokens, i, ...statements.flat());
					continue;
				}
			} else if (nextToken.value === '{') {
				const closingBraceToken = fnextToken(ind, tokens, 'OTHER', '}');
				const nameToken = fnextToken(ind, tokens, 'STRING');
				if (closingBraceToken) {
					const exportsTokens = tokens.slice(ind, closingBraceToken.ti);
					const exports = exportsTokens
						.filter((t) => t.type === 'IDENTIFIER')
						.map((t, i, arr) => t.value == 'as' ? [arr[i-1].value +': '+arr[i+1].value] : t.value)
						.flat(1)
						.filter((t, i, arr) => !arr[i+1]?.match(':') && !arr[i-1]?.match(':'))
						.join(', ');
					
					straceLog('==> !COMPLEX', exports, 'from', nameToken.value);

					if(useImp(nameToken, options)) updateAliases(aliases);
					result += `{ ${exports} } ${options.type == 'coffee' ? '=' : ':='} inc ${nameToken.value}`;
					i = nameToken.ti;
				}
			} else if (nextToken.value === '*') {
				const asToken = fnextToken(ind, tokens, 'IDENTIFIER', 'as');
				if (asToken) {
					const nameToken = fnextToken(asToken.ti, tokens, 'STRING');
					const nextToken = fnextToken(asToken.ti + 1, tokens, 'IDENTIFIER');
					defaultName = nextToken.value;
					straceLog('==> !COMPLEX', defaultName, 'from', nameToken.value);
					if(useImp(nameToken, options)) updateAliases(aliases);
					result += `${defaultName} ${options.type == 'coffee' ? '=' : ':='} inc ${nameToken.value}`;
					i = ind + 6;
					isAs = true;
				}
			} else if (nextToken) {
				const nameToken = fnextToken(ind, tokens, 'STRING');
				defaultName = nextToken.value;
				let { token: nextNextToken, n: n2 } = gnextToken(i + 2, 1, tokens) || {};
				if (nextNextToken?.type == 'OTHER' && nextNextToken?.value == ',') {
					const closingBraceToken = fnextToken(ind, tokens, 'OTHER', '}');
					if (closingBraceToken) {
						const exportsTokens = tokens.slice(ind, closingBraceToken.ti);
						const exports = exportsTokens
							.filter((t) => t.type === 'IDENTIFIER')
							.map((t, i, arr) => t.value == 'as' ? [arr[i-1].value +': '+arr[i+1].value] : t.value)
							.flat(1)
							.filter((t, i, arr) => !arr[i+1]?.match(':') && !arr[i-1]?.match(':'))
							.join(', ');
						straceLog('==> !COMPLEX', defaultName, 'and', exports, 'from', nameToken.value);
						if(useImp(nameToken, options)) updateAliases(aliases);
						result += `{ default: ${defaultName}, ${exports} } ${options.type == 'coffee' ? '=' : ':='} inc ${nameToken?.value || ''}`;
						i = closingBraceToken.ti + 4;
						usedDefault = true;
					}
				} else {
					if(useImp(nameToken || {}, options)) updateAliases(aliases);
					result += `{ default: ${defaultName} } ${options.type == 'coffee' ? '=' : ':='} inc ${nameToken?.value || ''}`;
					usedDefault = true;
					i = ind + 2;
				}
			}

			const nextLastToken = fnextToken(i, tokens, 'IDENTIFIER');

			if (nextLastToken?.value == 'assert') {
				result += ', ';
				const assertionToken = gnextToken(nextLastToken.ti, 2, tokens);
				straceLog('==> !ASSERT', assertionToken);
				if(assertionToken.token.type == 'OTHER' && assertionToken.token.value == '{'){
					hooks.push({
						index: assertionToken.token.ti,
						value: ' useDefaultForPackages: '+(isAs?'false':usedDefault.toString())+', '
					})
				} else {
					result += 'useDefaultForPackages: '+(isAs?'false':usedDefault.toString())+', '
				}
				i += 3;
			} else {
				result += ", { useDefaultForPackages: "+(isAs?'false':usedDefault.toString())+" }"
			}

			continue;
		}

		if (tokens[i-1]?.value !== '.' && token.type === 'IDENTIFIER' && (token.value === 'imp' || token.value === 'inc')) {
			straceLog('!+DIRECTIVE imp()');
			let { token: t1 } = gnextToken(i, 1, tokens) || {};
			let { token: t2 } = gnextToken(i, 2, tokens) || {};
			let r = '';

			if(t1?.value == '('){
				if(t2?.type == 'STRING'){
					r = useImp(t2, options);
				}
			} else if(t1?.type == 'STRING'){
				r = useImp(t1, options);
			}

			if(r) {
				updateAliases(aliases);
			}
		}

		if (
			tokens[i-1]?.value !== '.' &&
			token.type === 'IDENTIFIER' &&
			token.value === 'pub' &&
			nextToken &&
			nextToken.type === 'IDENTIFIER' &&
			nextToken.value &&
			nextToken.value !== 'undefined' && !options.keepImports
		) {
			straceLog('!+DIRECTIVE pub()');
			let next = {...nextToken}, isClass = false;
			if(next.value == 'default'){
				i += 2;
			}
			if(next.value == 'class'){
				next.value = gnextToken(i, n + 1, tokens)?.token.value || "default";
				isClass = true;
			}
			straceLog('==> !PUBLIC', next.value);
			hooks.push({
				index: i + 1,
				value: `"${next.value}", ${isClass ? `${next.value} = ` : ''}`,
			});
		}

		const aliasType = aliases[token.type];
		// if(token.value == 'sidewest') console.log(aliases, token.value, token.type);
    if (aliasType && Object.keys(aliasType).includes(token.value)) {
			straceLog('!+DIRECTIVE alias()', token.type);
      const aliasValue = aliasType[token.value];
      if (typeof aliasValue === 'function') {
				straceLog('==> INFO Execute alias:', token.value);
        result += aliasValue(token, tokens, result, hooks, i, (n) => i = n) || "";
      } else {
				straceLog('==> INFO Literal alias:', token.value);
        result += aliasValue;
      }
      continue;
    }

		// process.stdout.write(token.value);
		result += token.value;
		if (hooks.length) {
			hooks.forEach((hook, ind) => {
				if (i == hook.index) {
					result += hook.value;
					hooks.splice(ind, 1);
				}
			});
		}
		
		if (token.type === "COMMENT" && token.value.startsWith('#include')) {
			const includeContent = token.value.split(' ')[1].trim() || '';
			const _o = {...options};
			const r = includeFile(includeContent, _o);
			if(r){
				result += r == 'intentional-nothing' ? '' : r;
				updateAliases(aliases, _o.aliases);
			}
    }
	}

	if(options.included){
		options.aliases = aliases;
	}
	// console.log(aliases);
	// console.log(result);
	// return "";
	return result;
}

const compileCivetStuff = (file, options) => {
	straceLog('compileCivet(currentFile)');
	const preCompileOptions = {
		filename: file.path,
		...options
	};
	straceLog('prepareOptions(currentFile).as(', `"${JSON.stringify(preCompileOptions)}"`, ')');
	
	if(options?.type == 'js' || file?.path?.endsWith('.js')){
		return {
			compiled: file?.content || "",
			options: preCompileOptions
		}
	}
	
	const prepared = compileRewStuff(file.content, preCompileOptions);

	// console.log(prepared);

	const compileOptions = {
		...preCompileOptions,
		bare: true,
		filename: file.path,
		inlineMap: false,
		js: true
	};

	let compiled = options.async ? compileCivet(prepared, compileOptions) : wait(compileCivet, prepared, compileOptions);
	straceLog('==> !COMPILER civetCompile(fileContent)');

	return {
		compiled,
		options: preCompileOptions
	};
}

const cpl = (module.exports.compile = function (file, options = {}) {
	let compiledCode;
	const result = compileCivetStuff(file, {
		...options,
		parseOptions: { 
			coffeeCompat: options.type == "coffee",
		}
	});
	
	options = result.options;
	compiledCode = result.compiled;

	const babelify = (code, options) => {
		straceLog('!COMPILER babel()');
		if(doJSX) straceLog('==> INFO !-WITH JSX');
		if(doTypes) straceLog('==> INFO !-WITH Types');
		if(doDecorators) straceLog('==> INFO !-WITH DECORATORS');
		return babel.transformSync(code, {
			presets: [
				...(doJSX ? [[babelReact, { throwIfNamespace: false, pragmaFrag: options.jsxPragmaFrag || execOptions.jsxPragmaFrag, pragma: options.jsxPragma || execOptions.jsxPragma }]] : [])
			],
			plugins: [
				...(doDecorators ? [[require('@babel/plugin-proposal-decorators'), { version: '2023-05' }], [require('@babel/plugin-proposal-class-properties'), { loose: true }], [require('@babel/plugin-transform-class-static-block'), {}]] : []),
				// doJSX ? require('./jsx') : null
			].filter(Boolean),
		}).code;
	}

	const doJSX = execOptions.jsx || options.jsx;
	const doTypes = execOptions.typescript || options.typescript;
	const doDecorators = execOptions.decorators || options.decorators;
	const doBabel = doJSX || doTypes || doDecorators;

	if(compiledCode instanceof Promise){
		return compiledCode.then((compiledCode) => {
			if (doBabel) {
				compiledCode = babelify(compiledCode, options);
			}
			return compiledCode;
		});
	}

	// console.log(c);
	if (doBabel) {
		compiledCode = babelify(compiledCode, options);
	}
	return compiledCode;
});

module.exports.compileFile = function (filepath, options = {}) {
	straceLog('compile(currentFile)');
	const f = typeof filepath == "object" ? filepath : getFile(filepath);
	if(typeof filepath == "object") filepath = filepath.path;
	let qrew = false;

	if(options.qrew || path.extname(filepath) == '.qrew') {
		qrew = true
		f.content = from_qrew(readFileSync(f.path), options.package || findAppInfo(filepath)?.config.manifest.package || path.basename(filepath).split('.').slice(0, -1).join('.')).toString();
		options.type = f.content.split('\n')[0]?.match(/"initFile (.+)"/)?.[1]?.split('.').pop();
		straceLog('decodeCrew(currentFile).as(', `"${options.type}"`,')');
	}

	let compiled_code =  cpl(f, { ...options });

	if(options.onlyCompile && !qrew){
		straceLog('writeAndQuit(compileData)');
		if(compiled_code instanceof Promise){
			compiled_code.then((r) => {
				console.log(r);
				process.exit();
			});
		} else {
			console.log(compiled_code);
			process.exit();
		}
	}

	return {
		compiled_code,
		file: f,
	};
};
