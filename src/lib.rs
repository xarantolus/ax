mod functions;
mod instructions;

use instructions::{axecutor::Axecutor, errors::AxError};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

impl From<AxError> for JsValue {
    fn from(e: AxError) -> Self {
        JsValue::from_str(&e.to_string())
    }
}

#[wasm_bindgen]
pub async fn execute_sync_or_async(arr: Uint8Array, rip: u64) -> Result<JsValue, JsValue> {
    let ax = Axecutor::new(&arr.to_vec(), rip);

    Ok(JsValue::NULL)
    // functions::run_any_function(function_or_promise, vec![JsValue::from("a")]).await
}
