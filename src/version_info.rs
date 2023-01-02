use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn version() -> String {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    VERSION.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_version_pattern() {
        assert!(version().matches(r"^\d+\.\d+\.\d+$").count() == 1);
    }
}
