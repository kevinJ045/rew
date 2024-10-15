#[cfg(coal)]
pub(crate) mod coal;

use boa_engine::{
    context::DefaultHooks, js_string, object::{builtins::{JsArray, JsFunction}, shape::RootShape, NativeObject, ObjectInitializer}, property::{Attribute, PropertyKey}, realm::Realm, vm::RuntimeLimits, Context, JsArgs, JsError, JsNativeError, JsObject, JsResult, JsStr, JsString, JsValue, NativeFunction, Source
};
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::parser::{Error};
use boa_gc::{Finalize, Trace};
// use boa_interop::{js_class, Ignore, JsClass};
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::collections::HashMap;
use std::thread;
use std::sync::Arc;
use std::env;

mod civet;

use civet::get_civet_script;

const MAX_RECURSION_DEPTH: usize = 1000000; 

fn js_array_to_vec_u8(js_array: JsValue, context: &mut Context) -> JsResult<Vec<u8>> {
	if let Ok(array) = JsArray::from_object(js_array.as_object().unwrap().clone()) {
		let mut vec = Vec::new();
		for idx in 0..array.length(context)? {
				let element = array.get(idx, context)?;
				if let Some(num) = element.as_number() {
						vec.push(num as u8);
				} else {
						return Err(JsError::from_native(JsNativeError::typ()));
				}
		}
		Ok(vec)
	} else {
		Err(JsError::from_native(JsNativeError::typ()))
	}
}

fn vec_to_js_array(buffer: Vec<u8>, context: &mut Context) -> JsResult<JsArray> {
	let js_array = JsArray::new(context);
	for byte in buffer {
		js_array.push(JsValue::from(byte), context).unwrap();
	}
	Ok(js_array)
}

struct ContextManager {
	context: Context
}

impl ContextManager {

	fn new() -> ContextManager {
		let context = Context::default(); // Create a new context
		let mut man = ContextManager { context };
		man.define_context_props();
		man
	}

	fn get_context(&mut self) -> &mut Context {
		&mut self.context
	}

