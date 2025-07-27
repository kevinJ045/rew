//! Core utilities and types for the Rew runtime system

pub mod utils;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Build options for compiling Rew code
#[derive(Debug, Clone, Default)]
pub struct BuildOptions {
  pub bundle_all: bool,
  pub entry_file: Option<PathBuf>,
}

/// App configuration structure
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


#[derive(Default)]
pub struct RuntimeState {
  pub current_dir: PathBuf,
  pub args: Vec<String>,
}
