use crate::builder::{install_native_deps, parse_app_config};
use crate::logger;
use crate::repo::Package;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use rew_core::utils;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedApp {
  pub name: String,
  pub version: String,
  pub repo: String,
  pub url: String,
  pub install_path: PathBuf,
  pub installed_at: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppCache {
  pub apps: BTreeMap<String, CachedApp>,
}

fn get_cache_file_path() -> Option<PathBuf> {
  let pimmy_data_path = utils::pimmy_data_path()?;
  Some(
    pimmy_data_path
      .join("cache")
      .join("app-cache")
      .join("installed.json"),
  )
}

pub fn load_app_cache() -> AppCache {
  let Some(cache_file) = get_cache_file_path() else {
    return AppCache::default();
  };

  if !cache_file.exists() {
    return AppCache::default();
  }

  match fs::read_to_string(&cache_file) {
    Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
      logger::warn(&format!("Failed to parse cache file: {}", e));
      AppCache::default()
    }),
    Err(e) => {
      logger::warn(&format!("Failed to read cache file: {}", e));
      AppCache::default()
    }
  }
}

fn save_app_cache(cache: &AppCache) -> Result<(), Box<dyn std::error::Error>> {
  let Some(cache_file) = get_cache_file_path() else {
    return Err("Could not determine cache file path".into());
  };

  if let Some(parent) = cache_file.parent() {
    fs::create_dir_all(parent)?;
  }

  let json = serde_json::to_string_pretty(cache)?;
  fs::write(&cache_file, json)?;
  Ok(())
}

fn find_package_in_repos(app_name: &str) -> Option<Package> {
  let Some(pimmy_data_path) = utils::pimmy_data_path() else {
    return None;
  };
  let repo_main_yaml_path = pimmy_data_path.join("repo").join("main.yaml");

  let repos_file = fs::File::open(&repo_main_yaml_path).ok()?;
  let repos: BTreeMap<String, String> = serde_yaml::from_reader(repos_file).ok()?;
  let repo_cache_path = pimmy_data_path.join("cache").join("repo-cache");

  for (i, _) in repos.iter().enumerate() {
    let cache_file = repo_cache_path.join(format!("db_{}.bin", i));
    if !cache_file.exists() {
      continue;
    }

    if let Ok(content) = fs::read_to_string(&cache_file) {
      if let Ok(packages) = serde_json::from_str::<Vec<Package>>(&content) {
        for package in packages {
          if package.name == app_name {
            return Some(package);
          }
        }
      }
    }
  }

  None
}

// URL pattern: file+<archiver>[+sha(<hash>)]+<url>
struct UrlPattern {
  url: String,
  archiver: String,
  sha: Option<String>,
}

fn parse_url_pattern(input: &str) -> Result<UrlPattern, String> {
  let re = Regex::new(r"^file\+([a-zA-Z0-9._]+)(?:\+sha\(([a-fA-F0-9]+)\))?\+(.+)$")
    .map_err(|e| format!("Regex error: {}", e))?;

  if let Some(captures) = re.captures(input) {
    Ok(UrlPattern {
      archiver: captures.get(1).unwrap().as_str().to_string(),
      sha: captures.get(2).map(|m| m.as_str().to_string()),
      url: captures.get(3).unwrap().as_str().to_string(),
    })
  } else {
    Err("Invalid URL pattern format".to_string())
  }
}

fn resolve_github_url(
  github_url: &str,
) -> Result<(String, String, Option<String>, Option<String>), String> {
  // Parse github:user/repo[@branch][#commit]
  let parts: Vec<&str> = github_url.split(':').collect();
  if parts.len() != 2 || parts[0] != "github" {
    return Err("Invalid GitHub URL format".to_string());
  }

  let mut repo_part = parts[1].to_string();
  let mut branch = None;
  let mut commit = None;

  // Extract commit if present
  if let Some(hash_pos) = repo_part.find('#') {
    commit = Some(repo_part[hash_pos + 1..].to_string());
    repo_part = repo_part[..hash_pos].to_string();
  }

  // Extract branch if present
  if let Some(at_pos) = repo_part.find('@') {
    branch = Some(repo_part[at_pos + 1..].to_string());
    repo_part = repo_part[..at_pos].to_string();
  }

  let repo_parts: Vec<&str> = repo_part.split('/').collect();
  if repo_parts.len() != 2 {
    return Err("Invalid GitHub repo format".to_string());
  }

  Ok((
    repo_parts[0].to_string(),
    repo_parts[1].to_string(),
    branch,
    commit,
  ))
}

