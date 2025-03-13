use deno_core::{ JsRuntime, RuntimeOptions };
use std::path::Path;
use anyhow::Result;

use crate::compiler::CompilerResults;

use super::compiler::{ compile_rew_stuff, CompilerOptions };
use super::civet::get_civet_script;

pub struct RewRuntime {
  compiler_runtime: Option<JsRuntime>,
  runtime: Option<JsRuntime>,
}

// i love this one, though
impl RewRuntime {
  pub fn new() -> Result<Self> {
    let mut compiler_runtime = JsRuntime::new(RuntimeOptions::default());
    let runtime = JsRuntime::new(RuntimeOptions::default());
		
    compiler_runtime.execute_script("<civet>", get_civet_script())?;

    Ok(Self {
      compiler_runtime: Some(compiler_runtime),
      runtime: Some(runtime),
    })
  }

  pub async fn execute(&mut self, js_code: &String) -> Result<()> {
    if let Some(runtime) = &mut self.runtime {
      let result = runtime.execute_script("<exec>", js_code.clone())?;
      runtime.resolve(result).await?;
    }
    Ok(())
  }

  pub async fn compile_and_run(&mut self, source: &str, filepath: &Path) -> Result<String> {
    let processed = self.preprocess_rew(source)?;

    let code = format!(r#"
			const result = compile(`{}`);
			result
			"#, processed.code.replace("`", "\\`"));

    if let Some(compiler_runtime) = &mut self.compiler_runtime {
      let result = compiler_runtime.execute_script("<rew>", code.clone())?;
      let compiled = compiler_runtime.resolve_value(result).await?;
      let scope = &mut compiler_runtime.handle_scope();
      let result_code = compiled.open(scope).to_rust_string_lossy(scope);
      Ok(result_code)
    } else {
      Err(anyhow::anyhow!("Compiler runtime not available"))
    }
  }

  fn preprocess_rew(&mut self, source: &str) -> Result<CompilerResults> {
    let mut options = CompilerOptions::default();
    compile_rew_stuff(source, &mut options)
  }
}
impl Drop for RewRuntime {
  fn drop(&mut self) {
    // Drop in reverse order
    self.runtime.take();
    self.compiler_runtime.take();
  }
}
