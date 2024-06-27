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



function tokenizeCoffeeScript(code) {
  const tokens = [];
  let currentToken = '';
  let i = 0;

  while (i < code.length) {
    const char = code[i];
    const nextChar = code[i + 1];
    const nextNextChar = code[i + 2];

    if (char === '#') {
      // Comment
      const commentEnd = code.indexOf('\n', i);
      const comment = code.substring(i, commentEnd < 0 ? code.length : commentEnd + 1);
      tokens.push({ type: 'COMMENT', value: comment });
      i += comment.length - 1;
    } else if (char === '"' && nextChar === '"' && nextNextChar === '"') {
      // Triple-quoted string
      let string = '"""';
      i += 3;
      while (i < code.length && !(code[i] === '"' && code[i + 1] === '"' && code[i + 2] === '"')) {
        string += code[i];
        i++;
      }
      string += '"""'; // Include closing triple quotes
      tokens.push({ type: 'TRIPLE_STRING', value: string });
      i += 2; // Skip past the closing triple quotes
    } else if (char === '"' || char === "'") {
      // Single or double-quoted string
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
      string += char; // Include closing quote
      tokens.push({ type: 'STRING', value: string });
    } else if (char === '/' && nextChar !== ' ' && nextChar !== '/' && nextChar !== '*') {
      // Regular expression
      let regex = char;
      i++;
      while (i < code.length && (code[i] !== '/' || regex.endsWith('\\'))) {
        regex += code[i];
        i++;
      }
      regex += '/';
      tokens.push({ type: 'REGEX', value: regex });
    } else if (/\s/.test(char)) {
      // Whitespace
      if (tokens[tokens.length - 1]?.type === 'WHITESPACE') {
        tokens[tokens.length - 1].value += char;
      } else {
        tokens.push({ type: 'WHITESPACE', value: char });
      }
    } else if (/[a-zA-Z_$]/.test(char)) {
      // Identifier
      let identifier = char;
      i++;
      while (i < code.length && /[a-zA-Z0-9_$]/.test(code[i])) {
        identifier += code[i];
        i++;
      }
      tokens.push({ type: 'IDENTIFIER', value: identifier });
      i--; // Move back one character to recheck
    } else {
      // Other characters
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

const gnextToken = (i, n, tokens) => {
	return tokens[i + n] ? (tokens[i + n].type == 'WHITESPACE' ? gnextToken(i, n + 1, tokens) : { token: tokens[i + n], n }) : null;
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

  if (match) {
    const isPublic = !!match[1];
    const type = match[2] == "key" ? 'IDENTIFIER' : match[2];
    const name = match[3];
    const value = match[4].trim();

    const aliasValue = value.startsWith('${')
      ? new Function('token', 'tokens', 'code', value.slice(2, -1))
      : value;

    aliases[type] = aliases[type] || {};
    aliases[type][name] = aliasValue;

		if(isPublic){
			execOptions._syntaxAliases[type] = execOptions._syntaxAliases[type] || {};
			execOptions._syntaxAliases[type][name] = aliasValue;
		}
  }
}

function compileRewStuff(content, options) {
	const tokens = tokenizeCoffeeScript(content);
	let result = '';
  let multilineDeclareBuffer = [];
  let multilineDeclare = false;

	let hooks = [];


	const aliases = {
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
				if (token.value.includes(';')) {
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
			if(token.value.split('@jsx')[1].trim()){
				options.jsxPragma = token.value.split('@jsx')[1].trim();
			}
		}
		
    if (token.type === 'COMMENT' && token.value.slice(1).trim() === '@cls') {
			options.cls = true;
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


		if (token.type === 'IDENTIFIER' && token.value === 'export' && !options.keepImports) {
			token.value = 'pub';
		}

		if (token.type === 'IDENTIFIER' && token.value === 'using' && !options.disableUse) {
			const next = nextToken.value;
			if(next in USING_DEFAULT) {
				const { use } = USING_DEFAULT[next];
				use?.(options);
				nextToken.value = `"${nextToken.value}"`
			}
		}

		if (token.type === 'IDENTIFIER' && token.value === 'import' && !options.keepImports) {
			// console.log(nextToken.type);
			let ind = i + n + 2;

			let defaultName;
			if (nextToken.type === 'STRING') {
				result += `inc ${nextToken.value}`;
				i += n;
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

					result += `{ ${exports} } ${options.type == 'coffee' ? '=' : ':='} inc ${nameToken.value}`;
					i = nameToken.ti;
				}
			} else if (nextToken.value === '*') {
				const asToken = fnextToken(ind, tokens, 'IDENTIFIER', 'as');
				const nameToken = fnextToken(asToken.ri, tokens, 'STRING');
				if (asToken) {
					const nextToken = fnextToken(asToken.ti + 1, tokens, 'IDENTIFIER');
					defaultName = nextToken.value;
					result += `${defaultName} ${options.type == 'coffee' ? '=' : ':='} inc ${nameToken.value}`;
					i = ind + 6;
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
						result += `{ default: ${defaultName}, ${exports} } ${options.type == 'coffee' ? '=' : ':='} inc ${nameToken?.value || ''}`;
						i = closingBraceToken.ti + 4;
					}
				} else {
					result += `{ default: ${defaultName} } ${options.type == 'coffee' ? '=' : ':='} inc ${nameToken?.value || ''}`;
					i = ind + 2;
				}
			}

			const nextLastToken = fnextToken(i, tokens, 'IDENTIFIER');

			if (nextLastToken?.value == 'assert') {
				result += ', ';
				i += 3;
			}

			continue;
		}

		if (
			token.type === 'IDENTIFIER' &&
			token.value === 'pub' &&
			nextToken &&
			nextToken.type === 'IDENTIFIER' &&
			nextToken.value &&
			nextToken.value !== 'undefined' && !options.keepImports
		) {
			let next = {...nextToken};
			if(next.value == 'default'){
				i += 2;
			}
			if(next.value == 'class'){
				next.value = gnextToken(i, n + 1, tokens)?.token.value || "default";
			}
			hooks.push({
				index: i + 1,
				value: `"${next.value}", `,
			});
		}

		const aliasType = aliases[token.type];
    if (aliasType && aliasType[token.value]) {
      const aliasValue = aliasType[token.value];
      if (typeof aliasValue === 'function') {
        result += aliasValue(token, tokens, result) || "";
      } else {
        result += aliasValue;
      }
      continue;
    }

		// if(token.type === 'TRIPLE_STRING'){
		// 	token.value = '```'+token.value.slice(3).slice(0, -3).replace(/\#\{/g, '${')+'```';
		// }

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
			const includeContent = token.value.split(' ')[1] || '';
			const filename = options.filename ? path.resolve(path.dirname(options.filename), includeContent) : includeContent;
			if (existsSync(filename)) {
        result += '\n'+ compileRewStuff(readFileSync(filename).toString(), {
					...options,
					filename
				});
      }
    }
	}

	return result;
}

const compileCivetStuff = (file, options) => {
	const compileOptions = {
		...options,
		bare: true,
		filename: file.path,
		inlineMap: false,
		js: true
	};

	const prepared = compileRewStuff(file.content, {
		filename: file.path,
		...options
	});
	let compiled = options.async ? compileCivet(prepared, compileOptions) : wait(compileCivet, prepared, compileOptions);

	return {
		compiled,
		options
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

	const babelify = (code, options) => babel.transformSync(code, {
		presets: [
			...(doJSX ? [[babelReact, { pragma: options.jsxPragma || execOptions.jsxPragma }]] : [])
		],
		plugins: [
			...(doDecorators ? [[require('@babel/plugin-proposal-decorators'), { version: '2023-05' }], [require('@babel/plugin-proposal-class-properties'), { loose: true }], [require('@babel/plugin-transform-class-static-block'), {}]] : [])
		],
	}).code;

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
	const f = typeof filepath == "object" ? filepath : getFile(filepath);
	if(typeof filepath == "object") filepath = filepath.path;
	let qrew = false;

	if(options.qrew || path.extname(filepath) == '.qrew') {
		qrew = true
		f.content = from_qrew(readFileSync(f.path), options.package || findAppInfo(filepath)?.config.manifest.package || path.basename(filepath).split('.').slice(0, -1).join('.')).toString();
		options.type = f.content.split('\n')[0]?.match(/"initFile (.+)"/)?.[1]?.split('.').pop();
	} 

	let compiled_code =  cpl(f, { ...options });

	if(options.onlyCompile && !qrew){
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
