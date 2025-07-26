use rew_compiler::{get_civet_script, CompilerOptions, compile_rew_stuff, CompilerResults, Declaration, DeclarationEngine, compile_jsx};
use crate::builtins::BUILTIN_MODULES;
use crate::data_manager::{DataFormat, DataManager};
use rew_extensions::ext::{console, ffi, process, url, web, webidl};
use crate::runtime_script::get_runtime_script;
use rew_core::utils::find_app_path;
use rew_core::BuildOptions;
use crate::workers::{
  op_thread_message, op_thread_post_message, op_thread_receive, op_thread_spawn,
  op_thread_terminate,
};
use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use deno_core::OpState;
use deno_core::PollEventLoopOptions;
use deno_core::error::CoreError;
use deno_core::{JsRuntime, RuntimeOptions, extension, op2};
use deno_fs::{FileSystem, RealFs};
use deno_permissions::{
  AllowRunDescriptor, AllowRunDescriptorParseResult, DenyRunDescriptor, EnvDescriptor,
  EnvDescriptorParseError, FfiDescriptor, ImportDescriptor, NetDescriptor, NetDescriptorParseError,
  PathQueryDescriptor, PathResolveError, PermissionDescriptorParser, PermissionsContainer,
  ReadDescriptor, RunDescriptorParseError, RunQueryDescriptor, SysDescriptor,
  SysDescriptorParseError, WriteDescriptor,
};
use futures::stream::{self, StreamExt};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Mutex;

// use crate::shell::{op_shell_close, op_shell_kill, op_shell_read, op_shell_spawn, op_shell_write};

#[derive(Default)]
pub struct RuntimeArgs(pub Vec<String>);

/// Encodes the provided string content into Base64 format.
///
/// # Arguments
/// * `content` - The string to encode.
///
/// # Returns
/// * A Base64 encoded string.
fn encode_brew_file(content: &str) -> String {
  BASE64.encode(content.as_bytes())
}

/// Decodes a Base64 encoded string back into its original form.
///
/// # Arguments
/// * `encoded` - The Base64 encoded string to decode.
///
/// # Returns
/// * A `Result` containing the decoded string or an error message.
fn decode_brew_file(encoded: &str) -> Result<String> {
  let decoded = BASE64
    .decode(encoded.trim())
    .map_err(|e| anyhow::anyhow!("Failed to decode brew file: {}", e))?;

  String::from_utf8(decoded)
    .map_err(|e| anyhow::anyhow!("Failed to convert decoded bytes to string: {}", e))
}

pub static VIRTUAL_FILES: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| Mutex::new(vec![]));

/// Adds a virtual file to the runtime's virtual file storage.
///
/// # Arguments
/// * `path` - The path of the virtual file.
/// * `contents` - The contents of the virtual file.
pub fn add_virtual_file(path: &str, contents: &str) {
  let mut files = VIRTUAL_FILES.lock().unwrap();
  files.push((path.to_string(), contents.to_string()));
}

pub fn is_js_executable(mod_id: &str) -> bool {
  matches!(
    mod_id.rsplit('.').next(),
    Some("ts" | "js" | "coffee" | "civet" | "rew")
  )
}

#[derive(Default)]
struct RuntimeState {
  current_dir: PathBuf,
  args: Vec<String>,
}

#[derive(Debug, Clone)]
struct TestPermissionDescriptorParser;

impl TestPermissionDescriptorParser {
  fn join_path_with_root(&self, path: &str) -> PathBuf {
    if path.starts_with("C:\\") {
      PathBuf::from(path)
    } else {
      PathBuf::from("/").join(path)
    }
  }
}

impl PermissionDescriptorParser for TestPermissionDescriptorParser {
  fn parse_read_descriptor(&self, text: &str) -> Result<ReadDescriptor, PathResolveError> {
    Ok(ReadDescriptor(self.join_path_with_root(text)))
  }

  fn parse_write_descriptor(&self, text: &str) -> Result<WriteDescriptor, PathResolveError> {
    Ok(WriteDescriptor(self.join_path_with_root(text)))
  }

  fn parse_net_descriptor(&self, text: &str) -> Result<NetDescriptor, NetDescriptorParseError> {
    NetDescriptor::parse(text)
  }

  fn parse_import_descriptor(
    &self,
    text: &str,
  ) -> Result<ImportDescriptor, NetDescriptorParseError> {
    ImportDescriptor::parse(text)
  }

  fn parse_env_descriptor(&self, text: &str) -> Result<EnvDescriptor, EnvDescriptorParseError> {
    Ok(EnvDescriptor::new(text))
  }

  fn parse_sys_descriptor(&self, text: &str) -> Result<SysDescriptor, SysDescriptorParseError> {
    SysDescriptor::parse(text.to_string())
  }

  fn parse_allow_run_descriptor(
    &self,
    text: &str,
  ) -> Result<AllowRunDescriptorParseResult, RunDescriptorParseError> {
    Ok(AllowRunDescriptorParseResult::Descriptor(
      AllowRunDescriptor(self.join_path_with_root(text)),
    ))
  }

  fn parse_deny_run_descriptor(&self, text: &str) -> Result<DenyRunDescriptor, PathResolveError> {
    if text.contains("/") {
      Ok(DenyRunDescriptor::Path(self.join_path_with_root(text)))
    } else {
      Ok(DenyRunDescriptor::Name(text.to_string()))
    }
  }

  fn parse_ffi_descriptor(&self, text: &str) -> Result<FfiDescriptor, PathResolveError> {
    Ok(FfiDescriptor(self.join_path_with_root(text)))
  }

  fn parse_path_query(&self, path: &str) -> Result<PathQueryDescriptor, PathResolveError> {
    Ok(PathQueryDescriptor {
      resolved: self.join_path_with_root(path),
      requested: path.to_string(),
    })
  }

  fn parse_run_query(
    &self,
    requested: &str,
  ) -> Result<RunQueryDescriptor, RunDescriptorParseError> {
    RunQueryDescriptor::parse(requested).map_err(Into::into)
  }
}

