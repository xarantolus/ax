use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn version() -> String {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    VERSION.to_string()
}
