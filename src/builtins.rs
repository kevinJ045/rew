use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static BUILTIN_MODULES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
  let mut m = HashMap::new();
  m.insert("#std.ghost", "\nrew::io::out.print 'hi'");
  m
});
