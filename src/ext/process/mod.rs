use super::ExtensionTrait;
use deno_core::{extension, Extension};
use deno_process::NpmProcessStateProvider;
use std::sync::Arc;

extension!(
  init_process,
  esm_entry_point = "ext:init_process/init_process.js",
  esm = [ dir "src/ext/process", "init_process.js" ],
);
impl ExtensionTrait<()> for init_process {
  fn init((): ()) -> Extension {
    init_process::init()
  }
}
impl ExtensionTrait<Option<Arc<dyn NpmProcessStateProvider>>> for deno_process::deno_process {
  fn init(seed: Option<Arc<dyn NpmProcessStateProvider>>) -> Extension {
    deno_process::deno_process::init(None)
  }
}

pub fn extensions(is_snapshot: bool) -> Vec<Extension> {
  vec![
    deno_process::deno_process::build(None, is_snapshot),
    init_process::build((), is_snapshot),
  ]
}