extension!(
  rewextension,
  ops = [
    op_fs_read,
    op_fs_write,
    op_fs_exists,
    op_fs_rm,
    op_fs_mkdir,
    op_fs_readdir,
    op_fs_stats,
    op_fs_copy,
    op_fs_rename,
    op_fs_cwdir,
    op_to_base64,
    op_from_base64,
    op_find_app,
    op_yaml_to_string,
    op_string_to_yaml,
    op_app_loadconfig,
    op_data_read,
    op_data_write,
    op_data_delete,
    op_data_exists,
    op_data_list,
    op_data_read_binary,
    op_data_write_binary,
    op_data_read_yaml,
    op_data_write_yaml,
    op_data_get_info,
    op_data_get_path,
    op_thread_spawn,
    op_thread_message,
    op_thread_post_message,
    op_thread_terminate,
    op_thread_receive,
    op_fetch_env,
    op_get_args,
    op_os_info_os,
    op_os_info_arch,
    op_os_info_family,
    op_dyn_imp,
    op_fs_sha,
    op_rand_from,
    op_gen_uid,
    op_vfile_set,
    op_vfile_get,
    op_terminal_size
  ],
  state = |state| {
    let permissions =
      PermissionsContainer::allow_all(std::sync::Arc::new(TestPermissionDescriptorParser));

    state.put::<PermissionsContainer>(permissions.clone());
  }
);

pub fn get_rew_runtime(
  is_compiler: bool,
  is_main: bool,
  args: Option<Vec<String>>,
) -> Result<JsRuntime> {
  let mut extensions = vec![rewextension::init()];

  extensions.extend(webidl::extensions(false));
  extensions.extend(console::extensions(false));
  extensions.extend(url::extensions(false));
  extensions.extend(web::extensions(web::WebOptions::default(), false));
  extensions.extend(ffi::extensions(false));
  extensions.extend(rew_extensions::ext::telemetry::extensions(false));
  extensions.extend(rew_extensions::ext::networking::extensions(false));
  extensions.extend(rew_extensions::ext::http::extensions(false));
  extensions.extend(rew_extensions::ext::io::extensions(
    Some(deno_io::Stdio {
      stdin: deno_io::StdioPipe::inherit(),
      stderr: deno_io::StdioPipe::inherit(),
      stdout: deno_io::StdioPipe::inherit(),
    }),
    false,
  ));
  extensions.extend(rew_extensions::ext::fs::extensions(
    std::rc::Rc::new(RealFs) as std::rc::Rc<dyn FileSystem>,
    false,
  ));
  extensions.extend(rew_extensions::ext::os::extensions(false));
  extensions.extend(process::extensions(false));

  let mut runtime = JsRuntime::new(RuntimeOptions {
    extensions,
    // module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
    is_main,
    ..Default::default()
  });

  let current_dir = std::env::current_dir()?;

  let state = RuntimeState {
    current_dir: current_dir.clone(),
    args: args.unwrap_or_default(),
  };

  runtime.op_state().borrow_mut().put(state);
  runtime.execute_script(
    "<setup>",
    r#"
globalThis._execVM = (namespace, fn) => {
  with(namespace){
  eval(`(${fn.toString()})()`);
  }
}
globalThis._evalVM = (string) => eval(string);
"#,
  )?;
  runtime.execute_script("<setup>", get_runtime_script())?;
  if is_compiler {
    runtime
      .execute_script("<civet>", get_civet_script())
      .unwrap();
  }
  Ok(runtime)
}

fn get_storage_path(file_path_str: &str) -> PathBuf {
  if !file_path_str.starts_with("./") {
    return PathBuf::from(file_path_str);
  }
  if let Some(app_info) = rew_core::utils::find_app_info(Path::new(file_path_str)) {
    if let Some(manifest) = &app_info.config.manifest {
      if let Some(package) = &manifest.package {
        if let Ok(rel_path) = Path::new(file_path_str).strip_prefix(&app_info.path) {
          PathBuf::from(format!("{}/{}", package, rel_path.to_string_lossy()))
        } else {
          PathBuf::from(file_path_str)
        }
      } else {
        PathBuf::from(file_path_str)
      }
    } else {
      PathBuf::from(file_path_str)
    }
  } else {
    PathBuf::from(file_path_str)
  }
}

pub struct RewRuntime {
  pub runtime: JsRuntime,
  // pub compiler_runtime: JsRuntime,
  declaration_engine: DeclarationEngine,
  sourcemap: bool,
  inlinemap: bool,

  compile_options: Vec<String>,
}

impl RewRuntime {
  pub fn new(args: Option<Vec<String>>, jruntime: Option<JsRuntime>) -> Result<Self> {
    let runtime = jruntime.unwrap_or_else(|| get_rew_runtime(true, true, args).unwrap());
    // let mut compiler_runtime = get_compiler_runtime();

    let declaration_engine = DeclarationEngine {
      global_declarations: HashMap::new(),
    };

    Ok(Self {
      runtime,
      inlinemap: false,
      sourcemap: false,
      compile_options: vec![],
      // compiler_runtime,
      declaration_engine,
    })
  }

