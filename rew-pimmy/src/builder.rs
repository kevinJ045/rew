use crate::logger;
use colored::*;
use rew_runtime::RewRuntime;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::Command;

pub async fn build(app_name: &str, safe: bool) {
  let mode = if safe { "safe" } else { "normal" };
  logger::info(&format!("Building app: {} (mode: {})", app_name, mode));

  let app_path = Path::new(app_name);

  // Find the app in cache
  // let cache = cache::load_app_cache();
  // let Some(cached_app) = cache.apps.get(app_name) else {
  //   logger::error(&format!(
  //     "App '{}' is not installed. Install it first with --add --app {}",
  //     app_name, app_name
  //   ));
  //   return;
  // };

  // if !cached_app.install_path.exists() {
  //   logger::error(&format!(
  //     "App installation path does not exist: {}",
  //     cached_app.install_path.display()
  //   ));
  //   return;
  // }

  logger::info(&format!("Building app from: {}", app_path.display()));

  match build_rew_app(&app_path, safe).await.await {
    Ok(_) => {}
    Err(_) => {}
  }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct NativeDep {
  name: Option<String>,
  #[serde(rename = "type")]
  type_: Option<String>,
  check: Option<String>,
  managers: Option<HashMap<String, String>>,
  preinstall: Option<Vec<ShellCommand>>,
  postinstall: Option<Vec<ShellCommand>>,
  fallback: Option<Value>,
  content: Option<String>,
  url: Option<String>,
  // path: Option<String>,
  // url: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ShellCommand {
  shell: String,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
  pub manifest: Option<Value>,
  pub crates: Option<Vec<CrateConfig>>,
  pub cmods: Option<Vec<CModConfig>>,
  pub prefetch: Option<Vec<PrefetchConfig>>,
  pub build: Option<Vec<BuildConfig>>,
  pub native: Option<Value>,
  pub dependencies: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct CrateConfig {
  pub name: String,
  pub path: String,
  pub build: Option<bool>,
  pub files: Option<Vec<FileConfig>>,
  pub fallback_prefetch: Option<Vec<PrefetchConfig>>,
  pub cleanup: Option<Value>,
  pub set: Option<HashMap<String, Value>>,
  pub condition: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CModConfig {
  pub name: String,
  pub path: String,
  pub build: Option<bool>,
  pub compiler: Option<String>,
  pub flags: Option<Vec<String>>,
  pub files: Option<Vec<FileConfig>>,
  pub fallback_prefetch: Option<Vec<PrefetchConfig>>,
  pub cleanup: Option<Value>,
  pub set: Option<HashMap<String, Value>>,
  pub condition: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FileConfig {
  pub input: String,
  pub output: String,
  pub cleanup: Option<String>,
  pub system: Option<String>,
  pub fallback_prefetch: Option<PrefetchConfig>,
}

#[derive(Debug, Clone)]
pub struct PrefetchConfig {
  pub url: String,
  pub output: String,
  pub extract: Option<String>,
  pub system: Option<String>,
  pub build: Option<bool>,
  pub condition: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BuildConfig {
  pub input: String,
  pub output: String,
  pub using: Option<String>,
  pub args: Option<String>,
  pub cleanup: Option<Value>,
  pub set: Option<HashMap<String, Value>>,
  pub condition: Option<String>,
  pub id: Option<String>,
}

pub async fn build_rew_app<'a>(
  app_path: &'a Path,
  safe: bool,
) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + 'a>> {
  Box::pin(async move {
    let app_conf_path = app_path.join("app.yaml");

    if !app_conf_path.exists() {
      return Err("App not found - no app.yaml".into());
    }

    let config_content = fs::read_to_string(&app_conf_path)?;
    let config_yaml: Value = serde_yaml::from_str(&config_content)?;

    let app_config = parse_app_config(&config_yaml)?;

    let package_name = app_config
      .manifest
      .as_ref()
      .and_then(|m| m.get("package"))
      .and_then(|p| p.as_str())
      .unwrap_or("unknown");

    logger::info(&format!("Building App: {}", package_name));

    if app_config.crates.is_none() && app_config.build.is_none() && app_config.prefetch.is_none() {
      return Err("no build candidates found".into());
    }

    logger::info("Setting build lock...");

    let mut errors = 0;

    if let Some(native) = &app_config.native {
      if let Some(on) = native.get("on").and_then(|v| v.as_str()) {
        if on == "build" {
          if let Err(e) = install_native_deps(native, app_path, safe) {
            logger::error(&format!("Failed to install native dependencies: {}", e));
            errors += 1;
          }
        }
      }
    }

    if let Some(prefetch_configs) = &app_config.prefetch {
      logger::info("Processing prefetch configurations...");
      for prefetch in prefetch_configs {
        if let Err(e) = handle_prefetch(prefetch, app_path, safe).await {
          logger::error(&format!("Prefetch failed: {}", e));
          errors += 1;
        }
      }
    }

    if let Some(crate_configs) = &app_config.crates {
      if !is_cargo_available() {
        logger::warn("Cargo not available, using fallback prefetch for crates");
        // Add crate fallback_prefetch to prefetch list
        for crate_config in crate_configs {
          if let Some(fallback_prefetch) = &crate_config.fallback_prefetch {
            for prefetch in fallback_prefetch {
              if let Err(e) = handle_prefetch(prefetch, app_path, safe).await {
                logger::error(&format!("Crate fallback prefetch failed: {}", e));
                errors += 1;
              }
            }
          }
          if let Some(files) = &crate_config.files {
            for file in files {
              if let Some(fallback_prefetch) = &file.fallback_prefetch {
                if let Err(e) = handle_prefetch(fallback_prefetch, app_path, safe).await {
                  logger::error(&format!("File fallback prefetch failed: {}", e));
                  errors += 1;
                }
              }
            }
          }
        }
      } else {
        logger::info(&format!("Building app crates for {}", package_name));
        for crate_config in crate_configs {
          if !build_crate(crate_config, app_path, safe) {
            errors += 1;
          }
        }
      }
    }

    if let Some(cmod_configs) = &app_config.cmods {
      logger::info(&format!("Building app cmods for {}", package_name));
      for cmod_config in cmod_configs {
        if !build_cmod(cmod_config, app_path, safe) {
          errors += 1;
        }
      }
    }

    if let Some(build_configs) = &app_config.build {
      logger::info("Processing build configurations...");
      for build_config in build_configs {
        if let Err(e) = handle_build_step(build_config, app_path, safe).await {
          logger::error(&format!("Build step failed: {}", e));
          errors += 1;
        }
      }
    }

    // Clear building lock
    logger::info("Clearing build lock...");

    logger::info(&format!("Finished build with {} errors.", errors));

    if errors > 0 {
      Err(format!("Build completed with {} errors", errors).into())
    } else {
      Ok(())
    }
  })
}

fn parse_file_config(file_yaml: &Value) -> FileConfig {
  let fallback_prefetch = if let Some(fp_yaml) = file_yaml.get("fallback_prefetch") {
    Some(parse_prefetch_config(fp_yaml))
  } else {
    None
  };

  FileConfig {
    input: file_yaml
      .get("input")
      .and_then(|i| i.as_str())
      .unwrap_or("")
      .to_string(),
    output: file_yaml
      .get("output")
      .and_then(|o| o.as_str())
      .unwrap_or("")
      .to_string(),
    cleanup: file_yaml
      .get("cleanup")
      .and_then(|c| c.as_str())
      .map(|s| s.to_string()),
    system: file_yaml
      .get("system")
      .and_then(|s| s.as_str())
      .map(|s| s.to_string()),
    fallback_prefetch,
  }
}

fn parse_prefetch_config(prefetch_yaml: &Value) -> PrefetchConfig {
  PrefetchConfig {
    url: prefetch_yaml
      .get("url")
      .and_then(|u| u.as_str())
      .unwrap_or("")
      .to_string(),
    output: prefetch_yaml
      .get("output")
      .and_then(|o| o.as_str())
      .unwrap_or("")
      .to_string(),
    extract: prefetch_yaml
      .get("extract")
      .and_then(|e| e.as_str())
      .map(|s| s.to_string()),
    system: prefetch_yaml
      .get("system")
      .and_then(|s| s.as_str())
      .map(|s| s.to_string()),
    build: prefetch_yaml.get("build").and_then(|b| b.as_bool()),
    condition: None, // TODO
  }
}

pub fn parse_app_config(config_yaml: &Value) -> Result<AppConfig, Box<dyn std::error::Error>> {
  let mut app_config = AppConfig {
    manifest: config_yaml.get("manifest").cloned(),
    crates: None,
    cmods: None,
    prefetch: None,
    build: None,
    dependencies: config_yaml
      .get("dependencies")
      .and_then(|c| c.as_sequence())
      .map(|seq| {
        Some(
          seq
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        )
      })
      .unwrap_or(None),
    native: config_yaml.get("native").cloned(),
  };

  // Parse crates
  if let Some(crates_yaml) = config_yaml.get("crates").and_then(|c| c.as_sequence()) {
    let mut crates = Vec::new();
    for crate_yaml in crates_yaml {
      let crate_config = CrateConfig {
        name: crate_yaml
          .get("name")
          .and_then(|n| n.as_str())
          .unwrap_or("")
          .to_string(),
        path: crate_yaml
          .get("path")
          .and_then(|p| p.as_str())
          .unwrap_or("")
          .to_string(),
        build: crate_yaml.get("build").and_then(|b| b.as_bool()),
        files: if let Some(files_yaml) = crate_yaml.get("files").and_then(|f| f.as_sequence()) {
          let mut file_configs = Vec::new();
          for file_yaml in files_yaml {
            file_configs.push(parse_file_config(file_yaml));
          }
          Some(file_configs)
        } else {
          None
        },
        fallback_prefetch: if let Some(fp_yaml) = crate_yaml
          .get("fallback_prefetch")
          .and_then(|f| f.as_sequence())
        {
          let mut fp_configs = Vec::new();
          for prefetch_yaml in fp_yaml {
            fp_configs.push(parse_prefetch_config(prefetch_yaml));
          }
          Some(fp_configs)
        } else {
          None
        },
        cleanup: crate_yaml.get("cleanup").cloned(),
        set: None,       // TODO: parse set if needed
        condition: None, // TODO: parse condition if needed
      };
      crates.push(crate_config);
    }
    app_config.crates = Some(crates);
  }

  // Parse cmods
  if let Some(cmods_yaml) = config_yaml.get("cmods").and_then(|c| c.as_sequence()) {
    let mut cmods = Vec::new();
    for cmod_yaml in cmods_yaml {
      let flags = if let Some(flags_val) = cmod_yaml.get("flags") {
        match flags_val {
          Value::String(s) => Some(vec![s.clone()]),
          Value::Sequence(seq) => Some(
            seq
              .iter()
              .filter_map(|v| v.as_str())
              .map(|s| s.to_string())
              .collect(),
          ),
          _ => None,
        }
      } else {
        None
      };

      let cmod_config = CModConfig {
        name: cmod_yaml
          .get("name")
          .and_then(|n| n.as_str())
          .unwrap_or("")
          .to_string(),
        path: cmod_yaml
          .get("path")
          .and_then(|p| p.as_str())
          .unwrap_or("")
          .to_string(),
        build: cmod_yaml.get("build").and_then(|b| b.as_bool()),
        compiler: cmod_yaml
          .get("compiler")
          .and_then(|c| c.as_str())
          .map(|s| s.to_string()),
        flags,
        files: if let Some(files_yaml) = cmod_yaml.get("files").and_then(|f| f.as_sequence()) {
          let mut file_configs = Vec::new();
          for file_yaml in files_yaml {
            file_configs.push(parse_file_config(file_yaml));
          }
          Some(file_configs)
        } else {
          None
        },
        fallback_prefetch: if let Some(fp_yaml) = cmod_yaml
          .get("fallback_prefetch")
          .and_then(|f| f.as_sequence())
        {
          let mut fp_configs = Vec::new();
          for prefetch_yaml in fp_yaml {
            fp_configs.push(parse_prefetch_config(prefetch_yaml));
          }
          Some(fp_configs)
        } else {
          None
        }, // TODO: parse fallback_prefetch if needed
        cleanup: cmod_yaml.get("cleanup").cloned(),
        set: None,       // TODO: parse set if needed
        condition: None, // TODO: parse condition if needed
      };
      cmods.push(cmod_config);
    }
    app_config.cmods = Some(cmods);
  }

  // Parse prefetch
  if let Some(prefetch_yaml) = config_yaml.get("prefetch").and_then(|p| p.as_sequence()) {
    let mut prefetch_configs = Vec::new();
    for prefetch in prefetch_yaml {
      let prefetch_config = PrefetchConfig {
        url: prefetch
          .get("url")
          .and_then(|u| u.as_str())
          .unwrap_or("")
          .to_string(),
        output: prefetch
          .get("output")
          .and_then(|o| o.as_str())
          .unwrap_or("")
          .to_string(),
        extract: prefetch
          .get("extract")
          .and_then(|e| e.as_str())
          .map(|s| s.to_string()),
        system: prefetch
          .get("system")
          .and_then(|s| s.as_str())
          .map(|s| s.to_string()),
        build: prefetch.get("build").and_then(|b| b.as_bool()),
        condition: None, // TODO: parse condition if needed
      };
      prefetch_configs.push(prefetch_config);
    }
    app_config.prefetch = Some(prefetch_configs);
  }

  // Parse build
  if let Some(build_yaml) = config_yaml.get("build").and_then(|b| b.as_sequence()) {
    let mut build_configs = Vec::new();
    for build in build_yaml {
      let build_config = BuildConfig {
        input: build
          .get("input")
          .and_then(|i| i.as_str())
          .unwrap_or("")
          .to_string(),
        output: build
          .get("output")
          .and_then(|o| o.as_str())
          .unwrap_or("")
          .to_string(),
        using: build
          .get("using")
          .and_then(|u| u.as_str())
          .map(|s| s.to_string()),
        cleanup: build.get("cleanup").cloned(),
        args: build
          .get("args")
          .and_then(|u| u.as_str())
          .map(|s| s.to_string())
          .or(None),
        set: None,       // TODO: parse set if needed
        condition: None, // TODO: parse condition if needed
        id: build
          .get("id")
          .and_then(|i| i.as_str())
          .map(|s| s.to_string()),
      };
      build_configs.push(build_config);
    }
    app_config.build = Some(build_configs);
  }

  Ok(app_config)
}

fn is_cargo_available() -> bool {
  Command::new("cargo").arg("--version").output().is_ok()
}

fn build_files(files: &[FileConfig], root_path: PathBuf, app_path: &Path, safe: bool) {
  for file in files {
    if file.system.is_none() || (file.system.as_deref() == Some(std::env::consts::OS)) {
      let input_path = root_path.join(&file.input);
      let output_path = app_path.join(&file.output);

      if input_path.exists() {
        if let Some(parent) = output_path.parent() {
          let _ = fs::create_dir_all(parent);
        }
        if let Err(e) = fs::copy(&input_path, &output_path) {
          logger::error(&format!("Failed to copy file: {}", e));
        } else {
          logger::info(&format!("Copied {} -> {}", file.input, file.output));
        }

        if let Some(cleanup) = &file.cleanup {
          if !safe {
            let cleanup_path = app_path.join(cleanup);
            let _ = fs::remove_dir_all(&cleanup_path);
            logger::info(&format!("Cleaned up: {}", cleanup));
          }
        }
      } else {
        logger::error(&format!("File doesn't exist: {}", input_path.display()));
      }
    }
  }
}

fn build_cmod(cmod_config: &CModConfig, app_path: &Path, safe: bool) -> bool {
  if !cmod_config.build.unwrap_or(false) {
    return true;
  }

  let cmod_path = app_path.join(&cmod_config.path);
  logger::info(&format!(" Building cmod {}", cmod_config.name));

  if safe {
    logger::info(&format!(
      "[SAFE MODE] Would build cmod at: {}",
      cmod_path.display()
    ));
    return true;
  }

  let compiler = cmod_config.compiler.as_deref().unwrap_or("cc");
  let mut command = format!("{} ", compiler);

  if let Some(flags) = &cmod_config.flags {
    command.push_str(&flags.join(" "));
  }

  let result = run_command(&cmod_path, &command, safe);

  match result {
    Ok(_) => {
      logger::info(&format!("Built Cmod {}", cmod_config.name));

      if let Some(files) = &cmod_config.files {
        build_files(files, cmod_path, app_path, safe);
      }

      if let Some(cleanup) = &cmod_config.cleanup {
        if !safe {
          match cleanup {
            Value::String(path) => {
              let cleanup_path = app_path.join(path);
              let _ = fs::remove_dir_all(&cleanup_path);
              logger::info(&format!("Clean Up {}", cmod_config.name));
            }
            Value::Sequence(paths) => {
              for path_val in paths {
                if let Some(path) = path_val.as_str() {
                  let cleanup_path = app_path.join(path);
                  let _ = fs::remove_dir_all(&cleanup_path);
                }
              }
              logger::info(&format!("Clean Up {}", cmod_config.name));
            }
            _ => {}
          }
        }
      }

      true
    }
    Err(e) => {
      logger::error(&format!("Failed to build cmod: {}", e));
      false
    }
  }
}

fn build_crate(crate_config: &CrateConfig, app_path: &Path, safe: bool) -> bool {
  if !crate_config.build.unwrap_or(false) {
    return true;
  }

  let crate_path = app_path.join(&crate_config.path);
  logger::info(&format!(" Building crate {}", crate_config.name));

  if safe {
    logger::info(&format!(
      "[SAFE MODE] Would build crate at: {}",
      crate_path.display()
    ));
    return true;
  }

  let result = Command::new("cargo")
    .arg("build")
    .arg("--release")
    .current_dir(&crate_path)
    .output();

  match result {
    Ok(output) if output.status.success() => {
      logger::info(&format!("Built Crate {}", crate_config.name));

      if let Some(files) = &crate_config.files {
        build_files(files, crate_path, app_path, safe);
      }

      if let Some(cleanup) = &crate_config.cleanup {
        if !safe {
          match cleanup {
            Value::String(path) => {
              let cleanup_path = app_path.join(path);
              let _ = fs::remove_dir_all(&cleanup_path);
              logger::info(&format!("Clean Up {}", crate_config.name));
            }
            Value::Sequence(paths) => {
              for path_val in paths {
                if let Some(path) = path_val.as_str() {
                  let cleanup_path = app_path.join(path);
                  let _ = fs::remove_dir_all(&cleanup_path);
                }
              }
              logger::info(&format!("Clean Up {}", crate_config.name));
            }
            _ => {}
          }
        }
      }

      true
    }
    Ok(output) => {
      logger::error("Failed to build cargo");
      let stderr = String::from_utf8_lossy(&output.stderr);
      if !stderr.is_empty() {
        logger::error(&stderr);
      }
      false
    }
    Err(e) => {
      logger::error(&format!("Failed to execute cargo: {}", e));
      false
    }
  }
}

pub fn install_native_deps(
  native: &Value,
  app_path: &Path,
  safe: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  if safe {
    logger::info("[SAFE MODE] Would install native dependencies");
    return Ok(());
  }

  let os_slug = std::env::consts::OS;
  let deps_val = native
    .get(os_slug)
    .or_else(|| native.get(get_os_family(os_slug)));

  let Some(deps_val) = deps_val else {
    logger::info("No native dependencies for this OS");
    return Ok(());
  };

  let deps: Vec<NativeDep> = match serde_yaml::from_value(deps_val.clone()) {
    Ok(d) => d,
    Err(e) => {
      return Err(format!("Failed to parse native deps: {}", e).into());
    }
  };

  for dep in deps {
    logger::info(&format!(
      "Installing native dependency: {}",
      dep.name.as_deref().unwrap_or("?")
    ));

    let mut installed = false;

    if let Some(check_cmd) = &dep.check {
      if run_command(app_path, check_cmd, false).is_ok() {
        logger::info("Already installed");
        continue;
      }
    }

    if let Some(preinstall_cmds) = &dep.preinstall {
      for cmd in preinstall_cmds {
        let _ = run_command(app_path, &cmd.shell, false);
      }
    }

    match dep.type_.as_deref() {
      Some("shell") => {
        if let Some(content) = &dep.content {
          logger::info(&format!(
            "Running custom command for {}",
            dep.name.as_deref().unwrap_or("?")
          ));
          if run_command(app_path, content, false).is_ok() {
            installed = true;
          }
        }
      }
      Some("installer") => {
        if let Some(url) = &dep.url {
          logger::info(&format!(
            "Downloading installer for {}",
            dep.name.as_deref().unwrap_or("?")
          ));

          let file_name = url.split('/').last().unwrap_or("installer");
          let temp_dir = std::env::temp_dir();
          let dest_path = temp_dir.join(file_name);

          if let Ok(response) = reqwest::blocking::get(url) {
            if let Ok(bytes) = response.bytes() {
              if fs::write(&dest_path, &bytes).is_ok() {
                logger::info(&format!("Downloaded installer to {}", dest_path.display()));
                if run_command(&temp_dir, &dest_path.to_string_lossy(), false).is_ok() {
                  installed = true;
                }
              }
            }
          }
        }
      }
      Some("package") => {
        if let Some(managers) = &dep.managers {
          for (mgr, pkg) in managers {
            if is_command_available(mgr) {
              logger::info(&format!(
                "Installing {} using {}",
                dep.name.as_deref().unwrap_or("?"),
                mgr
              ));
              let cmd = get_install_command(mgr, pkg);
              if let Ok(_) = run_command(app_path, &cmd, false) {
                installed = true;
                break;
              }
            }
          }
        }
      }
      _ => {}
    }

    if !installed {
      if let Some(fallback) = &dep.fallback {
        if let Ok(fallback_dep) = serde_yaml::from_value(fallback.clone()) {
          let fb: NativeDep = fallback_dep;
          if let Some(content) = &fb.content {
            if run_command(app_path, content, false).is_ok() {
              installed = true;
            }
          }
        }
      }
    }

    if installed {
      if let Some(postinstall_cmds) = &dep.postinstall {
        for cmd in postinstall_cmds {
          let _ = run_command(app_path, &cmd.shell, safe);
        }
      }
      logger::info(&format!(
        "Successfully installed {}",
        dep.name.as_deref().unwrap_or("?")
      ));
    } else {
      logger::error(&format!(
        "Failed to install native dependency: {}",
        dep.name.as_deref().unwrap_or("?")
      ));
    }
  }

  Ok(())
}

fn get_os_family(os: &str) -> &str {
  match os {
    "windows" => "windows",
    "macos" => "unix",
    "linux" => "unix",
    _ => "unix",
  }
}

fn get_install_command(manager: &str, pkg: &str) -> String {
  match manager {
    "apt" | "apt-get" => format!("sudo apt-get install -y {}", pkg),
    "pacman" => format!("sudo pacman -S --noconfirm {}", pkg),
    "dnf" => format!("sudo dnf install -y {}", pkg),
    "zypper" => format!("sudo zypper install -y {}", pkg),
    "brew" => format!("brew install {}", pkg),
    "winget" => format!(
      "winget install --id={} -e --accept-source-agreements --accept-package-agreements",
      pkg
    ),
    "chocolatey" => format!("choco install {} -y", pkg),
    _ => "".to_string(),
  }
}

fn is_command_available(command: &str) -> bool {
  let check_command = if std::env::consts::OS == "windows" {
    "where"
  } else {
    "which"
  };
  Command::new(check_command)
    .arg(command)
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
}

async fn handle_prefetch(
  prefetch: &PrefetchConfig,
  app_path: &Path,
  safe: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  let output_path = app_path.join(&prefetch.output);

  logger::info(&format!(
    "Prefetching: {} -> {}",
    prefetch.url, prefetch.output
  ));

  if output_path.exists() {
    logger::info(&format!("File already exists: {}", prefetch.output));
    return Ok(());
  }

  if let Some(parent) = output_path.parent() {
    fs::create_dir_all(parent)?;
  }

  let response = reqwest::blocking::get(&prefetch.url)?;
  let content = response.bytes()?;
  fs::write(&output_path, &content)?;

  logger::info(&format!("Downloaded: {}", prefetch.output));

  if let Some(extract_path) = &prefetch.extract {
    let extract_dir = app_path.join(extract_path);
    logger::info(&format!("Extracting to: {}", extract_path));

    fs::create_dir_all(&extract_dir)?;
    logger::info(&format!("Extracted to: {}", extract_path));

    if prefetch.build.unwrap_or(false) {
      logger::info(&format!("Building extracted content in: {}", extract_path));
      // Recursive build call
      build_rew_app(&extract_dir, safe).await.await?;
    }
  }

  Ok(())
}

async fn handle_build_step(
  build_config: &BuildConfig,
  app_path: &Path,
  safe: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  let input_path = app_path.join(&build_config.input);
  let output_path = app_path.join(&build_config.output);

  if !input_path.exists() {
    return Err(format!("Input file {} not found", build_config.input).into());
  }

  logger::info(&format!(
    "Building: {} -> {}",
    build_config.input, build_config.output
  ));

  if let Some(parent) = output_path.parent() {
    fs::create_dir_all(parent)?;
  }

  // Handle different builders
  match build_config.using.as_deref() {
    Some("brew") => {
      logger::info("Building with brew compiler");
      let mut runtime = RewRuntime::new(None, None)?;
      let output_string = runtime
        .build_file(
          input_path,
          rew_core::BuildOptions {
            bundle_all: false,
            entry_file: None,
          },
        )
        .await?;

      fs::write(output_path, output_string.clone())?;
      logger::info("Brew compilation complete");
    }
    Some("qrew") => {
      logger::info("Building with qrew compiler");
      // TODO: Implement qrew compilation
      logger::warn("Qrew compilation not implemented yet");
    }
    Some("make") => {
      logger::info("Building with make");
      let working_dir = input_path.parent().unwrap_or(app_path);
      run_command(
        working_dir,
        &format!("make -f {}", input_path.display()),
        safe,
      )?;
    }
    Some("rustc") => {
      logger::info("Building with rustc");
      let working_dir = input_path.parent().unwrap_or(app_path);
      run_command(
        working_dir,
        &format!(
          "rustc {} -o {}",
          input_path.display(),
          output_path.display()
        ),
        safe,
      )?;
    }
    Some("cc" | "clang" | "gcc") => {
      logger::info(&format!(
        "Building with {}",
        build_config.using.as_deref().unwrap()
      ));
      let working_dir = input_path.parent().unwrap_or(app_path);
      run_command(
        working_dir,
        &format!(
          "{} {} -o {}",
          build_config.using.as_deref().unwrap(),
          input_path.display(),
          output_path.display()
        ),
        safe,
      )?;
    }
    Some("zig") => {
      logger::info("Building with zig");
      let working_dir = input_path.parent().unwrap_or(app_path);
      run_command(
        working_dir,
        &format!(
          "zig build-exe {} -o {}",
          input_path.display(),
          output_path.display()
        ),
        safe,
      )?;
    }
    Some(builder) => {
      return Err(format!("Builder {} does not exist", builder).into());
    }
    _ => {
      // Default behavior - just copy
      if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
      }
      fs::copy(&input_path, &output_path)?;
      logger::info(&format!(
        "Copied {} -> {}",
        build_config.input, build_config.output
      ));
    }
  }

  // Handle cleanup
  if let Some(cleanup) = &build_config.cleanup {
    if !safe {
      match cleanup {
        Value::String(path) => {
          let cleanup_path = app_path.join(path);
          let _ = fs::remove_file(&cleanup_path).or_else(|_| fs::remove_dir_all(&cleanup_path));
          logger::info(&format!("Cleaned up: {}", path));
        }
        Value::Sequence(paths) => {
          for path_val in paths {
            if let Some(path) = path_val.as_str() {
              let cleanup_path = app_path.join(path);
              let _ = fs::remove_file(&cleanup_path).or_else(|_| fs::remove_dir_all(&cleanup_path));
            }
          }
          logger::info("File Cleanup");
        }
        _ => {}
      }
    }
  }

  Ok(())
}

fn run_command(
  working_dir: &std::path::Path,
  command: &str,
  safe: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  if safe {
    logger::info(&format!("{}", "[SAFE MODE] Command run halted.".yellow()));
    return Ok(());
  }

  for line in command.lines() {
    let trimmed = line.trim();
    if trimmed.is_empty() {
      continue;
    }

    for cmd_str in trimmed.split(';') {
      let cmd_str = cmd_str.trim();
      if cmd_str.is_empty() {
        continue;
      }

      let parts: Vec<&str> = cmd_str.split_whitespace().collect();
      if parts.is_empty() {
        return Err("Empty command".into());
      }

      let command_name = parts[0];
      let mut cmd = Command::new(command_name);
      cmd.current_dir(working_dir);

      if parts.len() > 1 {
        cmd.args(&parts[1..]);
      }

      let output = cmd.output()?;

      if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        if !stdout.is_empty() {
          logger::info(&format!("{}", stdout));
        }
        if !stderr.is_empty() {
          logger::error(&format!("{}", stderr));
        }

        return Err(
          format!(
            "Command `{}` failed with exit code: {:?}",
            cmd_str,
            output.status.code()
          )
          .into(),
        );
      } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if command_name == "echo" {
          logger::info(&format!("{}", stdout.trim()));
        }
      }
    }
  }

  Ok(())
}
