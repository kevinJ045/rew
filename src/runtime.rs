use super::civet::get_civet_script;
use super::compiler::{compile_rew_stuff, CompilerOptions};
use crate::builtins::BUILTIN_MODULES;
use crate::compiler::CompilerResults;
use crate::data_manager::{DataFormat, DataManager};
use crate::declarations::{Declaration, DeclarationEngine};
use crate::ext::{console, ffi, url, web, webidl};
use crate::runtime_script::get_runtime_script;
use crate::utils::find_app_path;
use anyhow::{Context, Result};
use base64::decode;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use deno_core::error::CoreError;
use deno_core::OpState;
use deno_core::PollEventLoopOptions;
use deno_core::{extension, op2, JsRuntime, RuntimeOptions};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Mutex;
use crate::workers::{
  op_thread_spawn,
  op_thread_message,
  op_thread_post_message,
  op_thread_terminate,
  op_thread_receive,
};

fn encode_brew_file(content: &str) -> String {
    BASE64.encode(content.as_bytes())
}

fn decode_brew_file(encoded: &str) -> Result<String> {
    let decoded = BASE64.decode(encoded.trim())
        .map_err(|e| anyhow::anyhow!("Failed to decode brew file: {}", e))?;
    
    String::from_utf8(decoded)
        .map_err(|e| anyhow::anyhow!("Failed to convert decoded bytes to string: {}", e))
}

pub static VIRTUAL_FILES: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| {
  Mutex::new(vec![])
});

pub fn add_virtual_file(path: &str, contents: &str) {
  let mut files = VIRTUAL_FILES.lock().unwrap();
  files.push((path.to_string(), contents.to_string()));
}


#[derive(Default)]
struct RuntimeState {
  current_dir: PathBuf,
}


#[derive(Debug, Clone)]
pub struct BuildOptions {
  pub bundle_all: bool,
  pub entry_file: Option<PathBuf>,  
}

impl Default for BuildOptions {
  fn default() -> Self {
    BuildOptions {
      bundle_all: false,
      entry_file: None,
    }
  }
}

pub struct RewRuntime {
  pub runtime: JsRuntime,
  compiler_runtime: JsRuntime,
  declaration_engine: DeclarationEngine,
}

extension!(
  rewextension,
  ops = [
    op_fs_read,
    op_fs_write,
    op_fs_exists,
    op_fs_rm,
    op_fs_mkdir,
    op_fs_readdir,
    op_fs_stats,
    op_fs_copy,
    op_fs_rename,
    op_fs_cwd,
    op_to_base64,
    op_from_base64,
    op_find_app,
    op_yaml_to_string,
    op_string_to_yaml,
    op_app_loadconfig,
    op_data_read,
    op_data_write,
    op_data_delete,
    op_data_exists,
    op_data_list,
    op_data_read_binary,
    op_data_write_binary,
    op_data_read_yaml,
    op_data_write_yaml,
    op_data_get_info,
    op_thread_spawn,
    op_thread_message,
    op_thread_post_message,
    op_thread_terminate,
    op_thread_receive,
    op_get_env
  ]
);

fn get_compiler_runtime() -> JsRuntime {
  let mut compiler_runtime = JsRuntime::new(RuntimeOptions::default());
  compiler_runtime
    .execute_script("<civet>", get_civet_script())
    .unwrap();
  compiler_runtime
}

impl RewRuntime {
  pub fn new() -> Result<Self> {
    let compiler_runtime = get_compiler_runtime();

    let mut extensions = vec![rewextension::init_ops()];

    extensions.extend(webidl::extensions(false));
    extensions.extend(console::extensions(false));
    extensions.extend(url::extensions(false));
    extensions.extend(web::extensions(web::WebOptions::default(), false));
    extensions.extend(ffi::extensions(false));

    let mut runtime = JsRuntime::new(RuntimeOptions {
      extensions: extensions,
      module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
      ..Default::default()
    });

    let current_dir = std::env::current_dir()?;

    let state = RuntimeState {
      current_dir: current_dir.clone(),
    };

    runtime.op_state().borrow_mut().put(state);
    runtime.execute_script(
      "<setup>",
      r#"
	globalThis._execVM = (namespace, fn) => {
	 with(namespace){
		eval(`(${fn.toString()})()`);
	 }
	}
	"#,
    )?;
    runtime.execute_script("<setup>", get_runtime_script())?;

    let declaration_engine = DeclarationEngine {
      global_declarations: HashMap::new(),
    };

    Ok(Self {
      compiler_runtime,
      runtime,
      declaration_engine,
    })
  }

