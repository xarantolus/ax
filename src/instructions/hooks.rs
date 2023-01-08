extern crate lazy_static;
use js_sys::{self, Array, Function};

use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use std::fmt::Debug;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::instructions::generated::SupportedMnemonic;

use super::{axecutor::Axecutor, errors::AxError};

use crate::debug_log;

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
        debug_log!(
            "Calling Hook::run_before with {} hook function(s)",
            self.before.len()
        );
        ax.hooks.running = true;

        for js_fn in &self.before {
            let res = run_function(ax, js_fn.clone(), vec![JsValue::from(mnemonic as u32)]).await;
            if let Err(e) = res {
                debug_log!("Error running hook: {:?}", e);
                ax.hooks.running = false;
                return Err(e.into());
            }
        }

        debug_log!("Finished calling Hook::run_before");
        ax.hooks.running = false;
        Ok(())
    }

    pub async fn run_after(
        &self,
        ax: &mut Axecutor,
        mnemonic: SupportedMnemonic,
    ) -> Result<(), AxError> {
        debug_log!(
            "Calling Hook::run_after with {} hook function(s)",
            self.after.len()
        );
        ax.hooks.running = true;
        for js_fn in &self.after {
            let res = run_function(ax, js_fn.clone(), vec![JsValue::from(mnemonic as u32)]).await;
            if let Err(e) = res {
                debug_log!("Error running hook: {:?}", e);
                ax.hooks.running = false;
                return Err(e.into());
            }
        }

        debug_log!("Finished calling Hook::run_after");
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

impl Display for HookProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Same as code above
        write!(f, "{{")?;
        for (mnem, hook) in self.mnemonic_hooks.iter() {
            writeln!(
                f,
                "    {:?}: {{ before: {}, after: {} }}",
                mnem,
                hook.before.len(),
                hook.after.len()
            )?;
        }

        write!(f, "}}")
    }
}

#[wasm_bindgen]
impl Axecutor {
    /// Register a function to be called before a mnemonic is executed. The function will be called with the Axecutor object as first argument.
    /// The function may be sync or async and *MUST* return the result of one of the following functions:
    ///  - instance.commit(): Continue execution, keep data
    ///  - instance.stop(): Stop execution, keep data
    ///  - instance.unchanged(): Continue execution, but discard data changed in the hook
    /// You can register multiple functions for the same mnemonic, the order of execution is however not defined.
    pub fn hook_before_mnemonic(
        &mut self,
        mnemonic: SupportedMnemonic,
        cb: JsValue,
    ) -> Result<(), AxError> {
        debug_log!(
            "Calling Axecutor::hook_before_mnemonic, hooks_running={}",
            self.hooks.running
        );

        if self.hooks.running {
            return Err(AxError::from(
                "Cannot add hooks while another hook is running",
            ));
        }

        if cb.has_type::<js_sys::Function>() {
            let function = cb.dyn_into::<Function>().map_err(|_| {
                AxError::from("The provided callback is not a function. Please provide a function.")
            })?;

            debug_log!(
                "Previous entry: {:?}",
                self.hooks.mnemonic_hooks.entry(mnemonic)
            );

            self.hooks
                .mnemonic_hooks
                .entry(mnemonic)
                .or_insert_with(Hook::new)
                .before
                .push(function);

            debug_log!(
                "Updated entry: {:?}",
                self.hooks.mnemonic_hooks.entry(mnemonic)
            );

            Ok(())
        } else {
            debug_log!("hook_before_mnemonic: Provided callback is not a function");
            Err(AxError::from(&*format!(
                "hook_before_mnemonic: expected function or async function argument, but got {:?}",
                cb
            )))
        }
    }

    /// Register a function to be called after a mnemonic is executed. The function will be called with the Axecutor object as first argument.
    /// The function may be sync or async and *MUST* return the result of one of the following functions:
    ///  - instance.commit(): Continue execution, keep data
    ///  - instance.stop(): Stop execution, keep data
    ///  - instance.unchanged(): Continue execution, but discard data changed in the hook
    /// You can register multiple functions for the same mnemonic, the order of execution is however not defined.
    pub fn hook_after_mnemonic(
        &mut self,
        mnemonic: SupportedMnemonic,
        cb: JsValue,
    ) -> Result<(), AxError> {
        debug_log!(
            "Calling Axecutor::hook_after_mnemonic, hooks_running={}",
            self.hooks.running
        );
        if self.hooks.running {
            return Err(AxError::from(
                "Cannot add hooks while another hook is running",
            ));
        }

        if cb.has_type::<js_sys::Function>() {
            let function = cb.dyn_into::<Function>().map_err(|_| {
                AxError::from("The provided callback is not a function. Please provide a function.")
            })?;

            debug_log!(
                "Previous entry: {:?}",
                self.hooks.mnemonic_hooks.entry(mnemonic)
            );
            self.hooks
                .mnemonic_hooks
                .entry(mnemonic)
                .or_insert_with(Hook::new)
                .after
                .push(function);

            debug_log!(
                "Updated entry: {:?}",
                self.hooks.mnemonic_hooks.entry(mnemonic)
            );
            Ok(())
        } else {
            Err(AxError::from(&*format!(
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
    debug_log!("Calling run_promise");
    let promise = js_sys::Promise::from(promise_arg);
    let future = JsFuture::from(promise);
    future.await
}

async fn run_function(
    ax: &mut Axecutor,
    function: js_sys::Function,
    arguments: Vec<JsValue>,
) -> Result<JsValue, JsValue> {
    debug_log!("Calling run_function");

    let args = Array::new();

    let clone = ax.clone();

    // This seems to be the only way this works
    args.push(&JsValue::from(clone));

    for arg in arguments {
        args.push(&arg);
    }

    debug_log!("Calling function.apply");
    let mut result = function.apply(&JsValue::NULL, &args)?;

    // async funtions return promises
    if result.has_type::<js_sys::Promise>() {
        debug_log!("Result is a promise, calling run_promise");
        result = run_promise(result).await?;
        debug_log!("Finished calling run_promise");
    }

    if result.is_null() {
        debug_log!("Result is null, not updating our own state");
    } else {
        debug_log!("Updating Axecutor state after hook");
        ax.state_from_committed(result)?;
    }

    debug_log!("Finished calling run_function");
    Ok(JsValue::UNDEFINED)
}
