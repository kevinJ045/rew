use once_cell::sync::Lazy;
use std::collections::HashMap;

// Include built-in modules directly in the binary
const FFI_MODULE: &str = include_str!("../../lib/rew/builtins/ffi.js");
const CONF_MODULE: &str = include_str!("../../lib/rew/builtins/conf.js");
const FS_MODULE: &str = include_str!("../../lib/rew/builtins/fs.js");
const ENCODING_MODULE: &str = include_str!("../../lib/rew/builtins/encoding.js");
const ALL_MODULE: &str = include_str!("../../lib/rew/builtins/all.coffee");
const THREADS_MODULE: &str = include_str!("../../lib/rew/builtins/threads.js");
const OS_MODULE: &str = include_str!("../../lib/rew/builtins/os.js");
const PATH_MODULE: &str = include_str!("../../lib/rew/builtins/path.js");
const SHELL_MODULE: &str = include_str!("../../lib/rew/builtins/shell.js");
const HTTP_MODULE: &str = include_str!("../../lib/rew/builtins/http.js");
const NET_MODULE: &str = include_str!("../../lib/rew/builtins/net.js");
const TYPES_MODULE: &str = include_str!("../../lib/rew/builtins/types.js");
const YAML_MODULE: &str = include_str!("../../lib/rew/builtins/yaml.js");

pub static BUILTIN_MODULES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
  let mut m = HashMap::new();

  m.insert("#std.ffi", FFI_MODULE);
  m.insert("#std.conf", CONF_MODULE);
  m.insert("#std.encoding", ENCODING_MODULE);
  m.insert("#std.fs", FS_MODULE);
  m.insert("#std.threads", THREADS_MODULE);
  m.insert("#std.shell", SHELL_MODULE);
  m.insert("#std.os", OS_MODULE);
  m.insert("#std.path", PATH_MODULE);
  m.insert("#std.http", HTTP_MODULE);
  m.insert("#std.net", NET_MODULE);
  m.insert("#std.types", TYPES_MODULE);
  m.insert("#std.yaml", YAML_MODULE);
  m.insert("#std", ALL_MODULE);

  m
});
