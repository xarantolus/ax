extern crate lazy_static;

#[cfg(all(target_arch = "wasm32", not(test)))]
use js_sys::Array;
#[cfg(all(target_arch = "wasm32", not(test)))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", not(test)))]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(all(target_arch = "wasm32", not(test)))]
use wasm_bindgen_futures::JsFuture;

use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use crate::helpers::debug::debug_log;
use crate::instructions::generated::SupportedMnemonic;
use crate::{axecutor::Axecutor, helpers::errors::AxError};
use std::error::Error;
use std::fmt::Debug;

// (possible asynchronous) callback function (closure) taking an Axecutor and Mnemonic, returning a result
pub type RustCallbackFunction =
    dyn Fn(&mut Axecutor, SupportedMnemonic) -> Result<HookResult, Box<dyn Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HookResult {
    Handled,
    Unhandled,
}

#[derive(Clone)]
pub(crate) struct Hook {
    native_before: Vec<&'static RustCallbackFunction>,
    native_after: Vec<&'static RustCallbackFunction>,

    #[cfg(all(target_arch = "wasm32", not(test)))]
    js_before: Vec<js_sys::Function>,
    #[cfg(all(target_arch = "wasm32", not(test)))]
    js_after: Vec<js_sys::Function>,
}

impl Hook {
    pub fn new() -> Self {
        Self {
            native_before: Vec::new(),
            native_after: Vec::new(),
            #[cfg(all(target_arch = "wasm32", not(test)))]
            js_before: Vec::new(),
            #[cfg(all(target_arch = "wasm32", not(test)))]
            js_after: Vec::new(),
        }
    }

    async fn run_functions(
        &self,
        before: bool,
        ax: &mut Axecutor,
        mnemonic: SupportedMnemonic,
    ) -> Result<(), AxError> {
        ax.hooks.running = true;

        let functions = if before {
            &self.native_before
        } else {
            &self.native_after
        };

        for function in functions {
            let res = function(ax, mnemonic)?;
            if ax.state.finished || res == HookResult::Handled {
                ax.hooks.running = false;
                return Ok(());
            }
        }

        #[cfg(all(target_arch = "wasm32", not(test)))]
        {
            for fnt in if before {
                &self.js_before
            } else {
                &self.js_after
            } {
                let res = run_function(ax, fnt.clone(), vec![JsValue::from(mnemonic as u32)]).await;
                if let Err(e) = res {
                    debug_log!("Error running hook: {:?}", e);
                    ax.hooks.running = false;
                    return Err(e.into());
                }
                if ax.state.finished {
                    ax.hooks.running = false;
                    return Ok(());
                }
            }
        }

        ax.hooks.running = false;

        Ok(())
    }

    pub async fn run_before(
        &self,
        ax: &mut Axecutor,
        mnemonic: SupportedMnemonic,
    ) -> Result<(), AxError> {
        #[cfg(all(target_arch = "wasm32", not(test)))]
        debug_log!(
            "Calling Hook::run_before with {} native and {} JS hook function(s)",
            self.native_before.len(),
            self.js_before.len()
        );
        #[cfg(not(all(target_arch = "wasm32", not(test))))]
        debug_log!(
            "Calling Hook::run_before with {} native hook function(s)",
            self.native_before.len()
        );

        self.run_functions(true, ax, mnemonic).await?;

        debug_log!("Finished calling Hook::run_before");
        Ok(())
    }

    pub async fn run_after(
        &self,
        ax: &mut Axecutor,
        mnemonic: SupportedMnemonic,
    ) -> Result<(), AxError> {
        #[cfg(all(target_arch = "wasm32", not(test)))]
        debug_log!(
            "Calling Hook::run_after with {} native and {} JS hook function(s)",
            self.native_after.len(),
            self.js_after.len()
        );
        #[cfg(not(all(target_arch = "wasm32", not(test))))]
        debug_log!(
            "Calling Hook::run_after with {} native hook function(s)",
            self.native_after.len()
        );

        self.run_functions(false, ax, mnemonic).await?;

        debug_log!("Finished calling Hook::run_after");
        Ok(())
    }
}