  pub fn resolve_includes_recursive_from<P: AsRef<Path>>(
    filepath: P,
  ) -> Result<Vec<(PathBuf, String, bool)>> {
    let filepath = filepath
      .as_ref()
      .canonicalize()
      .with_context(|| "Failed to resolve import".to_string())?;

    let import_re = Regex::new(r#"(?m)^\s*import\s+(?:[^;]*?\s+from\s+)?["']([^"']+)["']"#)
      .context("Invalid regex pattern")?;
    let external_re =
      Regex::new(r#"(?m)^\s*// external\s+['"]([^'"]+)['"]"#).context("Invalid regex pattern")?;

    let mut visited = HashSet::new();
    let mut result = Vec::new();

    fn visit_file(
      file_path: &Path,
      visited: &mut HashSet<PathBuf>,
      result: &mut Vec<(PathBuf, String, bool)>,
      preprocess_import: bool,
      import_re: &Regex,
      external_re: &Regex,
    ) -> Result<()> {
      if visited.contains(file_path) {
        return Ok(());
      }

      let mut should_preprocess = preprocess_import;
      let file_path_unf = file_path.to_str().unwrap_or("");
      let file_path_str = if file_path_unf.ends_with('!') {
        should_preprocess = true;
        &file_path_unf[0..file_path_unf.len() - 1]
      } else {
        file_path_unf
      };

      let is_brew_file = file_path
        .extension()
        .is_some_and(|ext| ext == "brew" || ext == "qrew");

      let real_content = if let Some(v) = VIRTUAL_FILES
        .lock()
        .unwrap()
        .iter()
        .find(|(p, _)| p == file_path_str)
      {
        v.1.clone()
      } else if file_path_str.starts_with("#") {
        "".to_string()
      } else {
        fs::read_to_string(file_path).with_context(|| format!("Failed to read {:?}", file_path))?
      };

      let content = if file_path_str.starts_with("#") {
        if let Some(builtin_content) = BUILTIN_MODULES.get(file_path_str) {
          builtin_content.to_string()
        } else {
          return Err(anyhow::anyhow!(
            "Builtin module not found: {}",
            file_path_str
          ));
        }
      } else if is_brew_file {
        if let Ok(decoded) = decode_brew_file(&real_content) {
          decoded
        } else {
          real_content
        }
      } else {
        real_content
      };

      visited.insert(PathBuf::from(file_path_str));

      // Create a normalized path for storage
      let storage_path = get_storage_path(file_path_str);

      result.push((storage_path, content.clone(), should_preprocess));

      let parent = file_path.parent().unwrap_or(Path::new("."));

      if is_brew_file || content.starts_with("\"no-compile\"") {
        for cap in external_re.captures_iter(&content) {
          let external_app_path = cap[1].to_string();
          let mut should_preprocess_import = false;
          let external_app = if external_app_path.ends_with('!') {
            should_preprocess_import = true;
            &external_app_path[0..external_app_path.len() - 1]
          } else {
            &external_app_path
          };

          if external_app.contains('/') {
            let parts: Vec<&str> = external_app.split('/').collect();
            if parts.len() == 2 {
              let package_name = parts[0];
              let entry_name = parts[1];

              if let Some(app_entry) =
                rew_core::utils::resolve_app_entry(package_name, Some(entry_name))
              {
                visit_file(
                  &app_entry,
                  visited,
                  result,
                  should_preprocess_import,
                  import_re,
                  external_re,
                )?;
              }
            }
          } else {
            visit_file(
              &PathBuf::from(external_app),
              visited,
              result,
              should_preprocess_import,
              import_re,
              external_re,
            )?;
          }
        }
      } else {
        for cap in import_re.captures_iter(&content) {
          let relative_path_raw = cap[1].to_string();
          let mut should_preprocess_import = false;
          let relative_path = if relative_path_raw.ends_with('!') {
            should_preprocess_import = true;
            &relative_path_raw[0..relative_path_raw.len() - 1]
          } else {
            &relative_path_raw
          };

          if relative_path.starts_with("#") {
            let builtin_path = PathBuf::from(&relative_path);
            visit_file(
              &builtin_path,
              visited,
              result,
              should_preprocess_import,
              import_re,
              external_re,
            )?;
          } else if !relative_path.contains("/")
            && !relative_path.contains("\\")
            && !relative_path.starts_with(".")
          {
            if let Some(app_entry) = rew_core::utils::resolve_app_entry(relative_path, None) {
              visit_file(
                &app_entry,
                visited,
                result,
                should_preprocess_import,
                import_re,
                external_re,
              )?;
            } else {
              return Err(anyhow::anyhow!("App not found: {}", relative_path));
            }
          } else if relative_path.contains("/") && !relative_path.starts_with(".") {
            let parts: Vec<&str> = relative_path.splitn(2, "/").collect();
            if parts.len() == 2 {
              let package_name = parts[0];
              let entry_name = parts[1];

              if let Some(app_entry) =
                rew_core::utils::resolve_app_entry(package_name, Some(entry_name))
              {
                visit_file(
                  &app_entry,
                  visited,
                  result,
                  should_preprocess_import,
                  import_re,
                  external_re,
                )?;
              } else {
                return Err(anyhow::anyhow!(
                  "App entry not found: {}/{}",
                  package_name,
                  entry_name
                ));
              }
            } else {
              let included_path = parent
                .join(relative_path)
                .canonicalize()
                .with_context(|| "Failed to resolve import".to_string())?;

              visit_file(
                &included_path,
                visited,
                result,
                should_preprocess_import,
                import_re,
                external_re,
              )?;
            }
          } else {
            let included_path = parent
              .join(relative_path)
              .canonicalize()
              .with_context(|| "Failed to resolve import".to_string())?;

            visit_file(
              &included_path,
              visited,
              result,
              should_preprocess_import,
              import_re,
              external_re,
            )?;
          }
        }
      }

      Ok(())
    }

    visit_file(
      &filepath,
      &mut visited,
      &mut result,
      false,
      &import_re,
      &external_re,
    )?;
    Ok(result)
  }

  pub async fn prepare(
    &mut self,
    files: Vec<(PathBuf, String)>,
    entry: Option<&Path>,
  ) -> Result<String> {
    let mut module_wrappers = String::new();
    let mut entry_calls = Vec::new();

    let shared = std::rc::Rc::new(tokio::sync::Mutex::new(self));

    let results = stream::iter(files.into_iter().map(|(path_original, source)| {
      let path = path_original.clone();
      let source = source.clone();
      let shared = Rc::clone(&shared);

      async move {
        let res = {
          let mut runtime = shared.lock().await;
          runtime
            .compile_and_run(&source, &path, false)
            .await
            .map_err(|e| anyhow::anyhow!("Runtime error: {}", e))
        };

        res.map(|out| (path_original.clone(), out))
      }
    }))
    .buffer_unordered(16) // concurrency limit
    .collect::<Vec<_>>()
    .await;

    let entry_regex = Regex::new(r#"//\s*entry\s*"([^"]+)""#).unwrap();
    for result in results {
      let (path, compiled) = result?;
      let mod_id = path
        .to_str()
        .unwrap_or("unknown")
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('"', "\\\"");
      let mut mod_alias = String::new();

      if let Some(app_info) = rew_core::utils::find_app_info(&path) {
        if let Some(manifest) = &app_info.config.manifest {
          if let Some(package) = &manifest.package {
            if let Ok(rel_path) = path.strip_prefix(&app_info.path) {
              let rel_path_str = rel_path.to_str().unwrap_or("");

              if let Some(entries) = &app_info.config.entries {
                for (key, value) in entries {
                  if value == rel_path_str {
                    mod_alias.push_str(&format!("[\"app://{}/{}\"]", package, key));
                    break;
                  }
                }
              }

              let base_name = Path::new(rel_path_str)
                .with_extension("")
                .to_string_lossy()
                .into_owned();
              if mod_alias.is_empty() {
                mod_alias.push_str(&format!(
                  "[\"app://{}/{}\"]",
                  package,
                  base_name.replace('\\', "\\\\")
                ))
              }
            }
          }
        }
      }

      if mod_id.starts_with('#') {
        module_wrappers.push_str(&format!(
          "(function(module){{\n{compiled}\n}})({{filename: \"{id}\"}});",
          id = mod_id,
          compiled = compiled
        ));
      } else if mod_id.ends_with(".brew") || mod_id.ends_with(".qrew") {
        // if mod_id.ends_with(".brew") {
        //   module_wrappers.push_str(format!("globalThis.__filename__ = \"{}\";", mod_id).as_str());
        // }
        module_wrappers.push_str(compiled.as_str());

        for cap in entry_regex.captures_iter(&compiled) {
          let entry_file = cap[1].to_string();
          entry_calls.push(format!(
            "rew.prototype.mod.prototype.get('{}');",
            entry_file.replace('\\', "\\\\")
          ));
        }
      } else if is_js_executable(&mod_id) {
        module_wrappers.push_str(&format!(
          r#"rew.prototype.mod.prototype.defineNew("{id}", {{
"{id}"(globalThis){{
with (globalThis) {{
  {compiled}
}}
return globalThis.module.exports;
}}          
}}, {mod_alias});"#,
          id = mod_id,
          mod_alias = mod_alias,
          compiled = compiled
        ));
      } else {
        module_wrappers.push_str(&format!(
          r#"rew.prototype.mod.prototype.defineNew("{id}", function(globalThis){{
  return rew.prototype.mod.prototype.preprocess("{id}", `{compiled}`);
}}, {mod_alias});"#,
          id = mod_id,
          mod_alias = mod_alias,
          compiled = compiled.replace("`", "\\`").replace("\\", "\\\\")
        ));
      }
    }

    if let Some(entry) = entry {
      let entry_mod_id = entry.to_str().unwrap_or("entry");
      let mut entry_app_id = None;

      if let Some(app_info) = rew_core::utils::find_app_info(entry) {
        if let Some(manifest) = &app_info.config.manifest {
          if let Some(package) = &manifest.package {
            if let Ok(rel_path) = entry.strip_prefix(&app_info.path) {
              let rel_path_str = rel_path.to_str().unwrap_or("");

              if let Some(entries) = &app_info.config.entries {
                for (key, value) in entries {
                  if value == rel_path_str {
                    entry_app_id = Some(format!("app://{}/{}", package, key));
                    break;
                  }
                }
              }
            }
          }
        }
      }

      let final_entry_id = entry_app_id
        .unwrap_or_else(|| entry_mod_id.to_string())
        .replace('\\', "\\\\");
      if entry_calls.is_empty()
        && !final_entry_id.ends_with(".brew")
        && !final_entry_id.ends_with(".qrew")
      {
        entry_calls.push(format!(
          "rew.prototype.mod.prototype.get('{}');",
          final_entry_id
        ));
      }
    }

    for entry_call in entry_calls {
      module_wrappers.push_str(&format!("\n{}", entry_call));
    }

    // fs::write("out.js", module_wrappers.clone())?;

    Ok(module_wrappers.to_string())
  }

  pub async fn build_file<P: AsRef<Path>>(
    &mut self,
    filepath: P,
    options: BuildOptions,
  ) -> Result<String> {
    let filepath = filepath
      .as_ref()
      .canonicalize()
      .with_context(|| format!("Failed to resolve file path: {:?}", filepath.as_ref()))?;

    let files_with_flags = RewRuntime::resolve_includes_recursive_from(&filepath)?;
    let mut excluded: Vec<String> = Vec::new();

    let files_with_flags = if !options.bundle_all {
      let main_app_path = rew_core::utils::find_app_path(&filepath);

      files_with_flags
        .into_iter()
        .filter(|(path, source, _)| {
          if let Some(app_path) = &main_app_path {
            if !path.starts_with(app_path) || path.to_str().unwrap_or("").starts_with("#") {
              if path.to_str().unwrap_or("").starts_with("#") {
                excluded.push(path.to_str().unwrap_or("").to_string());
              } else if let Some(app_info) = rew_core::utils::find_app_info(path) {
                if let Some(manifest) = &app_info.config.manifest {
                  if let Some(package) = &manifest.package {
                    if let Ok(rel_path) = path.strip_prefix(&app_info.path) {
                      let rel_path_str = rel_path.to_str().unwrap_or("");

                      if let Some(entries) = &app_info.config.entries {
                        for (key, value) in entries {
                          if value == rel_path_str {
                            let entry_id = format!("{}/{}", package, key);

                            if !excluded.contains(&entry_id) {
                              excluded.push(entry_id.clone());
                            }
                            break;
                          }
                        }
                      }
                    }
                  }
                }
              }
              self.declaration_engine.process_script(source);
            }
            path.starts_with(app_path)
          } else {
            true
          }
        })
        .collect()
    } else {
      files_with_flags
    };

    let files: Vec<(PathBuf, String)> = files_with_flags
      .into_iter()
      .map(|(path, content, _)| (path, content))
      .collect();

    let entry_path = if let Some(entry) = options.entry_file {
      entry.canonicalize().unwrap_or(filepath)
    } else {
      filepath
    };
    let entry = get_storage_path(entry_path.to_str().unwrap());

    let mut string = self.prepare(files, None).await?;
    string.insert_str(
      0,
      format!(
        "{}\n",
        excluded
          .into_iter()
          .map(|item| format!("// external \"{}\"", item))
          .collect::<Vec<String>>()
          .join("\n")
      )
      .as_str(),
    );
    string.insert_str(
      0,
      format!("\n// entry \"{}\" \n", entry.to_str().unwrap_or("unknown")).as_str(),
    );
    string.push('\n');

    string = encode_brew_file(&string);

    Ok(string)
  }

  pub async fn include_and_run(
    &mut self,
    files: Vec<(PathBuf, String)>,
    entry: &Path,
  ) -> Result<()> {
    let final_script = self.prepare(files, Some(entry)).await?;

    self.runtime.execute_script("<main>", final_script)?;
    self
      .runtime
      .run_event_loop(PollEventLoopOptions::default())
      .await?;
    Ok(())
  }

  pub async fn compile_and_run(
    &mut self,
    source: &str,
    filepath: &Path,
    keep_imports: bool,
  ) -> Result<String> {
    if filepath
      .extension()
      .is_some_and(|ext| ext != "coffee" && ext != "civet" && ext != "rew")
      || source.starts_with("\"no-compile\"")
    {
      // self.declaration_engine.process_script(source);
      return Ok(source.to_string());
    }

    let local_declarations = self.declaration_engine.process_script(source);

    let global_declarations = self.declaration_engine.global_declarations.clone();

    let file_id = filepath
      .to_str()
      .unwrap_or("unknown")
      .replace(|c: char| !c.is_ascii_alphanumeric(), "_");

    let processed = self.preprocess_rew(
      source,
      local_declarations,
      global_declarations,
      keep_imports,
    )?;

    let mut civet_options: Vec<String> = vec![];
    civet_options.extend(processed.options.civet_options.clone());
    civet_options.extend(self.compile_options.clone());

    let code = format!(
      r#"
    (() => {{
      let options = {{
        coffeePrototype: true,
        autoLet: true,
        coffeeInterpolation: true,
        coffeeComment: true
      }};
      const forbidden = ["JSX"];
      ("{civet_options}").split(',').filter(i => !forbidden.includes(i)).map(i => {{
        if(i.indexOf('.')){{
          let [k, v] = i.split('.');
          options[k] = v == 'off' || v == 'disable' ? false : true
          if(options[k] == false) delete options[k];
        }} else {{
          if(i in options) options[i] = false;
          else options[i] = true; 
        }}
      }});
      let _compiled = compile({file_id}, {{
        parseOptions: options,
        sync: true,
        filename: '{file}.civet',
        bare: true,
        js: true,
        inlineMap: {inp},
        sourceMap: {smp},
      }});

      delete globalThis.{file_id};

      return _compiled;
    }})()
    "#,
      file_id = file_id,
      file = filepath
        .to_str()
        .unwrap_or("unknown")
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('"', "\\\""),
      smp = self.sourcemap,
      inp = self.inlinemap,
      civet_options = civet_options.join(",")
    );

    {
      let scope = &mut self.runtime.handle_scope();

      let context = scope.get_current_context();
      let global = context.global(scope);

      let key_str = deno_core::v8::String::new(scope, file_id.as_str()).unwrap();
      let val_str = deno_core::v8::String::new(scope, processed.code.as_str()).unwrap();

      global.set(scope, key_str.into(), val_str.into());
    }

    let result = self.runtime.execute_script("<rew>", code.clone())?;
    // let compiled = self.runtime.resolve(result).await?;
    let scope = &mut self.runtime.handle_scope();
    let mut result_code = result.open(scope).to_rust_string_lossy(scope);

    if processed.options.jsx || civet_options.contains(&"JSX".to_string()) {
      result_code = compile_jsx(result_code, Some("JSX.prototype.new".to_string()));
    }

    // if processed.options.civet_global {
    self
      .compile_options
      .extend(processed.options.civet_global.clone());
    // }

    Ok(result_code)
  }

  fn preprocess_rew(
    &mut self,
    source: &str,
    local_declarations: HashMap<String, Declaration>,
    global_declarations: HashMap<String, Declaration>,
    keep_imports: bool,
  ) -> Result<CompilerResults> {
    let mut options = CompilerOptions {
      keep_imports,
      civet_global: vec![],
      jsx: false,
      civet_options: vec![],
      cls: false,
      included: false,
      local_declarations,
      global_declarations,
    };

    compile_rew_stuff(source, &mut options)
  }

  pub async fn run_file<P: AsRef<Path>>(&mut self, filepath: P) -> Result<()> {
    let filepath = filepath
      .as_ref()
      .canonicalize()
      .with_context(|| format!("Failed to resolve file path: {:?}", filepath.as_ref()))?;

    let files_with_flags = RewRuntime::resolve_includes_recursive_from(&filepath)?;

    for (_, content, preprocess) in &files_with_flags {
      if *preprocess {
        let local_declarations = self.declaration_engine.process_script(content);

        for (name, decl) in local_declarations {
          self
            .declaration_engine
            .global_declarations
            .insert(name, decl);
        }
      }
    }

    if let Some(app_info) = rew_core::utils::find_app_info(&filepath) {
      if let Some(manifest) = &app_info.config.manifest {
        if let Some(package) = &manifest.package {
          self.runtime.execute_script(
            "<app-recognition>",
            format!(
              "globalThis[\"__app__{}\"] = \"{}\";",
              package,
              app_info.path.to_str().unwrap()
            ),
          )?;
        }
      }
    }

    let files: Vec<(PathBuf, String)> = files_with_flags
      .into_iter()
      .map(|(path, content, _)| (path, content))
      .collect();

    self
      .include_and_run(files, &get_storage_path(filepath.to_str().unwrap()))
      .await?;

    Ok(())
  }
}

