use deno_core::CoreError;
use deno_core::OpState;
use deno_core::op2;
use serde::Deserialize;
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::rc::Rc;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rew_data_manager::{DataFormat, DataManager};
use rew_core::utils::find_app_path;
use std::collections::HashMap;
use std::path::PathBuf;


#[op2]
#[serde]
fn op_get_args(state: Rc<RefCell<OpState>>) -> Result<serde_json::Value, CoreError> {
  let state = state.borrow();
  let runtime_args = state.borrow::<rew_runtime::RuntimeArgs>();
  Ok(serde_json::json!(runtime_args.0.clone()))
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
fn op_fetch_env(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
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
    "execPath": exec_path,
    "tempDir": std::env::temp_dir(),
    "rewPath": rew_core::utils::get_rew_root()
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

#[op2]
#[string]
fn op_data_get_path(
  #[string] app_package: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;

  Ok(data_manager.get_path("").to_string_lossy().to_string())
}

#[op2]
#[string]
fn op_os_info_os(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  Ok(std::env::consts::OS.to_string())
}

#[op2]
#[string]
fn op_os_info_arch(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  Ok(std::env::consts::ARCH.to_string())
}

#[op2]
#[string]
fn op_os_info_family(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  Ok(std::env::consts::FAMILY.to_string())
}

#[op2(async, reentrant)]
#[serde]
async fn op_dyn_imp(
  #[string] current_file: String,
  #[string] file: String,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let file_path = if current_file == "/" {
    Path::new(&file).to_path_buf()
  } else {
    let current_file_path = Path::new(&current_file);
    let base_dir = current_file_path.parent().unwrap_or(Path::new("."));
    base_dir.join(file)
  };

  let mut runtime = rew_runtime::RewRuntime::new(None, None)
    .map_err(|_| CoreError::Io(io::Error::new(io::ErrorKind::NotFound, "")))?;

  let files_with_flags = rew_runtime::RewRuntime::resolve_includes_recursive_from(file_path.clone())
    .map_err(|_| CoreError::Io(io::Error::new(io::ErrorKind::NotFound, "")))?;
  for (_, content, preprocess) in &files_with_flags {
    if *preprocess {
      let local_declarations = runtime.declaration_engine.process_script(content);

      for (name, decl) in local_declarations {
        runtime
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

  let prepared = runtime
    .prepare(files, None)
    .await
    .map_err(|_| CoreError::Io(io::Error::new(io::ErrorKind::NotFound, "")))?;

  let fp = fs::canonicalize(&file_path)
    .map_err(|_| CoreError::Io(io::Error::new(io::ErrorKind::NotFound, "")))?;

  Ok(serde_json::json!(vec![
    fp.to_string_lossy().to_string(),
    prepared
  ]))
}

use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng, distributions::Alphanumeric};
use std::hash::Hash;
use std::hash::Hasher;

#[op2]
#[serde]
fn op_rand_from(
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
fn op_vfile_set(#[string] full_path: String, #[string] content: String) -> String {
  rew_vfile::add_virtual_file(full_path.as_str(), content.as_str());
  "".to_string()
}

#[op2]
#[string]
fn op_vfile_get(#[string] full_path: String) -> String {
  if let Some(v) = rew_vfile::VIRTUAL_FILES
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
fn op_gen_uid(length: i32, #[string] seed: Option<String>) -> String {
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
fn op_terminal_size() -> Result<(u16, u16), std::io::Error> {
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
