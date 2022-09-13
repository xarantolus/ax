mod functions;

use std::vec;

use wasm_bindgen::{prelude::*};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: JsValue);
}

#[wasm_bindgen]
pub async fn execute_sync_or_async(function_or_promise: JsValue) {
    match functions::run_any_function(function_or_promise, vec![JsValue::from("a")]).await {
        Ok(v) => alert(v),
        Err(e) => alert(e),
    }
}