impl Drop for RewRuntime {
  fn drop(&mut self) {}
}

#[op2]
#[serde]
fn op_get_args(state: Rc<RefCell<OpState>>) -> Result<serde_json::Value, CoreError> {
  let state = state.borrow();
  let runtime_args = state.borrow::<RuntimeState>();
  Ok(serde_json::json!(runtime_args.args.clone()))
}

#[op2]
#[serde]
fn op_fs_read(
  #[string] current_file: String,
  #[string] filepath: String,
  #[serde] options: Option<ReadOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));
  let full_path = base_dir.join(filepath);

  let options = options.unwrap_or_default();

  if options.binary {
    let mut file = File::open(&full_path).map_err(CoreError::Io)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(CoreError::Io)?;

    Ok(serde_json::Value::Array(
      buffer
        .into_iter()
        .map(|b| serde_json::Value::Number(b.into()))
        .collect(),
    ))
  } else {
    let content = fs::read_to_string(&full_path).map_err(CoreError::Io)?;
    Ok(serde_json::Value::String(content))
  }
}

#[derive(Deserialize, Default)]
struct ReadOptions {
  binary: bool,
}

#[op2(async)]
async fn op_fs_write(
  #[string] current_file: String,
  #[string] filepath: String,
  #[serde] content: serde_json::Value,
  #[serde] options: Option<WriteOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  let options = options.unwrap_or_default();

  if let Some(parent) = full_path.parent() {
    if options.create_dirs {
      fs::create_dir_all(parent).map_err(CoreError::Io)?;
    }
  }

  if options.binary {
    if let serde_json::Value::Array(bytes) = content {
      let buffer: Result<Vec<u8>, _> = bytes
        .iter()
        .map(|v| {
          if let serde_json::Value::Number(n) = v {
            n.as_u64().map(|n| n as u8).ok_or_else(|| {
              CoreError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid byte value",
              ))
            })
          } else {
            Err(CoreError::Io(io::Error::new(
              io::ErrorKind::InvalidData,
              "Expected number in byte array",
            )))
          }
        })
        .collect();

      fs::write(&full_path, buffer?).map_err(CoreError::Io)?;
    } else {
      return Err(CoreError::Io(io::Error::new(
        io::ErrorKind::InvalidData,
        "Expected array of bytes for binary write",
      )));
    }
  } else if let serde_json::Value::String(text) = content {
    let mut file = File::create(&full_path).map_err(CoreError::Io)?;
    file.write_all(text.as_bytes()).map_err(CoreError::Io)?;
  } else {
    return Err(CoreError::Io(io::Error::new(
      io::ErrorKind::InvalidData,
      "Expected string for text write",
    )));
  }

  Ok(())
}

