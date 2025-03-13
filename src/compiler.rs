use std::collections::HashMap;
use anyhow::Result;
use deno_core::v8::ContextOptions;
use serde_json::Value;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Token {
  token_type: String,
  value: String,
}

#[derive(Debug)]
struct Hook {
  index: usize,
  value: String,
}

pub struct CompilerOptions {
  pub keep_imports: bool,
  pub disable_use: bool,
  pub jsx: bool,
  pub jsx_pragma: Option<String>,
  pub cls: bool,
  pub included: bool,
  pub filename: Option<String>,
  pub aliases: HashMap<String, HashMap<String, String>>,
  pub compiler_type: String,
}

pub struct CompilerResults {
  pub options: CompilerOptions,
  pub code: String,
}

impl Default for CompilerOptions {
  fn default() -> Self {
    CompilerOptions {
      keep_imports: false,
      disable_use: false,
      jsx: false,
      jsx_pragma: None,
      cls: false,
      included: false,
      filename: None,
      aliases: HashMap::new(),
      compiler_type: "coffee".to_string(),
    }
  }
}

// i hate this function
fn tokenize_coffee_script(code: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut i = 0;
  let chars: Vec<char> = code.chars().collect();

  while i < chars.len() {
    let char = chars[i];
    let prev_char = if i > 0 { Some(chars[i - 1]) } else { None };
    let next_char = chars.get(i + 1).copied();
    let next_next_char = chars.get(i + 2).copied();

    if char == '#' {
      let comment_end = code[i..].find('\n').unwrap_or(code.len() - i);
      let comment = &code[i..i + comment_end + 1];
      tokens.push(Token {
        token_type: "COMMENT".to_string(),
        value: comment.to_string(),
      });
      i += comment.len() - 1;
    } else if char == '"' && next_char == Some('"') && next_next_char == Some('"') {
      let mut string = "\"\"\"".to_string();
      i += 3;
      while
        i < chars.len() &&
        !(chars[i] == '"' && chars.get(i + 1) == Some(&'"') && chars.get(i + 2) == Some(&'"'))
      {
        string.push(chars[i]);
        i += 1;
      }
      string.push_str("\"\"\"");
      tokens.push(Token {
        token_type: "TRIPLE_STRING".to_string(),
        value: string,
      });
      i += 2;
    } else if char == '"' || char == '\'' {
      let mut string = char.to_string();
      let mut escaped = false;
      i += 1;
      while i < chars.len() && (chars[i] != char || escaped) {
        string.push(chars[i]);
        if chars[i] == '\\' && !escaped {
          escaped = true;
        } else {
          escaped = false;
        }
        i += 1;
      }
      string.push(char);
      tokens.push(Token {
        token_type: "STRING".to_string(),
        value: string,
      });
    } else if char.is_whitespace() {
      if let Some(last_token) = tokens.last_mut() {
        if last_token.token_type == "WHITESPACE" {
          last_token.value.push(char);
        } else {
          tokens.push(Token {
            token_type: "WHITESPACE".to_string(),
            value: char.to_string(),
          });
        }
      } else {
        tokens.push(Token {
          token_type: "WHITESPACE".to_string(),
          value: char.to_string(),
        });
      }
    } else if char.is_alphabetic() || char == '_' || char == '$' || char == '@' {
      let mut identifier = char.to_string();
      i += 1;
      while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '$') {
        identifier.push(chars[i]);
        i += 1;
      }
      tokens.push(Token {
        token_type: "IDENTIFIER".to_string(),
        value: identifier,
      });
      i -= 1;
    } else {
      tokens.push(Token {
        token_type: "OTHER".to_string(),
        value: char.to_string(),
      });
    }
    i += 1;
  }

  tokens
}

fn get_next_token(i: usize, n: i32, tokens: &[Token]) -> Option<(Token, i32, usize)> {
  let index = ((i as i32) + n) as usize;
  if index >= tokens.len() {
    return None;
  }

  if tokens[index].token_type == "WHITESPACE" {
    return get_next_token(i, n + 1, tokens);
  }

  Some((tokens[index].clone(), n, index))
}

