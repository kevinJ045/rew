use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "kind")]
pub enum SymbolMetadata {
    Function {
        name: String,
        signature: String,
    },
    Struct {
        name: String,
        fields: Vec<String>,
    },
}

static SYMBOLS: Lazy<Mutex<HashMap<String, SymbolMetadata>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_function(name: &str, signature: &str) {
    SYMBOLS.lock().unwrap().insert(
        name.to_string(),
        SymbolMetadata::Function {
            name: name.to_string(),
            signature: signature.to_string(),
        },
    );
}

pub fn register_struct(name: &str, fields: Vec<String>) {
    SYMBOLS.lock().unwrap().insert(
        name.to_string(),
        SymbolMetadata::Struct {
            name: name.to_string(),
            fields,
        },
    );
}

pub fn get_symbols_json() -> String {
    let symbols = SYMBOLS.lock().unwrap();
    serde_json::to_string_pretty(&*symbols).unwrap()
}
