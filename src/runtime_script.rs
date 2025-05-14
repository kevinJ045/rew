pub fn get_runtime_script() -> String {
  std::fs::read_to_string("./lib/rew/runtime.js").unwrap()
}