fn declare_alias(aliases: &mut HashMap<String, HashMap<String, String>>, token: &Token) {
  if token.value.contains(';') {
    // will add this one later --
    let parts: Vec<&str> = token.value.split(';').collect();
    for part in parts {
      if part.contains('=') {
        let kv: Vec<&str> = part.split('=').collect();
        if kv.len() == 2 {
          let key = kv[0].trim();
          let value = kv[1].trim();
          aliases
            .entry("IDENTIFIER".to_string())
            .or_insert_with(HashMap::new)
            .insert(key.to_string(), value.to_string());
        }
      }
    }
  }
}

fn get_string_until(tokens: &[Token], start: usize, end_chars: &[&str]) -> (String, usize) {
  let mut result = String::new();
  let mut i = start;
  while i < tokens.len() {
    if end_chars.contains(&tokens[i].value.as_str()) {
      break;
    }
    result.push_str(&tokens[i].value);
    i += 1;
  }
  (result, i)
}

fn finalize_handle_import(tokens: &[Token], current_idx: usize) -> Result<(bool, Option<Token>)> {
  match get_next_token(current_idx, 1, &tokens) {
    Some((current_token, _, _)) => {
      if current_token.token_type == "STRING" {
        return Ok((true, Some(current_token)));
      } else {
        return Ok((false, None));
      }
    }
    _ => Ok((false, None)),
  }
}

fn handle_import(tokens: &[Token], i: usize) -> (String, usize) {
  let mut result = String::new();
  let mut current_idx = i + 1;

  while current_idx < tokens.len() && tokens[current_idx].token_type == "WHITESPACE" {
    current_idx += 1;
  }

  if current_idx >= tokens.len() {
    return (String::new(), current_idx);
  }

  let token = &tokens[current_idx];

  match token.token_type.as_str() {
    "STRING" => {
      result.push_str(&format!("inc {}", token.value));
      current_idx += 1;
    }
    "IDENTIFIER" | "OTHER" => {
      if token.value == "{" {
        // Named imports: import { a, b } from "module"
        let (imports, new_idx) = get_string_until(tokens, current_idx + 1, &["}"]);
        current_idx = new_idx + 1;

        // Skip to "from" keyword
        while current_idx < tokens.len() && tokens[current_idx].value != "from" {
          current_idx += 1;
        }

        // Get module path
        current_idx += 1;

        if let Ok((should_handle, _)) = finalize_handle_import(tokens, current_idx) {
          if should_handle {
            let re = Regex::new(r"(\w+)\s+as\s+(\w+)").unwrap();
            let replaced_imports = re.replace_all(&imports, "$1: $2").to_string();
            result.push_str(&format!("{{ {} }} := inc", replaced_imports));
          }
        }
      } else {
        // Default import: import defaultExport from "module"
        let mut default_name = token.value.clone();

        // Skip to module path
        while current_idx < tokens.len() && tokens[current_idx].value != "from" {
					if tokens[current_idx].value == "as" {
						if let Some((token, _, _)) = get_next_token(current_idx + 1, 1, &tokens) {
							if token.token_type == "IDENTIFIER" {
								default_name = token.value;
							}
						}
					}
          current_idx += 1;
        }

        current_idx += 1;
				
        if let Ok((should_handle, _)) = finalize_handle_import(tokens, current_idx) {
          if should_handle {
            result.push_str(&format!("{} := inc ", default_name));
          }
        }
      }
    }
    _ => {}
  }

  (result, current_idx)
}