#[derive(Deserialize, Default)]
struct WriteOptions {
  binary: bool,
  create_dirs: bool,
}

use sha2::{Digest, Sha256};
#[op2]
#[string]
fn op_fs_sha(
  #[string] current_file: String,
  #[string] filepath: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  let file_bytes = fs::read(&full_path)?;
  let mut hasher = Sha256::new();
  hasher.update(file_bytes);
  let hash = hasher.finalize();

  Ok(format!("{:x}", hash))
}

#[op2(fast)]
fn op_fs_exists(
  #[string] current_file: String,
  #[string] filepath: String,
  _: Rc<RefCell<OpState>>,
) -> Result<bool, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  Ok(full_path.exists())
}

#[op2(async)]
async fn op_fs_rm(
  #[string] current_file: String,
  #[string] filepath: String,
  #[serde] options: Option<RemoveOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  let options = options.unwrap_or_default();

  if full_path.is_dir() {
    if options.recursive {
      fs::remove_dir_all(&full_path).map_err(CoreError::Io)?;
    } else {
      fs::remove_dir(&full_path).map_err(CoreError::Io)?;
    }
  } else {
    fs::remove_file(&full_path).map_err(CoreError::Io)?;
  }

  Ok(())
}

#[derive(Deserialize, Default)]
struct RemoveOptions {
  recursive: bool,
}