fn get_cache_path() -> Option<PathBuf> {
  utils::pimmy_data_path().map(|p| p.join("cache").join("app-cache"))
}

fn generate_cache_id(input: &str) -> String {
  use std::collections::hash_map::DefaultHasher;
  use std::hash::{Hash, Hasher};

  let mut hasher = DefaultHasher::new();
  input.hash(&mut hasher);
  let hash = hasher.finish();

  // Generate a 24-character ID similar to the CoffeeScript genUid
  format!("{:016x}{:08x}", hash, input.len())
}

fn generate_id_for_existing(app_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
  let app_yaml_path = app_path.join("app.yaml");
  if !app_yaml_path.exists() {
    return Err("app.yaml not found".into());
  }

  let content = fs::read_to_string(&app_yaml_path)?;
  let yaml: Value = serde_yaml::from_str(&content)?;

  let package = yaml
    .get("manifest")
    .and_then(|m| m.get("package"))
    .and_then(|p| p.as_str())
    .unwrap_or("unknown");

  let version = yaml
    .get("manifest")
    .and_then(|m| m.get("version"))
    .and_then(|v| v.as_str())
    .unwrap_or("");

  let input = format!("{}{}", package, version);
  Ok(generate_cache_id(&input) + package)
}

pub fn list(app_name: Option<&str>) {
  let cache = load_app_cache();

  if let Some(app_name) = app_name {
    if let Some(cached_app) = cache.apps.get(app_name) {
      println!(
        " {0: <20} | {1: <10} | {2: <15} | {3}",
        "Name", "Version", "Repo", "Install Path"
      );
      println!("{:-<20}-+-{:-<10}-+-{:-<15}-+-{:-<50}", "", "", "", "");
      println!(
        " {0: <20} | {1: <10} | {2: <15} | {3}",
        cached_app.name.green(),
        cached_app.version.blue(),
        cached_app.repo.yellow(),
        cached_app.install_path.display()
      );
    } else {
      logger::warn(&format!("App '{}' is not installed", app_name));
    }
  } else {
    if cache.apps.is_empty() {
      logger::info("No apps installed");
      return;
    }

    logger::info("Listing all cached apps:");
    println!(
      " {0: <20} | {1: <10} | {2: <15} | {3}",
      "Name", "Version", "Repo", "Installed At"
    );
    println!("{:-<20}-+-{:-<10}-+-{:-<15}-+-{:-<25}", "", "", "", "");

    for cached_app in cache.apps.values() {
      let installed_date = cached_app
        .installed_at
        .parse::<chrono::DateTime<chrono::Utc>>()
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|_| cached_app.installed_at.clone());

      println!(
        " {0: <20} | {1: <10} | {2: <15} | {3}",
        cached_app.name.green(),
        cached_app.version.blue(),
        cached_app.repo.yellow(),
        installed_date
      );
    }
  }
}

pub fn list_installed() {
  list(None);
}

pub fn remove_app_impl(app_name: &str, ignore: bool) {
  let mut cache = load_app_cache();

  if let Some(cached_app) = cache.apps.get(app_name) {
    let install_path = &cached_app.install_path;

    // Ask for confirmation unless ignore is true
    if !ignore {
      print!(
        "Are you sure you want to remove '{}' from {}? [y/N]: ",
        app_name,
        install_path.display()
      );
      std::io::stdout().flush().unwrap();

      let mut input = String::new();
      if std::io::stdin().read_line(&mut input).is_ok() {
        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
          logger::info("Removal cancelled");
          return;
        }
      } else {
        logger::error("Failed to read input");
        return;
      }
    }

    // Remove the installation directory
    if install_path.exists() {
      match fs::remove_dir_all(install_path) {
        Ok(_) => logger::info(&format!(
          "Removed installation directory: {}",
          install_path.display()
        )),
        Err(e) => {
          logger::error(&format!(
            "Failed to remove directory {}: {}",
            install_path.display(),
            e
          ));
          return;
        }
      }
    } else {
      logger::warn(&format!(
        "Installation directory {} does not exist",
        install_path.display()
      ));
    }

    // Remove from cache
    cache.apps.remove(app_name);

    if let Err(e) = save_app_cache(&cache) {
      logger::error(&format!("Failed to update cache: {}", e));
      return;
    }

    logger::info(&format!("Successfully removed '{}'", app_name));
  } else {
    logger::warn(&format!("App '{}' is not installed", app_name));
  }
}

