use clap::{ Parser, Subcommand };
use colored::*;
use std::path::PathBuf;
use std::path::Path;
use tokio;

mod runtime;
mod compiler;
mod civet;
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
  let mut runtime = RewRuntime::new()?;

  match cli.command {
    Commands::Run { file, watch, compile } => {
      let source = std::fs::read_to_string(&file)?;
      let result = runtime.compile_and_run(&source, &file).await?;

			println!("{}", result);

      if compile {
        println!("{}", result);
      } else {
        runtime.execute(&result).await?;
      }
    }
    Commands::Exec { code } => {
      let result = runtime.compile_and_run(&code, &PathBuf::from("repl")).await?;
      runtime.execute(&result).await?;
    }
  }

  Ok(())
}
