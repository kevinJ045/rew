use super::{DefaultWebPermissions, WebPermissions};
use std::sync::Arc;

/// Options for configuring the web related extensions
#[derive(Clone)]
pub struct WebOptions {
  /// Base URL for some `deno_web` OPs
  pub base_url: Option<deno_core::ModuleSpecifier>,

  /// Permissions manager for sandbox-breaking extensions
  pub permissions: Arc<dyn WebPermissions>,

  /// Blob store for the web related extensions
  pub blob_store: Arc<deno_web::BlobStore>,
}

impl Default for WebOptions {
  fn default() -> Self {
    Self {
      base_url: None,
      permissions: Arc::new(DefaultWebPermissions),
      blob_store: Arc::new(deno_web::BlobStore::default()),
    }
  }
}

impl WebOptions {}