  pub fn resolve_includes_recursive_from<P: AsRef<Path>>(
    filepath: P,
  ) -> Result<Vec<(PathBuf, String, bool)>> {
    let filepath = filepath
      .as_ref()
      .canonicalize()
      .with_context(|| format!("Failed to resolve import"))?;

    let import_re = Regex::new(r#"(?m)^\s*import\s+(?:[^;]*?\s+from\s+)?["']([^"']+)["']"#)
      .context("Invalid regex pattern")?;
    let external_re = Regex::new(r#"(?m)^\s*// external\s+['"]([^'"]+)['"]"#)
      .context("Invalid regex pattern")?;

    let mut visited = HashSet::new();
    let mut result = Vec::new();

    fn visit_file(
      file_path: &Path,
      visited: &mut HashSet<PathBuf>,
      result: &mut Vec<(PathBuf, String, bool)>,
      preprocess_import: bool,
      import_re: &Regex,
      external_re: &Regex,
    ) -> Result<()> {
      if visited.contains(file_path) {
        return Ok(());
      }

      let mut should_preprocess = preprocess_import.clone();
      let file_path_unf = file_path.to_str().unwrap_or("");
      let file_path_str = if file_path_unf.ends_with('!') {
        should_preprocess = true;
        &file_path_unf[0..file_path_unf.len() - 1]
      } else {
        file_path_unf
      };
      
      let is_brew_file = file_path.extension().map_or(false, |ext| ext == "brew" || ext == "qrew");

      let real_content = if let Some(v) = VIRTUAL_FILES.lock().unwrap().iter().find(|(p, _)| p == file_path_str) {
        v.1.clone()
      } else if file_path_str.starts_with("#") {
        "".to_string()
      } else {
        fs::read_to_string(file_path).with_context(|| format!("Failed to read {:?}", file_path))?
      };

      let content = if file_path_str.starts_with("#") {
        if let Some(builtin_content) = BUILTIN_MODULES.get(file_path_str) {
          builtin_content.to_string()
        } else {
          return Err(anyhow::anyhow!(
            "Builtin module not found: {}",
            file_path_str
          ));
        }
      } else if is_brew_file {
        if let Ok(decoded) = decode_brew_file(&real_content) {
          decoded
        } else {
          real_content
        }
      } else {
        real_content
      };

      visited.insert(PathBuf::from(file_path_str));

      result.push((
        PathBuf::from(file_path_str),
        content.clone(),
        should_preprocess,
      ));

      let parent = file_path.parent().unwrap_or(Path::new("."));


      if is_brew_file {
        
        for cap in external_re.captures_iter(&content) {
          let external_app_path = cap[1].to_string();
          let mut should_preprocess_import = false;
          let external_app = if external_app_path.ends_with('!') {
            should_preprocess_import = true;
            &external_app_path[0..external_app_path.len() - 1]
          } else {
            &external_app_path
          };
          
          if external_app.contains('/') {
            let parts: Vec<&str> = external_app.split('/').collect();
            if parts.len() == 2 {
              let package_name = parts[0];
              let entry_name = parts[1];

              if let Some(app_entry) = crate::utils::resolve_app_entry(package_name, Some(entry_name)) {
                visit_file(&app_entry, visited, result, should_preprocess_import, import_re, external_re)?;
              }
            }
          } else {
            visit_file(&PathBuf::from(external_app), visited, result, should_preprocess_import, import_re, external_re)?;
          }
        }
      } else {
        for cap in import_re.captures_iter(&content) {
          let relative_path_raw = cap[1].to_string();
          let mut should_preprocess_import = false;
          let relative_path = if relative_path_raw.ends_with('!') {
            should_preprocess_import = true;
            &relative_path_raw[0..relative_path_raw.len() - 1]
          } else {
            &relative_path_raw
          };

          if relative_path.starts_with("#") {
            let builtin_path = PathBuf::from(&relative_path);
            visit_file(&builtin_path, visited, result, should_preprocess_import, import_re, external_re)?;
          } else if !relative_path.contains("/")
            && !relative_path.contains("\\")
            && !relative_path.starts_with(".")
          {
            if let Some(app_entry) = crate::utils::resolve_app_entry(&relative_path, None) {
              visit_file(&app_entry, visited, result, should_preprocess_import, import_re, external_re)?;
            } else {
              return Err(anyhow::anyhow!("App not found: {}", relative_path));
            }
          } else if relative_path.contains("/") && !relative_path.starts_with(".") {
            let parts: Vec<&str> = relative_path.splitn(2, "/").collect();
            if parts.len() == 2 {
              let package_name = parts[0];
              let entry_name = parts[1];

              if let Some(app_entry) = crate::utils::resolve_app_entry(package_name, Some(entry_name))
              {
                visit_file(&app_entry, visited, result, should_preprocess_import, import_re, external_re)?;
              } else {
                return Err(anyhow::anyhow!(
                  "App entry not found: {}/{}",
                  package_name,
                  entry_name
                ));
              }
            } else {
              let included_path = parent
                .join(relative_path)
                .canonicalize()
                .with_context(|| format!("Failed to resolve import"))?;

              visit_file(&included_path, visited, result, should_preprocess_import, import_re, external_re)?;
            }
          } else {
            let included_path = parent
              .join(relative_path)
              .canonicalize()
              .with_context(|| format!("Failed to resolve import"))?;

            visit_file(&included_path, visited, result, should_preprocess_import, import_re, external_re)?;
          }
        }
      }

      Ok(())
    }

    visit_file(&filepath, &mut visited, &mut result, false, &import_re, &external_re)?;
    Ok(result)
  }

  pub async fn prepare(
    &mut self,
    files: Vec<(PathBuf, String)>,
    entry: Option<&Path>,
  ) -> Result<String> {
    let mut module_wrappers = String::new();
    let mut entry_calls = Vec::new();

    for (path, source) in files {
      let compiled = self.compile_and_run(&source, &path).await?;
      let mod_id = path.to_str().unwrap_or("unknown");
      let mut mod_alias = String::new();

      if let Some(app_info) = crate::utils::find_app_info(&path) {
        if let Some(manifest) = &app_info.config.manifest {
          if let Some(package) = &manifest.package {
            if let Some(rel_path) = path.strip_prefix(&app_info.path).ok() {
              let rel_path_str = rel_path.to_str().unwrap_or("");

              if let Some(entries) = &app_info.config.entries {
                for (key, value) in entries {
                  if value == rel_path_str {
                    mod_alias.push_str(&format!("[\"app://{}/{}\"]", package, key));
                    break;
                  }
                }
              }

              let base_name = Path::new(rel_path_str).with_extension("").to_string_lossy().into_owned();
              if mod_alias.is_empty() {
                mod_alias.push_str(&format!("[\"app://{}/{}\"]", package, base_name))
              }
            }
          }
        }
      }

      if mod_id.starts_with('#') {
        module_wrappers.push_str(&format!("(function(module){{\n{compiled}\n}})({{filename: \"{id}\"}});", 
        id = mod_id,
        compiled = compiled));
      } else if mod_id.ends_with(".brew") || mod_id.ends_with(".qrew") {
        module_wrappers.push_str(compiled.as_str());
        
        let entry_regex = Regex::new(r#"//\s*entry\s*"([^"]+)""#).unwrap();
        for cap in entry_regex.captures_iter(&compiled) {
          let entry_file = cap[1].to_string();
          entry_calls.push(format!("rew.prototype.mod.prototype.get('{}');", entry_file));
        }
      } else {
        module_wrappers.push_str(&format!(
          r#"rew.prototype.mod.prototype.defineNew("{id}", function(globalThis){{
with (globalThis) {{
  {compiled}
}}
return globalThis.module.exports;
}}, {mod_alias});"#,
          id = mod_id,
          mod_alias = mod_alias,
          compiled = compiled
        ));
      }
    }

    // Add the entry point from the function parameter if provided
    if let Some(entry) = entry {
      let entry_mod_id = entry.to_str().unwrap_or("entry");
      let mut entry_app_id = None;

      if let Some(app_info) = crate::utils::find_app_info(entry) {
        if let Some(manifest) = &app_info.config.manifest {
          if let Some(package) = &manifest.package {
            if let Some(rel_path) = entry.strip_prefix(&app_info.path).ok() {
              let rel_path_str = rel_path.to_str().unwrap_or("");

              if let Some(entries) = &app_info.config.entries {
                for (key, value) in entries {
                  if value == rel_path_str {
                    entry_app_id = Some(format!("app://{}/{}", package, key));
                    break;
                  }
                }
              }
            }
          }
        }
      }

      let final_entry_id = entry_app_id.unwrap_or_else(|| entry_mod_id.to_string());
      if !final_entry_id.ends_with(".brew") && !final_entry_id.ends_with(".qrew") {
        entry_calls.push(format!("rew.prototype.mod.prototype.get('{}');", final_entry_id));
      }
    }

    for entry_call in entry_calls {
      module_wrappers.push_str(&format!("\n{}", entry_call));
    }

    fs::write("out.js", module_wrappers.clone())?;

    Ok(module_wrappers.to_string())
  }


