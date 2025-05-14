use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;
use std::path::PathBuf;
use tokio;

mod civet;
mod compiler;
mod runtime;
mod runtime_script;
use runtime::RewRuntime;

#[derive(Parser)]
#[command(name = "rew")]
#[command(about = "A Rust-based Rew runtime using deno_core")]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Run {
    #[arg(name = "FILE")]
    file: PathBuf,

    #[arg(short, long)]
    watch: bool,

    #[arg(short, long)]
    compile: bool,
  },
  Exec {
    #[arg(name = "CODE")]
    code: String,
  },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let file = PathBuf::from("test/s.coffee");

  let source = std::fs::read_to_string(&file)?;

  // Create runtime in a block so it gets dropped when execution is complete
  {
    println!("New runtime initiation");
    let mut runtime = RewRuntime::new()?;
    runtime.run_file(&file).await?;
    println!("Execution Done");

    // Explicitly drop the runtime to clean up resources
  }

  #[allow(unreachable_code)]
  Ok(())
}
