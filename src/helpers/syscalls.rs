use std::{collections::HashMap, convert::TryFrom};

use rand::Rng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    auto::generated::SupportedMnemonic,
    axecutor::Axecutor,
    helpers::macros::assert_fatal,
    state::{hooks::HookResult, registers::SupportedRegister::*},
};

#[cfg(all(target_arch = "wasm32", not(test)))]
use wasm_bindgen::JsValue;

use super::{debug::debug_log, errors::AxError};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
/// Syscalls that can be registered for automatic handling
pub enum Syscall {
    Brk = 12,
    Pipe = 22,
    Exit = 60,
    ArchPrctl = 158,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SyscallState {
    registered: Vec<Syscall>,

    brk_start: u64,
    brk_length: u64,

    // Map write ends of pipes to read ends
    pipes_write_ends: HashMap<u64, u64>,
    // Map read ends of pipes to write ends
    pipes_read_ends: HashMap<u64, u64>,

    // This maps the read end of a pipe to the contents of the pipe
    // To write, resolve the read end of the pipe via pipe_write_ends and write to the pipe_contents
    pipe_contents: HashMap<u64, Vec<u8>>,
}

impl TryFrom<u16> for Syscall {
    type Error = AxError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            12 => Syscall::Brk,
            22 => Syscall::Pipe,
            60 => Syscall::Exit,
            158 => Syscall::ArchPrctl,
            _ => return Err(AxError::from(format!("Unknown syscall: {value}").as_str())),
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
            result.push(Syscall::try_from(n.round() as u16)?);
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
        if self.hooks.running {
            return Err(AxError::from(
                "Cannot register syscalls while hooks are running",
            ));
        }

        for syscall in list {
            if self.state.syscalls.registered.contains(&syscall) {
                continue;
            }

            match syscall {
                Syscall::Exit => self.register_exit()?,
                Syscall::Pipe => self.register_pipe()?,
                Syscall::Brk => self.register_brk()?,
                Syscall::ArchPrctl => self.register_arch_prctl()?,
            }

            self.state.syscalls.registered.push(syscall);
        }

        Ok(())
    }