  pub async fn build_file<P: AsRef<Path>>(&mut self, filepath: P, options: BuildOptions) -> Result<String> {
    let filepath = filepath
      .as_ref()
      .canonicalize()
      .with_context(|| format!("Failed to resolve file path: {:?}", filepath.as_ref()))?;

    let files_with_flags = RewRuntime::resolve_includes_recursive_from(&filepath)?;
    let mut excluded: Vec<String> = Vec::new();

    let files_with_flags = if !options.bundle_all {
      let main_app_path = crate::utils::find_app_path(&filepath);
      
      files_with_flags
        .into_iter()
        .filter(|(path, source, _)| {
          if let Some(app_path) = &main_app_path {
            if !path.starts_with(app_path) || path.to_str().unwrap_or("").starts_with("#") {
              if path.to_str().unwrap_or("").starts_with("#") {
                excluded.push(path.to_str().unwrap_or("").to_string());
              } else {
                if let Some(app_info) = crate::utils::find_app_info(&path) {
                  if let Some(manifest) = &app_info.config.manifest {
                    if let Some(package) = &manifest.package {
                      if let Some(rel_path) = path.strip_prefix(&app_info.path).ok() {
                        let rel_path_str = rel_path.to_str().unwrap_or("");
          
                        if let Some(entries) = &app_info.config.entries {
                          for (key, value) in entries {
                            if value == rel_path_str {
                              let entry_id = format!("{}/{}", package, key);

                              if !excluded.contains(&entry_id) {
                                excluded.push(entry_id.clone());
                              }
                              break;
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
              self.declaration_engine.process_script(source);
            }
            path.starts_with(app_path)
          } else {
            true
          }
        })
        .collect()
    } else {
      files_with_flags
    };

    let files: Vec<(PathBuf, String)> = files_with_flags
      .into_iter()
      .map(|(path, content, _)| (path, content))
      .collect();

    let entry = if let Some(entry) = options.entry_file {
      entry.canonicalize().unwrap_or(filepath)
    } else {
      filepath
    };

    let mut string = self.prepare(files, None).await?;
    string.insert_str(0, 
      format!("{}\n", excluded
      .into_iter()
      .map(|item| format!("// external \"{}\"", item))
      .collect::<Vec<String>>()
      .join("\n")
      ).as_str()
    );
    string.insert_str(0, format!("\n// entry \"{}\" \n", entry.to_str().unwrap_or("unknown")).as_str());
    string.push_str("\n");

    string = encode_brew_file(&string);

    Ok(string)
  }

  pub async fn include_and_run(
    &mut self,
    files: Vec<(PathBuf, String)>,
    entry: &Path,
  ) -> Result<()> {
    let final_script = self.prepare(files, Some(entry)).await?;
    

    self.runtime.execute_script("<main>", final_script)?;
    self
      .runtime
      .run_event_loop(PollEventLoopOptions::default())
      .await?;
    Ok(())
  }

  pub async fn compile_and_run(&mut self, source: &str, filepath: &Path) -> Result<String> {

    if filepath.extension().map_or(false, |ext| ext == "brew" || ext == "js" || ext == "qrew") || source.starts_with("\"no-compile\"") {
      // self.declaration_engine.process_script(source);
      return Ok(source.to_string());
    }

    let local_declarations = self.declaration_engine.process_script(source);

    let global_declarations = self.declaration_engine.global_declarations.clone();

    let processed = self.preprocess_rew(source, local_declarations, global_declarations)?;

    let code = format!(
      r#"
    globalThis.result = compile(`{}`, {{
     parseOptions: {{
     coffeeCompat: true,
     }},
    filename: '{}'
    }});
    globalThis.result
    "#,
      processed.code.replace("`", "\\`"),
      filepath.to_str().unwrap_or("unknown")
    );

    let result = self
      .compiler_runtime
      .execute_script("<rew>", code.clone())?;
    let compiled = self.compiler_runtime.resolve_value(result).await?;
    let scope = &mut self.compiler_runtime.handle_scope();
    let result_code = compiled.open(scope).to_rust_string_lossy(scope);

    Ok(result_code)
  }

  fn preprocess_rew(
    &mut self,
    source: &str,
    local_declarations: HashMap<String, Declaration>,
    global_declarations: HashMap<String, Declaration>,
  ) -> Result<CompilerResults> {
    let mut options = CompilerOptions {
      keep_imports: false,
      disable_use: false,
      jsx: false,
      jsx_pragma: None,
      cls: false,
      included: false,
      filename: None,
      compiler_type: "coffee".to_string(),
      local_declarations,
      global_declarations,
    };

    compile_rew_stuff(source, &mut options)
  }


  pub async fn run_file<P: AsRef<Path>>(&mut self, filepath: P) -> Result<()> {
    let filepath = filepath
      .as_ref()
      .canonicalize()
      .with_context(|| format!("Failed to resolve file path: {:?}", filepath.as_ref()))?;

    let files_with_flags = RewRuntime::resolve_includes_recursive_from(&filepath)?;

    for (path, content, preprocess) in &files_with_flags {
      if *preprocess {

        let local_declarations = self.declaration_engine.process_script(&content);

        for (name, decl) in local_declarations {
          self
            .declaration_engine
            .global_declarations
            .insert(name, decl);
        }
      }
    }

    let files: Vec<(PathBuf, String)> = files_with_flags
      .into_iter()
      .map(|(path, content, _)| (path, content))
      .collect();

    self.include_and_run(files, &filepath).await?;

    Ok(())
  }
}

impl Drop for RewRuntime {
  fn drop(&mut self) {}
}

#[op2(async)]
#[serde]
async fn op_fs_read(
  #[string] current_file: String,
  #[string] filepath: String,
  #[serde] options: Option<ReadOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));
  let full_path = base_dir.join(filepath);

  // println!("{}", current_file);

  let options = options.unwrap_or_default();

  if options.binary {
    let mut file = File::open(&full_path).map_err(CoreError::Io)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(CoreError::Io)?;

    Ok(serde_json::Value::Array(
      buffer
        .into_iter()
        .map(|b| serde_json::Value::Number(b.into()))
        .collect(),
    ))
  } else {
    let content = fs::read_to_string(&full_path).map_err(CoreError::Io)?;
    Ok(serde_json::Value::String(content))
  }
}

#[derive(Deserialize, Default)]
struct ReadOptions {
  binary: bool,
}

#[op2(async)]
async fn op_fs_write(
  #[string] current_file: String,
  #[string] filepath: String,
  #[serde] content: serde_json::Value,
  #[serde] options: Option<WriteOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  let options = options.unwrap_or_default();

  if let Some(parent) = full_path.parent() {
    if options.create_dirs {
      fs::create_dir_all(parent).map_err(CoreError::Io)?;
    }
  }

  if options.binary {
    if let serde_json::Value::Array(bytes) = content {
      let buffer: Result<Vec<u8>, _> = bytes
        .iter()
        .map(|v| {
          if let serde_json::Value::Number(n) = v {
            n.as_u64().map(|n| n as u8).ok_or_else(|| {
              CoreError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid byte value",
              ))
            })
          } else {
            Err(CoreError::Io(io::Error::new(
              io::ErrorKind::InvalidData,
              "Expected number in byte array",
            )))
          }
        })
        .collect();

      fs::write(&full_path, buffer?).map_err(CoreError::Io)?;
    } else {
      return Err(CoreError::Io(io::Error::new(
        io::ErrorKind::InvalidData,
        "Expected array of bytes for binary write",
      )));
    }
  } else {
    if let serde_json::Value::String(text) = content {
      let mut file = File::create(&full_path).map_err(CoreError::Io)?;
      file.write_all(text.as_bytes()).map_err(CoreError::Io)?;
    } else {
      return Err(CoreError::Io(io::Error::new(
        io::ErrorKind::InvalidData,
        "Expected string for text write",
      )));
    }
  }

  Ok(())
}

#[derive(Deserialize, Default)]
struct WriteOptions {
  binary: bool,
  create_dirs: bool,
}

#[op2(fast)]
fn op_fs_exists(
  #[string] current_file: String,
  #[string] filepath: String,
  _: Rc<RefCell<OpState>>,
) -> Result<bool, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  Ok(full_path.exists())
}

#[op2(async)]
async fn op_fs_rm(
  #[string] current_file: String,
  #[string] filepath: String,
  #[serde] options: Option<RemoveOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  let options = options.unwrap_or_default();

  if full_path.is_dir() {
    if options.recursive {
      fs::remove_dir_all(&full_path).map_err(CoreError::Io)?;
    } else {
      fs::remove_dir(&full_path).map_err(CoreError::Io)?;
    }
  } else {
    fs::remove_file(&full_path).map_err(CoreError::Io)?;
  }

  Ok(())
}

#[derive(Deserialize, Default)]
struct RemoveOptions {
  recursive: bool,
}

#[op2(async)]
async fn op_fs_mkdir(
  #[string] current_file: String,
  #[string] dirpath: String,
  #[serde] options: Option<MkdirOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(dirpath);

  let options = options.unwrap_or_default();

  if options.recursive {
    fs::create_dir_all(&full_path).map_err(CoreError::Io)?;
  } else {
    fs::create_dir(&full_path).map_err(CoreError::Io)?;
  }

  Ok(())
}

#[derive(Deserialize, Default)]
struct MkdirOptions {
  recursive: bool,
}

#[op2]
#[string]
fn op_fs_readdir(
  #[string] current_file: String,
  #[string] dirpath: String,
  #[serde] options: Option<ReaddirOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(dirpath);

  let options = options.unwrap_or_default();

  let entries = fs::read_dir(&full_path).map_err(CoreError::Io)?;

  let mut result = Vec::new();

  for entry in entries {
    let entry = entry.map_err(CoreError::Io)?;
    let file_type = entry.file_type().map_err(CoreError::Io)?;

    if !options.include_hidden {
      if let Some(file_name) = entry.path().file_name() {
        if let Some(name_str) = file_name.to_str() {
          if name_str.starts_with(".") {
            continue;
          }
        }
      }
    }

    if let Some(filter_type) = &options.filter_type {
      match filter_type.as_str() {
        "file" => {
          if !file_type.is_file() {
            continue;
          }
        }
        "directory" => {
          if !file_type.is_dir() {
            continue;
          }
        }
        "symlink" => {
          if !file_type.is_symlink() {
            continue;
          }
        }
        _ => {}
      }
    }

    let metadata = entry.metadata().map_err(CoreError::Io)?;

    let entry_info = DirEntryInfo {
      name: entry.file_name().to_string_lossy().to_string(),
      path: entry.path().to_string_lossy().to_string(),
      is_file: file_type.is_file(),
      is_directory: file_type.is_dir(),
      is_symlink: file_type.is_symlink(),
      size: metadata.len(),
      modified: metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs()),
      created: metadata
        .created()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs()),
    };

    result.push(entry_info);
  }

  if let Some(sort_by) = &options.sort_by {
    match sort_by.as_str() {
      "name" => result.sort_by(|a, b| a.name.cmp(&b.name)),
      "size" => result.sort_by(|a, b| a.size.cmp(&b.size)),
      "modified" => result.sort_by(|a, b| a.modified.cmp(&b.modified)),
      "type" => result.sort_by(|a, b| a.is_directory.cmp(&b.is_directory).reverse()),
      _ => {}
    }
  }

  serde_json::to_string(&result).map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[derive(Deserialize, Default)]
struct ReaddirOptions {
  include_hidden: bool,
  filter_type: Option<String>,
  sort_by: Option<String>,
}

#[derive(Serialize)]
struct DirEntryInfo {
  name: String,
  path: String,
  is_file: bool,
  is_directory: bool,
  is_symlink: bool,
  size: u64,
  modified: Option<u64>,
  created: Option<u64>,
}

#[op2]
#[string]
fn op_fs_stats(
  #[string] current_file: String,
  #[string] filepath: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  let metadata = fs::metadata(&full_path).map_err(CoreError::Io)?;

  let stats = serde_json::json!({
      "isFile": metadata.is_file(),
      "isDirectory": metadata.is_dir(),
      "isSymlink": metadata.file_type().is_symlink(),
      "size": metadata.len(),
      "modified": metadata.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()),
      "created": metadata.created().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()),
      "accessed": metadata.accessed().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()),
      "permissions": {
          "readonly": metadata.permissions().readonly(),
          // "mode": metadata.permissions().mode(),
      }
  });

  Ok(stats.to_string())
}

#[op2(async)]
async fn op_fs_copy(
  #[string] current_file: String,
  #[string] src: String,
  #[string] dest: String,
  #[serde] options: Option<CopyOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let src_path = base_dir.join(src);
  let dest_path = base_dir.join(dest);

  let options = options.unwrap_or_default();

  if src_path.is_dir() {
    if options.recursive {
      copy_dir_recursive(&src_path, &dest_path, &options).map_err(CoreError::Io)?;
    } else {
      return Err(CoreError::Io(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Source is a directory, but recursive option is not set",
      )));
    }
  } else {
    if let Some(parent) = dest_path.parent() {
      if options.create_dirs {
        fs::create_dir_all(parent).map_err(CoreError::Io)?;
      }
    }

    fs::copy(&src_path, &dest_path).map_err(CoreError::Io)?;
  }

