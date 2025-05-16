use regex::Regex;
use std::collections::HashMap;
use uuid;

#[derive(Debug, Clone)]
pub struct Declaration {
  pub trigger: String,
  pub replacement: String,
  pub is_definition: bool,
  pub is_constructor: bool,
}

impl Declaration {
  pub fn new(trigger: &str, replacement: &str) -> Self {
    let is_definition = trigger.ends_with('*');
    let is_constructor = trigger.ends_with('!');
    Self {
      trigger: trigger.trim_end_matches(['*', '!'].as_ref()).to_string(),
      replacement: replacement.to_string(),
      is_definition,
      is_constructor,
    }
  }
}

#[derive(Default)]
pub struct DeclarationEngine {
  pub global_declarations: HashMap<String, Declaration>,
}

impl DeclarationEngine {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn parse_declaration(
    &mut self,
    line: &str,
    local_declarations: &mut HashMap<String, Declaration>,
  ) -> bool {
    // Updated regex pattern for the new declaration format
    let re = Regex::new(r#"^#declare(\*?)\s+"([^"]+)"\s*=\s*(.+?)(?:;|$)"#).unwrap();
    if let Some(caps) = re.captures(line) {
      let is_global = &caps[1] == "*";
      let trigger = &caps[2];
      let replacement = &caps[3].trim();

      let decl = Declaration::new(trigger, replacement);
      
      // Generate a unique name for the declaration
      let name = format!("decl_{}", trigger);

      if is_global {
        if self.global_declarations.contains_key(&name) {
          self.global_declarations.remove(&name);
        }
        self.global_declarations.insert(name, decl);
      } else {
        if local_declarations.contains_key(&name) {
          local_declarations.remove(&name);
        }
        local_declarations.insert(name, decl);
      }
      true
    } else {
      false
    }
  }

  pub fn process_script(&mut self, script: &str) -> HashMap<std::string::String, Declaration> {
    let mut local_declarations = HashMap::new();

    for line in script.lines() {
      if self.parse_declaration(line, &mut local_declarations) {
        continue;
      }
    }

    return local_declarations;
  }
}
