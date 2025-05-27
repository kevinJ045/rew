#![allow(unused_variables)]
#![allow(clippy::derivable_impls)]
use deno_core::Extension;

trait ExtensionTrait<A> {
  fn init(options: A) -> Extension;

  /// Makes a call to `init_ops_and_esm` equivalent to `init_ops`
  fn set_esm(mut ext: Extension, is_snapshot: bool) -> Extension {
    if is_snapshot {
      ext.js_files = ::std::borrow::Cow::Borrowed(&[]);
      ext.esm_files = ::std::borrow::Cow::Borrowed(&[]);
      ext.esm_entry_point = ::std::option::Option::None;
    }
    ext
  }

  /// Builds an extension
  fn build(options: A, is_snapshot: bool) -> Extension {
    let ext = Self::init(options);
    Self::set_esm(ext, is_snapshot)
  }
}

pub mod console;
pub mod ffi;
pub mod url;
pub mod web;
pub mod webidl;

pub mod fs;
pub mod io;
pub mod os;
pub mod process;
pub mod telemetry;
pub mod networking;
pub mod http;
