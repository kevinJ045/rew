use libloading::Library;
use std::ffi::{CStr, CString};

#[repr(C)]
struct PluginFunction {
    name: *const std::os::raw::c_char,
    func: extern "C" fn(*const std::os::raw::c_char) -> *const std::os::raw::c_char,
}

#[repr(C)]
struct PluginManifest {
    count: usize,
    functions: *const PluginFunction,
}

fn load_plugin(path: &str) -> HashMap<String, Box<dyn Fn(String) -> String>> {
    let lib = Library::new(path).unwrap();
    unsafe {
        let init: libloading::Symbol<unsafe extern "C" fn() -> *const PluginManifest> =
            lib.get(b"init_plugin").unwrap();

        let manifest = init();
        let manifest = &*manifest;

        let mut functions = HashMap::new();
        for i in 0..manifest.count {
            let func = &*manifest.functions.add(i);
            let name = CStr::from_ptr(func.name).to_str().unwrap().to_string();

            let f = func.func;

            functions.insert(
                name.clone(),
                Box::new(move |arg_json: String| {
                    let cstr = CString::new(arg_json).unwrap();
                    let result_ptr = f(cstr.as_ptr());
                    let result_cstr = CStr::from_ptr(result_ptr);
                    result_cstr.to_string_lossy().to_string()
                }) as Box<dyn Fn(String) -> String>,
            );
        }

        functions
    }
}
