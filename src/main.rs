use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;
use tokio;
use tokio::task::LocalSet;

pub mod builtins;
mod civet;
mod compiler;
pub mod data_manager;
mod declarations;
pub mod ext;
pub mod runtime;
mod runtime_script;
// mod shell;
mod jsx;
mod utils;
mod workers;
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

    #[arg(short, long, help = "Specify an entry point for app packages")]
    entry: Option<String>,

    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
  },
  Compile {
    #[arg(name = "FILE")]
    file: PathBuf,
  },
  Exec {
    #[arg(name = "CODE")]
    code: String,
  },
  Brew {
    #[arg(name = "FILE")]
    file: Option<PathBuf>,

    #[arg(name = "OUTPUT", default_value = "output.brew")]
    output: PathBuf,

    #[arg(short, long, help = "Create a brew for your app.")]
    bundle_all: bool,

    #[arg(
      short,
      long,
      help = "Specify an entry file different from the main file"
    )]
    entry: Option<PathBuf>,
  },
}

fn main() -> anyhow::Result<()> {
  let local = LocalSet::new();
  tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()?
    .block_on(local.run_until(async {
      // let cli = Cli::parse_from(["rew", "run", "./test/imp.coffee"]);
      let cli = Cli::parse();

      // Ensure Rew directories exist
      ensure_rew_dirs()?;

      match &cli.command {
        Commands::Run {
          file,
          watch,
          entry,
          args,
        } => {
          if *watch {}
          if file.is_dir() {
            let app_yaml = file.join("app.yaml");
            if app_yaml.exists() {
              let entry_point = if let Some(entry_name) = entry {
                entry_name
              } else {
                &"main".to_string()
              };

              // Read app.yaml to find the entry file
              if let Ok(config_str) = fs::read_to_string(&app_yaml) {
                if let Ok(config) = serde_yaml::from_str::<utils::AppConfig>(&config_str) {
                  if let Some(entries) = config.entries {
                    if let Some(entry_file) = entries.get(&entry_point.clone()) {
                      let full_path = file.join(entry_file);

                      let mut runtime = RewRuntime::new(Some(args.clone()), None)?;
                      runtime.run_file(&full_path).await?;
                      return Ok(());
                    }
                  }
                }
              }
              println!("Failed to find entry point in app.yaml");
            } else {
              println!("No app.yaml found in directory");
            }
          } else if !file.exists()
            && !file.to_string_lossy().contains('/')
            && !file.to_string_lossy().contains('\\')
          {
            let package_name = file.to_string_lossy().to_string();

            let entry_name = entry.as_deref().unwrap_or("main");

            if let Some(app_entry) = utils::resolve_app_entry(&package_name, Some(entry_name)) {
              if utils::is_valid_utf8(app_entry.clone())? {
                let mut runtime = RewRuntime::new(Some(args.clone()), None)?;
                runtime.run_file(&app_entry).await?;
                return Ok(());
              } else {
                println!("App running binary");
                std::process::Command::new(app_entry.to_string_lossy().to_string())
                  .args(args.clone())
                  .stdout(std::process::Stdio::inherit())
                  .stderr(std::process::Stdio::inherit())
                  .stdin(std::process::Stdio::inherit())
                  .spawn()
                  .expect("Failed to start process")
                  .wait()
                  .expect("Failed to wait on child");
              }
            } else {
              println!("App package not found: {}", package_name.red());
            }
          } else {
            let mut runtime = RewRuntime::new(Some(args.clone()), None)?;
            runtime.run_file(file).await?;
          }
        }
        Commands::Compile { file } => {
          let mut runtime = RewRuntime::new(None, None)?;
          let content = fs::read_to_string(&file)?;
          let f = runtime.compile_and_run(&content, file, true).await?;
          println!("{}", f);
        }
        Commands::Exec { code } => {
          println!("Executing code: {}", code.blue());
          // TODO: Implement code execution
          // let mut runtime = RewRuntime::new()?;
          // TODO: Add a method to execute code directly
        }
        Commands::Brew {
          file,
          output,
          bundle_all,
          entry,
        } => {
          if let Some(file_path) = file {
            println!(
              "Building file: {} to {}",
              file_path.display().to_string().green(),
              output.display().to_string().green()
            );

            if *bundle_all {
              println!("Including all apps in build");
            } else {
              println!("Including only the main app in build");
            }

            if let Some(entry_path) = entry {
              println!(
                "Using custom entry: {}",
                entry_path.display().to_string().yellow()
              );
            }

            let mut runtime = RewRuntime::new(None, None)?;

            let options = runtime::BuildOptions {
              bundle_all: *bundle_all,
              entry_file: entry.clone(),
            };

            let output_string = runtime.build_file(file_path, options).await?;

            fs::write(output, output_string.clone())?;
          }
          println!("Building complete");
        }
      }
      Ok(())
    }))
}