pub fn list_directory_contents(path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
  let mut files = Vec::new();

  fn collect_files(
    dir: &PathBuf,
    files: &mut Vec<PathBuf>,
    base_path: &PathBuf,
  ) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();

      if path.is_dir() {
        collect_files(&path, files, base_path)?;
      } else {
        if let Ok(relative_path) = path.strip_prefix(base_path) {
          files.push(relative_path.to_path_buf());
        }
      }
    }
    Ok(())
  }

  collect_files(path, &mut files, path)?;
  files.sort();
  Ok(files)
}

pub async fn resolve_cache_entry(
  key: &str,
  update: bool,
  is_recursed: bool,
  silent: bool,
  cache: bool,
) -> Option<PathBuf> {
  if !is_recursed {
    logger::info(&format!("Resolve cache entry {}", key));
  }

  let Some(cache_path) = get_cache_path() else {
    if !silent {
      logger::error("Could not determine cache path");
    }
    return None;
  };

  let app_path = PathBuf::from(key);
  if app_path.exists() {
    match resolve_local_path(&app_path, &cache_path, silent, cache).await {
      Ok(path) => {
        if !silent {
          logger::info("Cache resolved");
        }
        return Some(path);
      }
      Err(e) => {
        if !silent {
          logger::error(&format!("Failed to resolve local path: {}", e));
        }
        return None;
      }
    }
  }

  if let Ok(url_pattern) = parse_url_pattern(key) {
    match resolve_url_pattern(&url_pattern, &cache_path, update, silent).await {
      Ok(path) => {
        if !silent {
          logger::info("Cache resolved");
        }
        return Some(path);
      }
      Err(e) => {
        if !silent {
          logger::error(&format!("Failed to resolve URL pattern: {}", e));
        }
        return None;
      }
    }
  }

  if key.starts_with("github:") {
    match resolve_github_entry(key, &cache_path, update, silent).await {
      Ok(path) => {
        if !silent {
          logger::info("Cache resolved");
        }
        return Some(path);
      }
      Err(e) => {
        if !silent {
          logger::error(&format!("Failed to resolve GitHub entry: {}", e));
        }
        return None;
      }
    }
  }

  // Check if key is in repo cache
  if let Some(package) = find_package_in_repos(key) {
    // Recursively resolve the package URL
    return Box::pin(resolve_cache_entry(
      &package.url,
      update,
      true,
      silent,
      cache,
    ))
    .await;
  }

  if !silent {
    logger::error(&format!("Couldn't resolve to cache entry {}", key));
  }
  None
}

async fn resolve_local_path(
  app_path: &Path,
  cache_path: &Path,
  silent: bool,
  cache: bool,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
  if !cache {
    return Ok(PathBuf::from(app_path));
  }
  let cache_id = generate_id_for_existing(app_path)?;
  let cache_entry_path = cache_path.join(&cache_id);

  // Remove existing cache entry if it exists
  if cache_entry_path.exists() {
    fs::remove_dir_all(&cache_entry_path)?;
  }

  // Copy app to cache
  copy_directory(app_path, &cache_entry_path)?;

  // Validate app.yaml exists
  let app_yaml_path = cache_entry_path.join("app.yaml");
  if !app_yaml_path.exists() {
    if !silent {
      logger::error(
        "Not a compatible rew app, seed file app.yaml could not be found. A bare minimum of a manifest with a package name is required for a rew app to be cached and processed",
      );
    }
    return Err("app.yaml not found".into());
  }

  match build_cache_entry(cache_entry_path.clone()).await {
    Ok(_) => {}
    Err(_) => {
      return Err("Build error occured".into());
    }
  };

  Ok(cache_entry_path)
}