// i hate this function too!
pub fn compile_rew_stuff(content: &str, options: &mut CompilerOptions) -> Result<CompilerResults> {
  let tokens = tokenize_coffee_script(content);
  let mut result = String::new();
  let mut i = 0;
  let mut hooks: Vec<Hook> = Vec::new();
  let mut multiline_declare_buffer: Vec<String> = Vec::new();
  let mut multiline_declare = false;
  let mut aliases = options.aliases.clone();

  while i < tokens.len() {
    let token = &tokens[i];
    let next_token = get_next_token(i, 1, &tokens);
    let prev_token = if i > 1 { get_next_token(i, -2, &tokens) } else { None };

    // Skip shebang
    if token.token_type == "COMMENT" && i < 2 && token.value.starts_with("#!") {
      i += 1;
      continue;
    }

    if token.token_type == "IDENTIFIER" && token.value == "fn" && i < 2 {
      if let Some((next, _, _)) = next_token {
        if
          prev_token.clone().map_or(true, |(t, _, _)| t.value != ".") &&
          next.token_type == "IDENTIFIER"
        {
          result.push_str("function");
          i += 1;
          continue;
        }
      }
    }
		
    if
      (token.token_type == "COMMENT" && multiline_declare) ||
      (token.token_type != "COMMENT" && multiline_declare)
    {
      if token.token_type == "COMMENT" {
        let value = if token.value.starts_with("###") {
          token.value[3..].to_string()
        } else {
          token.value[1..].to_string()
        };
        multiline_declare_buffer.push(value);

        if token.value.trim().ends_with(';') {
          multiline_declare = false;
          let combined = multiline_declare_buffer.join("\n");
          declare_alias(
            &mut aliases,
            &(Token {
              token_type: "COMMENT".to_string(),
              value: combined,
            })
          );
          multiline_declare_buffer.clear();
        }
      } else {
        multiline_declare = false;
        multiline_declare_buffer.clear();
      }
    }

    if token.token_type == "COMMENT" && token.value.starts_with("#alias") {
      let mut value = "#declare".to_string();

      if token.value.contains("#alias*") {
        value.push('*');
      }

      let subs = token.value.replace("#alias", "");

      value.push_str(" key");

      let re = Regex::new(r"([\S]+)\s*=\s*([\S]+)").unwrap();
      let replaced = re.replace_all(&subs, "\"$1\" = $2").to_string();

      value.push_str(replaced.trim());
      value.push(';');

      declare_alias(
        &mut aliases,
        &(Token {
          token_type: "COMMENT".to_string(),
          value,
        })
      );
    }

    // Handle JSX pragma
    if token.token_type == "COMMENT" && token.value[1..].trim().starts_with("@jsx") {
      options.jsx = true;
      if let Some(pragma) = token.value.split("@jsx").nth(1) {
        if !pragma.trim().is_empty() {
          options.jsx_pragma = Some(pragma.trim().to_string());
        }
      }
    }


    if token.token_type == "COMMENT" && token.value[1..].trim() == "@cls" {
      options.cls = true;
    }


    if
      prev_token.clone().map_or(true, |(t, _, _)| t.value != ".") &&
      token.token_type == "IDENTIFIER" &&
      token.value == "export" &&
      !options.keep_imports
    {
      result.push_str("pub");
      i += 1;
      continue;
    }


    if
      prev_token.map_or(true, |(t, _, _)| t.value != ".") &&
      token.token_type == "IDENTIFIER" &&
      token.value == "import" &&
      !options.keep_imports
    {
      let (import_str, new_idx) = handle_import(&tokens, i);
      println!("import_str: {}", import_str);
      result.push_str(&import_str);
      i = new_idx;
      continue;
    }


    if let Some(alias_map) = aliases.get(&token.token_type) {
      if let Some(replacement) = alias_map.get(&token.value) {
        result.push_str(replacement);
        i += 1;
        continue;
      }
    }


    result.push_str(&token.value);
    hooks.retain(|hook| {
      if hook.index == i {
        result.push_str(&hook.value);
        false
      } else {
        true
      }
    });

    i += 1;
  }

  if options.included {
    options.aliases = aliases;
  }

  let compiler_results = CompilerResults {
    options: std::mem::replace(options, CompilerOptions {
      keep_imports: false,
      disable_use: false,
      jsx: false,
      jsx_pragma: None,
      cls: false,
      included: false,
      filename: None,
      aliases: HashMap::new(),
      compiler_type: String::new(),
    }),
    code: result,
  };

  Ok(compiler_results)
}
