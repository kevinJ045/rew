use std::collections::HashMap;
use deno_core::{ extension, op2, Extension, JsRuntime, RuntimeOptions };
use std::path::{ Path, PathBuf };
use anyhow::Result;
use std::rc::Rc;

use crate::compiler::CompilerResults;
use super::compiler::{ compile_rew_stuff, CompilerOptions };
use super::civet::get_civet_script;

pub struct RewRuntime {
  compiler_runtime: JsRuntime,
  runtimes: HashMap<PathBuf, JsRuntime>,
  current_dir: PathBuf,
}

#[op2(async)]
#[string]
async fn op_inc(
  #[string] url: String
) -> String {
  let mut newUrl: String = String::new();
  newUrl.push_str("hello world");
  newUrl.push_str(&url);
  println!("{}", newUrl);
  return url;
}


extension!(
  rewextension,
  ops = [
    op_inc,
  ]
);


struct RuntimeState {
  runtime: Rc<RewRuntime>,
  current_dir: PathBuf,
}

impl RewRuntime {
  pub fn new() -> Result<Self> {
    let mut compiler_runtime = JsRuntime::new(RuntimeOptions::default());
    compiler_runtime.execute_script("<civet>", get_civet_script())?;

    Ok(Self {
      compiler_runtime,
      runtimes: HashMap::new(),
      current_dir: std::env::current_dir()?,
    })
  }

  pub async fn execute_in_runtime(&mut self, filepath: &Path, js_code: &str) -> Result<()> {
    let script_name: String = filepath.to_str().unwrap_or("<exec>").to_string();
    let runtime = self.get_or_create_runtime(&filepath)?;
    let result = runtime.execute_script(
      Box::leak(script_name.into_boxed_str()),
      js_code.to_string()
    )?;
    runtime.resolve(result).await?;
    Ok(())
  }

  pub async fn execute(&mut self, js_code: &str) -> Result<()> {
    let runtime = self.get_or_create_runtime(Path::new("<exec>"))?;
    let result = runtime.execute_script(
      "<exec>",
      js_code.to_string()
    )?;
    runtime.resolve(result).await?;
    Ok(())
  }

  fn get_or_create_runtime(&mut self, filepath: &Path) -> Result<&mut JsRuntime> {
    if !self.runtimes.contains_key(filepath) {
      let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![rewextension::init_ops()],
        ..Default::default()
      });
      
      runtime.execute_script("<setup>", r#"
        globalThis.Rew = {...globalThis.Deno};
        delete globalThis.Deno;
        globalThis.inc = function(path) {
          return Deno.core.ops.op_inc(path);
        };
      "#)?;

      self.runtimes.insert(filepath.to_path_buf(), runtime);
    }
    
    Ok(self.runtimes.get_mut(filepath).unwrap())
  }

  pub async fn compile_and_run(&mut self, source: &str, filepath: &Path) -> Result<String> {
    let processed = self.preprocess_rew(source)?;

    let code = format!(
      r#"
            const result = compile(`{}`);
            result
        "#,
      processed.code.replace("`", "\\`")
    );

    let result = self.compiler_runtime.execute_script("<rew>", code.clone())?;
    let compiled = self.compiler_runtime.resolve_value(result).await?;
    let scope = &mut self.compiler_runtime.handle_scope();
    let result_code = compiled.open(scope).to_rust_string_lossy(scope);

    Ok(result_code)
  }

  fn preprocess_rew(&mut self, source: &str) -> Result<CompilerResults> {
    let mut options = CompilerOptions::default();
    compile_rew_stuff(source, &mut options)
  }
}

impl Drop for RewRuntime {
  fn drop(&mut self) {
    self.runtimes.clear();
  }
}
