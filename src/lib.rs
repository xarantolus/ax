mod functions;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn execute_sync_or_async(function_or_promise: JsValue) -> Result<JsValue, JsValue> {
    functions::run_any_function(function_or_promise, vec![JsValue::from("a")]).await
}
