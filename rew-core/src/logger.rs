use colored::*;
use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(verbose: bool) {
  VERBOSE.store(verbose, Ordering::Relaxed);
}

fn get_symbol(name: &str) -> &str {
  match name {
    "info" => "",
    "types" => "",
    "warn" => "",
    "file" => "",
    "folder" => "",
    "err" => "",
    "suc" => "",
    "question" => "",
    "package" => "",
    "git" => "󰊢",
    "github" => "",
    "download" => "",
    "build" => "",
    "terminal" => "",
    _ => "",
  }
}

pub fn info(message: &str) {
  let symbol = get_symbol("info").blue();
  println!("{} {}", symbol, message);
}

pub fn warn(message: &str) {
  let symbol = get_symbol("warn").yellow();
  let warn_text = " WARN ".black().on_yellow();
  println!("{} {} {}", symbol, warn_text, message.yellow());
}

pub fn error(message: &str) {
  let symbol = get_symbol("err").red();
  let err_text = " ERROR ".black().on_red();
  println!("{} {} {}", symbol, err_text, message.red());
}

pub fn verbose(message: &str) {
  if VERBOSE.load(Ordering::Relaxed) {
    let symbol = get_symbol("terminal").dimmed();
    println!("{} {}", symbol, message);
  }
}
