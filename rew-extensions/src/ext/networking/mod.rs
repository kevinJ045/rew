use super::ExtensionTrait;
use deno_core::{Extension, extension};

extension!(
  init_networking,
  esm_entry_point = "ext:init_networking/init_networking.js",
  esm = [ dir "src/ext/networking", "init_networking.js" ],
);
impl ExtensionTrait<()> for init_networking {
  fn init((): ()) -> Extension {
    init_networking::init()
  }
}

impl ExtensionTrait<()> for deno_net::deno_net {
  fn init((): ()) -> Extension {
    deno_net::deno_net::init::<deno_permissions::PermissionsContainer>(None, None)
  }
}

impl ExtensionTrait<()> for deno_websocket::deno_websocket {
  fn init((): ()) -> Extension {
    deno_websocket::deno_websocket::init::<deno_permissions::PermissionsContainer>(
      "rew".to_string(),
      None,
      None,
    )
  }
}

impl ExtensionTrait<()> for deno_fetch::deno_fetch {
  fn init((): ()) -> Extension {
    deno_fetch::deno_fetch::init::<deno_permissions::PermissionsContainer>(
      deno_fetch::Options::default(),
    )
  }
}

pub fn extensions(is_snapshot: bool) -> Vec<Extension> {
  vec![
    deno_fetch::deno_fetch::build((), is_snapshot),
    deno_net::deno_net::build((), is_snapshot),
    deno_websocket::deno_websocket::build((), is_snapshot),
    init_networking::build((), is_snapshot),
  ]
}
