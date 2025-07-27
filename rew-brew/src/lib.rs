use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use anyhow::Result;

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
