use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{axecutor::Axecutor, instructions::generated::SupportedMnemonic};

use super::{debug::debug_log, errors::AxError};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Syscall {
    Exit = 60,
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
    pub fn handle_syscalls(&mut self, list: Vec<wasm_bindgen::JsValue>) -> Result<(), AxError> {
        self.handle_syscalls_impl(from_js_vec(list)?)
    }
}

#[cfg(all(target_arch = "wasm32", not(test)))]
fn from_js_vec(vec: Vec<wasm_bindgen::JsValue>) -> Result<Vec<Syscall>, AxError> {
    let mut result = Vec::new();
    for s in vec {
        let number = s
            .as_f64()
            .ok_or_else(|| AxError::from(format!("Invalid syscall: {:?}", s).as_str()))?;
        result.push(Syscall::try_from(number as isize)?);
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
            match syscall {
                Syscall::Exit => self.register_exit()?,
            }
        }

        Ok(())
    }

    fn register_exit(&mut self) -> Result<(), AxError> {
        self.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|ax: &mut Axecutor, _| {
            debug_log!("Running native exit syscall");
            ax.state.finished = true;
            Ok(())
        })
    }
}
