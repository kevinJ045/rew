pub mod builtins;
pub mod runtime_script;
pub mod workers;

use crate::builtins::BUILTIN_MODULES;
use crate::runtime_script::get_runtime_script;
use crate::workers::{
  op_thread_message, op_thread_post_message, op_thread_receive, op_thread_spawn,
  op_thread_terminate,
};
use anyhow::{Context, Result};
use deno_core::OpState;
use deno_core::PollEventLoopOptions;
use deno_core::error::{CoreError, CoreErrorKind};
use deno_core::{JsRuntime, RuntimeOptions, extension, op2};
use deno_fs::{FileSystem, RealFs};
use deno_permissions::PermissionsContainer;
use futures::stream::{self, StreamExt};
use regex::Regex;
use rew_brew::{decode_brew_file, encode_brew_file};
use rew_compiler::{
  CompilerOptions, CompilerResults, Declaration, DeclarationEngine, compile_jsx, compile_rew_stuff,
  get_civet_script,
};
use rew_core::{BuildOptions, RuntimeState};
use rew_extensions::ext::{console, ffi, process, url, web, webidl};
use rew_ops::*;
use rew_permissions::TestPermissionDescriptorParser;
use rew_utils::is_js_executable;
use rew_vfile::VIRTUAL_FILES;
pub use rew_vfile::add_virtual_file;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Default)]
pub struct RuntimeArgs(pub Vec<String>);

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
    op_fs_sha,
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
    op_rand_from,
    op_gen_uid,
    op_vfile_set,
    op_vfile_get,
    op_terminal_size,
    op_p_exit,
    op_p_panic,
    op_p_sleep,
    op_async_to_sync,
    op_start_loop,
    op_stop_loop,
    op_p_loop
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
  extensions.extend(rew_extensions::ext::telemetry::extensions(false));
  extensions.extend(ffi::extensions(false));
  extensions.extend(rew_extensions::ext::tls::extensions(false));
  // extensions.extend(rew_extensions::ext::networking::extensions(false));
  // extensions.extend(rew_extensions::ext::http::extensions(false));
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
    let filepath = if filepath.as_ref().starts_with("/internal/") {
      PathBuf::from(filepath.as_ref())
    } else {
      filepath
        .as_ref()
        .canonicalize()
        .with_context(|| format!("Failed to resolve file path: {:?}", filepath.as_ref()))?
    };

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
          if entry
            .unwrap_or(Path::new(""))
            .to_str()
            .unwrap_or("")
            .to_string()
            == entry_file.replace(".coffee", ".brew")
          {
            entry_calls.push(format!(
              "rew.prototype.mod.prototype.get('{}');",
              entry_file.replace('\\', "\\\\")
            ));
          }
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
    let filepath = if filepath.as_ref().starts_with("/internal/") {
      PathBuf::from(filepath.as_ref())
    } else {
      filepath
        .as_ref()
        .canonicalize()
        .with_context(|| format!("Failed to resolve file path: {:?}", filepath.as_ref()))?
    };

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
    .map_err(|_| CoreErrorKind::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "")))?;

  let files_with_flags = RewRuntime::resolve_includes_recursive_from(file_path.clone())
    .map_err(|_| CoreErrorKind::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "")))?;
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
    .map_err(|_| CoreErrorKind::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "")))?;

  let fp = fs::canonicalize(&file_path)
    .map_err(|_| CoreErrorKind::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "")))?;

  Ok(serde_json::json!([
    fp.to_string_lossy().to_string(),
    prepared
  ]))
}
