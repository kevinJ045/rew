use crate::utils;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
pub struct DataManager {
  data_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataFormat {
  Text,
  Json,
  Yaml,
  Binary,
}

impl DataManager {
  pub fn new(user_id: &str, app_package: &str) -> Result<Self> {
    let rew_root = utils::get_rew_root();
    let data_dir = rew_root.join("data").join(user_id).join(app_package);

    // Ensure the directory exists
    fs::create_dir_all(&data_dir)
      .with_context(|| format!("Failed to create data directory: {:?}", data_dir))?;

    Ok(Self { data_dir })
  }

  pub fn get_path(&self, key: &str) -> PathBuf {
    self.data_dir.join(key)
  }

  pub fn read(&self, key: &str) -> Result<String> {
    let path = self.get_path(key);
    fs::read_to_string(&path).with_context(|| format!("Failed to read data file: {:?}", path))
  }

  pub fn read_json(&self, key: &str) -> Result<Value> {
    let content = self.read(key)?;
    serde_json::from_str(&content)
      .with_context(|| format!("Failed to parse JSON from data file: {}", key))
  }

  pub fn write(&self, key: &str, content: &str) -> Result<()> {
    let path = self.get_path(key);

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create parent directory: {:?}", parent))?;
    }

    fs::write(&path, content).with_context(|| format!("Failed to write data file: {:?}", path))
  }

  pub fn write_json(&self, key: &str, value: &Value) -> Result<()> {
    let content = serde_json::to_string_pretty(value)
      .with_context(|| format!("Failed to serialize JSON for data file: {}", key))?;
    self.write(key, &content)
  }

  pub fn delete(&self, key: &str) -> Result<()> {
    let path = self.get_path(key);
    if path.exists() {
      fs::remove_file(&path).with_context(|| format!("Failed to delete data file: {:?}", path))?;
    }
    Ok(())
  }

  pub fn exists(&self, key: &str) -> bool {
    self.get_path(key).exists()
  }

  pub fn list(&self, prefix: &str) -> Result<Vec<String>> {
    let dir = self.data_dir.join(prefix);
    if !dir.exists() {
      return Ok(Vec::new());
    }

    let mut result = Vec::new();
    self.list_recursive(&dir, &mut result, prefix)?;
    Ok(result)
  }

  fn list_recursive(&self, dir: &Path, result: &mut Vec<String>, _prefix: &str) -> Result<()> {
    if !dir.exists() {
      return Ok(());
    }

    for entry in
      fs::read_dir(dir).with_context(|| format!("Failed to read directory: {:?}", dir))?
    {
      let entry = entry?;
      let path = entry.path();

      if path.is_file() {
        let rel_path = path
          .strip_prefix(&self.data_dir)
          .unwrap_or(&path)
          .to_string_lossy()
          .to_string();
        result.push(rel_path);
      } else if path.is_dir() {
        self.list_recursive(&path, result, _prefix)?;
      }
    }

    Ok(())
  }

  // Read binary data
  pub fn read_binary(&self, key: &str) -> Result<Vec<u8>> {
    let path = self.get_path(key);
    fs::read(&path).with_context(|| format!("Failed to read binary data file: {:?}", path))
  }

  // Write binary data
  pub fn write_binary(&self, key: &str, data: &[u8]) -> Result<()> {
    let path = self.get_path(key);

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create parent directory: {:?}", parent))?;
    }

    fs::write(&path, data).with_context(|| format!("Failed to write binary data file: {:?}", path))
  }

  // Read YAML data
  pub fn read_yaml(&self, key: &str) -> Result<Value> {
    let content = self.read(key)?;
    serde_yaml::from_str(&content)
      .with_context(|| format!("Failed to parse YAML from data file: {}", key))
  }

  // Write YAML data
  pub fn write_yaml(&self, key: &str, value: &Value) -> Result<()> {
    let content = serde_yaml::to_string(value)
      .with_context(|| format!("Failed to serialize YAML for data file: {}", key))?;
    self.write(key, &content)
  }

  // Get file info including format detection
  pub fn get_file_info(&self, key: &str) -> Result<(bool, DataFormat)> {
    let path = self.get_path(key);

    if !path.exists() {
      return Ok((false, DataFormat::Text)); // Default to text for non-existent files
    }

    // Try to detect format based on extension
    let format = if let Some(ext) = path.extension() {
      match ext.to_str().unwrap_or("").to_lowercase().as_str() {
        "json" => DataFormat::Json,
        "yaml" | "yml" => DataFormat::Yaml,
        "bin" | "dat" => DataFormat::Binary,
        _ => {
          // Try to detect based on content
          self.detect_format(&path)?
        }
      }
    } else {
      // No extension, try to detect based on content
      self.detect_format(&path)?
    };

    Ok((true, format))
  }

  // Detect format based on file content
  fn detect_format(&self, path: &Path) -> Result<DataFormat> {
    // Read a small sample of the file
    let mut file = fs::File::open(path)?;
    let mut buffer = [0u8; 512]; // Read first 512 bytes
    let bytes_read = file.read(&mut buffer)?;

    if bytes_read == 0 {
      return Ok(DataFormat::Text); // Empty file, default to text
    }

    // Check for binary content
    let sample = &buffer[..bytes_read];
    if sample
      .iter()
      .any(|&b| b < 9 || (b > 13 && b < 32 && b != 27))
    {
      return Ok(DataFormat::Binary);
    }

    // Try to parse as JSON
    let content = String::from_utf8_lossy(sample);
    if (content.trim_start().starts_with('{') || content.trim_start().starts_with('['))
      && serde_json::from_str::<Value>(&content).is_ok()
    {
      return Ok(DataFormat::Json);
    }

    // Try to parse as YAML
    if content.contains(':')
      && !content.contains('{')
      && serde_yaml::from_str::<Value>(&content).is_ok()
    {
      return Ok(DataFormat::Yaml);
    }

    // Default to text
    Ok(DataFormat::Text)
  }
}
