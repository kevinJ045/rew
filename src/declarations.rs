use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Declaration {
  pub trigger: String,
  pub replacement: String,
  #[allow(unused)]
  pub is_definition: bool,
  #[allow(unused)]
  pub is_constructor: bool,
  #[allow(unused)]
  pub is_macro: bool,
  pub condition_prev: Option<String>, // New field for ONLYIF prev condition
  pub condition_next: Option<String>, // New field for ONLYIF next condition
}

impl Declaration {
  pub fn new(trigger: &str, replacement: &str) -> Self {
    let is_definition = trigger.starts_with('=');
    let is_constructor = trigger.ends_with('*');
    let is_macro = trigger.ends_with('!');

    let (replacement_text, condition_prev, condition_next) = Self::parse_onlyif(replacement);

    Self {
      trigger: trigger
        .trim_end_matches(['*', '!'].as_ref())
        .trim_start_matches(['='].as_ref())
        .to_string(),
      replacement: replacement_text,
      is_definition,
      is_constructor,
      is_macro,
      condition_prev,
      condition_next,
    }
  }

  // New method to parse ONLYIF conditions
  fn parse_onlyif(replacement: &str) -> (String, Option<String>, Option<String>) {
    // Check if ONLYIF is present in the replacement
    let re = Regex::new(r"ONLYIF\(([^)]+)\)").unwrap();

    if let Some(caps) = re.captures(replacement) {
      let conditions = &caps[1];
      let mut prev_condition = None;
      let mut next_condition = None;

      // Parse the conditions
      for condition in conditions.split(',') {
        let parts: Vec<&str> = condition.split('=').collect();
        if parts.len() == 2 {
          let key = parts[0].trim();
          let value = parts[1].trim().trim_matches('"').trim_matches('\'');

          if key == "prev" {
            prev_condition = Some(value.to_string());
          } else if key == "next" {
            next_condition = Some(value.to_string());
          }
        }
      }

      // Remove the ONLYIF part from the replacement
      let replacement_text = re.replace(replacement, "").trim().to_string();

      (replacement_text, prev_condition, next_condition)
    } else {
      // No ONLYIF condition found
      (replacement.to_string(), None, None)
    }
  }
}

#[derive(Default, Clone)]
pub struct DeclarationEngine {
  pub global_declarations: HashMap<String, Declaration>,
}

impl DeclarationEngine {
  pub fn parse_declaration(
    &mut self,
    line: &str,
    local_declarations: &mut HashMap<String, Declaration>,
  ) -> bool {
    let tokens =
      crate::compiler::tokenize_coffee_script(line.replace("#", "").replace("//", "").as_str());
    // println!("{}", line);
    // Check if this is a declaration
    if tokens.len() < 4 {
      return false;
    }

    if tokens[0].value != "declare" {
      return false;
    }

    // Check for global marker (*)
    let mut token_index = 1;
    let is_global = if token_index < tokens.len() && tokens[token_index].value == "*" {
      token_index += 1;
      true
    } else {
      false
    };

    // Skip whitespace
    while token_index < tokens.len() && tokens[token_index].token_type == "WHITESPACE" {
      token_index += 1;
    }

    // Get the trigger (should be a string)
    if token_index >= tokens.len() || tokens[token_index].token_type != "STRING" {
      return false;
    }

    // Extract the trigger without quotes
    let trigger = tokens[token_index]
      .value
      .trim_matches('"')
      .trim_matches('\'');
    token_index += 1;

    // Skip whitespace
    while token_index < tokens.len() && tokens[token_index].token_type == "WHITESPACE" {
      token_index += 1;
    }

    // Check for equals sign
    if token_index >= tokens.len() || tokens[token_index].value != "=" {
      return false;
    }
    token_index += 1;

    // Skip whitespace
    while token_index < tokens.len() && tokens[token_index].token_type == "WHITESPACE" {
      token_index += 1;
    }

    // Get the replacement (everything until semicolon or end)
    let mut replacement = String::new();
    while token_index < tokens.len() && tokens[token_index].value != ";" {
      replacement.push_str(&tokens[token_index].value);
      token_index += 1;
    }

    // println!("Parsed declaration: trigger={}, replacement={}", trigger, replacement);

    // Create the declaration
    let decl = Declaration::new(trigger, replacement.trim());

    // Print the parsed declaration for debugging
    // println!("Created declaration: trigger={}, replacement={}, prev={:?}, next={:?}",
    //          decl.trigger, decl.replacement, decl.condition_prev, decl.condition_next);

    // Generate a unique name for the declaration
    let name = format!("decl_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));

    if is_global {
      self.global_declarations.insert(name, decl);
    } else {
      local_declarations.insert(name, decl);
    }

    true
  }

  pub fn process_script(&mut self, script: &str) -> HashMap<String, Declaration> {
    let mut local_declarations = HashMap::new();
    let mut in_multiline = false;
    let mut multiline_buffer = String::new();

    for line in script.lines() {
      let trimmed = line.trim();

      // Handle multiline declarations
      if in_multiline {
        multiline_buffer.push_str(line);
        multiline_buffer.push('\n');

        if trimmed.ends_with(';') {
          // End of multiline declaration
          in_multiline = false;
          self.parse_declaration(&multiline_buffer, &mut local_declarations);
          multiline_buffer.clear();
        }
        continue;
      }

      // Check for declaration start
      if trimmed.starts_with("#declare") || trimmed.starts_with("//declare") {
        if trimmed.ends_with(';') {
          // Single line declaration
          self.parse_declaration(trimmed, &mut local_declarations);
        } else {
          // Start of multiline declaration
          in_multiline = true;
          multiline_buffer = trimmed.to_string();
          multiline_buffer.push('\n');
        }
      }
    }

    // Handle any remaining multiline declaration
    if in_multiline && !multiline_buffer.is_empty() {
      self.parse_declaration(&multiline_buffer, &mut local_declarations);
    }

    local_declarations
  }
}
