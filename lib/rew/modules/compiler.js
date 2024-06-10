const { compile } = require('../../coffeescript/coffeescript');
const { execOptions } = require('../const/opt');
const { findAppInfo } = require('../misc/findAppInfo');
const { from_qrew } = require('../qrew/compile');
const { getFile, file } = require('./fs');
const babel = require('@babel/core');
const path = require('path');
const babelReact = require('@babel/preset-react');
const { readFileSync } = require('fs');

function tokenizeCoffeeScript(code) {
	const tokens = [];
	let currentToken = '';

	for (let i = 0; i < code.length; i++) {
		const char = code[i];
		const nextChar = code[i + 1];

		if (char === '#') {
			// Comment
			tokens.push({
				type: 'COMMENT',
				value: char + code.substring(i + 1).split('\n')[0] + '\n',
			});
			const ind = code.indexOf('\n', i);
			i = ind < 0 ? code.length-1 : ind;
		} else if (char === '"' || char === "'") {
			// String
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
		} else if (char === '/' && (nextChar === '/' || nextChar === '*')) {
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
			if (tokens[tokens.length - 1]?.type == 'WHITESPACE' && tokens[tokens.length - 1].value[0] == char) {
				tokens[tokens.length - 1].value += char;
			} else {
				tokens.push({ type: 'WHITESPACE', value: char });
			}
		} else if (/[a-zA-Z\._$]/.test(char)) {
			// Identifier
			let identifier = char;
			i++;
			while (i < code.length && /[a-zA-Z0-9\._$]/.test(code[i])) {
				identifier += code[i];
				i++;
			}
			tokens.push({ type: 'IDENTIFIER', value: identifier });
			i--; // Move back one character to recheck
		} else {
			// Other characters
			tokens.push({ type: 'OTHER', value: char });
		}
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

function compileRewStuff(content, options) {
	const tokens = tokenizeCoffeeScript(content);
	let result = '';

	let hooks = [];

	for (let i = 0; i < tokens.length; i++) {
		const token = tokens[i];
		let { token: nextToken, n } = gnextToken(i, 1, tokens) || {};

		if (token.type === 'IDENTIFIER' && token.value === 'opt') {
			const { token: nextNextToken } = gnextToken(i, 2, tokens) || {};
			if (nextNextToken && nextNextToken.value == 'jsxPragma') {
				const { token: nextLastToken } = gnextToken(i, 5, tokens) || {};
				execOptions.jsxPragma = nextLastToken.value.slice(1).slice(0, -1);
			}
		}

		if (token.type === 'COMMENT' && token.value.slice(1).trim() === '@jsx') {
			options.jsx = true;
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


		if (token.type === 'IDENTIFIER' && token.value === 'let') {
			result += '`'
			hooks.push({
				index: fnextToken(i, tokens, 'OTHER', ';').ti,
				value: `\``,
			});
		}

		if (token.type === 'IDENTIFIER' && token.value === 'import') {
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
						.map((t) => t.value)
						.join(', ');
					result += `{ ${exports} } = inc ${nameToken.value}`;
					i = nameToken.ti;
				}
			} else if (nextToken.value === '*') {
				const asToken = fnextToken(ind, tokens, 'IDENTIFIER', 'as');
				const nameToken = fnextToken(asToken.ri, tokens, 'STRING');
				if (asToken) {
					const nextToken = fnextToken(asToken.ti + 1, tokens, 'IDENTIFIER');
					defaultName = nextToken.value;
					result += `${defaultName} = inc ${nameToken.value}`;
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
							.map((t) => t.value)
							.join(', ');
						result += `{ default: ${defaultName}, ${exports} } = inc ${nameToken?.value || ''}`;
						i = closingBraceToken.ti + 4;
					}
				} else {
					result += `{ default: ${defaultName} } = inc ${nameToken?.value || ''}`;
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
			nextToken.value !== 'undefined'
		) {
			hooks.push({
				index: i + 1,
				value: `"${nextToken.value}", `,
			});
		}

		result += token.value;
		if (hooks.length) {
			hooks.forEach((hook, ind) => {
				if (i == hook.index) {
					result += hook.value;
					hooks.splice(ind, 1);
				}
			});
		}
	}

	// console.log(result)

	return result;
}

const cpl = (module.exports.compile = function (file, options = {}) {
	let c = options.type == 'js' || options.compile == false ? file.content : compile(compileRewStuff(file.content, options), {
		...options,
		filename: file.path,
		bare: false,
		inlineMap: false,
	});
	// console.log(c);
	if (execOptions.jsx || options.jsx) {
		c = babel.transformSync(c, {
			presets: [[babelReact, { pragma: execOptions.jsxPragma }]],
			plugins: [],
		}).code;
	}
	return c;
});

module.exports.compileFile = function (filepath, options = {}) {
	const f = getFile(filepath);
	let qrew = false;

	if(options.qrew || path.extname(filepath) == '.qrew') {
		qrew = true
		f.content = from_qrew(readFileSync(f.path), options.package || findAppInfo(filepath)?.config.manifest.package || path.basename(filepath).split('.').slice(0, -1).join('.')).toString();
	} 

	let compiled_code =  cpl(f, { ...options });

	if(options.onlyCompile && !qrew){
		console.log(compiled_code);
		process.exit();
	}

	return {
		compiled_code,
		file: f,
	};
};