#[op2(async)]
async fn op_fs_mkdir(
  #[string] current_file: String,
  #[string] dirpath: String,
  #[serde] options: Option<MkdirOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(dirpath);

  let options = options.unwrap_or_default();

  if options.recursive {
    fs::create_dir_all(&full_path).map_err(CoreError::Io)?;
  } else {
    fs::create_dir(&full_path).map_err(CoreError::Io)?;
  }

  Ok(())
}

#[derive(Deserialize, Default)]
struct MkdirOptions {
  recursive: bool,
}

#[op2]
#[string]
fn op_fs_readdir(
  #[string] current_file: String,
  #[string] dirpath: String,
  #[serde] options: Option<ReaddirOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(dirpath);

  let options = options.unwrap_or_default();

  let entries = fs::read_dir(&full_path).map_err(CoreError::Io)?;

  let mut result = Vec::new();

  for entry in entries {
    let entry = entry.map_err(CoreError::Io)?;
    let file_type = entry.file_type().map_err(CoreError::Io)?;

    if !options.include_hidden {
      if let Some(file_name) = entry.path().file_name() {
        if let Some(name_str) = file_name.to_str() {
          if name_str.starts_with(".") {
            continue;
          }
        }
      }
    }

    if let Some(filter_type) = &options.filter_type {
      match filter_type.as_str() {
        "file" => {
          if !file_type.is_file() {
            continue;
          }
        }
        "directory" => {
          if !file_type.is_dir() {
            continue;
          }
        }
        "symlink" => {
          if !file_type.is_symlink() {
            continue;
          }
        }
        _ => {}
      }
    }

    let metadata = entry.metadata().map_err(CoreError::Io)?;

    let entry_info = DirEntryInfo {
      name: entry.file_name().to_string_lossy().to_string(),
      path: entry.path().to_string_lossy().to_string(),
      is_file: file_type.is_file(),
      is_directory: file_type.is_dir(),
      is_symlink: file_type.is_symlink(),
      size: metadata.len(),
      modified: metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs()),
      created: metadata
        .created()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs()),
    };

    result.push(entry_info);
  }

  if let Some(sort_by) = &options.sort_by {
    match sort_by.as_str() {
      "name" => result.sort_by(|a, b| a.name.cmp(&b.name)),
      "size" => result.sort_by(|a, b| a.size.cmp(&b.size)),
      "modified" => result.sort_by(|a, b| a.modified.cmp(&b.modified)),
      "type" => result.sort_by(|a, b| a.is_directory.cmp(&b.is_directory).reverse()),
      _ => {}
    }
  }

  serde_json::to_string(&result).map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[derive(Deserialize, Default)]
struct ReaddirOptions {
  include_hidden: bool,
  filter_type: Option<String>,
  sort_by: Option<String>,
}

#[derive(Serialize)]
struct DirEntryInfo {
  name: String,
  path: String,
  is_file: bool,
  is_directory: bool,
  is_symlink: bool,
  size: u64,
  modified: Option<u64>,
  created: Option<u64>,
}

#[op2]
#[string]
fn op_fs_stats(
  #[string] current_file: String,
  #[string] filepath: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let full_path = base_dir.join(filepath);

  let metadata = fs::metadata(&full_path).map_err(CoreError::Io)?;

  let stats = serde_json::json!({
      "isFile": metadata.is_file(),
      "isDirectory": metadata.is_dir(),
      "isSymlink": metadata.file_type().is_symlink(),
      "size": metadata.len(),
      "modified": metadata.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()),
      "created": metadata.created().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()),
      "accessed": metadata.accessed().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()),
      "permissions": {
          "readonly": metadata.permissions().readonly(),
          // "mode": metadata.permissions().mode(),
      }
  });

  Ok(stats.to_string())
}

#[op2(async)]
async fn op_fs_copy(
  #[string] current_file: String,
  #[string] src: String,
  #[string] dest: String,
  #[serde] options: Option<CopyOptions>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let src_path = base_dir.join(src);
  let dest_path = base_dir.join(dest);

  let options = options.unwrap_or_default();

  if src_path.is_dir() {
    if options.recursive {
      copy_dir_recursive(&src_path, &dest_path, &options).map_err(CoreError::Io)?;
    } else {
      return Err(CoreError::Io(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Source is a directory, but recursive option is not set",
      )));
    }
  } else {
    if let Some(parent) = dest_path.parent() {
      if options.create_dirs {
        fs::create_dir_all(parent).map_err(CoreError::Io)?;
      }
    }

    fs::copy(&src_path, &dest_path).map_err(CoreError::Io)?;
  }

  Ok(())
}