  Ok(())
}

#[derive(Deserialize, Default)]
struct CopyOptions {
  recursive: bool,
  create_dirs: bool,
  overwrite: bool,
}

fn copy_dir_recursive(src: &Path, dest: &Path, options: &CopyOptions) -> io::Result<()> {
  if !dest.exists() {
    fs::create_dir_all(dest)?;
  }

  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let src_path = entry.path();
    let dest_path = dest.join(entry.file_name());

    if src_path.is_dir() {
      copy_dir_recursive(&src_path, &dest_path, options)?;
    } else {
      if dest_path.exists() && !options.overwrite {
        continue;
      }
      fs::copy(&src_path, &dest_path)?;
    }
  }

  Ok(())
}

#[op2(async)]
async fn op_fs_rename(
  #[string] current_file: String,
  #[string] src: String,
  #[string] dest: String,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let src_path = base_dir.join(src);
  let dest_path = base_dir.join(dest);

  fs::rename(&src_path, &dest_path).map_err(CoreError::Io)?;

  Ok(())
}

#[op2]
#[string]
fn op_fs_cwd(state: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  let state = state.borrow();
  let runtime_state = state.borrow::<RuntimeState>();

  Ok(runtime_state.current_dir.to_string_lossy().to_string())
}

// Base64 encoding/decoding operations
#[op2]
#[string]
fn op_to_base64(#[serde] data: serde_json::Value) -> Result<String, CoreError> {
  match data {
    serde_json::Value::String(text) => Ok(BASE64.encode(text.as_bytes())),
    serde_json::Value::Array(bytes) => {
      let buffer: Result<Vec<u8>, _> = bytes
        .iter()
        .map(|v| {
          if let serde_json::Value::Number(n) = v {
            n.as_u64().map(|n| n as u8).ok_or_else(|| {
              CoreError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid byte value",
              ))
            })
          } else {
            Err(CoreError::Io(io::Error::new(
              io::ErrorKind::InvalidData,
              "Expected number in byte array",
            )))
          }
        })
        .collect();

      match buffer {
        Ok(bytes) => Ok(BASE64.encode(bytes)),
        Err(e) => Err(e),
      }
    }
    _ => Err(CoreError::Io(io::Error::new(
      io::ErrorKind::InvalidData,
      "Expected string or array of bytes for base64 encoding",
    ))),
  }
}

