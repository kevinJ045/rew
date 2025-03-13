extern crate deno_core;

use deno_core::{JsRuntime, RuntimeOptions, serde_v8, v8};
use std::rc::Rc;

use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::collections::HashMap;
use std::thread;
use std::sync::Arc;
use std::env;

mod civet;

use civet::get_civet_script;

struct ContextManager {
    runtime: JsRuntime,
}

impl ContextManager {
    fn new() -> Self {
        let runtime = JsRuntime::new(RuntimeOptions::default());
        ContextManager { runtime }
    }

    fn get_runtime(&mut self) -> &mut JsRuntime {
        &mut self.runtime
    }

    fn define_context_props(&mut self) {
        let runtime = self.get_runtime();

        // Register global functions and objects here
        runtime.execute_script("<anon>", r#"
            globalThis.Drw = {
                prototype: {
                    core: {},
                    fs: {},
                    process: {},
                    io: {},
                    namespace: {},
                    env: { data: {} }
                }
            };
        "#).unwrap();
    }
}

struct PodManager {
    cman: ContextManager,
}

impl PodManager {
    fn new() -> Self {
        let cman = ContextManager::new();
        PodManager { cman }
    }

    fn set_current_filepath(&mut self, filename: &str, is_main: bool) -> anyhow::Result<()> {
        let runtime = self.cman.get_runtime();
        let context = runtime.handle_scope().get_current_context();

        let scope = &mut runtime.handle_scope();
        let v8_context = v8::Local::new(scope, context);

        let filepath = v8::String::new(scope, filename).unwrap();
        let is_main = v8::Boolean::new(scope, is_main);
        let pid = v8::Integer::new(scope, std::process::id() as i32);

        let obj = v8::Object::new(scope);
        obj.set(scope, v8::String::new(scope, "filepath").unwrap().into(), filepath.into());
        obj.set(scope, v8::String::new(scope, "main").unwrap().into(), is_main.into());
        obj.set(scope, v8::String::new(scope, "pid").unwrap().into(), pid.into());

        let drw: v8::Local<v8::Object> = v8_context.global(scope);

        drw.set(scope, v8::String::new(scope, "current").unwrap().into(), obj.into());

        Ok(())
    }

    fn execute(&mut self, filename: &str, is_main: bool) -> anyhow::Result<serde_v8::Value> {
        let code = std::fs::read_to_string(filename)?;
        self.execute_string(&code, filename, is_main)
    }

    fn execute_string(&mut self, code: &str, filename: &str, is_main: bool) -> anyhow::Result<serde_v8::Value> {
        self.set_current_filepath(filename, is_main)?;
        let runtime = self.cman.get_runtime();
        let result = runtime.execute_script(filename, code)?;
        Ok(result)
    }
}

struct RuntimeManager {
    pman: PodManager,
}

impl RuntimeManager {
    fn new() -> Self {
        let pman = PodManager::new();
        RuntimeManager { pman }
    }

    fn srun(filename: &str, func: Option<Box<dyn FnOnce(&mut PodManager)>>) -> anyhow::Result<serde_v8::Value> {
        let mut r = RuntimeManager::new();
        r.run_file(filename, func)
    }

    fn run_file(&mut self, filename: &str, func: Option<Box<dyn FnOnce(&mut PodManager)>>) -> anyhow::Result<serde_v8::Value> {
        let code = if filename.ends_with(".coffee") || filename.ends_with(".civet") {
            CompileManager::compile(filename)?
        } else {
            std::fs::read_to_string(filename)?
        };

        let result = self.pman.execute_string(&code, filename, true)?;

        if let Some(func) = func {
            func(&mut self.pman);
        }

        Ok(result)
    }
}

struct CompileManager;

impl CompileManager {
    fn compile(filename: &str) -> anyhow::Result<String> {
        let mut pman = PodManager::new();
        let code_raw = std::fs::read_to_string(filename)?;
        pman.cman.runtime.execute_script("<anon>", &format!("globalThis.__to_compile__ = `{}`;", code_raw))?;
        let compiled = pman.execute_string(get_civet_script(), "system::compiler", false)?;
        Ok(compiled.to_string())
    }
}

fn main() {
    RuntimeManager::srun("/home/makano/workspace/rew-rust/test/s.coffee", None)?;
}