    fn register_exit(&mut self) -> Result<(), AxError> {
        self.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|ax: &mut Axecutor, _| {
            if ax.reg_read_64(RAX)? != Syscall::Exit as u64 {
                return Ok(HookResult::Unhandled);
            }

            debug_log!(
                "Running native exit syscall with code {}",
                ax.reg_read_64(RDI)?
            );

            ax.state.finished = true;

            Ok(HookResult::Handled)
        })
    }

    fn register_pipe(&mut self) -> Result<(), AxError> {
        self.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|ax: &mut Axecutor, _| {
            if ax.reg_read_64(RAX)? != Syscall::Pipe as u64 {
                return Ok(HookResult::Unhandled);
            }

            debug_log!("Running native pipe syscall");

            let read_end = rand::thread_rng().gen::<u16>() as u64 + 1024;
            let write_end = rand::thread_rng().gen::<u16>() as u64 + 1024;
            assert_fatal!(
                !ax.state.syscalls.pipes_read_ends.contains_key(&read_end),
                "Duplicate read end for pipe"
            );
            assert_fatal!(
                !ax.state.syscalls.pipes_write_ends.contains_key(&read_end),
                "Duplicate read end for pipe"
            );
            assert_fatal!(
                !ax.state.syscalls.pipes_read_ends.contains_key(&write_end),
                "Duplicate write end for pipe"
            );
            assert_fatal!(
                !ax.state.syscalls.pipes_write_ends.contains_key(&write_end),
                "Duplicate write end for pipe"
            );

            ax.state
                .syscalls
                .pipes_read_ends
                .insert(read_end, write_end);
            ax.state
                .syscalls
                .pipes_write_ends
                .insert(write_end, read_end);
            ax.state.syscalls.pipe_contents.insert(read_end, Vec::new());

            let fd_ptr = ax.reg_read_64(RDI)?;

            ax.mem_write_64(fd_ptr, read_end)?;
            ax.mem_write_64(fd_ptr + 8, write_end)?;

            ax.reg_write_64(RAX, 0)?;

            debug_log!(
                "pipe syscall created read end {} and write end {}",
                read_end,
                write_end
            );

            Ok(HookResult::Handled)
        })?;

        // Read system call for pipes
        self.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|ax: &mut Axecutor, _| {
            if ax.reg_read_64(RAX)? != 0u64 {
                return Ok(HookResult::Unhandled);
            }

            let fd = ax.reg_read_64(RDI)?;
            let buf = ax.reg_read_64(RSI)?;
            let count = ax.reg_read_64(RDX)?;

            let available_content = match ax.state.syscalls.pipe_contents.get(&fd) {
                Some(bytes) => bytes.clone(),
                // Maybe another hook will handle this fd
                None => return Ok(HookResult::Unhandled),
            };

            debug_log!(
                "Running native read syscall for pipe with fd {}, buf {:#x}, count {}",
                fd,
                buf,
                count
            );

            let max_bytes = std::cmp::min(count, available_content.len() as u64);
            ax.mem_write_bytes(buf, &available_content[..max_bytes as usize])?;
            ax.reg_write_64(RAX, max_bytes)?;

            ax.state
                .syscalls
                .pipe_contents
                .insert(fd, available_content[max_bytes as usize..].to_vec());

            // Skip the rest -- that way users that register read syscalls won't ever see this
            Ok(HookResult::Handled)
        })?;

        // Write system call for pipes
        self.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|ax: &mut Axecutor, _| {
            if ax.reg_read_64(RAX)? != 1u64 {
                return Ok(HookResult::Unhandled);
            }

            let fd = ax.reg_read_64(RDI)?;
            let buf = ax.reg_read_64(RSI)?;
            let count = ax.reg_read_64(RDX)?;

            let write_end = match ax.state.syscalls.pipes_write_ends.get(&fd) {
                Some(write_end) => *write_end,
                // Maybe another hook will handle this fd
                None => return Ok(HookResult::Unhandled),
            };

            debug_log!(
                "Running native write syscall for pipe with fd {}, buf {:#x}, count {}",
                fd,
                buf,
                count
            );

            let bytes = ax.mem_read_bytes(buf, count)?;

            ax.state
                .syscalls
                .pipe_contents
                .entry(write_end)
                .and_modify(|content| content.extend_from_slice(&bytes))
                .or_insert(bytes);

            ax.reg_write_64(RAX, count)?;

            // Skip the rest -- that way users that register write syscalls won't ever see this
            Ok(HookResult::Handled)
        })?;

        Ok(())
    }

    fn register_brk(&mut self) -> Result<(), AxError> {
        self.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|ax: &mut Axecutor, _| {
            if ax.reg_read_64(RAX)? != Syscall::Brk as u64 {
                return Ok(HookResult::Unhandled);
            }

            let brk = ax.reg_read_64(RDI)?;
            debug_log!(
                "Running native brk syscall with argument {:#x}, current brk_start={:#x}, len={:#x}",
                brk, ax.state.syscalls.brk_start, ax.state.syscalls.brk_length
            );


            // If this is the first time we're running brk, we make up some memory
            // and set the start of the brk to that
            if ax.state.syscalls.brk_start == 0 {
                let lenght = 0x1000;
                ax.state.syscalls.brk_start = ax.mem_init_zero_anywhere(lenght)?;
                ax.state.syscalls.brk_length = lenght;

                debug_log!(
                    "Initialized first brk at 0x{:x}",
                    ax.state.syscalls.brk_start
                );
            }

            // If the argument is 0, we just return the current brk_start
            if brk == 0 {
                ax.reg_write_64(RAX, ax.state.syscalls.brk_start)?;
                return Ok(HookResult::Handled);
            }

            // Otherwise, we resize the brk section to the new size
            let new_length = brk - ax.state.syscalls.brk_start;
            ax.mem_resize_section(ax.state.syscalls.brk_start, new_length)?;

            ax.state.syscalls.brk_length = new_length;

            ax.reg_write_64(RAX, ax.state.syscalls.brk_start + new_length)?;

            Ok(HookResult::Handled)
        })
    }

    fn register_arch_prctl(&mut self) -> Result<(), AxError> {
        self.hook_before_mnemonic_native(SupportedMnemonic::Syscall, &|ax: &mut Axecutor, _| {
            if ax.reg_read_64(RAX)? != Syscall::ArchPrctl as u64 {
                return Ok(HookResult::Unhandled);
            }

            // TODO: Make sure this implements the syscall to spec when the memory implementation has been overhauled

            let code = ax.reg_read_64(RDI)?;
            let addr = ax.reg_read_64(RSI)?;

            debug_log!(
                "Running native arch_prctl syscall with code {:#x} and addr {:#x}",
                code,
                addr
            );

            if ax.mem_read_8(addr).is_err() {
                // return EFAULT -- invalid address
                ax.reg_write_64(RAX, 14)?;
                return Ok(HookResult::Handled);
            }

            match code {
                0x1002 => {
                    // ARCH_SET_FS
                    ax.write_fs(addr);
                }
                0x1003 => {
                    // ARCH_SET_GS
                    ax.write_gs(addr);
                }
                0x1001 => {
                    // ARCH_GET_FS
                    ax.reg_write_64(RAX, ax.read_fs())?;
                }
                0x1004 => {
                    // ARCH_GET_GS
                    ax.reg_write_64(RAX, ax.read_gs())?;
                }
                _ => {
                    // return EINVAL -- invalid code
                    ax.reg_write_64(RAX, 22)?;
                }
            }

            Ok(HookResult::Handled)
        })
    }
}

/*
// TODO: Make this test pass
#[cfg(test)]
mod tests {
    use crate::helpers::tests::test_async;

    use super::*;

    test_async![test_syscall; async {
        let mut ax = Axecutor::from_binary(include_bytes!("../../testdata/exit_c.bin"))
        .expect("Failed to load binary");

        ax.init_stack_program_start(0x1000, vec!["/bin/my_binary".to_string()], vec!["USER=test".to_string()]).expect("Failed to init stack");

        ax.handle_syscalls(vec![Syscall::Exit, Syscall::Brk, Syscall::ArchPrctl])
        .expect("Failed to register syscalls");

        ax.execute().await.expect("Failed to run binary");
    }];
}
*/
