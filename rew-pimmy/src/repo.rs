use crate::logger;
use colored::*;
use rew_core::utils;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
  pub name: String,
  pub repo: String,
  pub version: String,
  pub description: Option<String>,
  pub url: String,
  pub readme: Option<String>,
  pub tags: Vec<String>,
  pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialPackageDetails {
  pub url: String,
  pub readme: Option<String>,
  pub description: Option<String>,
  #[serde(default)]
  pub tags: Vec<String>,
  pub version: Option<String>,
  pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PackageDetails {
  Simple(String),
  Partial(PartialPackageDetails),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoFile {
  #[serde(default)]
  pub imports: Vec<String>,
  pub packages: BTreeMap<String, PackageDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppManifest {
  pub manifest: Manifest,
  pub assets: Option<Assets>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
  pub version: String,
  pub description: Option<String>,
  pub readme: Option<String>,
  #[serde(default)]
  pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
  pub icon: Option<String>,
}

fn resolve_github_url(github_url: &str) -> Option<(String, String)> {
  let parts: Vec<&str> = github_url.split(':').collect();
  if parts.len() != 2 || parts[0] != "github" {
    return None;
  }
  let repo_parts: Vec<&str> = parts[1].split('/').collect();
  if repo_parts.len() != 2 {
    return None;
  }
  Some((repo_parts[0].to_string(), repo_parts[1].to_string()))
}

async fn resolve_github(
  client: &reqwest::Client,
  pkg_name: &str,
  github_url: &str,
) -> Option<Package> {
  let (user, repo) = resolve_github_url(github_url)?;
  let app_yaml_url = format!(
    "https://raw.githubusercontent.com/{}/{}/main/app.yaml",
    user, repo
  );

  let response = match client.get(&app_yaml_url).send().await {
    Ok(res) => res,
    Err(_) => return None, // Silently fail if app.yaml is not found
  };

  if !response.status().is_success() {
    return None;
  }

  let text = response.text().await.ok()?;
  let manifest: AppManifest = serde_yaml::from_str(&text).ok()?;

  Some(Package {
    name: pkg_name.to_string(),
    repo: repo.to_string(),
    version: manifest.manifest.version,
    description: manifest.manifest.description,
    url: github_url.to_string(),
    readme: manifest.manifest.readme,
    tags: manifest.manifest.tags,
    icon: manifest.assets.and_then(|a| a.icon),
  })
}

async fn parse_repo(client: &reqwest::Client, repo_name: &str, repo_url: &str) -> Vec<Package> {
  logger::info(&format!("Parsing repo: {}", repo_name));
  let url = if repo_url.starts_with("//") {
    format!("https:/{}", repo_url)
  } else {
    repo_url.to_string()
  };

  let response = match client.get(&url).send().await {
    Ok(res) => res,
    Err(e) => {
      logger::error(&format!("Failed to fetch repo {}: {}", repo_name, e));
      return vec![];
    }
  };

  let text = match response.text().await {
    Ok(text) => text,
    Err(e) => {
      logger::error(&format!(
        "Failed to read repo response {}: {}",
        repo_name, e
      ));
      return vec![];
    }
  };

  let repo_file: RepoFile = match serde_yaml::from_str(&text) {
    Ok(data) => data,
    Err(e) => {
      logger::error(&format!("Failed to parse repo YAML {}: {}", repo_name, e));
      return vec![];
    }
  };

  let mut packages = Vec::new();
  for (name, details) in repo_file.packages {
    let pkg = match details {
      PackageDetails::Simple(url) => {
        if url.starts_with("github:") {
          resolve_github(client, &name, &url).await
        } else {
          None
        }
      }
      PackageDetails::Partial(partial) => Some(Package {
        name: name.to_string(),
        repo: repo_name.to_string(),
        version: partial.version.unwrap_or_else(|| "unknown".to_string()),
        description: partial.description,
        url: partial.url,
        readme: partial.readme,
        tags: partial.tags,
        icon: partial.icon,
      }),
    };
    if let Some(p) = pkg {
      packages.push(p);
    }
  }

  packages
}

pub async fn sync_all(repo_name: Option<String>) {
  logger::info("Syncing repositories...");

  let Some(pimmy_data_path) = utils::pimmy_data_path() else {
    return;
  };
  let repo_main_yaml_path = pimmy_data_path.join("repo").join("main.yaml");

  let repos_file = match fs::File::open(&repo_main_yaml_path) {
    Ok(file) => file,
    Err(e) => {
      logger::error(&format!("Failed to open {:?}: {}", repo_main_yaml_path, e));
      return;
    }
  };

  let repos: BTreeMap<String, String> = match serde_yaml::from_reader(repos_file) {
    Ok(data) => data,
    Err(e) => {
      logger::error(&format!("Failed to parse {:?}: {}", repo_main_yaml_path, e));
      return;
    }
  };

  let client = reqwest::Client::new();
  let mut index = 0;

  for (name, url) in repos {
    if let Some(ref r_name) = repo_name {
      if &name != r_name {
        continue;
      }
    }

    let packages = parse_repo(&client, &name, &url).await;

    let cache_path = pimmy_data_path
      .join("cache/repo-cache")
      .join(format!("db_{}.bin", index));
    let data = match serde_json::to_string(&packages) {
      Ok(d) => d,
      Err(e) => {
        logger::error(&format!("Failed to serialize packages: {}", e));
        continue;
      }
    };

    if let Err(e) = fs::write(&cache_path, data) {
      logger::error(&format!("Failed to write cache file for {}: {}", name, e));
    }

    index += 1;
  }

  logger::info("Finished syncing repositories.");
}

pub fn list(repo_name: Option<String>) {
  if let Some(repo_name) = repo_name {
    // TODO: List packages in a specific repo
    logger::info(&format!("Listing packages for repo: {}", repo_name));
  } else {
    list_repos();
  }
}

fn list_repos() {
  logger::info("Available repositories:");

  let Some(pimmy_data_path) = utils::pimmy_data_path() else {
    return;
  };
  let repo_main_yaml_path = pimmy_data_path.join("repo").join("main.yaml");

  let repos_file = match fs::File::open(&repo_main_yaml_path) {
    Ok(file) => file,
    Err(e) => {
      logger::error(&format!("Failed to open {:?}: {}", repo_main_yaml_path, e));
      return;
    }
  };

  let repos: BTreeMap<String, String> = match serde_yaml::from_reader(repos_file) {
    Ok(data) => data,
    Err(e) => {
      logger::error(&format!("Failed to parse {:?}: {}", repo_main_yaml_path, e));
      return;
    }
  };

  println!(" {0: <15} | {1: <60} | {2: <10}", "Repo", "URL", "Cached");
  println!("{:-<15}-+-{:-<60}-+-{:-<10}", "", "", "");

  let repo_cache_path = pimmy_data_path.join("cache/repo-cache");

  for (i, (name, url)) in repos.iter().enumerate() {
    let cache_file = repo_cache_path.join(format!("db_{}.bin", i));
    let cached_status = if cache_file.exists() {
      "Yes".green()
    } else {
      "No".red()
    };
    println!(
      " {0: <15} | {1: <60} | {2: <10}",
      name.blue(),
      url.green(),
      cached_status
    );
  }
}

pub fn init() {
  logger::info("Initializing repositories.");

  let Some(pimmy_data_path) = utils::pimmy_data_path() else {
    return;
  };

  let cache_path = pimmy_data_path.join("cache");
  let app_cache_path = cache_path.join("app-cache");
  let repo_cache_path = cache_path.join("repo-cache");
  let repo_config_path = pimmy_data_path.join("repo");

  for path in &[&app_cache_path, &repo_cache_path, &repo_config_path] {
    if let Err(e) = fs::create_dir_all(path) {
      logger::error(&format!("Failed to create directory {:?}: {}", path, e));
      return;
    }
  }

  let repo_main_yaml_path = repo_config_path.join("main.yaml");
  if !repo_main_yaml_path.exists() {
    let mut default_repo = BTreeMap::new();
    default_repo.insert(
      "rewpkgs".to_string(),
      "//raw.githubusercontent.com/kevinJ045/rewpkgs/main/main.yaml".to_string(),
    );

    let yaml_str = match serde_yaml::to_string(&default_repo) {
      Ok(s) => s,
      Err(e) => {
        logger::error(&format!("Failed to serialize default repo config: {}", e));
        return;
      }
    };

    if let Err(e) = fs::write(&repo_main_yaml_path, yaml_str) {
      logger::error(&format!(
        "Failed to write to {:?}: {}",
        repo_main_yaml_path, e
      ));
      return;
    }
  }
}

pub fn add(name: String, url: String) {
  let Some(pimmy_data_path) = utils::pimmy_data_path() else {
    return;
  };
  let repo_config_path = pimmy_data_path.join("repo");
  let repo_main_yaml_path = repo_config_path.join("main.yaml");

  let mut repos: BTreeMap<String, String> = if repo_main_yaml_path.exists() {
    match fs::File::open(&repo_main_yaml_path) {
      Ok(file) => match serde_yaml::from_reader(file) {
        Ok(d) => d,
        Err(e) => {
          logger::error(&format!("Failed to parse {:?}: {}", repo_main_yaml_path, e));
          return;
        }
      },
      Err(e) => {
        logger::error(&format!("Failed to open {:?}: {}", repo_main_yaml_path, e));
        return;
      }
    }
  } else {
    BTreeMap::new()
  };

  repos.insert(name.clone(), url.clone());

  match serde_yaml::to_string(&repos) {
    Ok(s) => {
      if let Err(e) = fs::write(&repo_main_yaml_path, s) {
        logger::error(&format!("Failed to write {:?}: {}", repo_main_yaml_path, e));
        return;
      }
    }
    Err(e) => {
      logger::error(&format!("Failed to serialize repos: {}", e));
      return;
    }
  }

  logger::info(&format!("Added repo '{}' -> {}", name, url));
}

pub fn remove(name: String) {
  let Some(pimmy_data_path) = utils::pimmy_data_path() else {
    return;
  };
  let repo_config_path = pimmy_data_path.join("repo");
  let repo_main_yaml_path = repo_config_path.join("main.yaml");

  let mut repos: BTreeMap<String, String> = match fs::File::open(&repo_main_yaml_path) {
    Ok(file) => match serde_yaml::from_reader(file) {
      Ok(d) => d,
      Err(e) => {
        logger::error(&format!("Failed to parse {:?}: {}", repo_main_yaml_path, e));
        return;
      }
    },
    Err(e) => {
      logger::error(&format!("Failed to open {:?}: {}", repo_main_yaml_path, e));
      return;
    }
  };

  if repos.remove(&name).is_some() {
    match serde_yaml::to_string(&repos) {
      Ok(s) => {
        if let Err(e) = fs::write(&repo_main_yaml_path, s) {
          logger::error(&format!("Failed to write {:?}: {}", repo_main_yaml_path, e));
          return;
        }
      }
      Err(e) => {
        logger::error(&format!("Failed to serialize repos: {}", e));
        return;
      }
    }
    logger::info(&format!("Removed repo '{}'", name));
  } else {
    logger::warn(&format!("Repo '{}' not found", name));
  }
}

fn load_repos_with_index() -> Option<(BTreeMap<String, String>, PathBuf)> {
  let Some(pimmy_data_path) = utils::pimmy_data_path() else {
    return None;
  };
  let repo_main_yaml_path = pimmy_data_path.join("repo").join("main.yaml");
  let repos_file = fs::File::open(&repo_main_yaml_path).ok()?;
  let repos: BTreeMap<String, String> = serde_yaml::from_reader(repos_file).ok()?;
  Some((repos, pimmy_data_path))
}

pub fn list_packages_in_repo(repo_name: &str) {
  let Some((repos, pimmy_data_path)) = load_repos_with_index() else {
    return;
  };
  let mut index = None;
  for (i, (name, _)) in repos.iter().enumerate() {
    if name == repo_name {
      index = Some(i);
      break;
    }
  }
  let Some(i) = index else {
    logger::warn(&format!("Repo '{}' not found in config", repo_name));
    return;
  };

  let cache_file = pimmy_data_path
    .join("cache/repo-cache")
    .join(format!("db_{}.bin", i));
  if !cache_file.exists() {
    logger::warn("No cache for repo. Run --sync first.");
    return;
  }

  let data = match fs::read_to_string(&cache_file) {
    Ok(s) => s,
    Err(e) => {
      logger::error(&format!("Failed to read {:?}: {}", cache_file, e));
      return;
    }
  };

  let packages: Vec<Package> = match serde_json::from_str(&data) {
    Ok(p) => p,
    Err(e) => {
      logger::error(&format!("Failed to parse cache {:?}: {}", cache_file, e));
      return;
    }
  };

  println!(
    " {0: <30} | {1: <10} | {2}",
    "Package", "Version", "Description"
  );
  println!("{:-<30}-+-{:-<10}-+-{:-<60}", "", "", "");
  for p in packages {
    println!(
      " {0: <30} | {1: <10} | {2}",
      p.name.blue(),
      p.version.green(),
      p.description.unwrap_or_default()
    );
  }
}

pub fn find_app(name: &str) {
  let Some((repos, pimmy_data_path)) = load_repos_with_index() else {
    return;
  };
  let repo_cache_path = pimmy_data_path.join("cache/repo-cache");

  let mut found = Vec::new();
  for (i, (repo_name, _)) in repos.iter().enumerate() {
    let cache_file = repo_cache_path.join(format!("db_{}.bin", i));
    if !cache_file.exists() {
      continue;
    }
    if let Ok(s) = fs::read_to_string(&cache_file) {
      if let Ok(packages) = serde_json::from_str::<Vec<Package>>(&s) {
        for p in packages {
          if p.name.contains(name) {
            found.push((repo_name.clone(), p));
          }
        }
      }
    }
  }

  if found.is_empty() {
    logger::warn("No matches (maybe run --sync?)");
    return;
  }

  println!(
    " {0: <20} | {1: <30} | {2: <10} | {3}",
    "Repo", "Package", "Version", "URL"
  );
  println!("{:-<20}-+-{:-<30}-+-{:-<10}-+-{:-<40}", "", "", "", "");
  for (repo_name, p) in found {
    println!(
      " {0: <20} | {1: <30} | {2: <10} | {3}",
      repo_name.blue(),
      p.name.green(),
      p.version,
      p.url
    );
  }
}

pub async fn readme(app: &str) {
  // look up in cache by exact name
  let Some((repos, pimmy_data_path)) = load_repos_with_index() else {
    return;
  };
  let repo_cache_path = pimmy_data_path.join("cache/repo-cache");

  for (i, _) in repos.iter().enumerate() {
    let cache_file = repo_cache_path.join(format!("db_{}.bin", i));
    if !cache_file.exists() {
      continue;
    }
    if let Ok(s) = fs::read_to_string(&cache_file) {
      if let Ok(packages) = serde_json::from_str::<Vec<Package>>(&s) {
        for p in packages {
          if p.name == app {
            if let Some(readme_path) = p.readme {
              // Try to fetch and print
              let url = if readme_path.starts_with("http") {
                readme_path
              } else if p.url.starts_with("github:") {
                if let Some((user, repo)) = resolve_github_url(&p.url) {
                  format!(
                    "https://raw.githubusercontent.com/{}/{}/main/{}",
                    user, repo, readme_path
                  )
                } else {
                  readme_path
                }
              } else {
                readme_path
              };

              let client = reqwest::Client::new();
              match client.get(&url).send().await {
                Ok(resp) if resp.status().is_success() => {
                  if let Ok(text) = resp.text().await {
                    println!("{}", text);
                    return;
                  }
                }
                _ => {
                  logger::warn(&format!("Failed to fetch readme at {}", url));
                  println!("{}", url);
                  return;
                }
              }
            } else {
              logger::warn("No readme field in package");
              return;
            }
          }
        }
      }
    }
  }
  logger::warn("App not found in cache. Try --sync then --readme.");
}
