use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// App configuration structure
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppManifest {
  pub package: Option<String>,
  pub version: Option<String>,
  pub description: Option<String>,
  pub entries: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
  pub manifest: Option<AppManifest>,
  pub entries: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct AppInfo {
  pub path: PathBuf,
  pub config: AppConfig,
}

// Get the Rew root directory
pub fn get_rew_root() -> PathBuf {
  // First check for REW_ROOT environment variable
  if let Ok(rew_root) = env::var("REW_ROOT") {
    return PathBuf::from(rew_root);
  }

  // Otherwise use the default location based on platform
  #[cfg(target_os = "windows")]
  {
    let local_app_data = env::var("LOCALAPPDATA").unwrap_or_else(|_| {
      let home = env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
      format!("{}\\AppData\\Local", home)
    });
    PathBuf::from(format!("{}\\rew", local_app_data))
  }

  #[cfg(not(target_os = "windows"))]
  {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(format!("{}/.rew", home))
  }
}

// Find an app by package name
pub fn find_app_by_package(package_name: &str) -> Option<AppInfo> {
  let rew_root = get_rew_root();
  let apps_dir = rew_root.join("apps");

  if !apps_dir.exists() {
    return None;
  }

  // let app_dir = apps_dir.join(package_name);
  // let config_path = app_dir.join("app.yaml");

  // println!("{}", config_path.display());

  let app_dirs = fs::read_dir(&apps_dir).ok()?;

  for dir_entry in app_dirs {
    if let Ok(entry) = dir_entry {
      let app_dir = entry.path();
      if !app_dir.is_dir() {
        continue;
      }

      let config_path = app_dir.join("app.yaml");
      if !config_path.exists() {
        continue;
      }

      // Read and parse the app.yaml file
      let config_str = fs::read_to_string(&config_path).ok()?;
      let config: AppConfig = serde_yaml::from_str(&config_str).ok()?;

      // Check if this app has the requested package name
      if let Some(manifest) = &config.manifest {
        if let Some(pkg) = &manifest.package {
          if pkg == package_name {
            return Some(AppInfo {
              path: app_dir,
              config,
            });
          }
        }
      }
    }
  }

  None
}

// Find app info for a file path
pub fn find_app_info(file_path: &Path) -> Option<AppInfo> {
  let mut current = file_path;

  // Walk up the directory tree looking for app.yaml
  while let Some(parent) = current.parent() {
    let config_path = parent.join("app.yaml");
    if config_path.exists() {
      // Found an app.yaml, parse it
      let config_str = fs::read_to_string(&config_path).ok()?;
      let config: AppConfig = serde_yaml::from_str(&config_str).ok()?;

      return Some(AppInfo {
        path: parent.to_path_buf(),
        config,
      });
    }
    current = parent;
  }

  None
}

// Resolve an app entry point
pub fn resolve_app_entry(package_name: &str, entry_name: Option<&str>) -> Option<PathBuf> {
  let app_info = find_app_by_package(package_name)?;

  // Get the entries from the config
  let entries = app_info.config.entries.as_ref()?;

  // Determine which entry to use
  let entry_key = entry_name.unwrap_or("main");
  let entry_path = entries.get(entry_key)?;

  // Resolve the entry path relative to the app directory
  Some(app_info.path.join(entry_path))
}

// Find the app path for a given file
pub fn find_app_path(dir_path: &Path) -> Option<PathBuf> {
  let mut current = dir_path;

  // Walk up the directory tree looking for app.yaml
  while let Some(parent) = current.parent() {
    let config_path = parent.join("app.yaml");
    // println!("{}", config_path.display());
    if config_path.exists() {
      return Some(parent.to_path_buf());
    }
    current = parent;
  }

  None
}

#[allow(unused)]
pub fn is_valid_utf8<P: AsRef<Path>>(path: P) -> std::io::Result<bool> {
  let bytes = std::fs::read(path)?;
  Ok(std::str::from_utf8(&bytes).is_ok())
}