	fn define_context_props(&mut self) {
		let context = self.get_context();

		// context.register_global_class::<Buffer>();
		let drw_obj = ObjectInitializer::new(context).build();
		context.register_global_property(js_string!("Drw"), drw_obj, Attribute::all());
		context.runtime_limits().set_recursion_limit(MAX_RECURSION_DEPTH);
		
		self.rns("Drw.prototype");
		self.rns("Drw.prototype.core");
		self.rns("Drw.prototype.fs");
		self.rns("Drw.prototype.process");
		self.rns("Drw.prototype.io");
		self.rns("Drw.prototype.namespace");

		fn buffer_to_string(
				_this: &JsValue,
				args: &[JsValue],
				context: &mut boa_engine::Context,
		) -> JsResult<JsValue> {
			let this_object = _this.as_object().unwrap();
			let js_array = this_object.get(js_string!("data"), context)?;
	
			let the_vec = js_array_to_vec_u8(js_array, context)?;
	
			let as_string = String::from_utf8(the_vec).unwrap_or_else(|_| "Invalid UTF-8".to_string());
			
			Ok(JsValue::from(js_string!(as_string)))
		}
		
		fn buffer_method(
				_this: &JsValue,
				args: &[JsValue],
				context: &mut boa_engine::Context,
		) -> JsResult<JsValue> {
			let binding = JsValue::from(JsArray::new(context));
			let array = args.get(0).unwrap_or(&binding);
	
			let js_array = JsArray::from_object(array.as_object().unwrap().clone())?;
	
			let buffer_item = ObjectInitializer::new(context)
					.property(js_string!("data"), JsValue::from(js_array), Attribute::all())
					.function(
							NativeFunction::from_fn_ptr(buffer_to_string), // Use the regular function here
							js_string!("toString"),
							0,
					)
					.build();
	
			Ok(JsValue::from(buffer_item))
		}
		self.rfunc("Drw.prototype.fs.buffer", buffer_method);

		fn fs_read_wrapper(
			filename: String,
			context: &mut Context
		) -> JsResult<JsValue> {
			match read_file_fn(filename) {
				Ok(buffer) => {
					Ok(JsValue::from(vec_to_js_array(buffer, context)?)) // Return the buffer as a JsValue
				}
				Err(_) => {
					Ok(JsValue::from(js_string!("Failed to read file.")))
				}
			}
		}

		fn fs_read_buff(
			_this: &JsValue,
			args: &[JsValue],
			context: &mut boa_engine::Context,
		) -> JsResult<JsValue> {
			let filename = to_std_str(args.get_or_undefined(0), context);
			Ok(
				fs_read_wrapper(filename, context)?
			)
		}

		self.rfunc("Drw.prototype.fs.readRaw", fs_read_buff);

		fn fs_read(
			_this: &JsValue,
			args: &[JsValue],
			context: &mut boa_engine::Context,
		) -> JsResult<JsValue> {
			let filename = to_std_str(args.get_or_undefined(0), context);

			let options: Option<&JsObject> = args.get(1).and_then(JsValue::as_object);

			let buffer = fs_read_wrapper(filename, context);

			let mut option_type = JsValue::from(js_string!("string"));

			match options {
        Some(opts) => {
					if opts.has_property(js_string!("type"), context)? {
						option_type = opts.get(js_string!("type"), context)?
					}
				},
        None => {},
    	};

			if to_std_str(&option_type, context) == "buffer" {
				Ok(JsValue::from(buffer?))
			} else {
				let the_vec = js_array_to_vec_u8(JsValue::from(buffer?), context)?;
				let as_string = String::from_utf8(the_vec).unwrap_or_else(|_| "Invalid UTF-8".to_string());

				Ok(JsValue::from(js_string!(as_string)))
			}
			
		}

		self.rfunc("Drw.prototype.fs.read", fs_read);

		fn fs_write(
			_this: &JsValue,
			args: &[JsValue],
			context: &mut boa_engine::Context,
	) -> JsResult<JsValue> {
			let filepath = to_std_str(args.get(0).unwrap_or(&JsValue::undefined()), context);
			
			let data_value = args.get(1).unwrap_or(&JsValue::undefined()).clone();
			
			let options: Option<&JsObject> = args.get(2).and_then(JsValue::as_object);
	
			let is_buffer = options
					.and_then(|opts| opts.get(js_string!("type"), context).ok())
					.filter(|typ| to_std_str(typ, context).as_str() == "buffer")
					.is_some();
	
			let data_to_write = if is_buffer {
					match js_array_to_vec_u8(data_value.clone(), context) {
							Ok(buffer) => buffer,
							Err(_) => return Err(JsError::from_native(JsNativeError::typ())),
					}
			} else {
					to_std_str(&data_value, context).into_bytes()
			};
	
			match std::fs::write(&filepath, data_to_write) {
					Ok(_) => Ok(JsValue::undefined()),
					Err(err) => {
							eprintln!("Failed to write to file '{}': {:?}", filepath, err);
							Err(JsError::from_native(JsNativeError::typ()))
					}
			}
		}
	
		self.rfunc("Drw.prototype.fs.write", fs_write);

		for (key, value) in env::vars() {
			let keyFull = String::from("Drw.prototype.env.data.") + key.as_str();
			self.rval(keyFull.as_str(), JsValue::from(js_string!(value)));
		}

		fn find_env(
			this: &JsValue,
			args: &[JsValue],
			context: &mut boa_engine::Context,
		) -> JsResult<JsValue> {
			let env_key = to_std_str(args.get_or_undefined(0), context);
	
			match std::env::var(env_key) {
				Ok(value) => Ok(JsValue::from(js_string!(value))), 
				Err(_) => Ok(JsValue::undefined()),
			}
		}

		self.rfunc("Drw.prototype.env.get", find_env);

		fn io_out_print(
			_this: &JsValue,
			args: &[JsValue],
			context: &mut boa_engine::Context,
		) -> JsResult<JsValue> {
			let mut output = String::new();
			for arg in args {
					let arg_str = arg.to_string(context)?.to_std_string().unwrap_or_default();
					output.push_str(&arg_str);
					output.push(' '); 
			}
			println!("{}", output.trim_end()); 
			Ok(JsValue::undefined()) 
		}

		self.rfunc("Drw.prototype.io.print", io_out_print);

		fn io_in_read(
			_this: &JsValue,
			args: &[JsValue],
			context: &mut boa_engine::Context,
		) -> JsResult<JsValue> {
			let prompt = to_std_str(args.get(0).unwrap_or(&JsValue::undefined()), context);
			if !prompt.is_empty() {
					print!("{}", prompt); 
					io::stdout().flush().expect("Failed to flush stdout"); 
			}

			let mut input = String::new();
			std::io::stdin()
					.read_line(&mut input)
					.expect("Failed to read line");
			Ok(JsValue::from(js_string!(input.trim())))
		}

		self.rfunc("Drw.prototype.io.input", io_in_read);


		fn merge_js_objects(
			_this: &JsValue,
			objects: &[JsValue],
			context: &mut boa_engine::Context,
		) -> JsResult<JsValue> {
			let merged_object = ObjectInitializer::new(context).build();
	
			for object in objects {
					if let Some(obj) = object.as_object() {
							let keys = obj.own_property_keys(context)?;
	
							for key in keys {
									let value = obj.get(key.clone(), context)?;
	
									merged_object.set(key, value, true, context)?;
							}
					}
			}
	
			Ok(JsValue::from(merged_object))
		}
		self.rfunc("Drw.prototype.core.merge", merge_js_objects);

	}
	