#[op2]
#[serde]
fn op_from_base64(
  #[string] encoded: String,
  #[serde] options: Option<Base64DecodeOptions>,
) -> Result<serde_json::Value, CoreError> {
  let options = options.unwrap_or_default();

  let decoded = BASE64
    .decode(encoded.as_bytes())
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  if options.as_string {
    let text = String::from_utf8(decoded)
      .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;
    Ok(serde_json::Value::String(text))
  } else {
    Ok(serde_json::Value::Array(
      decoded
        .into_iter()
        .map(|b| serde_json::Value::Number(b.into()))
        .collect(),
    ))
  }
}

#[derive(Deserialize, Default)]
struct Base64DecodeOptions {
  as_string: bool,
}

#[op2]
#[string]
fn op_find_app(#[string] filepath: String, _: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  let current_file = Path::new(&filepath);

  let app_path = find_app_path(current_file);

  Ok(String::from(
    app_path.unwrap_or(PathBuf::from("")).to_str().unwrap(),
  ))
}

#[op2]
#[string]
fn op_yaml_to_string(
  #[serde] data: serde_json::Value,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let yaml = serde_yaml::to_string(&data)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  Ok(yaml)
}

#[op2]
#[serde]
fn op_string_to_yaml(
  #[string] yaml_str: String,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let value: serde_json::Value = serde_yaml::from_str(&yaml_str)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  Ok(value)
}