impl Debug for Hook {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[cfg(all(target_arch = "wasm32", not(test)))]
        {
            f.debug_struct("Hook")
                .field("js_before", &self.js_before.len())
                .field("js_after", &self.js_after.len())
                .field("native_before", &self.native_before.len())
                .field("native_after", &self.native_after.len())
                .finish()
        }
        #[cfg(not(all(target_arch = "wasm32", not(test))))]
        f.debug_struct("Hook")
            .field("native_before", &self.native_before.len())
            .field("native_after", &self.native_after.len())
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
        write!(f, "{{")?;
        for (mnem, hook) in self.mnemonic_hooks.iter() {
            #[cfg(all(target_arch = "wasm32", not(test)))]
            {
                write!(
                    f,
                    "\n    {:?}: {{ js_before: {}, js_after: {}, native_before: {}, native_after: {} }},",
                    mnem,
                    hook.js_before.len(),
                    hook.js_after.len(),
                    hook.native_before.len(),
                    hook.native_after.len()
                )?;
            }

            #[cfg(not(all(target_arch = "wasm32", not(test))))]
            {
                write!(
                    f,
                    "\n    {:?}: {{ native_before: {}, native_after: {} }},",
                    mnem,
                    hook.native_before.len(),
                    hook.native_after.len()
                )?;
            }
        }

        write!(f, "\n}}")
    }
}

impl Axecutor {
    pub(crate) fn mnemonic_hooks(&self, mnemonic: SupportedMnemonic) -> Option<Hook> {
        self.hooks.mnemonic_hooks.get(&mnemonic).cloned()
    }
}

