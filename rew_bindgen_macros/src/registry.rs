
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub args: Vec<String>,
    pub return_type: String,
}

#[derive(Clone)]
pub struct StructInfo {
    pub name: String,
    pub fields: Vec<(String, String)>,
}

pub static FUNCTION_REGISTRY: Lazy<Mutex<Vec<FunctionInfo>>> = Lazy::new(|| Mutex::new(vec![]));
pub static STRUCT_REGISTRY: Lazy<Mutex<Vec<StructInfo>>> = Lazy::new(|| Mutex::new(vec![]));

#[no_mangle]
pub extern "C" fn __rew_symbols() -> *mut libc::c_char {
    let functions = FUNCTION_REGISTRY.lock().unwrap().clone();
    let structs = STRUCT_REGISTRY.lock().unwrap().clone();

    let json = serde_json::json!({
        "functions": functions.iter().map(|f| {
            serde_json::json!({
                "name": f.name,
                "args": f.args,
                "return_type": f.return_type,
            })
        }).collect::<Vec<_>>(),
        "structs": structs.iter().map(|s| {
            serde_json::json!({
                "name": s.name,
                "fields": s.fields,
            })
        }).collect::<Vec<_>>(),
    });

    let c_string = std::ffi::CString::new(json.to_string()).unwrap();
    c_string.into_raw()
}
