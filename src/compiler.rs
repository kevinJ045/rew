use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

use crate::declarations::Declaration;

#[derive(Debug, Clone)]
pub struct Token {
  pub token_type: String,
  pub value: String,
}

#[derive(Debug)]
struct Hook {
  index: usize,
  value: String,
}

pub struct CompilerOptions {
  pub keep_imports: bool,
  pub jsx: bool,
  pub civet_options: Vec<String>,
  pub civet_global: Vec<String>,
  pub cls: bool,
  pub included: bool,

  pub local_declarations: HashMap<String, Declaration>,
  pub global_declarations: HashMap<String, Declaration>,
}

impl Default for CompilerOptions {
  fn default() -> Self {
    CompilerOptions {
      keep_imports: false,
      jsx: false,
      civet_options: vec![],
      civet_global: vec![],
      cls: false,
      included: false,
      local_declarations: HashMap::new(),
      global_declarations: HashMap::new(),
    }
  }
}

pub struct CompilerResults {
  #[allow(unused)]
  pub options: CompilerOptions,
  pub code: String,
}

// i hate this function
pub fn tokenize_coffee_script(code: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut i = 0;
  let chars: Vec<char> = code.chars().collect();

  while i < chars.len() {
    let char = chars[i];
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
      while i < chars.len()
        && !(chars[i] == '"' && chars.get(i + 1) == Some(&'"') && chars.get(i + 2) == Some(&'"'))
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


fn get_prev_token(i: usize, n: i32, tokens: &[Token]) -> Option<(Token, i32, usize)> {
  let index = ((i as i32) - n) as usize;
  if index >= tokens.len() {
    return None;
  }

  if tokens[index].token_type == "WHITESPACE" {
    return get_prev_token(i, n + 1, tokens);
  }

  Some((tokens[index].clone(), n, index))
}

fn find_next_token(
  start: usize,
  tokens: &[Token],
  expected_type: &str,
  expected_value: Option<&str>,
  break_on_find: Option<(&str, Option<&str>)>,
) -> Option<(Token, usize)> {
  let mut idx = start;
  while idx < tokens.len() {
    let token = &tokens[idx];

    if let Some((break_type, break_value)) = break_on_find {
      if token.token_type == break_type {
        if let Some(val) = break_value {
          if token.value == val {
            return None;
          }
        } else {
          return None;
        }
      }
    }

    let pass = if expected_type == "WHITESPACE" {
      token.token_type == "WHITESPACE"
    } else {
      token.token_type != "WHITESPACE"
    };

    if pass {
      if token.token_type == expected_type {
        if let Some(val) = expected_value {
          if token.value == val {
            return Some((token.clone(), idx));
          }
        } else if expected_type == "WHITESPACE" {
          if let Some(val) = expected_value {
            if token.value.contains(val) {
              return Some((token.clone(), idx));
            }
          } else {
            return Some((token.clone(), idx));
          }
        } else {
          return Some((token.clone(), idx));
        }
      }
    }
    idx += 1;
  }
  None
}

fn apply_declarations(
  token: &Token,
  index: usize,
  tokens: &[Token],
  local_declarations: &HashMap<String, Declaration>,
  global_declarations: &HashMap<String, Declaration>,
) -> Option<(usize, String)> {
  let mut additional_idx = 0;
  if token.token_type == "IDENTIFIER" {
    let values = global_declarations
      .values()
      .chain(local_declarations.values());
    for decl in values {
      let mut is_declaration = false;
      let trigger = if decl.trigger.starts_with("=") {
        is_declaration = true;
        decl.trigger.replace("=", "")
      } else {
        decl.trigger.clone()
      };

      // println!("==> Token value: {}, needed: {}", token.value, decl.trigger.clone());

      if token.value == trigger {
        let mut conditions_met = true;

        if let Some(prev_condition) = &decl.condition_prev {
          let mut prev_idx = index;
          while prev_idx > 0 {
            prev_idx -= 1;
            if tokens[prev_idx].token_type != "WHITESPACE" {
              break;
            }
          }

          // println!("==> Prev value: {}", tokens[prev_idx].value);

          if prev_idx < index {
            if tokens[prev_idx].value != *prev_condition {
              conditions_met = false;
            }
          } else {
            conditions_met = false;
          }
        }

        if let Some(next_condition) = &decl.condition_next {
          if let Some((next_token, _, _)) = get_next_token(index, 1, tokens) {
            if next_token.value != *next_condition {
              conditions_met = false;
            }
          } else {
            conditions_met = false;
          }
        }

        if conditions_met {
          if is_declaration {
            let mut str = String::new();
            let mut args = String::new();
            let mut cidx = index;
            let mut next_token = if let Some((token, _, idx)) = get_next_token(index, 1, tokens) {
              cidx = idx;
              token
            } else {
              Token {
                token_type: "OTHER".to_string(),
                value: "".to_string(),
              }
            };
            if next_token.token_type == "OTHER" && next_token.value == "(" {
              if let Some((_, bc_idx)) = find_next_token(index, tokens, "OTHER", Some(")"), None) {
                let mut arg_tokens = Vec::new();
                let mut arg_idx = cidx + 1;

                while arg_idx < bc_idx {
                  arg_tokens.push(&tokens[arg_idx]);
                  arg_idx += 1;
                }

                args = arg_tokens
                  .iter()
                  .map(|t| t.value.clone())
                  .collect::<Vec<String>>()
                  .join("");

                next_token = if let Some((token, _, new_idx)) = get_next_token(bc_idx, 1, tokens) {
                  cidx = new_idx;
                  token
                } else {
                  Token {
                    token_type: "OTHER".to_string(),
                    value: "".to_string(),
                  }
                };
              }
            }
            if next_token.token_type == "IDENTIFIER" {
              if let Some((eq_token, _, _)) = get_next_token(cidx, 1, tokens) {
                if eq_token.value == "=" {
                  str.push_str(
                    format!("{} = {} ", next_token.value, decl.replacement.clone()).as_str(),
                  );
                  if let Some((_, eq_idx)) = find_next_token(
                    index,
                    tokens,
                    "OTHER",
                    Some("="),
                    Some(("WHITESPACE", Some("\n"))),
                  ) {
                    if !args.is_empty() {
                      str.push_str(args.as_str());
                      str.push_str(",");
                    }
                    additional_idx = eq_idx - index
                  } else {
                    if !args.is_empty() {
                      str.push_str(args.as_str());
                    } else {
                      str = String::from(str.trim());
                      str.push_str("()");
                    }
                    additional_idx = cidx - index;
                  }
                } else {
                  return None;
                }
              } else {
                return None;
              }
            } else {
              return None;
            }
            return Some((index + 1 + additional_idx, str));
          } else {
            return Some((index + 1 + additional_idx, decl.replacement.clone()));
          }
        }
      }
    }
  }
  None
}

fn get_string_until(
  tokens: &[Token],
  start: usize,
  end_chars: &[&str],
  end_types: &[&str],
) -> (String, usize) {
  let mut result = String::new();
  let mut i = start;
  while i < tokens.len() {
    if end_chars.contains(&tokens[i].value.as_str()) {
      break;
    }
    if end_types.contains(&tokens[i].token_type.as_str()) {
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
      result.push_str(&format!("rew::mod::find module, {}", token.value));
      current_idx += 1;
    }
    "IDENTIFIER" | "OTHER" => {
      if token.value == "{" {
        let (imports, new_idx) = get_string_until(tokens, current_idx + 1, &["}"], &[]);
        current_idx = new_idx + 1;

        while current_idx < tokens.len() && tokens[current_idx].value != "from" {
          current_idx += 1;
        }

        current_idx += 1;

        if let Ok((should_handle, _)) = finalize_handle_import(tokens, current_idx) {
          if should_handle {
            let re = Regex::new(r"(\w+)\s+as\s+(\w+)").unwrap();
            let replaced_imports = re.replace_all(&imports, "$1: $2").to_string();
            result.push_str(&format!(
              "{{ {} }} := rew::mod::find module, ",
              replaced_imports
            ));
          }
        }
      } else {
        let mut default_name = token.value.clone();
        let mut used_multiple = false;

        while current_idx < tokens.len() && tokens[current_idx].value != "from" {
          if tokens[current_idx].value == "as" {
            if let Some((token, _, _)) = get_next_token(current_idx + 1, 1, &tokens) {
              if token.token_type == "IDENTIFIER" {
                default_name = token.value;
              }
            }
          } else if tokens[current_idx].value == "{" {
            let (imports, new_idx) = get_string_until(tokens, current_idx + 1, &["}"], &[]);
            let re = Regex::new(r"(\w+)\s+as\s+(\w+)").unwrap();
            let replaced_imports = re.replace_all(&imports, "$1: $2").to_string();
            default_name.insert_str(0, &format!("{{ {} }} = ", replaced_imports));
            used_multiple = true;
            current_idx = new_idx + 1;
          }
          current_idx += 1;
        }

        current_idx += 1;

        if let Ok((should_handle, _)) = finalize_handle_import(tokens, current_idx) {
          let slug = if used_multiple { "=" } else { ":=" };
          if should_handle {
            result.push_str(&format!(
              "{} {} rew::mod::find module, ",
              default_name, slug
            ));
          }
        }
      }
    }
    _ => {}
  }

  if let Some((_, assert_idx)) =
    find_next_token(current_idx, tokens, "IDENTIFIER", Some("assert"), None)
  {
    if let Some((from_token, _)) = find_next_token(current_idx - 1, tokens, "STRING", None, None) {
      result.push_str(&format!("{}, ", from_token.value.trim()));
    }
    current_idx = assert_idx + 1;
    // if let Some((brace_token, brace_idx)) =
    //     find_next_token(assert_idx + 1, tokens, "OTHER", Some("{"))
    // {
    //     let (assert_obj, new_idx) = get_string_until(tokens, brace_idx + 1, &["}"]);
    //     result.push_str(&format!("{}", assert_obj.trim()));
    //     current_idx = new_idx + 1;
    // }
  }

  (result, current_idx)
}

fn handle_compiler_options(
  tokens: &[Token],
  options: &mut CompilerOptions,
  i: usize,
  is_pub: bool,
) -> usize {
  let mut current_idx = i + 1;

  if let Some((name_token, idx)) = find_next_token(current_idx, &tokens, "IDENTIFIER", None, None) {
    let mut name = name_token.value.clone();
    current_idx = idx + 1;
    if let Some((_dot, _, idx)) = get_next_token(idx, 1, &tokens) {
      if _dot.value == "." {
        if let Some((_state, _, idx)) = get_next_token(idx, 1, &tokens) {
          current_idx = idx + 1;
          if _state.token_type == "IDENTIFIER" {
            name.push_str(format!(".{}", _state.value).as_str())
          }
        }
      }
    }

    options.civet_options.push(name.clone());
    if is_pub {
      options.civet_global.push(name.clone());
    }
  }

  current_idx
}

// i hate this function too!
pub fn compile_rew_stuff(content: &str, options: &mut CompilerOptions) -> Result<CompilerResults> {
  let tokens = tokenize_coffee_script(content);
  let mut result = String::new();
  let mut i = 0;
  let mut hooks: Vec<Hook> = Vec::new();
  let local_declarations = options.local_declarations.clone();
  let global_declarations = options.global_declarations.clone();

  while i < tokens.len() {
    let token = &tokens[i];
    let next_token = get_next_token(i, 1, &tokens);
    let prev_token = if i > 1 {
      get_prev_token(i, 1, &tokens)
    } else {
      None
    };

    // shebang
    if token.token_type == "COMMENT" && i < 2 && token.value.starts_with("#!") {
      i += 1;
      continue;
    }

    if token.token_type == "IDENTIFIER" && token.value == "fn" && i < 2 {
      if let Some((next, _, _)) = next_token.clone() {
        if prev_token.clone().map_or(true, |(t, _, _)| t.value != ".")
          && next.token_type == "IDENTIFIER"
        {
          result.push_str("function");
          i += 1;
          continue;
        }
      }
    }
     
    if prev_token.clone().map_or(true, |(t, _, _)| t.value != ".")
      && prev_token.clone().map_or(true, |(t, _, _)| t.value != ":")
      && token.value == "function"
      && next_token
        .clone()
        .map_or(false, |(t, _, _)| t.token_type == "IDENTIFIER")
    {
      if let Some((_, _, idx)) = next_token.clone() {
        if let Some((next_token, _, idx)) = get_next_token(idx, 1, &tokens) {
          if next_token.value == "." || next_token.value == ":" {
            let (_, start_idx) = find_next_token(idx, &tokens, "OTHER", Some("("), None).unwrap();
            let (_, end_idx) = find_next_token(idx, &tokens, "OTHER", Some(")"), None).unwrap();
            hooks.push(Hook {
              index: start_idx - 1,
              value: " = ".to_string()
            });
            if let Some((after_end, _, idx)) = get_next_token(end_idx, 1, &tokens) {
              if after_end.value == ":" {
                let (_, identifier_idx) = find_next_token(idx, &tokens, "IDENTIFIER", None, None).unwrap();
                 hooks.push(Hook {
                  index: identifier_idx,
                  value: " ->".to_string()
                });
              } else {
                hooks.push(Hook {
                  index: end_idx,
                  value: " ->".to_string()
                });
              }
            } else {
              hooks.push(Hook {
                index: end_idx,
                value: " ->".to_string()
              });
            }
            i += 2;
            continue;
          }
        }
      }
    }

    if prev_token.clone().map_or(true, |(t, _, _)| t.value != ".")
      && prev_token.clone().map_or(true, |(t, _, _)| t.value != ":")
      && token.value == "using"
      && next_token
        .clone()
        .map_or(false, |(t, _, _)| t.value == "JSX")
    {
      options.jsx = true;
    }

    if prev_token.clone().map_or(true, |(t, _, _)| t.value != ".")
      && prev_token.clone().map_or(true, |(t, _, _)| t.value != ":")
      && token.value == "using"
      && next_token
        .clone()
        .map_or(false, |(t, _, _)| t.value == "compiler" || t.value == "pub")
    {
      if let Some((next, _, idx)) = next_token.clone() {
        if next.value == "pub" {
          if let Some((next_token, _, idx)) = get_next_token(idx, 1, &tokens) {
            if next_token.value == "compiler" {
              i = handle_compiler_options(&tokens, options, idx, true);
              continue;
            } else if next_token.value == "JSX" {
              options.jsx = true;
              options.civet_global.push("JSX".to_string());
            }
          } 
        } else {
          i = handle_compiler_options(&tokens, options, idx, false);
          continue;
        }
      }
    }

    if token.token_type == "COMMENT" && token.value[1..].trim() == "@cls" {
      options.cls = true;
    }

    if prev_token
      .clone()
      .map_or(true, |(t, _, _)| t.value == "export")
      && token.token_type == "IDENTIFIER"
      && token.value == "default"
      && !options.keep_imports
    {
      i += 1;
      continue;
    }

    if prev_token.clone().map_or(true, |(t, _, _)| t.value != ".")
      && prev_token.clone().map_or(true, |(t, _, _)| t.value != ":")
      && token.token_type == "IDENTIFIER"
      && token.value == "package"
    {
      if let Some((next_token, _, idx)) = get_next_token(i, 1, &tokens) {
        if next_token.token_type == "IDENTIFIER" {
          let (item, new_idx) = get_string_until(&tokens, idx, &[";"], &["WHITESPACE"]);
          result.push_str(format!("rew::mod::package \"{}\"", item).as_str());
          i = new_idx;
        } else {
          i = i + 1;
        }
        continue;
      }
    }

    if prev_token.clone().map_or(true, |(t, _, _)| t.value != ".")
      && prev_token.clone().map_or(true, |(t, _, _)| t.value != ":")
      && token.token_type == "IDENTIFIER"
      && token.value == "export"
      && !options.keep_imports
    {
      // println!("{:?}", prev_token.clone().map_or("".to_string(), |(t, _, _)| t.value));
      if let Some((next_token, _, idx)) = get_next_token(i, 1, &tokens) {
        if next_token.value == "{" {
          result.push_str("module.exports = ");
        } else {
          let mut title = next_token.value.clone();
          if next_token.value == "default" {
            i += 1;
          }
          if next_token.value == "class" {
            if let Some((next_token, _, _)) = get_next_token(idx, 1, &tokens) {
              title = next_token.value.clone();
            }
          }
          if next_token.value == "function" {
            if let Some((next_token, _, _)) = get_next_token(idx, 1, &tokens) {
              title = next_token.value.clone();
            }
          }

          result.push_str(format!("module.exports.{} = ", title).as_str());
        }
      }
      i += 1;
      continue;
    }

    if prev_token.map_or(true, |(t, _, _)| t.value != ".")
      && token.token_type == "IDENTIFIER"
      && token.value == "import"
      && !options.keep_imports
    {
      let (import_str, new_idx) = handle_import(&tokens, i);
      result.push_str(&import_str);
      i = new_idx;
      continue;
    }

    if let Some((new_idx, replacement)) =
      apply_declarations(token, i, &tokens, &local_declarations, &global_declarations)
    {
      result.push_str(&replacement);
      i = new_idx;
      continue;
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
    options.local_declarations = local_declarations;
  }

  let compiler_results = CompilerResults {
    options: std::mem::replace(
      options,
      CompilerOptions {
        keep_imports: false,
        civet_options: vec![],
        civet_global: vec![],
        jsx: false,
        cls: false,
        included: false,
        local_declarations: HashMap::new(),
        global_declarations: HashMap::new(),
      },
    ),
    code: result,
  };

  // println!("{}", compiler_results.code);
  Ok(compiler_results)
}
