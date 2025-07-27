use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static VIRTUAL_FILES: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| Mutex::new(vec![]));

/// Adds a virtual file to the runtime's virtual file storage.
///
/// # Arguments
/// * `path` - The path of the virtual file.
/// * `contents` - The contents of the virtual file.
pub fn add_virtual_file(path: &str, contents: &str) {
  let mut files = VIRTUAL_FILES.lock().unwrap();
  files.push((path.to_string(), contents.to_string()));
}
