use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;
use std::path::PathBuf;
use tokio;

pub mod ext;
mod civet;
mod compiler;
mod runtime;
mod runtime_script;
mod utils;
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
  let cli = Cli::parse();

  match &cli.command {
    Commands::Run { file, watch, compile } => {
      println!("Running file: {}", file.display().to_string().green());
      
      if *watch {
        println!("Watch mode enabled");
        // TODO: Implement watch mode
      }
      
      if *compile {
        println!("Compile mode enabled");
        // TODO: Implement compile-only mode
      }
      
      println!("New runtime initiation");
      let mut runtime = RewRuntime::new()?;
      runtime.run_file(file).await?;
      println!("Execution Done");
    },
    Commands::Exec { code } => {
      println!("Executing code: {}", code.blue());
      // TODO: Implement code execution
      let mut runtime = RewRuntime::new()?;
      // TODO: Add a method to execute code directly
    }
  }

  Ok(())
}
