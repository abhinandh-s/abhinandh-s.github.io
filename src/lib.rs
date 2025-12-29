use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    "Hello from Rust + Web Assembly".to_string()
}