#[derive(Deserialize, Default)]
struct CopyOptions {
  recursive: bool,
  create_dirs: bool,
  overwrite: bool,
}

fn copy_dir_recursive(src: &Path, dest: &Path, options: &CopyOptions) -> io::Result<()> {
  if !dest.exists() {
    fs::create_dir_all(dest)?;
  }

  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let src_path = entry.path();
    let dest_path = dest.join(entry.file_name());

    if src_path.is_dir() {
      copy_dir_recursive(&src_path, &dest_path, options)?;
    } else {
      if dest_path.exists() && !options.overwrite {
        continue;
      }
      fs::copy(&src_path, &dest_path)?;
    }
  }

  Ok(())
}

#[op2(async)]
async fn op_fs_rename(
  #[string] current_file: String,
  #[string] src: String,
  #[string] dest: String,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let current_file_path = Path::new(&current_file);
  let base_dir = current_file_path.parent().unwrap_or(Path::new("."));

  let src_path = base_dir.join(src);
  let dest_path = base_dir.join(dest);

  fs::rename(&src_path, &dest_path).map_err(CoreError::Io)?;

  Ok(())
}

#[op2]
#[string]
fn op_fs_cwdir(state: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  let state = state.borrow();
  let runtime_state = state.borrow::<RuntimeState>();

  Ok(runtime_state.current_dir.to_string_lossy().to_string())
}

// Base64 encoding/decoding operations
#[op2]
#[string]
fn op_to_base64(#[serde] data: serde_json::Value) -> Result<String, CoreError> {
  match data {
    serde_json::Value::String(text) => Ok(BASE64.encode(text.as_bytes())),
    serde_json::Value::Array(bytes) => {
      let buffer: Result<Vec<u8>, _> = bytes
        .iter()
        .map(|v| {
          if let serde_json::Value::Number(n) = v {
            n.as_u64().map(|n| n as u8).ok_or_else(|| {
              CoreError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid byte value",
              ))
            })
          } else {
            Err(CoreError::Io(io::Error::new(
              io::ErrorKind::InvalidData,
              "Expected number in byte array",
            )))
          }
        })
        .collect();

      match buffer {
        Ok(bytes) => Ok(BASE64.encode(bytes)),
        Err(e) => Err(e),
      }
    }
    _ => Err(CoreError::Io(io::Error::new(
      io::ErrorKind::InvalidData,
      "Expected string or array of bytes for base64 encoding",
    ))),
  }
}

#[op2]
#[serde]
fn op_from_base64(
  #[string] encoded: String,
  #[serde] options: Option<Base64DecodeOptions>,
) -> Result<serde_json::Value, CoreError> {
  let options = options.unwrap_or_default();

  let decoded = BASE64
    .decode(encoded.as_bytes())
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  if options.as_string {
    let text = String::from_utf8(decoded)
      .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;
    Ok(serde_json::Value::String(text))
  } else {
    Ok(serde_json::Value::Array(
      decoded
        .into_iter()
        .map(|b| serde_json::Value::Number(b.into()))
        .collect(),
    ))
  }
}

#[derive(Deserialize, Default)]
struct Base64DecodeOptions {
  as_string: bool,
}

#[op2]
#[string]
fn op_find_app(#[string] filepath: String, _: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  let current_file = Path::new(&filepath);

  let app_path = find_app_path(current_file);

  Ok(String::from(
    app_path.unwrap_or(PathBuf::from("")).to_str().unwrap(),
  ))
}

#[op2]
#[string]
fn op_yaml_to_string(
  #[serde] data: serde_json::Value,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let yaml = serde_yaml::to_string(&data)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  Ok(yaml)
}

#[op2]
#[serde]
fn op_string_to_yaml(
  #[string] yaml_str: String,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let value: serde_json::Value = serde_yaml::from_str(&yaml_str)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  Ok(value)
}

#[op2]
#[serde]
fn op_app_loadconfig(
  #[string] app_path: String,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let app_path = Path::new(&app_path);

  if !app_path.exists() {
    return Err(CoreError::Io(io::Error::new(
      io::ErrorKind::NotFound,
      format!("App path not found: {}", app_path.display()),
    )));
  }

  let config_path = app_path.join("app.yaml");

  if !config_path.exists() {
    return Err(CoreError::Io(io::Error::new(
      io::ErrorKind::NotFound,
      format!("App config not found: {}", config_path.display()),
    )));
  }

  let config_str = fs::read_to_string(&config_path)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;

  let config: serde_json::Value = serde_yaml::from_str(&config_str)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

  Ok(config)
}

