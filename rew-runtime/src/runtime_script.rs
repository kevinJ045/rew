const RUNTIME_SCRIPT: &str = include_str!("../../lib/rew/runtime.js");

pub fn get_runtime_script() -> String {
  // std::fs::read_to_string("./lib/rew/runtime.js").unwrap()
  RUNTIME_SCRIPT.to_string()
}
