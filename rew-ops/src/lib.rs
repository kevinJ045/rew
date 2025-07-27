use rew_data_manager::{DataFormat, DataManager};
use rew_core::utils::find_app_path;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use deno_core::OpState;
use deno_core::error::CoreError;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use deno_core::{op2};
use rew_core::{RuntimeState};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use rew_vfile::{VIRTUAL_FILES, add_virtual_file};

#[op2]
#[serde]
pub fn op_get_args(state: Rc<RefCell<OpState>>) -> Result<serde_json::Value, CoreError> {
  let state = state.borrow();
  let runtime_args = state.borrow::<RuntimeState>();
  Ok(serde_json::json!(runtime_args.args.clone()))
}


// Base64 encoding/decoding operations
#[op2]
#[string]
pub fn op_to_base64(#[serde] data: serde_json::Value) -> Result<String, CoreError> {
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
pub fn op_from_base64(
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
pub fn op_find_app(#[string] filepath: String, _: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  let current_file = Path::new(&filepath);

  let app_path = find_app_path(current_file);

  Ok(String::from(
    app_path.unwrap_or(PathBuf::from("")).to_str().unwrap(),
  ))
}

#[op2]
#[string]
pub fn op_yaml_to_string(
  #[serde] data: serde_json::Value,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let yaml = serde_yaml::to_string(&data)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  Ok(yaml)
}

#[op2]
#[serde]
pub fn op_string_to_yaml(
  #[string] yaml_str: String,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let value: serde_json::Value = serde_yaml::from_str(&yaml_str)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  Ok(value)
}

#[op2]
#[serde]
pub fn op_app_loadconfig(
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
pub fn get_data_manager_for_package(app_package: &str) -> Result<DataManager, CoreError> {
  // For now, use "default" as the user ID
  // In a real implementation, you'd get this from user authentication
  let user_id = "default";

  DataManager::new(user_id, app_package)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[string]
pub fn op_data_read(
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
pub async fn  op_data_write(
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
pub async fn  op_data_delete(
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
pub fn op_data_exists(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<bool, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  Ok(data_manager.exists(&key))
}

#[op2]
#[string]
pub fn op_data_list(
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
pub fn op_data_read_binary(
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
pub fn op_fetch_env(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  let env_vars: HashMap<String, String> = std::env::vars().collect();
  let cwd = std::env::current_dir()?
    // .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;
    .to_string_lossy()
    .to_string();
  let exec_path = std::env::current_exe()?
    // .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;
    .to_string_lossy()
    .to_string();

  let result = serde_json::json!({
    "env": env_vars,
    "cwd": cwd,
    "execPath": exec_path,
    "tempDir": std::env::temp_dir(),
    "rewPath": rew_core::utils::get_rew_root()
  });

  serde_json::to_string(&result)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))
}

#[op2(async)]
pub async fn  op_data_write_binary(
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
pub fn op_data_read_yaml(
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
pub async fn  op_data_write_yaml(
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
pub fn op_data_get_info(
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

#[op2]
#[string]
pub fn op_data_get_path(
  #[string] app_package: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;

  Ok(data_manager.get_path("").to_string_lossy().to_string())
}

#[op2]
#[string]
pub fn op_os_info_os(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  Ok(std::env::consts::OS.to_string())
}

#[op2]
#[string]
pub fn op_os_info_arch(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  Ok(std::env::consts::ARCH.to_string())
}

#[op2]
#[string]
pub fn op_os_info_family(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  Ok(std::env::consts::FAMILY.to_string())
}

use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng, distributions::Alphanumeric};
use std::hash::Hash;
use std::hash::Hasher;

#[op2]
#[serde]
pub fn op_rand_from(
  #[bigint] min: usize,
  #[bigint] max: usize,
  #[string] seed: Option<String>,
) -> usize {
  let mut rng: Box<dyn RngCore> = match seed {
    Some(s) => {
      let mut hasher = std::collections::hash_map::DefaultHasher::new();
      s.hash(&mut hasher);
      Box::new(StdRng::seed_from_u64(hasher.finish()))
    }
    _ => Box::new(rand::thread_rng()),
  };

  if min == max {
    return min;
  }

  let (low, high) = if min < max { (min, max) } else { (max, min) };

  rng.gen_range(low..=high)
}

#[op2]
#[string]
pub fn op_vfile_set(#[string] full_path: String, #[string] content: String) -> String {
  add_virtual_file(full_path.as_str(), content.as_str());
  "".to_string()
}

#[op2]
#[string]
pub fn op_vfile_get(#[string] full_path: String) -> String {
  if let Some(v) = VIRTUAL_FILES
    .lock()
    .unwrap()
    .iter()
    .find(|(p, _)| *p == full_path)
  {
    return v.1.clone();
  }
  "".to_string()
}

#[op2]
#[string]
pub fn op_gen_uid(length: i32, #[string] seed: Option<String>) -> String {
  if let Some(seed_str) = seed {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    seed_str.hash(&mut hasher);

    let seed = hasher.finish();
    let mut rng = StdRng::seed_from_u64(seed);

    (0..length)
      .map(|_| rng.sample(Alphanumeric) as char)
      .collect()
  } else {
    let mut rng = rand::thread_rng();

    (0..length)
      .map(|_| rng.sample(Alphanumeric) as char)
      .collect()
  }
}

#[op2]
#[serde]
pub fn op_terminal_size() -> Result<(u16, u16), std::io::Error> {
  #[cfg(unix)]
  {
    use libc::{STDOUT_FILENO, TIOCGWINSZ, ioctl, winsize};

    let mut ws: winsize = unsafe { std::mem::zeroed() };

    let result = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) };

    if result == -1 {
      return Err(std::io::Error::last_os_error());
    }

    Ok((ws.ws_col, ws.ws_row))
  }

  #[cfg(windows)]
  {
    use std::mem::zeroed;
    use std::ptr::null_mut;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFO, GetConsoleScreenBufferInfo};

    unsafe {
      let handle = GetStdHandle(STD_OUTPUT_HANDLE);
      if handle == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error());
      }

      let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = zeroed();
      if GetConsoleScreenBufferInfo(handle, &mut csbi) == 0 {
        return Err(std::io::Error::last_os_error());
      }

      let width = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16;
      let height = (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16;

      Ok((width, height))
    }
  }
}


#[op2]
#[serde]
pub fn op_fs_read(
  #[string] current_file: String,
  #[string] filepath: String,
  #[serde] options: Option<ReadOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));
  let full_path = base_dir.join(filepath);

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
pub async fn op_fs_write(
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
  } else if let serde_json::Value::String(text) = content {
    let mut file = File::create(&full_path).map_err(CoreError::Io)?;
    file.write_all(text.as_bytes()).map_err(CoreError::Io)?;
  } else {
    return Err(CoreError::Io(io::Error::new(
      io::ErrorKind::InvalidData,
      "Expected string for text write",
    )));
  }

  Ok(())
}

#[derive(Deserialize, Default)]
struct WriteOptions {
  binary: bool,
  create_dirs: bool,
}

use sha2::{Digest, Sha256};
#[op2]
#[string]
pub fn  op_fs_sha(
  #[string] current_file: String,
  #[string] filepath: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  let file_bytes = fs::read(&full_path)?;
  let mut hasher = Sha256::new();
  hasher.update(file_bytes);
  let hash = hasher.finalize();

  Ok(format!("{:x}", hash))
}

#[op2(fast)]
pub fn  op_fs_exists(
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
pub async fn op_fs_rm(
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
pub async fn op_fs_mkdir(
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
pub fn  op_fs_readdir(
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
pub fn  op_fs_stats(
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
pub async fn op_fs_copy(
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
pub struct CopyOptions {
  pub recursive: bool,
  pub create_dirs: bool,
  pub overwrite: bool,
}

pub fn copy_dir_recursive(src: &Path, dest: &Path, options: &CopyOptions) -> io::Result<()> {
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
pub async fn op_fs_rename(
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
pub fn  op_fs_cwdir(state: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  let state = state.borrow();
  let runtime_state = state.borrow::<RuntimeState>();

  Ok(runtime_state.current_dir.to_string_lossy().to_string())
}
