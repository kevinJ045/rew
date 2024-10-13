#[cfg(coal)]
pub(crate) mod coal;

use boa_engine::{
    context::DefaultHooks, js_string, object::{builtins::{JsArray, JsFunction}, shape::RootShape, NativeObject, ObjectInitializer}, property::{Attribute, PropertyKey}, realm::Realm, Context, JsArgs, JsError, JsNativeError, JsObject, JsResult, JsStr, JsString, JsValue, NativeFunction, Source
};
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::parser::{Error};
use boa_gc::{Finalize, Trace};
// use boa_interop::{js_class, Ignore, JsClass};
use std::fs::File;
use std::io::Read;
use std::io::{self};

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
		&mut self.context // Return a reference to the context
	}

	fn define_context_props(&mut self) {
		let context = self.get_context();

		// context.register_global_class::<Buffer>();
		
		self.rns("Drw.prototype");
		self.rns("Drw.prototype.core");
		self.rns("Drw.prototype.fs");
		self.rns("Drw.prototype.io");
		self.rns("Drw.prototype.namespace");
		
		fn buffer_to_string(
				_this: &JsValue,
				args: &[JsValue],
				context: &mut boa_engine::Context,
		) -> JsResult<JsValue> {
			// You need to get the 'data' property from the current object (`this`)
			let this_object = _this.as_object().unwrap();
			let js_array = this_object.get(js_string!("data"), context)?;
	
			// Convert the JsArray back to a Vec<u8> (you can define this function)
			let the_vec = js_array_to_vec_u8(js_array, context)?;
	
			// Convert the Vec<u8> to a string and return it
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
	
			// Build the object
			let buffer_item = ObjectInitializer::new(context)
					.property(js_string!("data"), JsValue::from(js_array), Attribute::all())
					.function(
							NativeFunction::from_fn_ptr(buffer_to_string), // Use the regular function here
							js_string!("toString"),
							0,
					)
					.build();
	
			// Return the constructed object
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
        None => {}, // Default if no options are provided
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

	}
	
	fn gns(&mut self, name: &str) -> JsObject {
		let names: Vec<&str> = name.split('.').collect();
		let context = self.get_context();

		let mut current_object: JsObject = context.global_object();

		for &part in &names {
			// Check if the object already exists
			if current_object.has_property(js_string!(part), context).unwrap_or(false) {
				match current_object.get(js_string!(part), context) {
					Ok(value) => current_object = value.as_object()
					.map(|obj| obj.clone())
					.unwrap(),
					Err(err) => {
						eprintln!("Error getting property '{}': {:?}", part, err);
						return ObjectInitializer::new(context).build(); // Exit if there's an error (handle as needed)
					}
				}
			} else {
				return ObjectInitializer::new(context).build();
			}
		
		}

		return current_object;
	}
		
		/// Registers a namespace in the context.
	fn rns(&mut self, name: &str) -> JsObject {
		let names: Vec<&str> = name.split('.').collect();
		let context = self.get_context();
		
		// Start with the global object
		let mut current_object: JsObject = context.global_object();

		for &part in &names {
				// Check if the object already exists
				if current_object.has_property(js_string!(part), context).unwrap_or(false) {
					// Get the existing object
					match current_object.get(js_string!(part), context) {
						Ok(value) => current_object = value.as_object()
						.map(|obj| obj.clone())
						.unwrap(),
						Err(err) => {
								eprintln!("Error getting property '{}': {:?}", part, err);
								return ObjectInitializer::new(context).build(); // Exit if there's an error (handle as needed)
						}
					}
				} else {
						// Create a new object if it doesn't exist
						let new_object = ObjectInitializer::new(context).build();
						match current_object.set(js_string!(part), new_object.clone(), true, context) {
							Ok(_) => {}
							Err(_) => {}
						}
						current_object = new_object; // Move to the new object for the next part
				}
		}

		return current_object;
	}

	/// Registers a function in the context.
	fn rfunc(&mut self, name: &str, func: boa_engine::native_function::NativeFunctionPointer) {
		let js_function = NativeFunction::from_fn_ptr(func);

		// Register the function in the final object
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
		
		// Get the object to register the function on
		let current_object: JsObject = if parts.len() > 1 {
			// If the name has a namespace, get the corresponding object
			self.rns(&name[..name.len() - prop_name.len() - 1])
		} else {
			// If there's no namespace, get the global object
			self.get_context().global_object()
		};

		let context = self.get_context();
		match current_object.set(js_string!(*prop_name), JsValue::from(val), true, context) {
			Ok(_) => {}
			Err(_) => {}
		};
	}

	// /// Registers a property in the context.
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
				Ok(fsn) => fsn, // return the String directly
				Err(_) => String::new(), // return an empty String if conversion fails
			}
		}
		Err(_) => String::new(), // return an empty String if to_string fails
	}
}


fn main() -> JsResult<()> {
  let js_code = r#"
		const filename = "/etc/nixos/home.nix";
    Drw.prototype.fs.read(filename);
  "#;

  // Instantiate the execution context
  let mut cman = ContextManager::new();

  // Parse the source code
  let result = cman.context.eval(Source::from_bytes(js_code))?;

  println!("{}", result.display());

  Ok(())
}
