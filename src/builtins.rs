use once_cell::sync::Lazy;
use std::collections::HashMap;

// Include built-in modules directly in the binary
const FFI_MODULE: &str = include_str!("../lib/rew/builtins/ffi.coffee");
const CONF_MODULE: &str = include_str!("../lib/rew/builtins/conf.coffee");
const ALL_MODULE: &str = include_str!("../lib/rew/builtins/all.coffee");

pub static BUILTIN_MODULES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    m.insert("#std.ffi", FFI_MODULE);
    m.insert("#std.conf", CONF_MODULE);
    m.insert("#std", ALL_MODULE);
    
    m
});
