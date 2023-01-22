use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    axecutor::Axecutor, instructions::generated::SupportedMnemonic,
    state::registers::SupportedRegister,
};

#[cfg(all(target_arch = "wasm32", not(test)))]
use wasm_bindgen::JsValue;

use super::{debug::debug_log, errors::AxError};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Syscalls that can be registered for automatic handling
pub enum Syscall {
    Exit = 60,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct SyscallState {
    registered: Vec<Syscall>,
}

impl TryFrom<isize> for Syscall {
    type Error = AxError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Ok(match value {
            60 => Syscall::Exit,
            _ => {
                return Err(AxError::from(
                    format!("Unknown syscall: {}", value).as_str(),
                ))
            }
        })
    }
}

#[cfg(all(target_arch = "wasm32", not(test)))]
#[wasm_bindgen]
#[cfg(all(target_arch = "wasm32", not(test)))]
impl Axecutor {
    /// Register syscalls that the emulator should handle automatically.
    /// The function takes variadic arguments, where each argument is a number of one of the supported Syscalls
    #[wasm_bindgen(variadic)]
    pub fn handle_syscalls(&mut self, syscalls: wasm_bindgen::JsValue) -> Result<(), AxError> {
        self.handle_syscalls_impl(get_js_list(syscalls)?)
    }
}

#[cfg(all(target_arch = "wasm32", not(test)))]
fn get_js_list(vec: JsValue) -> Result<Vec<Syscall>, AxError> {
    use js_sys::Array;

    let mut result = Vec::new();

    let array = Array::try_from(vec).map_err(|_| AxError::from("Invalid syscall list"))?;

    for i in 0..array.length() {
        let item = array.get(i);

        if let Some(n) = item.as_f64() {
            result.push(Syscall::try_from(n.round() as isize)?);
        } else {
            return Err(AxError::from("Invalid syscall list"));
        }
    }

    Ok(result)
}

impl Axecutor {
    #[cfg(not(all(target_arch = "wasm32", not(test))))]
    pub fn handle_syscalls(&mut self, list: Vec<Syscall>) -> Result<(), AxError> {
        self.handle_syscalls_impl(list)
    }

    pub fn handle_syscalls_impl(&mut self, list: Vec<Syscall>) -> Result<(), AxError> {
        for syscall in list {
            if self.state.syscalls.registered.contains(&syscall) {
                continue;
            }

            match syscall {
                Syscall::Exit => self.register_exit()?,
            }

            self.state.syscalls.registered.push(syscall);
        }

        Ok(())
    }

    fn register_exit(&mut self) -> Result<(), AxError> {
        self.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|ax: &mut Axecutor, _| {
            if ax.reg_read_64(SupportedRegister::RAX)? != Syscall::Exit as u64 {
                return Ok(());
            }

            debug_log!("Running native exit syscall");
            ax.state.finished = true;
            Ok(())
        })
    }
}