#[op2]
#[serde]
fn op_app_loadconfig(
  #[string] app_path: String,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let app_path = Path::new(&app_path);

  if !app_path.exists() {
    return Err(CoreError::Io(io::Error::new(
      io::ErrorKind::NotFound,
      format!("App path not found: {}", app_path.display()),
    )));
  }

  let config_path = app_path.join("app.yaml");

  if !config_path.exists() {
    return Err(CoreError::Io(io::Error::new(
      io::ErrorKind::NotFound,
      format!("App config not found: {}", config_path.display()),
    )));
  }

  let config_str = fs::read_to_string(&config_path)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;

  let config: serde_json::Value = serde_yaml::from_str(&config_str)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  Ok(config)
}

// Helper function to get DataManager for a specific app package
fn get_data_manager_for_package(app_package: &str) -> Result<DataManager, CoreError> {
  // For now, use "default" as the user ID
  // In a real implementation, you'd get this from user authentication
  let user_id = "default";

  DataManager::new(user_id, app_package)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[string]
fn op_data_read(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .read(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(async)]
async fn op_data_write(
  #[string] app_package: String,
  #[string] key: String,
  #[string] content: String,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .write(&key, &content)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(async)]
async fn op_data_delete(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .delete(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(fast)]
fn op_data_exists(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<bool, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  Ok(data_manager.exists(&key))
}

#[op2]
#[string]
fn op_data_list(
  #[string] app_package: String,
  #[string] prefix: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  let files = data_manager
    .list(&prefix)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;

  serde_json::to_string(&files).map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[serde]
fn op_data_read_binary(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<Vec<u8>, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .read_binary(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}


#[op2]
#[string]
fn op_get_env(
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let env_vars: HashMap<String, String> = std::env::vars().collect();
  let cwd = std::env::current_dir()
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?
    .to_string_lossy()
    .to_string();
  let exec_path = std::env::current_exe()
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?
    .to_string_lossy()
    .to_string();

  let result = serde_json::json!({
    "env": env_vars,
    "cwd": cwd,
    "execPath": exec_path
  });

  serde_json::to_string(&result)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))
}

#[op2(async)]
async fn op_data_write_binary(
  #[string] app_package: String,
  #[string] key: String,
  #[serde] data: Vec<u8>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .write_binary(&key, &data)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[serde]
fn op_data_read_yaml(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .read_yaml(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(async)]
async fn op_data_write_yaml(
  #[string] app_package: String,
  #[string] key: String,
  #[serde] data: serde_json::Value,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .write_yaml(&key, &data)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[serde]
fn op_data_get_info(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<(bool, String), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  let (exists, format) = data_manager
    .get_file_info(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;

  let format_str = match format {
    DataFormat::Text => "text",
    DataFormat::Json => "json",
    DataFormat::Yaml => "yaml",
    DataFormat::Binary => "binary",
  };

  Ok((exists, format_str.to_string()))
}