async fn resolve_url_pattern(
  url_pattern: &UrlPattern,
  cache_path: &Path,
  update: bool,
  silent: bool,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
  let cache_id = generate_cache_id(&url_pattern.url);
  let cache_entry_path = cache_path.join(&cache_id);

  if !silent {
    logger::info("Found URL entry");
    logger::info(&format!(
      "Downloading URL entry {} as cache entry {}",
      url_pattern.url, cache_id
    ));
  }

  fs::create_dir_all(&cache_entry_path)?;

  let cache_file =
    cache_entry_path.join(format!("entry.{}", url_pattern.archiver.replace("_", ".")));

  // Download file if needed
  if !update && cache_file.exists() {
    if let Some(expected_sha) = &url_pattern.sha {
      let file_sha = calculate_sha256(&cache_file)?;
      if file_sha != *expected_sha {
        download_file(&url_pattern.url, &cache_file, silent).await?;
      } else if !silent {
        logger::info("Found Cache skipping Download");
      }
    } else if !silent {
      logger::info("Found Cache skipping Download");
    }
  } else {
    download_file(&url_pattern.url, &cache_file, silent).await?;
  }

  // Verify SHA if provided
  if let Some(expected_sha) = &url_pattern.sha {
    let file_sha = calculate_sha256(&cache_file)?;
    if file_sha != *expected_sha {
      return Err(
        format!(
          "SHA unmatched.\nExpected: {}\nFound: {}\n",
          expected_sha, file_sha
        )
        .into(),
      );
    }
  }

  let unarchive_path = cache_entry_path.join("_out");

  match build_cache_entry(unarchive_path.clone()).await {
    Ok(pass) => {
      if pass {
        return Ok(unarchive_path);
      }
    }
    Err(_) => {
      return Err("Build error occured".into());
    }
  };

  fs::create_dir_all(&unarchive_path)?;

  // Extract archive
  unarchive(&url_pattern.archiver, &cache_file, &unarchive_path)?;

  Ok(unarchive_path)
}

async fn build_cache_entry(cache_entry_path: PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
  let built_path = cache_entry_path.join(".built");

  if built_path.exists() {
    return Ok(true);
  }

  let app_yaml_path = cache_entry_path.join("app.yaml");
  if !app_yaml_path.exists() {
    return Err("app.yaml not found".into());
  }

  let config_content = fs::read_to_string(&app_yaml_path)?;
  let config: Value = serde_yaml::from_str(&config_content)?;

  if let Some(install) = config.get("install") {
    if install
      .get("build")
      .and_then(|b| b.as_bool())
      .unwrap_or(false)
    {
      crate::builder::build_rew_app(&cache_entry_path, false)
        .await
        .await?;
    }

    if let Some(cleanup) = install.get("cleanup") {
      match cleanup {
        Value::String(path) => {
          let cleanup_path = cache_entry_path.join(path);
          if cleanup_path.exists() {
            if cleanup_path.is_dir() {
              fs::remove_dir_all(&cleanup_path)?;
            } else {
              fs::remove_file(&cleanup_path)?;
            }
          }
        }
        Value::Sequence(paths) => {
          for path_val in paths {
            if let Some(path) = path_val.as_str() {
              let cleanup_path = cache_entry_path.join(path);
              if cleanup_path.exists() {
                if cleanup_path.is_dir() {
                  fs::remove_dir_all(&cleanup_path)?;
                } else {
                  fs::remove_file(&cleanup_path)?;
                }
              }
            }
          }
        }
        _ => {}
      }
    }
  }

  fs::write(&built_path, "")?;

  Ok(false)
}

