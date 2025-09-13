use rew_core::logger;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

// Template constants based on the CoffeeScript templates
const CIVET_CONFIG: &str = r#"{
  "parseOptions": {
    "coffeePrototype": true,
    "autoLet": true,
    "coffeeInterpolation": true,
    "coffeeComment": true
  }
}"#;

const TYPES_TEMPLATE: &str = r#"// rew-global.d.ts

declare namespace Rew {
  interface ChannelContext {
    stop(): ChannelContext;
    start(): ChannelContext;
    setpoll(cb: () => void): ChannelContext;
  }

  interface Usage {
    name: string;
    system: (ctx: any, ...args: any[]) => void;
    args?: any[];
  }

  interface SubPackage {
    define(name: string, value: any): void;
    prototype?: Record<string, any>;
    packageName?: string;
    name?: string;
  }
}

declare const module: {
  filename: string,
  exports: any,
  options: Record<string, any>,
  app: {
    path: string,
    config: {
      manifest: {
        package: string,
        [key: string]: any
      },
      [key: string]: any
    }
  }
};

declare function print(...args: any[]): void;
declare function printf(format: string, ...args: any[]): void;
declare function input(...args: any[]): string;
"#;

const GITIGNORE_TEMPLATE: &str = r#"
**/target/
**/**.qrew
**/**.brew
.artifacts/
**/.artifacts/
**/pimmy.lock"#;

const MAIN_CODE: &str = r#"
using namespace rew::ns;

export function main()
  print "Hello!"
"#;

const TSC_CONFIG: &str = r#"{
  "compilerOptions": {
    "lib": ["ESNext", "DOM"]
  },
  "include": ["./_types/**.ts"]
}"#;

fn yesify(value: bool) -> String {
  if value {
    "yes".to_string() // In actual implementation, this would be colored cyan
  } else {
    "no".to_string() // In actual implementation, this would be colored yellow
  }
}

fn is_yes(input: &str) -> bool {
  input.trim().to_lowercase().starts_with('y')
}

fn prompt_user(message: &str) -> bool {
  print!("{} ", message);
  io::stdout().flush().unwrap();

  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => is_yes(&input),
    Err(_) => false,
  }
}

fn optionify(logs: &[&str], ignore: bool, _key: &str, current_value: bool) -> bool {
  if ignore {
    logger::info(&format!("{} {}", logs.join(" "), yesify(current_value)));
    current_value
  } else {
    let prompt = format!("{} ", logs.join(" "));
    prompt_user(&prompt)
  }
}

fn create_app_yaml(
  path: &Path,
  app_name: &str,
  use_types: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  let main_file = if use_types {
    "main.civet"
  } else {
    "main.coffee"
  };

  let content = format!(
    "manifest:\n  package: {}\nentries:\n  main: {}\n",
    app_name, main_file
  );

  let app_yaml_path = path.join("app.yaml");
  fs::write(&app_yaml_path, content)?;

  logger::info(&format!("Created file {}", "app.yaml"));
  Ok(())
}

fn create_main_file(path: &Path, use_types: bool) -> Result<(), Box<dyn std::error::Error>> {
  let (filename, content) = if use_types {
    ("main.civet", MAIN_CODE)
  } else {
    ("main.coffee", MAIN_CODE)
  };

  let main_path = path.join(filename);
  fs::write(&main_path, content)?;

  logger::info(&format!("Created file {}", filename));
  Ok(())
}

fn create_types_files(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
  // Create _types directory
  let types_dir = path.join("_types");
  fs::create_dir_all(&types_dir)?;

  let index_path = types_dir.join("index.d.ts");
  fs::write(&index_path, TYPES_TEMPLATE)?;
  logger::info(&format!("Created file {}", "index.d.ts"));

  let tsconfig_path = path.join("tsconfig.json");
  fs::write(&tsconfig_path, TSC_CONFIG)?;
  logger::info(&format!("Created file {}", "tsconfig.json"));

  let civet_config_path = path.join("civet.config.json");
  fs::write(&civet_config_path, CIVET_CONFIG)?;
  logger::info(&format!("Created file {}", "civet.config.json"));

  Ok(())
}

fn create_git_files(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
  logger::info("git init");
  let output = Command::new("git")
    .args(&["init", "."])
    .current_dir(path)
    .output();

  match output {
    Ok(result) => {
      if !result.status.success() {
        logger::warn(&format!(
          "Git init failed: {}",
          String::from_utf8_lossy(&result.stderr)
        ));
      }
    }
    Err(e) => {
      logger::warn(&format!("Failed to run git init: {}", e));
    }
  }

  // Create .gitignore
  let gitignore_path = path.join(".gitignore");
  fs::write(&gitignore_path, GITIGNORE_TEMPLATE)?;
  logger::info(&format!("Created file {}", ".gitignore"));

  Ok(())
}

pub fn new(path: String, git: bool, ignore: bool, types: bool) {
  // Determine the target path
  let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
  let new_path = if path == "." || path.is_empty() {
    current_dir.clone()
  } else {
    current_dir.join(path.clone())
  };

  // Check if directory exists and is populated
  if new_path.exists() {
    match fs::read_dir(&new_path) {
      Ok(entries) => {
        if entries.count() > 0 {
          logger::error("Cannot overwrite a populated directory");
          return;
        }
      }
      Err(e) => {
        logger::error(&format!("Failed to read directory: {}", e));
        return;
      }
    }
  }

  let display_path = if path == "true" || path.is_empty() {
    ".".to_string()
  } else {
    path.clone()
  };

  logger::info(&format!("Creating at {}", display_path));

  let app_name = new_path
    .file_name()
    .and_then(|name| name.to_str())
    .unwrap_or("app")
    .to_string();

  logger::info(&format!("package: {}", app_name));

  let use_git = optionify(&["git?"], ignore, "git", git);

  let use_types = optionify(&["types?"], ignore, "types", types);

  logger::info("Options set");
  logger::info("Creating files");

  if let Err(e) = fs::create_dir_all(&new_path) {
    logger::error(&format!("Failed to create directory: {}", e));
    return;
  }

  if let Err(e) = create_app_yaml(&new_path, &app_name, use_types) {
    logger::error(&format!("Failed to create app.yaml: {}", e));
    return;
  }

  if let Err(e) = create_main_file(&new_path, use_types) {
    logger::error(&format!("Failed to create main file: {}", e));
    return;
  }

  if use_types {
    if let Err(e) = create_types_files(&new_path) {
      logger::error(&format!("Failed to create types files: {}", e));
      return;
    }
  }

  if use_git {
    if let Err(e) = create_git_files(&new_path) {
      logger::error(&format!("Failed to create git files: {}", e));
      return;
    }
  }

  logger::info("Files Created");
  logger::info(&format!("Project '{}' created successfully!", app_name));
}
