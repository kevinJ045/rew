use super::civet::get_civet_script;
use super::compiler::{compile_rew_stuff, CompilerOptions};
use crate::builtins::BUILTIN_MODULES;
use crate::compiler::CompilerResults;
use crate::declarations::{Declaration, DeclarationEngine};
use crate::ext::{console, ffi, url, web, webidl};
use crate::runtime_script::get_runtime_script;
use crate::utils::find_app_path;
use crate::data_manager::{DataManager, DataFormat};
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use deno_core::error::CoreError;
use deno_core::{extension, op2, JsRuntime, RuntimeOptions};
use deno_core::{OpState};
use deno_core::{PollEventLoopOptions};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use serde_yaml;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Default)]
struct RuntimeState {
  current_dir: PathBuf,
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

    let mut visited = HashSet::new();
    let mut result = Vec::new();

    fn visit_file(
      file_path: &Path,
      visited: &mut HashSet<PathBuf>,
      result: &mut Vec<(PathBuf, String, bool)>,
      import_re: &Regex,
    ) -> Result<()> {
      if visited.contains(file_path) {
        return Ok(());
      }


      let mut should_preprocess = false;
      let file_path_unf = file_path.to_str().unwrap_or("");
      let file_path_str = if file_path_unf.ends_with('!') {
        should_preprocess = true;
        &file_path_unf[0..file_path_unf.len()-1]
      } else {
        file_path_unf
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
      } else {
        fs::read_to_string(file_path).with_context(|| format!("Failed to read {:?}", file_path))?
      };

      visited.insert(PathBuf::from(file_path_str));
      
      
      result.push((PathBuf::from(file_path_str), content.clone(), should_preprocess));

      let parent = file_path.parent().unwrap_or(Path::new("."));

      for cap in import_re.captures_iter(&content) {
        let relative_path = cap[1].to_string();
        
        if relative_path.starts_with("#") {
          
          let builtin_path = PathBuf::from(&relative_path);
          visit_file(&builtin_path, visited, result, import_re)?;
        } else if !relative_path.contains("/") && !relative_path.contains("\\") && !relative_path.starts_with(".") {
          
          if let Some(app_entry) = crate::utils::resolve_app_entry(&relative_path, None) {
            visit_file(&app_entry, visited, result, import_re)?;
          } else {
            return Err(anyhow::anyhow!(
              "App not found: {}",
              relative_path
            ));
          }
        } else if relative_path.contains("/") && !relative_path.starts_with(".") {
          
          let parts: Vec<&str> = relative_path.splitn(2, "/").collect();
          if parts.len() == 2 {
            let package_name = parts[0];
            let entry_name = parts[1];
            
            if let Some(app_entry) = crate::utils::resolve_app_entry(package_name, Some(entry_name)) {
              visit_file(&app_entry, visited, result, import_re)?;
            } else {
              return Err(anyhow::anyhow!(
                "App entry not found: {}/{}",
                package_name, entry_name
              ));
            }
          } else {
            
            let included_path = parent
              .join(relative_path)
              .canonicalize()
              .with_context(|| format!("Failed to resolve import"))?;

            visit_file(&included_path, visited, result, import_re)?;
          }
        } else {
          
          let included_path = parent
            .join(relative_path)
            .canonicalize()
            .with_context(|| format!("Failed to resolve import"))?;

          visit_file(&included_path, visited, result, import_re)?;
        }
      }

      Ok(())
    }

    visit_file(&filepath, &mut visited, &mut result, &import_re)?;
    Ok(result)
  }

  pub async fn include_and_run(
    &mut self,
    files: Vec<(PathBuf, String)>,
    entry: &Path,
  ) -> Result<()> {
    let mut module_wrappers = String::new();

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
            }
          }
        }
      }

      if mod_id.starts_with('#') {
        module_wrappers.push_str(&format!(
          r#"{compiled}"#,
          compiled = compiled
        ));
      } else {
        module_wrappers.push_str(&format!(
          r#"rew.prototype.mod.prototype.defineNew("{id}", function(context){{
            with (context) {{
              {compiled}
            }}
            return context.module.exports;
          }}, {mod_alias});"#,
          id = mod_id,
          mod_alias = mod_alias,
          compiled = compiled
        ));
      }
    }

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

    let final_script = format!(
      "{}\nrew.prototype.mod.prototype.get('{}');",
      module_wrappers, final_entry_id
    );

    fs::write("out.js", final_script.clone())?;

    self.runtime.execute_script("<main>", final_script)?;
    self
      .runtime
      .run_event_loop(PollEventLoopOptions::default())
      .await?;
    Ok(())
  }

  pub async fn compile_and_run(&mut self, source: &str, filepath: &Path) -> Result<String> {
    
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

  fn preprocess_rew(&mut self, source: &str, local_declarations: HashMap<String, Declaration>, global_declarations: HashMap<String, Declaration>) -> Result<CompilerResults> {
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
        // println!("Preprocessing declarations from: {}", path.display());
        
        let local_declarations = self.declaration_engine.process_script(&content);
      
        
        for (name, decl) in local_declarations {
          self.declaration_engine.global_declarations.insert(name, decl);
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
    serde_json::Value::String(text) => {
      
      Ok(BASE64.encode(text.as_bytes()))
    }
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
fn op_find_app(
  #[string] filepath: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
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
    data_manager.read(&key)
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
    data_manager.write(&key, &content)
        .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(async)]
async fn op_data_delete(
    #[string] app_package: String,
    #[string] key: String,
    _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
    let data_manager = get_data_manager_for_package(&app_package)?;
    data_manager.delete(&key)
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
    let files = data_manager.list(&prefix)
        .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;
    
    serde_json::to_string(&files)
        .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[serde]
fn op_data_read_binary(
    #[string] app_package: String,
    #[string] key: String,
    _: Rc<RefCell<OpState>>,
) -> Result<Vec<u8>, CoreError> {
    let data_manager = get_data_manager_for_package(&app_package)?;
    data_manager.read_binary(&key)
        .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(async)]
async fn op_data_write_binary(
    #[string] app_package: String,
    #[string] key: String,
    #[serde] data: Vec<u8>,
    _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
    let data_manager = get_data_manager_for_package(&app_package)?;
    data_manager.write_binary(&key, &data)
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
    data_manager.read_yaml(&key)
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
    data_manager.write_yaml(&key, &data)
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
    let (exists, format) = data_manager.get_file_info(&key)
        .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;
    
    // Convert format to string
    let format_str = match format {
        DataFormat::Text => "text",
        DataFormat::Json => "json",
        DataFormat::Yaml => "yaml",
        DataFormat::Binary => "binary",
    };
    
    Ok((exists, format_str.to_string()))
}

