use super::ExtensionTrait;
use deno_core::{Extension, extension};

extension!(
  init_os,
  esm_entry_point = "ext:init_os/init_os.js",
  esm = [ dir "src/ext/os", "init_os.js" ],
);
impl ExtensionTrait<()> for init_os {
  fn init((): ()) -> Extension {
    init_os::init()
  }
}
impl ExtensionTrait<()> for deno_os::deno_os {
  fn init((): ()) -> Extension {
    deno_os::deno_os::init(None)
  }
}

pub fn extensions(is_snapshot: bool) -> Vec<Extension> {
  vec![
    deno_os::deno_os::build((), is_snapshot),
    init_os::build((), is_snapshot),
  ]
}
