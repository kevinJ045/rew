

fn buffer_to_js_value(buffer: Vec<u8>, context: &mut Context) -> JsResult<JsValue> {
	let js_array = JsArray::new(context);
	for byte in buffer {
			js_array.push(JsValue::from(byte), context).unwrap();
	}
	
	// First, get the `Buffer` class from the global object and handle the Result
	let buffer_class = context.global_object().get(js_string!("Buffer"), context)?;

	// Create an argument array as a slice for the constructor
	let args = &[JsValue::from(js_string!("hh"))];

	// Call the `construct` method correctly with references
	let buf = <Buffer as Class>::construct(&buffer_class, args, context)?;
	Ok(JsValue::from(buf))
}

#[derive(Debug, Trace, Finalize)]
struct  Buffer {
	data: Vec<u8>
}

impl boa_engine::object::JsData for Buffer {}

impl Buffer {
	// Helper function to convert a JsValue (expected to be an array) into a Vec<u8>
	fn from_js_array(array: &JsValue, context: &mut Context) -> JsResult<Vec<u8>> {
		let j = array.as_object().unwrap();

		// Collect elements into a Vec<u8>
		let mut buffer = Vec::new();

		if let Ok(jarray) = JsArray::from_object(j.clone()) {
			let js_array: JsArray = jarray;
			if let Ok(length) = js_array.length(context) {
				for i in 0..length {
					let value = js_array.get(i, context).unwrap_or_default();
					if let Ok(num) = value.to_number(context) {
						buffer.push(num as u8);
					} else {
						return Err(JsError::from(Error::AbruptEnd));
					}
				}
			}
		}

		Ok(buffer)
	}
}

impl Class for Buffer {
	const NAME: &'static str = "Buffer";
	const LENGTH: usize = 2;

	fn data_constructor(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<Self> {

		if let Ok(data) = Buffer::from_js_array(args.get_or_undefined(0), context) {	
			let buffer = Buffer {
				data
			};

			Ok(buffer)
		} else {
			Ok(Buffer {
				data: Vec::new()
			})
		}

	}

	fn init(class: &mut ClassBuilder) -> JsResult<()> {
		// class.method("say_hello", 0, Self::say_hello);

		Ok(())
	}
}