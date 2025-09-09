use clap::{Parser, Subcommand};
use colored::*;
use rew_core::utils;
use rew_runtime::{RewRuntime, add_virtual_file};
use std::fs;
use std::path::PathBuf;
use tokio::task::LocalSet;

/// Ensures that necessary directories for the Rew runtime exist.
///
/// This function creates the following directories if they are missing:
/// - Root directory for Rew
/// - Subdirectories: `apps`, `bin`, `data`, and `config`
///
/// Returns:
/// - `Ok(())` if all directories are created successfully.
/// - An `anyhow::Result` error if directory creation fails.
fn ensure_rew_dirs() -> anyhow::Result<()> {
  let rew_root = utils::get_rew_root();

  // Helper function to create directories if they don't exist
  fn create_dir_if_missing(path: &PathBuf) -> anyhow::Result<()> {
    if !path.exists() {
      fs::create_dir_all(path)?;
    }
    Ok(())
  }

  create_dir_if_missing(&rew_root)?;
  create_dir_if_missing(&rew_root.join("apps"))?;
  create_dir_if_missing(&rew_root.join("bin"))?;
  create_dir_if_missing(&rew_root.join("data"))?;
  create_dir_if_missing(&rew_root.join("config"))?;
  create_dir_if_missing(&rew_root.join(".pimmy"))?;

  Ok(())
}

#[derive(Parser)]
#[command(name = "rew")]
#[command(version = env!("CARGO_PKG_VERSION"),)]
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

    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
  },
  Test {
    #[arg(name = "FILE")]
    file: PathBuf,

    #[arg(short, long, help = "Specify the tests to run")]
    test: Option<String>,
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
  App {
    #[arg(name = "APP")]
    app: String,

    #[arg(short = 'r', long)]
    remove: bool,

    #[arg(short = 'a', long)]
    add: bool,

    #[arg(short = 'c', long)]
    cache: bool,

    #[arg(short = 'q', long)]
    query: Option<String>,
  },
  Repo {
    #[arg(name = "APP")]
    repo: String,

    #[arg(short = 'r', long)]
    remove: bool,

    #[arg(short = 'a', long)]
    add: Option<String>,

    #[arg(short = 's', long)]
    sync: bool,

    #[arg(short = 'q', long)]
    query: Option<String>,
  },
  Build {
    #[arg(name = "APP")]
    app: String,

    #[arg(short = 's', long)]
    safe: bool,

    // #[arg(short = 'c', long)]
    // cache: bool,
  },
  List {
    #[arg(short = 'a', long)]
    app: bool,

    #[arg(short = 'r', long)]
    repo: Option<String>,

    #[arg(short = 'c', long)]
    cache: bool,
  },
  Readme {
    #[arg(name = "APP")]
    app: String,
  },
  New {
    #[arg(name = "APP")]
    app: String,
    
    #[arg(short = 'g', long)]
    git: bool,
    
    #[arg(short = 't', long)]
    types: bool,
    
    #[arg(short = 'i', long)]
    ignore: bool,
  }
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
          watch: _,
          entry,
          args,
        } => {
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
          let content = fs::read_to_string(file)?;
          let f = runtime.compile_and_run(&content, file, true).await?;
          println!("{}", f);
        }
        Commands::Exec { code, args } => {
          let mut runtime = RewRuntime::new(Some(args.clone()), None)?;
          add_virtual_file(
            "/internal/_repl.coffee",
            format!(
              "import \"#std!\";\nusing namespace rew::ns;\n{}",
              &code.clone()
            )
            .as_str(),
          );

          runtime.run_file("/internal/_repl.coffee").await?;
        }
        Commands::Test { file, test } => {
          let mut runtime = RewRuntime::new(None, None)?;
          add_virtual_file(
            "/internal/_testing.coffee",
            format!(
              "import \"#std.testing!\";tests = \"{}\";\nrew::testing::runAll(tests ? tests.split(',') : []);",
              test.clone().unwrap_or("".to_string())
            )
            .as_str(),
          );

          runtime.run_file(file).await?;
          runtime.run_file("/internal/_testing.coffee").await?;
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
              file_path.display().to_string().blue(),
              output.display().to_string().green()
            );
            let mut runtime = RewRuntime::new(None, None)?;
            let output_string = runtime
              .build_file(
                file_path,
                rew_core::BuildOptions {
                  bundle_all: *bundle_all,
                  entry_file: entry.clone(),
                },
              )
              .await?;

            fs::write(output, output_string.clone())?;
          } else {
            println!("No file specified for brewing");
          }
        },
        Commands::New { app, git, types, ignore } => {
          rew_pimmy::repo::init();
          rew_pimmy::logger::begin();
          rew_pimmy::project::new(app.into(), *git, *ignore, *types);
          rew_pimmy::logger::end();
        },
        Commands::App { app, remove, add, query, cache } => {
          rew_pimmy::repo::init();
          rew_pimmy::logger::begin();
          if *add {
            if let Some(cache_entry) = rew_pimmy::cache::resolve_cache_entry(app, true, true, false, *cache).await {
              rew_pimmy::cache::install_from(cache_entry, Some(true)).await;
            } else {
              rew_pimmy::cache::install(app, Some(true)).await;
            }
          } else if *remove {
            rew_pimmy::cache::remove_app_impl(app, true);
          } else if let Some(query) = query {
            rew_pimmy::repo::find_app(query);
          } else {
            rew_pimmy::logger::info("Nothing to do");
          }
          rew_pimmy::logger::end();
        },
        Commands::Build { app, safe} => {
          rew_pimmy::repo::init();
          rew_pimmy::logger::begin();
          rew_pimmy::builder::build(app, *safe).await;
          rew_pimmy::logger::end();
        },
        Commands::Repo { repo, remove, add, query, sync } => {
          rew_pimmy::repo::init();
          rew_pimmy::logger::begin();
          if *sync {
            rew_pimmy::repo::sync_all(Some(repo.into())).await;
          } else if let Some(add) = add {
            rew_pimmy::repo::add(repo.into(), add.into());
          } else if *remove {
            rew_pimmy::repo::remove(repo.into());
          } else if let Some(query) = query {
            rew_pimmy::repo::find_app(query);
          } else {
            rew_pimmy::logger::info("Nothing to do");
          }
          rew_pimmy::logger::end();
        },
        Commands::Readme { app } => {
          rew_pimmy::repo::init();
          rew_pimmy::repo::readme(app).await;
        },
        Commands::List { app, repo, cache } => {
          rew_pimmy::repo::init();
          rew_pimmy::logger::begin();
          if let Some(repo) = repo {
            rew_pimmy::repo::list_packages_in_repo(repo);
          } else if *app || *cache {
            rew_pimmy::cache::list_installed();
          } else {
            rew_pimmy::repo::list(None)
          }
          rew_pimmy::logger::end();
        }
      }

      Ok(())
    }))
}
