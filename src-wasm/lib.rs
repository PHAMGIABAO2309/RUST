use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rust_function() -> String {
    "Hello from Rust!".to_string()
}