async fn resolve_github_entry(
  github_url: &str,
  cache_path: &Path,
  _update: bool,
  silent: bool,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
  let cache_id = generate_cache_id(github_url);
  let cache_entry_path = cache_path.join(&cache_id);

  let (user, repo, branch, commit) =
    resolve_github_url(github_url).map_err(|e| format!("Failed to parse GitHub URL: {}", e))?;

  let home_url = format!("https://github.com/{}/{}.git", user, repo);

  if !silent {
    logger::info("Found GIT entry");
    logger::info(&format!(
      "Cloning repo {} as cache entry {}",
      home_url, cache_id
    ));
  }

  if cache_entry_path.exists() {
    if _update {
      fs::remove_dir_all(&cache_entry_path)?;
    }
  }

  // Clone the repository
  let output = Command::new("git")
    .args(&["clone", &home_url, &cache_entry_path.to_string_lossy()])
    .output()?;

  if !output.status.success() {
    return Err(
      format!(
        "Git clone failed: {}",
        String::from_utf8_lossy(&output.stderr)
      )
      .into(),
    );
  }

  // Checkout branch if specified
  if let Some(branch_name) = branch {
    let output = Command::new("git")
      .args(&["checkout", &branch_name])
      .current_dir(&cache_entry_path)
      .output()?;

    if !output.status.success() {
      return Err(
        format!(
          "Git checkout failed: {}",
          String::from_utf8_lossy(&output.stderr)
        )
        .into(),
      );
    }
  }

  // Reset to commit if specified
  if let Some(commit_hash) = commit {
    let output = Command::new("git")
      .args(&["reset", "--hard", &commit_hash])
      .current_dir(&cache_entry_path)
      .output()?;

    if !output.status.success() {
      return Err(
        format!(
          "Git reset failed: {}",
          String::from_utf8_lossy(&output.stderr)
        )
        .into(),
      );
    }
  }

  match build_cache_entry(cache_entry_path.clone()).await {
    Ok(pass) => {
      if pass {
        return Ok(cache_entry_path);
      }
    }
    Err(_) => {
      return Err("Build error occured".into());
    }
  };

  Ok(cache_entry_path)
}

pub async fn resolve(app_name: &str) -> Option<String> {
  let cache = load_app_cache();

  if let Some(cached_app) = cache.apps.get(app_name) {
    // Check if installation still exists
    if cached_app.install_path.exists() {
      logger::info(&format!(
        "Resolved '{}' to cached installation: {}",
        app_name,
        cached_app.install_path.display()
      ));
      return Some(cached_app.install_path.to_string_lossy().to_string());
    } else {
      logger::warn(&format!(
        "Cached app '{}' installation path no longer exists: {}",
        app_name,
        cached_app.install_path.display()
      ));
    }
  }

  logger::warn(&format!("App '{}' not found in cache", app_name));
  None
}

