use anyhow::Result;
use deno_core::v8::ContextOptions;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

use crate::declarations::Declaration;

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
  pub compiler_type: String,

  pub local_declarations: HashMap<String, Declaration>,
  pub global_declarations: HashMap<String, Declaration>,
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
      compiler_type: "coffee".to_string(),
      local_declarations: HashMap::new(),
      global_declarations: HashMap::new(),
    }
  }
}

pub struct CompilerResults {
  pub options: CompilerOptions,
  pub code: String,
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
    
    // Check if we should break on this token type/value
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
    
    // Check if this is the token we're looking for
    if token.token_type != "WHITESPACE" {
      if token.token_type == expected_type {
        if let Some(val) = expected_value {
          if token.value == val {
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

// New function to process declaration comments with the new pattern
fn process_declaration_comment(
  comment: &str,
  local_declarations: &mut HashMap<String, Declaration>,
  global_declarations: &mut HashMap<String, Declaration>,
) -> bool {
  if comment.trim_start().starts_with("#declare") {
    // New simplified pattern: #declare(*|nothing) "trigger" = replacement;
    let re = Regex::new(r#"#declare(\*?)\s+"([^"]+)"\s*=\s*(.+?)(?:;|$)"#).unwrap();
    if let Some(caps) = re.captures(comment) {
      let is_global = &caps[1] == "*";
      let trigger = &caps[2];
      let replacement = &caps[3].trim();

      let decl = Declaration::new(trigger, replacement);
      
      // Generate a unique name for the declaration
      let name = format!("decl_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
      
      if is_global {
        global_declarations.insert(name, decl);
      } else {
        local_declarations.insert(name, decl);
      }
      return true;
    }
  }
  false
}

// New function to apply declarations to a token
fn apply_declarations(
  token: &Token,
  index: usize,
  tokens: &[Token],
  local_declarations: &HashMap<String, Declaration>,
  global_declarations: &HashMap<String, Declaration>,
) -> Option<(usize, String)> {
  let mut additional_idx = 0;
  if token.token_type == "IDENTIFIER" {
    let values = global_declarations.values().chain(local_declarations.values());
    for decl in values {
      let mut isDeclaration = false;
      let trigger = if decl.trigger.starts_with("=") {
        isDeclaration = true;
        decl.trigger.replace("=", "")
      } else {
        decl.trigger.clone()
      };
      // println!("trigget: {} value: {}", trigger, token.value);
      if token.value == trigger {
        println!("Token Replacement Found");
        if isDeclaration {
          let mut str = String::new();
          let mut args = String::new();
          let mut cidx = index;
          let mut next_token = if let Some((token, _, idx)) = get_next_token(index, 1, tokens) {
            cidx = idx;
            token
          } else {
            Token {
              token_type: "OTHER".to_string(),
              value: "".to_string()
            }
          };
          if next_token.token_type == "OTHER" && next_token.value == "(" {
            if let Some((_, bc_idx)) =
              find_next_token(index, tokens, "OTHER", Some(")"), None)
            {
              let mut arg_tokens = Vec::new();
              let mut arg_idx = cidx + 1;
              
              while arg_idx < bc_idx {
                arg_tokens.push(&tokens[arg_idx]);
                arg_idx += 1;
              }
              
              args = arg_tokens.iter().map(|t| t.value.clone()).collect::<Vec<String>>().join("");
              
              next_token = if let Some((token, _, new_idx)) = get_next_token(bc_idx, 1, tokens) {
                cidx = new_idx;
                token
              } else {
                Token {
                  token_type: "OTHER".to_string(),
                  value: "".to_string()
                }
              };
            }
          }
          if next_token.token_type == "IDENTIFIER" {
            println!("Token found");
            println!("{}", next_token.value);
            str.push_str(format!(
              "{} = {} ",
              next_token.value,
              decl.replacement.clone()
            ).as_str());
            if let Some((_, eq_idx)) =
              find_next_token(index, tokens, "OTHER", Some("="), Some(("WHITESPACE", Some("\n"))))
            {
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
          return Some((index + 1 + additional_idx, str));
        } else {
          return Some((index + 1 + additional_idx, decl.replacement.clone()));
        }
      }
    }
  }
  None
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
      result.push_str(&format!("rew::mod::find module, {}", token.value));
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
            result.push_str(&format!(
              "{{ {} }} := rew::mod::find module, ",
              replaced_imports
            ));
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
            result.push_str(&format!("{} := rew::mod::find module, ", default_name));
          }
        }
      }
    }
    _ => {}
  }

  if let Some((assert_token, assert_idx)) =
    find_next_token(current_idx, tokens, "IDENTIFIER", Some("assert"), None)
  {
    if let Some((from_token, _)) = find_next_token(current_idx - 1, tokens, "STRING", None, None)
    {
      result.push_str(&format!("{}, ", from_token.value.trim()));
    }
    current_idx = assert_idx + 1;
    // // Found 'assert' keyword, so parse assertion object
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

// New function to handle declaration-based transformations
fn transform_line_with_declarations(
  line: &str,
  local_declarations: &HashMap<String, Declaration>,
  global_declarations: &HashMap<String, Declaration>,
) -> String {
  let mut output = line.to_string();

  // First, check for function-style declarations: sayhello(arg1, arg2) VAR = SOMETHING
  let func_pattern = Regex::new(r"(\w+)\(([^)]*)\)\s+(\w+)\s*=\s*(.+)").unwrap();
  if let Some(caps) = func_pattern.captures(&output) {
    let func_name = &caps[1];
    let args = &caps[2];
    let var_name = &caps[3];
    let value = &caps[4];

    // Check if this function name matches a declaration
    for decl in local_declarations.values().chain(global_declarations.values()) {
      if decl.trigger == func_name {
        return format!("{} = {} {}, {}", var_name, decl.replacement, args, value);
      }
    }
  }

  // Then check for simple declarations: sayhello VAR = SOMETHING
  let simple_pattern = Regex::new(r"(\w+)\s+(\w+)\s*=\s*(.+)").unwrap();
  if let Some(caps) = simple_pattern.captures(&output) {
    let trigger = &caps[1];
    let var_name = &caps[2];
    let value = &caps[3];

    // Check if this trigger matches a declaration
    for decl in local_declarations.values().chain(global_declarations.values()) {
      if decl.trigger == trigger {
        if decl.is_constructor {
          return format!("{} = new {}({})", var_name, decl.replacement, value);
        } else if decl.is_definition {
          return format!("{} = {}()", var_name, decl.replacement);
        } else {
          return format!("{} = {} {}", var_name, decl.replacement, value);
        }
      }
    }
  }

  output
}

// i hate this function too!
pub fn compile_rew_stuff(content: &str, options: &mut CompilerOptions) -> Result<CompilerResults> {
  let tokens = tokenize_coffee_script(content);
  let mut result = String::new();
  let mut i = 0;
  let mut hooks: Vec<Hook> = Vec::new();
  let mut multiline_declare_buffer: Vec<String> = Vec::new();
  let mut multiline_declare = false;
  let mut local_declarations = options.local_declarations.clone();
  let mut global_declarations = options.global_declarations.clone();

  while i < tokens.len() {
    let token = &tokens[i];
    let next_token = get_next_token(i, 1, &tokens);
    let prev_token = if i > 1 {
      get_next_token(i, -2, &tokens)
    } else {
      None
    };

    // Skip shebang
    if token.token_type == "COMMENT" && i < 2 && token.value.starts_with("#!") {
      i += 1;
      continue;
    }

    if token.token_type == "IDENTIFIER" && token.value == "fn" && i < 2 {
      if let Some((next, _, _)) = next_token {
        if prev_token.clone().map_or(true, |(t, _, _)| t.value != ".")
          && next.token_type == "IDENTIFIER"
        {
          result.push_str("function");
          i += 1;
          continue;
        }
      }
    }

    // Handle multiline declarations
    if (token.token_type == "COMMENT" && multiline_declare)
      || (token.token_type != "COMMENT" && multiline_declare)
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
          process_declaration_comment(&combined, &mut local_declarations, &mut global_declarations);
          multiline_declare_buffer.clear();
        }
      } else {
        multiline_declare = false;
        multiline_declare_buffer.clear();
      }
    }

    // Process single-line declarations
    if token.token_type == "COMMENT" && token.value.trim_start().starts_with("#declare") {
      let value = token.value.trim_start().to_string();

      if value.ends_with(';') {
        // Single-line declare
        process_declaration_comment(&value, &mut local_declarations, &mut global_declarations);
      } else {
        // Begin multiline declare block
        multiline_declare = true;
        multiline_declare_buffer.clear();
        let cleaned = if value.starts_with("###") {
          value[3..].trim().to_string()
        } else if value.starts_with("#") {
          value[1..].trim().to_string()
        } else {
          value
        };
        multiline_declare_buffer.push(cleaned);
      }
      i += 1;
      continue;
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

    if prev_token.clone().map_or(true, |(t, _, _)| t.value != ".")
      && token.token_type == "IDENTIFIER"
      && token.value == "export"
      && !options.keep_imports
    {
      result.push_str("pub");
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

    // Apply declarations
    if let Some((new_idx, replacement)) = apply_declarations(token, i, &tokens, &local_declarations, &global_declarations) {
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
        disable_use: false,
        jsx: false,
        jsx_pragma: None,
        cls: false,
        included: false,
        filename: None,
        compiler_type: String::new(),
        local_declarations: HashMap::new(),
        global_declarations: HashMap::new(),
      },
    ),
    code: result,
  };

  println!("{}", compiler_results.code);
  Ok(compiler_results)
}
