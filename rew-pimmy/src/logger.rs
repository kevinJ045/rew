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

const BEGIN: &str         = "╭";
const SEPARATOR: &str     = "│";
const MIDDLE_PREFIX: &str = "├";
const END: &str           = "╰";

fn print_norm(logs: String) {
  println!("{}", SEPARATOR.dimmed());
  println!("{} {}", MIDDLE_PREFIX.dimmed(), logs);
}

pub fn begin() {
  // let symbol = get_symbol("info").blue();
  println!("{}", BEGIN.dimmed());
}

pub fn end() {
  // let symbol = get_symbol("info").blue();
  println!("{}", END.dimmed());
}

pub fn info(message: &str) {
  // let symbol = get_symbol("info").blue();
  print_norm(format!("{}", message));
}

pub fn warn(message: &str) {
  let symbol = get_symbol("warn").yellow();
  let warn_text = " WARN ".black().on_yellow();
  print_norm(format!("{} {} {}", symbol, warn_text, message.yellow()));
}

pub fn error(message: &str) {
  let symbol = get_symbol("err").red();
  let err_text = " ERROR ".black().on_red();
  print_norm(format!("{} {} {}", symbol, err_text, message.red()));
}

pub fn verbose(message: &str) {
  if VERBOSE.load(Ordering::Relaxed) {
    let symbol = get_symbol("terminal").dimmed();
    print_norm(format!("{} {}", symbol, message));
  }
}