pub async fn install_from(app_path: PathBuf, sync: Option<bool>, ignore_deps: bool) {
  let mut cache = load_app_cache();

  let app_yaml_path = app_path.join("app.yaml");
  if !app_yaml_path.exists() {
    logger::error("app.yaml not found".into());
    return;
  }

  let content = match fs::read_to_string(&app_yaml_path) {
    Ok(s) => s,
    Err(e) => {
      logger::error(&format!(
        "Unable to read app.yaml at {}: {}",
        app_yaml_path.display(),
        e
      ));
      return;
    }
  };
  let manifest: Value = match serde_yaml::from_str(&content).ok() {
    Some(s) => s,
    _ => {
      logger::error(&format!(
        "Unable to read app.yaml at {}",
        app_yaml_path.display()
      ));
      return;
    }
  };

  let package = manifest
    .get("manifest")
    .and_then(|m| m.get("package"))
    .and_then(|p| p.as_str())
    .unwrap_or("unknown");

  let version = manifest
    .get("manifest")
    .and_then(|m| m.get("version"))
    .and_then(|v| v.as_str())
    .unwrap_or("");

  let apps_dir = utils::get_rew_root().join("apps");
  if let Err(e) = fs::create_dir_all(&apps_dir) {
    logger::error(&format!("Failed to create paths: {}", e));
  }

  let install_path = apps_dir.join(&package);

  if install_path.exists() {
    if let Some(sync) = sync {
      if !sync {
        logger::error(&format!("App {} already installed.", package));
      }
    }
    if let Err(e) = fs::remove_dir_all(&install_path) {
      logger::error(&format!("Failed to remove existing install: {}", e));
    }
  }

  let mut options = fs_extra::dir::CopyOptions::new();
  options.overwrite = true;
  options.copy_inside = true;
  options.content_only = false;

  match fs_extra::dir::copy(app_path.clone(), install_path.clone(), &options) {
    Ok(_) => {}
    Err(e) => {
      logger::error(&format!("Failed to install: {}", e));
      return;
    }
  };

  if let Ok(app_config) = parse_app_config(&manifest) {
    if let Some(native) = &app_config.native {
      if let Some(on) = native.get("on").and_then(|v| v.as_str()) {
        if on == "install" {
          if let Err(e) = install_native_deps(native, Path::new(&app_path), false) {
            logger::error(&format!("Failed to install native dependencies: {}", e));
            return;
          }
        }
      }
    }
    if let Some(dependencies) = &app_config.dependencies {
      if ignore_deps == false {
        for dep in dependencies {
          if let Some(entry) = Box::pin(resolve_cache_entry(dep, true, true, false, true)).await {
            Box::pin(install_from(entry, sync, false)).await;
          }
        }
      }
    }
  }

  let cached_app = CachedApp {
    name: package.to_string(),
    version: version.to_string(),
    repo: "unknown".to_string(),
    url: "unknown".to_string(),
    install_path,
    installed_at: chrono::Utc::now().to_rfc3339(),
  };

  cache.apps.insert(package.to_string(), cached_app);

  if let Err(e) = save_app_cache(&cache) {
    logger::error(&format!("Failed to save cache: {}", e));
    return;
  }

  logger::info(&format!("Successfully installed '{}'", package));
}

pub async fn install(app_name: &str, sync: Option<bool>) {
  let action = if sync.unwrap_or(false) {
    "Syncing"
  } else {
    "Installing"
  };
  logger::info(&format!("{} app: {}", action, app_name));

  // Check if already installed and sync is false
  let mut cache = load_app_cache();
  if !sync.unwrap_or(false) && cache.apps.contains_key(app_name) {
    logger::warn(&format!("App '{}' is already installed.", app_name));
    return;
  }

  // Find package in repositories
  let Some(package) = find_package_in_repos(app_name) else {
    logger::error(&format!(
      "Package '{}' not found in any repository. Try running repo [repo] --sync first.",
      app_name
    ));
    return;
  };

  logger::info(&format!(
    "Found package: {} v{} from {}",
    package.name, package.version, package.repo
  ));

  // Download and install
  match download_and_install(&package).await {
    Ok(install_path) => {
      let app_yaml_path = install_path.join("app.yaml");
      if !app_yaml_path.exists() {
        logger::error("app.yaml not found".into());
        return;
      }

      let content = match fs::read_to_string(&app_yaml_path) {
        Ok(s) => s,
        Err(e) => {
          logger::error(&format!(
            "Unable to read app.yaml at {}: {}",
            app_yaml_path.display(),
            e
          ));
          return;
        }
      };

      let manifest: Value = match serde_yaml::from_str(&content).ok() {
        Some(s) => s,
        _ => {
          logger::error(&format!(
            "Unable to read app.yaml at {}",
            app_yaml_path.display()
          ));
          return;
        }
      };

      if let Ok(app_config) = parse_app_config(&manifest) {
        if let Some(native) = &app_config.native {
          if let Some(on) = native.get("on").and_then(|v| v.as_str()) {
            if on == "install" {
              logger::info("    Installing native dependencies");
              if let Err(e) = install_native_deps(native, Path::new(&install_path), false) {
                logger::error(&format!("Failed to install native dependencies: {}", e));
                return;
              }
              logger::info("    Dependencies installed");
            }
          }
        }
      }

      let cached_app = CachedApp {
        name: package.name.clone(),
        version: package.version.clone(),
        repo: package.repo.clone(),
        url: package.url.clone(),
        install_path,
        installed_at: chrono::Utc::now().to_rfc3339(),
      };

      cache.apps.insert(package.name.clone(), cached_app);

      if let Err(e) = save_app_cache(&cache) {
        logger::error(&format!("Failed to save cache: {}", e));
        return;
      }

      logger::info(&format!("Successfully installed '{}'", package.name));
    }
    Err(e) => {
      logger::error(&format!("Failed to install '{}': {}", app_name, e));
    }
  }
}

