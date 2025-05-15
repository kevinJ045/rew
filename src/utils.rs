use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
  pub manifest: Option<Manifest>,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
  pub package: Option<String>,
}

pub struct AppInfo {
  pub path: PathBuf,
  pub config: AppConfig,
}

pub fn find_app_info(filepath: &Path) -> Option<AppInfo> {
  let app_path = find_app_path(filepath.parent()?);

  if let Some(ref app_path) = app_path {
    let yaml_path = app_path.join("app.yaml");

    match fs::read_to_string(&yaml_path) {
      Ok(contents) => match serde_yaml::from_str::<AppConfig>(&contents) {
        Ok(config) => {
          return Some(AppInfo {
            path: app_path.clone(),
            config,
          });
        }
        Err(e) => {
          eprintln!("Failed to parse YAML: {}", e);
        }
      },
      Err(e) => {
        eprintln!("Failed to read app.yaml: {}", e);
      }
    }
  }

  None
}

pub fn find_app_path(current_dir: &Path) -> Option<PathBuf> {
  let app_yaml = current_dir.join("app.yaml");
  if app_yaml.exists() {
    return Some(current_dir.to_path_buf());
  }

  let parent = current_dir.parent()?;
  if parent == current_dir {
    return None;
  }

  find_app_path(parent)
}
