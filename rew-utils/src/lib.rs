pub fn is_js_executable(mod_id: &str) -> bool {
  matches!(
    mod_id.rsplit('.').next(),
    Some("ts" | "js" | "coffee" | "civet" | "rew")
  )
}
