use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[allow(dead_code)]
pub fn version() -> String {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    VERSION.to_string()
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn commit() -> String {
    const COMMIT: &'static str = env!("GIT_COMMIT");
    COMMIT.to_string()
}