// Helper function to get DataManager for a specific app package
fn get_data_manager_for_package(app_package: &str) -> Result<DataManager, CoreError> {
  // For now, use "default" as the user ID
  // In a real implementation, you'd get this from user authentication
  let user_id = "default";

  DataManager::new(user_id, app_package)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[string]
fn op_data_read(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .read(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(async)]
async fn op_data_write(
  #[string] app_package: String,
  #[string] key: String,
  #[string] content: String,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .write(&key, &content)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(async)]
async fn op_data_delete(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .delete(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(fast)]
fn op_data_exists(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<bool, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  Ok(data_manager.exists(&key))
}

#[op2]
#[string]
fn op_data_list(
  #[string] app_package: String,
  #[string] prefix: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  let files = data_manager
    .list(&prefix)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;

  serde_json::to_string(&files).map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[serde]
fn op_data_read_binary(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<Vec<u8>, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .read_binary(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[string]
fn op_fetch_env(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  let env_vars: HashMap<String, String> = std::env::vars().collect();
  let cwd = std::env::current_dir()
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?
    .to_string_lossy()
    .to_string();
  let exec_path = std::env::current_exe()
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?
    .to_string_lossy()
    .to_string();

  let result = serde_json::json!({
    "env": env_vars,
    "cwd": cwd,
    "execPath": exec_path,
    "tempDir": std::env::temp_dir(),
    "rewPath": rew_core::utils::get_rew_root()
  });

  serde_json::to_string(&result)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))
}

#[op2(async)]
async fn op_data_write_binary(
  #[string] app_package: String,
  #[string] key: String,
  #[serde] data: Vec<u8>,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .write_binary(&key, &data)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[serde]
fn op_data_read_yaml(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .read_yaml(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2(async)]
async fn op_data_write_yaml(
  #[string] app_package: String,
  #[string] key: String,
  #[serde] data: serde_json::Value,
  _: Rc<RefCell<OpState>>,
) -> Result<(), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  data_manager
    .write_yaml(&key, &data)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))
}

#[op2]
#[serde]
fn op_data_get_info(
  #[string] app_package: String,
  #[string] key: String,
  _: Rc<RefCell<OpState>>,
) -> Result<(bool, String), CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;
  let (exists, format) = data_manager
    .get_file_info(&key)
    .map_err(|e| CoreError::Io(io::Error::new(io::ErrorKind::Other, e)))?;

  let format_str = match format {
    DataFormat::Text => "text",
    DataFormat::Json => "json",
    DataFormat::Yaml => "yaml",
    DataFormat::Binary => "binary",
  };

  Ok((exists, format_str.to_string()))
}

#[op2]
#[string]
fn op_data_get_path(
  #[string] app_package: String,
  _: Rc<RefCell<OpState>>,
) -> Result<String, CoreError> {
  let data_manager = get_data_manager_for_package(&app_package)?;

  Ok(data_manager.get_path("").to_string_lossy().to_string())
}

#[op2]
#[string]
fn op_os_info_os(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  Ok(std::env::consts::OS.to_string())
}

#[op2]
#[string]
fn op_os_info_arch(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  Ok(std::env::consts::ARCH.to_string())
}

#[op2]
#[string]
fn op_os_info_family(_: Rc<RefCell<OpState>>) -> Result<String, CoreError> {
  Ok(std::env::consts::FAMILY.to_string())
}

#[op2(async, reentrant)]
#[serde]
async fn op_dyn_imp(
  #[string] current_file: String,
  #[string] file: String,
  _: Rc<RefCell<OpState>>,
) -> Result<serde_json::Value, CoreError> {
  let file_path = if current_file == "/" {
    Path::new(&file).to_path_buf()
  } else {
    let current_file_path = Path::new(&current_file);
    let base_dir = current_file_path.parent().unwrap_or(Path::new("."));
    base_dir.join(file)
  };

  let mut runtime = RewRuntime::new(None, None)
    .map_err(|_| CoreError::Io(io::Error::new(io::ErrorKind::NotFound, "")))?;

  let files_with_flags = RewRuntime::resolve_includes_recursive_from(file_path.clone())
    .map_err(|_| CoreError::Io(io::Error::new(io::ErrorKind::NotFound, "")))?;
  for (_, content, preprocess) in &files_with_flags {
    if *preprocess {
      let local_declarations = runtime.declaration_engine.process_script(content);

      for (name, decl) in local_declarations {
        runtime
          .declaration_engine
          .global_declarations
          .insert(name, decl);
      }
    }
  }

  let files: Vec<(PathBuf, String)> = files_with_flags
    .into_iter()
    .map(|(path, content, _)| (path, content))
    .collect();

  let prepared = runtime
    .prepare(files, None)
    .await
    .map_err(|_| CoreError::Io(io::Error::new(io::ErrorKind::NotFound, "")))?;

  let fp = fs::canonicalize(&file_path)
    .map_err(|_| CoreError::Io(io::Error::new(io::ErrorKind::NotFound, "")))?;

  Ok(serde_json::json!(vec![
    fp.to_string_lossy().to_string(),
    prepared
  ]))
}

use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng, distributions::Alphanumeric};
use std::hash::Hash;
use std::hash::Hasher;

#[op2]
#[serde]
fn op_rand_from(
  #[bigint] min: usize,
  #[bigint] max: usize,
  #[string] seed: Option<String>,
) -> usize {
  let mut rng: Box<dyn RngCore> = match seed {
    Some(s) => {
      let mut hasher = std::collections::hash_map::DefaultHasher::new();
      s.hash(&mut hasher);
      Box::new(StdRng::seed_from_u64(hasher.finish()))
    }
    _ => Box::new(rand::thread_rng()),
  };

  if min == max {
    return min;
  }

  let (low, high) = if min < max { (min, max) } else { (max, min) };

  rng.gen_range(low..=high)
}

#[op2]
#[string]
fn op_vfile_set(#[string] full_path: String, #[string] content: String) -> String {
  add_virtual_file(full_path.as_str(), content.as_str());
  "".to_string()
}

#[op2]
#[string]
fn op_vfile_get(#[string] full_path: String) -> String {
  if let Some(v) = VIRTUAL_FILES
    .lock()
    .unwrap()
    .iter()
    .find(|(p, _)| *p == full_path)
  {
    return v.1.clone();
  }
  "".to_string()
}

#[op2]
#[string]
fn op_gen_uid(length: i32, #[string] seed: Option<String>) -> String {
  if let Some(seed_str) = seed {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    seed_str.hash(&mut hasher);

    let seed = hasher.finish();
    let mut rng = StdRng::seed_from_u64(seed);

    (0..length)
      .map(|_| rng.sample(Alphanumeric) as char)
      .collect()
  } else {
    let mut rng = rand::thread_rng();

    (0..length)
      .map(|_| rng.sample(Alphanumeric) as char)
      .collect()
  }
}

#[op2]
#[serde]
fn op_terminal_size() -> Result<(u16, u16), std::io::Error> {
  #[cfg(unix)]
  {
    use libc::{STDOUT_FILENO, TIOCGWINSZ, ioctl, winsize};

    let mut ws: winsize = unsafe { std::mem::zeroed() };

    let result = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) };

    if result == -1 {
      return Err(std::io::Error::last_os_error());
    }

    Ok((ws.ws_col, ws.ws_row))
  }

  #[cfg(windows)]
  {
    use std::mem::zeroed;
    use std::ptr::null_mut;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFO, GetConsoleScreenBufferInfo};

    unsafe {
      let handle = GetStdHandle(STD_OUTPUT_HANDLE);
      if handle == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error());
      }

      let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = zeroed();
      if GetConsoleScreenBufferInfo(handle, &mut csbi) == 0 {
        return Err(std::io::Error::last_os_error());
      }

      let width = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16;
      let height = (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16;

      Ok((width, height))
    }
  }
}
