use deno_core::v8::Local;
use deno_core::{JsRuntime, RuntimeOptions, v8, OpState};
use deno_core::error::AnyError;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;

// Rust function that reads a file and returns it as a Vec<u8>
fn read_file(
    _state: &mut OpState,
    filename: String,
    _: (),
) -> Result<Vec<u8>, AnyError> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

// Rust function for `Drw.core.out.println`
fn print_ln(
  _state: &mut OpState, 
  text: String,
  _: (),
) -> Result<(), ()> {
  println!("{}", text); // Print with a newline
  Ok(())
}

fn stringify<'s>(scope: &mut v8::HandleScope<'s>, text: &str) -> v8::Local<'s, v8::String> {
  v8::String::new(scope, &text).unwrap()
}

// Rust function for `Drw.core.out.print`
fn print(
  _state: &mut OpState,
  text: String,
  _: (),
) -> Result<(), ()> {
  print!("{}", text); // Print without a newline
  Ok(())
}

fn runIt() {
    // Create a new JavaScript runtime
    let mut runtime = JsRuntime::new(RuntimeOptions::default());

    {

    // Access the V8 Isolate and global scope
    let scope = &mut runtime.handle_scope();

    // Access the global object
    let global = scope.get_current_context().global(scope);

    // Create a new `Drw` object
    let drw = v8::Object::new(scope);

    // Create a new `core` object for `Drw.core`
    let core = v8::Object::new(scope);

    // Create a new `out` object for `Drw.core.out`
    let out = v8::Object::new(scope);

    // Create a Rust function to be bound to `Drw.core.readFile`
    let read_file_fn = v8::Function::new(scope, read_file_wrapper).unwrap();

    // Create a Rust function for `println`
    let println_fn = v8::Function::new(scope, println_wrapper).unwrap();

    // Create a Rust function for `print`
    let print_fn = v8::Function::new(scope, print_wrapper).unwrap();

    // Set the `print` function on `Drw.core.out`
    {
      // Create a temporary scope for the property setting
      let key_println = v8::String::new(scope, "println").unwrap();
      out.set(scope, key_println.into(), println_fn.into()).unwrap();
  }

  {
      // Another temporary scope for `print` to avoid multiple borrows
      let key_print = v8::String::new(scope, "print").unwrap();
      out.set(scope, key_print.into(), print_fn.into()).unwrap();
  }

    // Set the `out` object on `Drw.core.out`
  {
    let key_out = v8::String::new(scope, "out").unwrap();
    core.set(
      scope,
      key_out.into(),
      out.into(),
    ).unwrap();
  }

  // Set the `core` object on `Drw`
  {
    let key_core = stringify(scope, "core").into();
    drw.set(
      scope,
      key_core,
      core.into(),
  ).unwrap();
  }
    
    // Set `Drw` on the global object
  {
    let key_drw = stringify(scope, "Drw").into();
    global.set(
      scope,
      key_drw,
      drw.into(),
    ).unwrap();
  }

    }
    

    // Execute some JavaScript that uses Drw.core.readFile
    let js_code = r#"
        const fileBuffer = Drw.core.readFile('example.txt');
        console.log(`File read, size: ${fileBuffer.byteLength}`);
    "#;

    runtime.execute_script("<run>", js_code).unwrap();
}


// Rust wrapper for `Drw.core.out.println`
fn println_wrapper(
  scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  mut rv: v8::ReturnValue,
) {
  // Get the string argument passed from JavaScript
  let text = args.get(0).to_rust_string_lossy(scope);

  // Call the Rust `println` function
  if let Err(_) = print_ln(&mut OpState::default(), text, ()) {
      rv.set(v8::null(scope).into());
  }
}

// Rust wrapper for `Drw.core.out.print`
fn print_wrapper(
  scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  mut rv: v8::ReturnValue,
) {
  // Get the string argument passed from JavaScript
  let text = args.get(0).to_rust_string_lossy(scope);

  // Call the Rust `print` function
  if let Err(_) = print(&mut OpState::default(), text, ()) {
      rv.set(v8::null(scope).into());
  }
}

// Rust wrapper function for read_file that can be called from JavaScript
fn read_file_wrapper(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    // Get the filename from the JavaScript arguments
    let filename = args.get(0).to_rust_string_lossy(scope);

    // Call the Rust function to read the file
    let result = read_file(&mut OpState::default(), filename, ());

    match result {
        Ok(buffer) => {
            // Convert the Vec<u8> to a Uint8Array for JavaScript
            let uint8_array = v8::Uint8Array::new(
                scope,
                v8::ArrayBuffer::with_backing_store(scope, v8::SharedArrayBuffer::new(buffer, 10)),
                0,
                10
            ).unwrap();
            rv.set(uint8_array.into());
        }
        Err(_) => {
            // Handle error case, returning null or throwing an error
            rv.set(v8::null(scope).into());
        }
    }
}
