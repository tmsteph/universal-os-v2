// Rust entry point
use wasm_bindgen::prelude::*;

// This function will be exposed to JavaScript
#[wasm_bindgen]
pub fn greet() {
    alert("Hello from Universal OS v2!");
}

// For browser alert
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}