use super::ExtensionTrait;
use deno_core::{Extension, extension};

extension!(
  deno_telemetry,
  esm_entry_point = "ext:deno_telemetry/telemetry.ts",
  esm = [ dir "src/ext/telemetry", "telemetry.ts", "util.ts" ],
);
impl ExtensionTrait<()> for deno_telemetry {
  fn init((): ()) -> Extension {
    deno_telemetry::init()
  }
}

impl ExtensionTrait<()> for ::deno_telemetry::deno_telemetry {
  fn init((): ()) -> Extension {
    ::deno_telemetry::deno_telemetry::init()
  }
}

pub fn extensions(is_snapshot: bool) -> Vec<Extension> {
  vec![
    ::deno_telemetry::deno_telemetry::build((), true),
    deno_telemetry::build((), is_snapshot),
  ]
}