	fn gns(&mut self, name: &str) -> JsObject {
		let names: Vec<&str> = name.split('.').collect();
		let context = self.get_context();

		let mut current_object: JsObject = context.global_object();

		for &part in &names {
			if current_object.has_property(js_string!(part), context).unwrap_or(false) {
				match current_object.get(js_string!(part), context) {
					Ok(value) => current_object = value.as_object()
					.map(|obj| obj.clone())
					.unwrap(),
					Err(err) => {
						eprintln!("Error getting property '{}': {:?}", part, err);
						return ObjectInitializer::new(context).build();
					}
				}
			} else {
				return ObjectInitializer::new(context).build();
			}
		
		}

		return current_object;
	}
		
	fn rns(&mut self, name: &str) -> JsObject {
		let names: Vec<&str> = name.split('.').collect();
		let context = self.get_context();
		
		let mut current_object: JsObject = context.global_object();

		for &part in &names {
				if current_object.has_property(js_string!(part), context).unwrap_or(false) {
					match current_object.get(js_string!(part), context) {
						Ok(value) => current_object = value.as_object()
						.map(|obj| obj.clone())
						.unwrap(),
						Err(err) => {
								eprintln!("Error getting property '{}': {:?}", part, err);
								return ObjectInitializer::new(context).build(); 
						}
					}
				} else {
						let new_object = ObjectInitializer::new(context).build();
						match current_object.set(js_string!(part), new_object.clone(), true, context) {
							Ok(_) => {}
							Err(_) => {}
						}
						current_object = new_object;
				}
		}

		return current_object;
	}

	fn rfunc(&mut self, name: &str, func: boa_engine::native_function::NativeFunctionPointer) {
		let js_function = NativeFunction::from_fn_ptr(func);

		{
			let shape = RootShape::default();
			let hooks = DefaultHooks {};
			let realm_result = Realm::create(&hooks, &shape);

			match realm_result {
				Ok(realm) => {
					self.rval(name, JsValue::from(
						js_function.to_js_function(&realm)
					));
				}
				Err(err) => {
					eprintln!("Failed to create realm: {:?}", err);
				}
			}
		}
	}

	fn rval(&mut self, name: &str, val: JsValue){
		let parts: Vec<&str> = name.split('.').collect();
		let prop_name = parts.last().unwrap();
		
		let current_object: JsObject = if parts.len() > 1 {
			self.rns(&name[..name.len() - prop_name.len() - 1])
		} else {
			self.get_context().global_object()
		};

		let context = self.get_context();
		match current_object.set(js_string!(*prop_name), JsValue::from(val), true, context) {
			Ok(_) => {}
			Err(_) => {}
		};
	}

	// fn rprop(&self, name: &str, value: JsValue, context: &mut Context) {
	// 		let names: Vec<&str> = name.split('.').collect();
	// 		let mut current_object = context.global_object();

	// 		for &part in &names[..names.len() - 1] {
	// 				let existing_value = current_object.get_field(part);
	// 				if existing_value.is_object() {
	// 						current_object = existing_value.as_object().unwrap();
	// 				} else {
	// 						let new_object = ObjectInitializer::new(context).build();
	// 						current_object.set_field(part, new_object, Attribute::all());
	// 						current_object = new_object;
	// 				}
	// 		}

	// 		// Register the property in the final object
	// 		current_object.set_field(names.last().unwrap(), value, Attribute::all());
	// }
}

fn read_file_fn(filename: String) -> Result<Vec<u8>, io::Error> {
	let file_result = File::open(filename);
	let mut buffer = Vec::new();

	match file_result {
		Ok(mut file) => {
			file.read_to_end(&mut buffer)?;
			Ok(buffer)
		}
		Err(err) => {
			eprintln!("Failed to read file: {:?}", err);
			Err(err)
		}
	}
}

fn to_std_str(val: &JsValue, context: &mut Context) -> String {
	match val.to_string(context) {
		Ok(js_string) => {
			match js_string.to_std_string() {
				Ok(fsn) => fsn,
				Err(_) => String::new(),
			}
		}
		Err(_) => String::new(),
	}
}


struct PodManager {
	cman: ContextManager,
}

