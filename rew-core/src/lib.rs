//! Core utilities and types for the Rew runtime system

pub mod utils;
pub mod logger;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{
  fs::File,
  io::{Read, Seek, SeekFrom}
};

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

pub fn load_embedded_script() -> Option<String> {
  let exe = std::env::current_exe().ok()?;
  let mut f = File::open(&exe).ok()?;

  // footer = [len (u64 LE)] [magic 4 bytes]
  f.seek(SeekFrom::End(-12)).ok()?;
  let mut footer = [0u8; 12];
  f.read_exact(&mut footer).ok()?;

  if &footer[8..12] != b"REW!" {
      return None; // not patched
  }
  let len = u64::from_le_bytes(footer[0..8].try_into().unwrap());

  // read payload
  f.seek(SeekFrom::End(-(12 + len as i64))).ok()?;
  let mut data = vec![0u8; len as usize];
  f.read_exact(&mut data).ok()?;

  String::from_utf8(data).ok()
}

