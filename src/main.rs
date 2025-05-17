use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;
use tokio;
use std::fs;

pub mod builtins;
pub mod data_manager;
mod civet;
mod compiler;
mod declarations;
pub mod ext;
mod runtime;
mod runtime_script;
mod utils;
use runtime::RewRuntime;

fn ensure_rew_dirs() -> anyhow::Result<()> {
  let rew_root = utils::get_rew_root();
  
  // Create the root directory if it doesn't exist
  if !rew_root.exists() {
    fs::create_dir_all(&rew_root)?;
  }
  
  // Create the apps directory if it doesn't exist
  let apps_dir = rew_root.join("apps");
  if !apps_dir.exists() {
    fs::create_dir_all(&apps_dir)?;
  }
  
  // Create other directories as needed
  let data_dir = rew_root.join("data");
  if !data_dir.exists() {
    fs::create_dir_all(&data_dir)?;
  }
  
  let config_dir = rew_root.join("config");
  if !config_dir.exists() {
    fs::create_dir_all(&config_dir)?;
  }
  
  Ok(())
}

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
  
  // Ensure Rew directories exist
  ensure_rew_dirs()?;

  match &cli.command {
    Commands::Run {
      file,
      watch,
      compile,
    } => {
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
    }
    Commands::Exec { code } => {
      println!("Executing code: {}", code.blue());
      // TODO: Implement code execution
      let mut runtime = RewRuntime::new()?;
      // TODO: Add a method to execute code directly
    }
  }

  Ok(())
}