// WASM implementation
#[cfg(all(target_arch = "wasm32", not(test)))]
#[wasm_bindgen]
impl Axecutor {
    /// Register a function to be called before a mnemonic is executed.
    /// The function will be called with the Axecutor object and mnemonic as arguments.
    /// The function may be sync or async and *MUST* return the result of one of the following functions:
    ///  - instance.commit(): Continue execution, keep data
    ///  - instance.stop(): Stop execution, keep data
    ///  - instance.unchanged(): Continue execution, but discard data changed in the hook
    /// You can register multiple functions for the same mnemonic, the order of execution is however not defined.
    pub fn hook_before_mnemonic(
        &mut self,
        mnemonic: SupportedMnemonic,
        cb: js_sys::Function,
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
            let function = cb.dyn_into::<js_sys::Function>().map_err(|_| {
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
                .js_before
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

    /// Register a function to be called after a mnemonic is executed.
    /// The function will be called with the Axecutor object and mnemonic as arguments.
    /// The function may be sync or async and *MUST* return the result of one of the following functions:
    ///  - instance.commit(): Continue execution, keep data
    ///  - instance.stop(): Stop execution, keep data
    ///  - instance.unchanged(): Continue execution, but discard data changed in the hook
    /// You can register multiple functions for the same mnemonic, the order of execution is however not defined.
    pub fn hook_after_mnemonic(
        &mut self,
        mnemonic: SupportedMnemonic,
        cb: js_sys::Function,
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
            let function = cb.dyn_into::<js_sys::Function>().map_err(|_| {
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
                .js_after
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

// Normal implementation
impl Axecutor {
    /// Register a function to be called before a mnemonic is executed.
    /// Unlike the JS API, you don't need to return any special values.
    /// The function will be called with the Axecutor object and mnemonic as arguments.
    /// You can register multiple functions for the same mnemonic, the order of execution is however not defined.
    pub fn hook_before_mnemonic_native(
        &mut self,
        mnemonic: SupportedMnemonic,
        cb: &'static RustCallbackFunction,
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

        debug_log!(
            "Previous entry: {:?}",
            self.hooks.mnemonic_hooks.entry(mnemonic)
        );
        self.hooks
            .mnemonic_hooks
            .entry(mnemonic)
            .or_insert_with(Hook::new)
            .native_before
            .push(cb);

        debug_log!(
            "Updated entry: {:?}",
            self.hooks.mnemonic_hooks.entry(mnemonic)
        );
        Ok(())
    }

    /// Register a function to be called after a mnemonic is executed.
    /// Unlike the JS API, you don't need to return any special values.
    /// The function will be called with the Axecutor object and mnemonic as arguments.
    /// You can register multiple functions for the same mnemonic, the order of execution is however not defined.
    pub fn hook_after_mnemonic_native(
        &mut self,
        mnemonic: SupportedMnemonic,
        cb: &'static RustCallbackFunction,
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

        debug_log!(
            "Previous entry: {:?}",
            self.hooks.mnemonic_hooks.entry(mnemonic)
        );
        self.hooks
            .mnemonic_hooks
            .entry(mnemonic)
            .or_insert_with(Hook::new)
            .native_after
            .push(cb);

        debug_log!(
            "Updated entry: {:?}",
            self.hooks.mnemonic_hooks.entry(mnemonic)
        );
        Ok(())
    }
}

// Functions for actually running functions/promises in JS

#[cfg(all(target_arch = "wasm32", not(test)))]
async fn run_promise(promise_arg: JsValue) -> Result<JsValue, JsValue> {
    debug_log!("Calling run_promise");
    let promise = js_sys::Promise::from(promise_arg);
    let future = JsFuture::from(promise);
    future.await
}

#[cfg(all(target_arch = "wasm32", not(test)))]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_reg_value, test_async, write_reg_value};
    use iced_x86::Register::*;

    test_async![hook_before_mnemonic; async {
        let mut ax = Axecutor::new(
            &[
                0x48, 0xc7, 0xc0, 0x5, 0, 0, 0, // mov rax, 5
                0xf, 0x5, // syscall
            ],
            0x1000,
            0x1000,
        )
        .expect("Failed to create axecutor");

        let fnt: &RustCallbackFunction = &|ax: &mut Axecutor, mnemonic: SupportedMnemonic| {
            assert_eq!(mnemonic, SupportedMnemonic::Syscall, "Wrong mnemonic passed to hook handling Syscall");

            assert_reg_value!(q; ax; RAX; 5);
            write_reg_value!(q; ax; RAX; 10);

            Ok(HookResult::Handled)
        };

        ax.hook_before_mnemonic_native(SupportedMnemonic::Syscall, fnt)
            .expect("Failed to add hook");

        ax.execute().await.expect("Failed to execute");

        assert_reg_value!(q; ax; RAX; 10);
    }];

    test_async![hook_after_mnemonic; async {
        let mut ax = Axecutor::new(
            &[
                0x48, 0xc7, 0xc0, 0x5, 0, 0, 0, // mov rax, 5
                0xf, 0x5, // syscall
            ],
            0x1000,
            0x1000,
        )
        .expect("Failed to create axecutor");

        let fnt: &RustCallbackFunction = &|ax: &mut Axecutor, mnemonic: SupportedMnemonic| {
            assert_eq!(mnemonic, SupportedMnemonic::Syscall, "Wrong mnemonic passed to hook handling Syscall");

            assert_reg_value!(q; ax; RAX; 5);
            write_reg_value!(q; ax; RAX; 10);

            Ok(HookResult::Handled)
        };

        ax.hook_after_mnemonic_native(SupportedMnemonic::Syscall, fnt)
            .expect("Failed to add hook");

        ax.execute().await.expect("Failed to execute");

        assert_reg_value!(q; ax; RAX; 10);
    }];

    test_async![stop_hook; async {
        let mut ax = Axecutor::new(
            &[
                0x48, 0xc7, 0xc0, 0x5, 0, 0, 0, // mov rax, 5
                0xf, 0x5, // syscall
            ],
            0x1000,
            0x1000,
        ).expect("Failed to create axecutor");

        ax.hook_after_mnemonic_native(SupportedMnemonic::Mov, &|ax: &mut Axecutor, mnemonic: SupportedMnemonic| {
            assert_eq!(mnemonic, SupportedMnemonic::Mov, "Wrong mnemonic passed to hook handling Mov");

            ax.stop();

            Ok(HookResult::Handled)
        }).expect("Failed to add hook");

        ax.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|_: &mut Axecutor, _: SupportedMnemonic| {
            unreachable!("Syscall hook should not be called as we stop before it");
        }).expect("Failed to add hook");

        ax.execute().await.expect("Failed to execute");

        assert_reg_value!(q; ax; RAX; 5);
    }];

    test_async![modify_outside_vars; async {
        let mut ax = Axecutor::new(
            &[
                0x48, 0xc7, 0xc0, 0x5, 0, 0, 0, // mov rax, 5
                0xf, 0x5, // syscall
            ],
            0x1000,
            0x1000,
        ).expect("Failed to create axecutor");

        // outside_var should last for static so we can use it in a closure/callback
        #[allow(non_upper_case_globals)]
        static mut outside_var: u64 = 0;

        // Make sure we keep access to the outside variable without "temporary value dropped while borrowed" error
        ax.hook_after_mnemonic_native(SupportedMnemonic::Mov, &|_: &mut Axecutor, mnemonic: SupportedMnemonic| {
            assert_eq!(mnemonic, SupportedMnemonic::Mov, "Wrong mnemonic passed to hook handling Mov");

            unsafe {
                outside_var = 10;
            }

            Ok(HookResult::Handled)
        }).expect("Failed to add hook");

        ax.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|_: &mut Axecutor, _: SupportedMnemonic| {
            unsafe {
                assert_eq!(outside_var, 10, "Outside variable was not modified");
            }

            Ok(HookResult::Handled)
        }).expect("Failed to add hook");

        ax.execute().await.expect("Failed to execute");

        unsafe {
            assert_eq!(outside_var, 10, "Outside variable was not modified");
        }
    }];
}
