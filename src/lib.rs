use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn render() {
    console::log_1(&"Hello using web-sys".into());
}