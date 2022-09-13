use js_sys::{self, Array, Function};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

async fn run_promise(promise_arg: JsValue) -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::from(promise_arg);
    let future = JsFuture::from(promise);
    future.await
}

fn run_function(function_arg: JsValue, arguments: Vec<JsValue>) -> Result<JsValue, JsValue> {
    let args = Array::new();
    for arg in arguments {
        args.push(&arg);
    }
    let function = function_arg.dyn_into::<Function>()?;
    function.apply(&JsValue::NULL, &args)
}

pub async fn run_any_function(
    function_or_promise: JsValue,
    arguments: Vec<JsValue>,
) -> Result<JsValue, JsValue> {
    if function_or_promise.has_type::<js_sys::Function>() {
        let result = run_function(function_or_promise, arguments)?;

        // Handle functions defined like "async function(args) {}"
        if result.has_type::<js_sys::Promise>() {
            return run_promise(result).await;
        } else {
            Ok(result)
        }
    } else {
        Err(JsValue::from(JsError::new(&*format!(
            "run_any_function: expected function or async function argument, but got {:?}",
            function_or_promise
        ))))
    }
}
