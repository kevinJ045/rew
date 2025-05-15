use super::civet::get_civet_script;
use super::compiler::{compile_rew_stuff, CompilerOptions};
use crate::compiler::CompilerResults;
use crate::runtime_script::get_runtime_script;
use crate::utils::find_app_path;
use crate::ext::{url, web, webidl, console, ffi};
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use deno_core::error::CoreError;
use deno_core::{resolve_path, OpState};
use deno_core::{extension, op2, Extension, JsRuntime, RuntimeOptions};
use deno_core::{v8, PollEventLoopOptions};
use deno_ffi::deno_ffi;
use deno_permissions::PermissionsContainer;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{self, DirEntry, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Default)]
struct RuntimeState {
  current_dir: PathBuf,
}

pub struct RewRuntime {
  runtime: JsRuntime,
  compiler_runtime: JsRuntime,
  current_dir: PathBuf,
}

#[op2(async)]
#[string]
async fn op_inc(
  #[string] _source_file: String,
  #[string] _url: String,
  state: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  Err(CoreError::Io(std::io::Error::new(
    std::io::ErrorKind::Other,
    "Import not used in inclusion mode",
  )))
}

extension!(
  rewextension,
  ops = [
    op_inc,
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
    op_from_base64
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

    let blob_store = Arc::new(deno_web::BlobStore::default());
    let location = None;
    
    let console_ext = deno_console::deno_console::init_ops_and_esm();
    let url_ext = deno_url::deno_url::init_ops_and_esm();
    let web_ext = deno_web::deno_web::init_ops::<PermissionsContainer>(blob_store, location);
    let ffi_ext = deno_ffi::init_ops::<PermissionsContainer>();

    // let main_module = resolve_path("./lib/rew/main.js", Path::new(".")).unwrap();

    println!("Bar0");

    let mut extensions = vec![
      rewextension::init_ops(),
    ];

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

    println!("Bar");

    let current_dir = std::env::current_dir()?;
    println!("foo");

    let state = RuntimeState {
      current_dir: current_dir.clone(),
    };
    println!("foo2");
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
    println!("foo3");

    Ok(Self {
      compiler_runtime,
      runtime,
      current_dir,
    })
  }

  pub fn resolve_includes_recursive_from<P: AsRef<Path>>(
    filepath: P,
  ) -> Result<Vec<(PathBuf, String)>> {
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
      result: &mut Vec<(PathBuf, String)>,
      import_re: &Regex,
    ) -> Result<()> {
      if visited.contains(file_path) {
        return Ok(());
      }

      let content =
        fs::read_to_string(file_path).with_context(|| format!("Failed to read {:?}", file_path))?;
      visited.insert(file_path.to_path_buf());
      result.push((file_path.to_path_buf(), content.clone()));

      let parent = file_path.parent().unwrap_or(Path::new("."));

      for cap in import_re.captures_iter(&content) {
        let relative_path = cap[1].to_string();
        let included_path = parent
          .join(relative_path)
          .canonicalize()
          .with_context(|| format!("Failed to resolve import"))?;

        visit_file(&included_path, visited, result, import_re)?;
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
      // .replace("\\", "_")
      // .replace("/", "_");

      module_wrappers.push_str(&format!(
        r#"rew.prototype.mod.prototype.defineNew("{id}", function(context, options){{
					with (context) {{
						{compiled}
					}}
					return context.module.exports;
				}});"#,
        id = mod_id,
        compiled = compiled
      ));
    }

    let entry_mod_id = entry.to_str().unwrap_or("entry");

    // println!("From: {}", module_wrappers);

    let final_script = format!(
      "{}\nrew.prototype.mod.prototype.get('{}');",
      module_wrappers, entry_mod_id
    );

    // println!("{}", final_script);

    self.runtime.execute_script("<main>", final_script)?;
    self
      .runtime
      .run_event_loop(PollEventLoopOptions::default())
      .await?;
    Ok(())
  }

  pub async fn compile_and_run(&mut self, source: &str, filepath: &Path) -> Result<String> {
    let processed = self.preprocess_rew(source)?;

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

  fn preprocess_rew(&mut self, source: &str) -> Result<CompilerResults> {
    let mut options = CompilerOptions::default();
    compile_rew_stuff(source, &mut options)
  }

  pub async fn run_file<P: AsRef<Path>>(&mut self, filepath: P) -> Result<()> {
    let filepath = filepath
      .as_ref()
      .canonicalize()
      .with_context(|| format!("Failed to resolve file path: {:?}", filepath.as_ref()))?;

    // Resolve all imports recursively - use associated function syntax
    let files = RewRuntime::resolve_includes_recursive_from(&filepath)?;

    // Execute the resolved files with the entry point
    self.include_and_run(files, &filepath).await?;

    Ok(())
  }
}

impl Drop for RewRuntime {
  fn drop(&mut self) {}
}

// File system operations

// Read file with options
#[op2(async)]
#[serde]
async fn op_fs_read(
  #[string] current_file: String,
  #[string] filepath: String,
  #[serde] options: Option<ReadOptions>,
  state: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));
  let full_path = base_dir.join(filepath);

  let options = options.unwrap_or_default();

  if options.binary {
    // For binary reads, return an array of bytes
    let mut file = File::open(&full_path).map_err(CoreError::Io)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(CoreError::Io)?;

    // Return as a byte array that JavaScript can handle
    Ok(serde_json::Value::Array(
      buffer
        .into_iter()
        .map(|b| serde_json::Value::Number(b.into()))
        .collect(),
    ))
  } else {
    // For text reads, return a string
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
  state: Rc<RefCell<OpState>>,
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
    // For binary writes, expect an array of bytes
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
    // For text writes, expect a string
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
  state: Rc<RefCell<OpState>>,
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
  state: Rc<RefCell<OpState>>,
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
  state: Rc<RefCell<OpState>>,
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
  state: Rc<RefCell<OpState>>,
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

// Get file stats
#[op2]
#[string]
fn op_fs_stats(
  #[string] current_file: String,
  #[string] filepath: String,
  state: Rc<RefCell<OpState>>,
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
  state: Rc<RefCell<OpState>>,
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
  state: Rc<RefCell<OpState>>,
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
      // Encode string to base64
      Ok(BASE64.encode(text.as_bytes()))
    }
    serde_json::Value::Array(bytes) => {
      // Convert array of numbers to bytes and encode to base64
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
    // Convert decoded bytes to string
    let text = String::from_utf8(decoded)
      .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;
    Ok(serde_json::Value::String(text))
  } else {
    // Return as array of bytes
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
  state: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let current_file = Path::new(&filepath);
  let dir_path = current_file.parent().unwrap_or(Path::new("/"));

  let app_path = find_app_path(dir_path);

  Ok(String::from(app_path.unwrap_or(PathBuf::from("")).to_str().unwrap()))
}