impl PodManager {
	fn new() -> PodManager {
		let cman = ContextManager::new();
		let pman = PodManager { cman };
		pman
	}

	fn set_current_filepath(&mut self, filename: &str, is_main: bool) -> JsResult<()> {
		let cman = &mut self.cman;
		let context = cman.get_context();
		let myobj = ObjectInitializer::new(context)
			.property(js_string!("filepath"), js_string!(filename), Attribute::all())
			.property(js_string!("main"), JsValue::from(is_main), Attribute::all())
			.property(js_string!("pid"), JsValue::from(std::process::id()), Attribute::all())
			/* More properties */
			.build();
		cman.rval("Drw.prototype.process.current", JsValue::from(myobj));
		Ok(())
	}


	fn execute(&mut self, filename: &str, is_main: bool) -> JsResult<JsValue> {
		let code = std::fs::read_to_string(filename).expect("Failed to read the file");

		self.execute_string(&code, filename, is_main)
	}

	fn execute_string(&mut self, code: &str, filename: &str, is_main: bool) -> JsResult<JsValue> {
		self.set_current_filepath(filename, is_main)?;

		match self.cman
		.get_context()
		.eval(Source::from_bytes(code.as_bytes())) {
			Ok(result) => {
				Ok(result)
			}
			Err(err) => {
				println!("{}", err);
				Err(JsError::from_native(JsNativeError::typ()))
			}
		}
	}

	fn execute_in_new_thread(&mut self, filename: &str, is_main: bool) {
		let filename = filename.to_string();
		thread::spawn(move || {
			let mut pod_manager = PodManager::new();
			pod_manager
					.execute(&filename, is_main)
					.expect("Failed to execute in new thread");
		});
	}

	fn execute_string_in_new_thread(&mut self, code: &str, filename: &str, is_main: bool) {
		let code = code.to_string();
		let filename = filename.to_string();
		thread::spawn(move || {
			let mut pod_manager = PodManager::new();
			if let Ok(result) = pod_manager
					.execute_string(&code, &filename, is_main) {
						println!("{}", result.display());
					}
			else {
				println!("Err");
			}
		});
	}
}


struct RuntimeManager {
	pman: PodManager
}

impl RuntimeManager {
	fn new() -> RuntimeManager {
		let pman = PodManager::new();
		let runtime = RuntimeManager { pman };
		runtime
	}

	fn srun(filename: &str, func: Option<Box<dyn for<'a> FnOnce(&'a mut PodManager)>>) -> JsResult<JsValue>
	{
		let mut r = RuntimeManager::new();
		r.runFile(filename, func)
	}

	fn runFile(&mut self, filename: &str, func: Option<Box<dyn for<'a> FnOnce(&'a mut PodManager)>>) -> JsResult<JsValue>
	{
		let mut val = JsValue::undefined();

		let code = if filename.ends_with(".coffee") || filename.ends_with(".civet") {
			CompileManager::compile(filename)?
		} else {
			std::fs::read_to_string(filename).expect("Failed to read the file")
		};

		if let Ok(result) = self.pman.execute_string(&code, filename, true) {
			let context = self.pman.cman.get_context();
			let object_main = context.global_object();
	
			// Check if the global object has a function named "main"
			if object_main.has_property(js_string!("main"), context)? {
				// Get the `main` function from the global object
				let main_function = object_main.get(js_string!("main"), context)?;
	
				// Check if it is callable
				if main_function.is_callable() {
						// Prepare argv or any necessary arguments to pass
						let argv: Vec<JsValue> = vec![
								JsValue::from(js_string!("arg1"))
						];
						val = JsValue::from(main_function).as_callable().unwrap().call(&JsValue::from(object_main), &argv, context)?;
						
				}
			}
		}

		Ok(val)
	}
}


struct CompileManager {}

impl CompileManager {

	fn compile(filename: &str) -> JsResult<String> {

		let mut pman = PodManager::new();

		let code_raw = std::fs::read_to_string(filename).expect("Failed to read the file");

		println!("{}", code_raw);

		pman.cman.rval("__to__compile__", JsValue::from(js_string!(code_raw)));

		let compiled = pman.execute_string_in_new_thread(get_civet_script().as_str(), "system::compiler", false);	

		// println!("{}", compiled.display());
		
		// Ok(to_std_str(&compiled, &mut pman.cman.context))
		Ok(String::from("Drw"))
	}

}

fn main() -> JsResult<()> {

	// RuntimeManager::srun("/home/makano/workspace/dabo/test/some.js", Some(Box::new(|_| {

	// })));

	RuntimeManager::srun("/home/makano/workspace/dabo/test/s.coffee", None);

  Ok(())
}
