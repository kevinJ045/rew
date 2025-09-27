use super::ExtensionTrait;
use deno_core::{Extension, extension};

extension!(
  init_tls,
  esm_entry_point = "ext:init_tls/init_tls.js",
  esm = [ dir "src/ext/tls", "init_tls.js" ],
);
impl ExtensionTrait<()> for init_tls {
  fn init((): ()) -> Extension {
    init_tls::init()
  }
}
impl ExtensionTrait<()> for deno_tls::deno_tls {
  fn init((): ()) -> Extension {
    deno_tls::deno_tls::init()
  }
}

pub fn extensions(is_snapshot: bool) -> Vec<Extension> {
  vec![
    deno_tls::deno_tls::build((), is_snapshot),
    init_tls::build((), is_snapshot),
  ]
}
