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
    before: Vec<js_sys::Function>,
    after: Vec<js_sys::Function>,
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
        ax.hooks.running = true;

        for js_fn in &self.before {
            // TODO: Handle the function stopping the emulator etc
            let res = run_function(ax, js_fn.clone(), vec![JsValue::from(mnemonic as u32)]).await;
            if let Err(e) = res {
                ax.hooks.running = false;
                return Err(e.into());
            }
        }

        ax.hooks.running = false;
        Ok(())
    }

    pub async fn run_after(
        &self,
        ax: &mut Axecutor,
        mnemonic: SupportedMnemonic,
    ) -> Result<(), AxError> {
        ax.hooks.running = true;
        for js_fn in &self.after {
            // TODO: Handle the function stopping the emulator etc
            let res = run_function(ax, js_fn.clone(), vec![JsValue::from(mnemonic as u32)]).await;
            if let Err(e) = res {
                ax.hooks.running = false;
                return Err(e.into());
            }
        }
        ax.hooks.running = false;

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

#[derive(Debug, Clone, Default)]
pub(crate) struct HookProcessor {
    pub(crate) mnemonic_hooks: HashMap<SupportedMnemonic, Hook>,

    pub(crate) running: bool,
}

impl HookProcessor {
    pub(crate) fn default() -> Self {
        Self {
            mnemonic_hooks: HashMap::new(),
            running: false,
        }
    }
}

#[wasm_bindgen]
impl Axecutor {
    pub fn hook_before_mnemonic(
        &mut self,
        mnemonic: SupportedMnemonic,
        cb: JsValue,
    ) -> Result<(), JsError> {
        if self.hooks.running {
            return Err(JsError::new(
                "Cannot add hooks while another hook is running",
            ));
        }

        if cb.has_type::<js_sys::Function>() {
            let function = cb.dyn_into::<Function>().map_err(|_| {
                JsError::new("The provided callback is not a function. Please provide a function.")
            })?;

            self.hooks
                .mnemonic_hooks
                .entry(mnemonic)
                .or_insert_with(Hook::new)
                .before
                .push(function);
            Ok(())
        } else {
            Err(JsError::new(&*format!(
                "hook_before_mnemonic: expected function or async function argument, but got {:?}",
                cb
            )))
        }
    }

    pub fn hook_after_mnemonic(
        &mut self,
        mnemonic: SupportedMnemonic,
        cb: JsValue,
    ) -> Result<(), JsError> {
        if self.hooks.running {
            return Err(JsError::new(
                "Cannot add hooks while another hook is running",
            ));
        }

        if cb.has_type::<js_sys::Function>() {
            let function = cb.dyn_into::<Function>().map_err(|_| {
                JsError::new("The provided callback is not a function. Please provide a function.")
            })?;
            self.hooks
                .mnemonic_hooks
                .entry(mnemonic)
                .or_insert_with(Hook::new)
                .after
                .push(function);
            Ok(())
        } else {
            Err(JsError::new(&*format!(
                "hook_after_mnemonic: expected function or async function argument, but got {:?}",
                cb
            )))
        }
    }
}

impl Axecutor {
    pub(crate) fn mnemonic_hooks(&self, mnemonic: SupportedMnemonic) -> Option<Hook> {
        self.hooks.mnemonic_hooks.get(&mnemonic).cloned()
    }
}

async fn run_promise(promise_arg: JsValue) -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::from(promise_arg);
    let future = JsFuture::from(promise);
    future.await
}

async fn run_function(
    ax: &mut Axecutor,
    function: js_sys::Function,
    arguments: Vec<JsValue>,
) -> Result<JsValue, JsValue> {
    let args = Array::new();

    let _old_hooks = ax.hooks.clone();

    let clone = ax.clone();

    // This seems to be the only way this works
    args.push(&JsValue::from(clone));

    for arg in arguments {
        args.push(&arg);
    }

    let mut result = function.apply(&JsValue::NULL, &args)?;

    // async funtions return promises
    if result.has_type::<js_sys::Promise>() {
        result = run_promise(result).await?;
    }

    if !result.is_null() {
        ax.state_from_committed(result)?;
    }

    Ok(JsValue::UNDEFINED)
}
