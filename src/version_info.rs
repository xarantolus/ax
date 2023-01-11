use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[allow(dead_code)]
/// Returns the version of `ax`
pub fn version() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    VERSION.to_string()
}

#[wasm_bindgen]
#[allow(dead_code)]
/// Returns the commit hash of this `ax` version
pub fn commit() -> String {
    const COMMIT: &str = env!("GIT_COMMIT");
    COMMIT.to_string()
}