async fn download_and_install(
  package: &Package,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
  let apps_dir = utils::get_rew_root().join("apps");
  fs::create_dir_all(&apps_dir)?;

  let install_path = apps_dir.join(&package.name);

  if install_path.exists() {
    fs::remove_dir_all(&install_path)?;
  }

  if package.url.starts_with("github:") {
    download_github_repo(package, &install_path).await
  } else if package.url.starts_with("http://") || package.url.starts_with("https://") {
    download_http_archive(package, &install_path).await
  } else {
    return Err(format!("Unsupported URL format: {}", package.url).into());
  }
}

async fn download_github_repo(
  package: &Package,
  install_path: &PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
  let (user, repo, _branch, _commit) =
    resolve_github_url(&package.url).map_err(|e| format!("Invalid GitHub URL: {}", e))?;

  let archive_url = format!(
    "https://github.com/{}/{}/archive/refs/heads/{}.zip",
    user,
    repo,
    _branch.unwrap_or("main".to_string())
  );
  logger::info(&format!("Downloading from GitHub: {}/{}", user, repo));

  let client = reqwest::Client::new();
  let response = client.get(&archive_url).send().await?;

  if !response.status().is_success() {
    return Err(format!("Failed to download: HTTP {}", response.status()).into());
  }

  let total_size = response.content_length().unwrap_or(0);
  let pb = ProgressBar::new(total_size);
  pb.set_style(
    ProgressStyle::default_bar()
      .template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
      )?
      .progress_chars("#>-"),
  );

  let content = response.bytes().await?;
  pb.finish_with_message("Downloaded");

  // Extract ZIP archive
  logger::info("Extracting archive...");
  extract_zip_archive(&content, install_path, &format!("{}-main", repo))?;

  logger::info(&format!("Installed to: {}", install_path.display()));
  Ok(install_path.clone())
}

async fn download_http_archive(
  package: &Package,
  install_path: &PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
  logger::info(&format!("Downloading from URL: {}", package.url));

  let client = reqwest::Client::new();
  let response = client.get(&package.url).send().await?;

  if !response.status().is_success() {
    return Err(format!("Failed to download: HTTP {}", response.status()).into());
  }

  let total_size = response.content_length().unwrap_or(0);
  let pb = ProgressBar::new(total_size);
  pb.set_style(
    ProgressStyle::default_bar()
      .template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
      )?
      .progress_chars("#>-"),
  );

  let content = response.bytes().await?;
  pb.finish_with_message("Downloaded");

  // Determine file type and extract accordingly
  if package.url.ends_with(".zip") {
    logger::info("Extracting ZIP archive...");
    extract_zip_archive(&content, install_path, "")?;
  } else if package.url.ends_with(".tar.gz") || package.url.ends_with(".tgz") {
    logger::info("Extracting TAR.GZ archive...");
    extract_tar_gz_archive(&content, install_path)?;
  } else {
    // Treat as single file download
    fs::create_dir_all(install_path)?;
    let filename = package.url.split('/').last().unwrap_or("download");
    let file_path = install_path.join(filename);
    fs::write(&file_path, &content)?;
  }

  logger::info(&format!("Installed to: {}", install_path.display()));
  Ok(install_path.clone())
}

fn extract_zip_archive(
  content: &[u8],
  extract_to: &PathBuf,
  strip_prefix: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  use std::io::Cursor;

  let reader = Cursor::new(content);
  let mut archive = zip::ZipArchive::new(reader)?;

  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let mut outpath = extract_to.clone();

    // Handle path extraction and strip prefix if needed
    let file_path = file.name();
    let clean_path = if !strip_prefix.is_empty() && file_path.starts_with(strip_prefix) {
      &file_path[strip_prefix.len()..]
    } else {
      file_path
    };

    // Skip if path becomes empty after stripping
    if clean_path.is_empty() || clean_path == "/" {
      continue;
    }

    // Remove leading slash if present
    let clean_path = clean_path.strip_prefix('/').unwrap_or(clean_path);
    outpath.push(clean_path);

    if file.name().ends_with('/') {
      // Directory
      fs::create_dir_all(&outpath)?;
    } else {
      // File
      if let Some(parent) = outpath.parent() {
        fs::create_dir_all(parent)?;
      }

      let mut outfile = fs::File::create(&outpath)?;
      std::io::copy(&mut file, &mut outfile)?;
    }
  }

  Ok(())
}

