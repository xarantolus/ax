extern crate lazy_static;
use js_sys::{self, Array, Function};

use std::{collections::HashMap, fmt::Formatter};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

use std::fmt::Debug;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::instructions::generated::SupportedMnemonic;

use super::{axecutor::Axecutor, errors::AxError};

#[derive(Clone)]
pub(crate) struct Hook {
    before: Vec<JsValue>,
    after: Vec<JsValue>,
}

impl Hook {
    pub fn new() -> Self {
        Self {
            before: Vec::new(),
            after: Vec::new(),
        }
    }

    pub async fn run_before(
        &self,
        ax: &mut Axecutor,
        mnemonic: SupportedMnemonic,
    ) -> Result<(), AxError> {
        for js_fn in &self.before {
            // TODO: Handle errors, the function stopping the emulator etc
            run_any_function(ax, js_fn.clone(), vec![JsValue::from(mnemonic as u32)]).await;
        }

        Ok(())
    }

    pub async fn run_after(
        &self,
        ax: &mut Axecutor,
        mnemonic: SupportedMnemonic,
    ) -> Result<(), AxError> {
        for js_fn in &self.after {
            // TODO: Handle errors, the function stopping the emulator etc
            run_any_function(ax, js_fn.clone(), vec![JsValue::from(mnemonic as u32)]).await;
        }

        Ok(())
    }
}

impl Debug for Hook {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hook")
            .field("before", &self.before.len())
            .field("after", &self.after.len())
            .finish()
    }
}

#[derive(Debug)]
pub(crate) struct HookProcessor {
    pub(crate) mnemonic_hooks: HashMap<SupportedMnemonic, Hook>,
}

impl HookProcessor {
    pub(crate) fn new() -> Self {
        Self {
            mnemonic_hooks: HashMap::new(),
        }
    }
}

#[wasm_bindgen]
impl Axecutor {
    pub fn hook_before_mnemonic(&mut self, mnemonic: SupportedMnemonic, cb: JsValue) {
        self.hooks
            .mnemonic_hooks
            .entry(mnemonic)
            .or_insert_with(Hook::new)
            .before
            .push(cb);
    }

    pub fn hook_after_mnemonic(&mut self, mnemonic: SupportedMnemonic, cb: JsValue) {
        self.hooks
            .mnemonic_hooks
            .entry(mnemonic)
            .or_insert_with(Hook::new)
            .after
            .push(cb);
    }
}

impl Axecutor {
    pub(crate) fn mnemonic_hooks(&self, mnemonic: SupportedMnemonic) -> Option<Hook> {
        self.hooks.mnemonic_hooks.get(&mnemonic).cloned()
    }
}

#[cfg(test)]
mod tests {

    // TODO: Test
    // #[test]
    // fn test_hook_before() {
    //     let mut ax = Axecutor::new(
    //         // mov rax, 5
    //         &[0x48, 0xc7, 0xc0, 0x5, 0x0, 0x0, 0x0],
    //         0x1000,
    //         0x1000,
    //     )
    //     .expect("failed to create axecutor");
    //     let mut called = false;
    //     // ax.hook_before_mnemonic(Mnemonic::Mov.into(), |_, _| {
    //     //     called = true;
    //     //     Ok(())
    //     // });

    //     ax.execute().await.expect("failed to execute");

    //     assert!(called, "hook_before_mnemonic was not called");
    // }
}

async fn run_promise(promise_arg: JsValue) -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::from(promise_arg);
    let future = JsFuture::from(promise);
    future.await
}

fn run_function(
    _ax: &mut Axecutor,
    function_arg: JsValue,
    arguments: Vec<JsValue>,
) -> Result<JsValue, JsValue> {
    let args = Array::new();
    for arg in arguments {
        args.push(&arg);
    }
    let function = function_arg.dyn_into::<Function>()?;
    function.apply(&JsValue::NULL, &args)
}

pub(crate) async fn run_any_function(
    ax: &mut Axecutor,
    function_or_promise: JsValue,
    arguments: Vec<JsValue>,
) -> Result<JsValue, JsValue> {
    if function_or_promise.has_type::<js_sys::Function>() {
        let result = run_function(ax, function_or_promise, arguments)?;

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
