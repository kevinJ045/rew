use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::io::Write;
use std::path::PathBuf;

/// Encodes the provided string content into Base64 format.
///
/// # Arguments
/// * `content` - The string to encode.
///
/// # Returns
/// * A Base64 encoded string.
pub fn encode_brew_file(content: &str) -> String {
  BASE64.encode(content.as_bytes())
}

/// Decodes a Base64 encoded string back into its original form.
///
/// # Arguments
/// * `encoded` - The Base64 encoded string to decode.
///
/// # Returns
/// * A `Result` containing the decoded string or an error message.
pub fn decode_brew_file(encoded: &str) -> Result<String> {
  let decoded = BASE64
    .decode(encoded.trim())
    .map_err(|e| anyhow::anyhow!("Failed to decode brew file: {}", e))?;

  String::from_utf8(decoded)
    .map_err(|e| anyhow::anyhow!("Failed to convert decoded bytes to string: {}", e))
}

pub fn patch_binary(bin_path: &PathBuf, script_path: &PathBuf) -> std::io::Result<()> {
  let mut bin = std::fs::OpenOptions::new().append(true).open(bin_path)?;
  let script = std::fs::read(script_path)?;
  let len = script.len() as u64;

  bin.write_all(&script)?;
  bin.write_all(&len.to_le_bytes())?;
  bin.write_all(b"REW!")?;
  Ok(())
}

pub fn make_qrew(output: &PathBuf, file: &PathBuf) -> std::io::Result<()> {
  if let Some(exe_path) = std::env::current_exe().ok() {
    std::fs::copy(exe_path, output)?;
    patch_binary(&output, file)
  } else {
    Err(std::io::Error::new(
      std::io::ErrorKind::InvalidData,
      "Could not find main qrew stub",
    ))
  }
}

pub fn to_qrew(file: PathBuf) -> PathBuf {
  let mut new_file = file.clone();

  let stem = file.file_stem().unwrap_or_default().to_string_lossy();

  #[cfg(target_os = "windows")]
  let new_name = format!("{}.qrew.exe", stem);

  #[cfg(not(target_os = "windows"))]
  let new_name = format!("{}.qrew", stem);

  new_file.set_file_name(new_name);
  new_file
}