fn extract_tar_gz_archive(
  content: &[u8],
  extract_to: &PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  use flate2::read::GzDecoder;
  use std::io::Cursor;
  use tar::Archive;

  let reader = Cursor::new(content);
  let gz_decoder = GzDecoder::new(reader);
  let mut archive = Archive::new(gz_decoder);

  archive.unpack(extract_to)?;
  Ok(())
}

// Helper functions for cache operations

fn copy_directory(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
  fs::create_dir_all(dst)?;

  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let path = entry.path();
    let dest_path = dst.join(entry.file_name());

    if path.is_dir() {
      copy_directory(&path, &dest_path)?;
    } else {
      fs::copy(&path, &dest_path)?;
    }
  }

  Ok(())
}

fn calculate_sha256(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
  use sha2::{Digest, Sha256};

  let content = fs::read(file_path)?;
  let mut hasher = Sha256::new();
  hasher.update(&content);
  let result = hasher.finalize();

  Ok(format!("{:x}", result))
}

async fn download_file(
  url: &str,
  output_path: &Path,
  silent: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let response = client.get(url).send().await?;

  if !response.status().is_success() {
    return Err(format!("HTTP error: {}", response.status()).into());
  }

  let total_size = response.content_length().unwrap_or(0);

  if !silent {
    let pb = ProgressBar::new(total_size);
    pb.set_style(
      ProgressStyle::default_bar()
        .template(" Downloading [{bar:20.cyan/blue}] {percent}%")?
        .progress_chars("=>-"),
    );

    let content = response.bytes().await?;
    pb.finish_with_message("Downloaded");

    // Create parent directories if needed
    if let Some(parent) = output_path.parent() {
      fs::create_dir_all(parent)?;
    }

    fs::write(output_path, &content)?;
  } else {
    let content = response.bytes().await?;

    // Create parent directories if needed
    if let Some(parent) = output_path.parent() {
      fs::create_dir_all(parent)?;
    }

    fs::write(output_path, &content)?;
  }

  Ok(())
}

fn unarchive(
  archiver: &str,
  input_path: &Path,
  output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
  match archiver {
    "zip" => {
      let content = fs::read(input_path)?;
      match extract_zip_archive(&content, &output_path.to_path_buf(), "") {
        Ok(_) => (),
        Err(err) => {
          return Err(format!("Unarchive error: {}", err).into());
        }
      };
    }
    "tar" => {
      let file = fs::File::open(input_path)?;
      let mut archive = tar::Archive::new(file);
      archive.unpack(output_path)?;
    }
    "tar_gz" | "tar.gz" => {
      let content = fs::read(input_path)?;
      match extract_tar_gz_archive(&content, &output_path.to_path_buf()) {
        Ok(_) => (),
        Err(err) => {
          return Err(format!("Unarchive error: {}", err).into());
        }
      };
    }
    "tar_xz" | "tar.xz" => {
      // TODO: Implement tar.xz extraction
      logger::warn("tar.xz extraction not implemented yet");
    }
    "tar_bz2" | "tar.bz2" => {
      // TODO: Implement tar.bz2 extraction
      logger::warn("tar.bz2 extraction not implemented yet");
    }
    "tar_zst" | "tar.zst" => {
      // TODO: Implement tar.zst extraction
      logger::warn("tar.zst extraction not implemented yet");
    }
    "rar" => {
      // TODO: Implement RAR extraction
      logger::warn("RAR extraction not implemented yet");
    }
    "sevenz" | "7z" => {
      // TODO: Implement 7z extraction
      logger::warn("7z extraction not implemented yet");
    }
    _ => {
      return Err(format!("Unsupported archiver: {}", archiver).into());
    }
  }

  Ok(())
}
