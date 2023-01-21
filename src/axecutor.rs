use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::helpers::debug::debug_log;
use crate::state::flags::FLAG_TO_NAMES;

use crate::helpers::errors::AxError;
use crate::state::hooks::HookProcessor;
use crate::state::memory::MemoryArea;
use crate::state::registers::{randomized_register_set, SupportedRegister};

extern crate console_error_panic_hook;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Axecutor is the main struct that is used for emulation.
///
/// It can be instantiated in JavaScript using one of the following methods:
///  - `let ax = new Axecutor(code: Uint8Array, code_start_addr: bigint, initial_rip: bigint)`
///  - `let ax = Axecutor.fromBinary(elf_binary: Uint8Array)`.
///
/// Afterwards, one can register hooks before/after instructions:
///  - `ax.hook_before_mnemonic(Mnemonic.Syscall, (axInstance: Axecutor) => {...});`
///  - `ax.hook_after_mnemonic(Mnemonic.Syscall, (axInstance: Axecutor) => {...});`
pub struct Axecutor {
    // code_start_address is the memory address/RIP of the first instruction, which isn't necessarily the entrypoint (can be set by writing to RIP)
    pub(crate) code_start_address: u64,
    // code_length is the length of the encoded instructions in bytes
    pub(crate) code_length: u64,

    // code holds the encoded instructions
    pub(crate) code: Vec<u8>,

    // stack_initial_rsp is the initial RSP address when starting, this allows top-level `ret`s to finish without errors. It's set by init_stack
    pub(crate) stack_top: u64,

    // state holds the current state of the execution
    // NOTE THAT ANY VALUE NOT STORED IN THE STATE WILL *NOT* BE KEPT BETWEEN HOOKS -- ANY STATE MUST LIVE IN THE STATE
    pub(crate) state: MachineState,

    #[serde(skip)]
    pub(crate) hooks: HookProcessor,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MachineState {
    // TODO: memory could better be modeled with some kind of interval tree that allows storing additional data with each interval
    pub(crate) memory: Vec<MemoryArea>,
    pub(crate) registers: HashMap<SupportedRegister, u64>,
    pub(crate) rflags: u64,
    pub(crate) fs: u64,
    // finished is true if the execution has finished. State may be mutated or read after execution, but no further step-calls must be made
    pub(crate) finished: bool,
    // executed_instructions_count is the number of instructions that have been executed so far
    pub(crate) executed_instructions_count: u64,
}

#[wasm_bindgen]
impl MachineState {
    #[wasm_bindgen(js_name = toStringIdent)]
    pub fn to_string_ident(&self, i: usize) -> String {
        let mut s = String::new();

        s.push_str("MachineState {\n");
        s.push_str(&format!("{}    memory: [\n", " ".repeat(i * 4)));
        for area in &self.memory {
            s.push_str(&format!(
                "{}        {},\n",
                " ".repeat(i * 4),
                area.to_string_ident(i + 2)
            ));
        }

        s.push_str(&format!("{}    ],\n", " ".repeat(i * 4)));
        s.push_str(&format!("{}    registers: {{\n", " ".repeat(i * 4)));

        // Iterate over all registers, sorted by the order i like
        for register in crate::state::registers::NATURAL_REGISTER_ORDER.iter() {
            if let Some(value) = self.registers.get(register) {
                s.push_str(&format!(
                    "{}        {}: {}{:#018x},\n",
                    " ".repeat(i * 4),
                    register.name(),
                    if register.name().len() == 2 { " " } else { "" },
                    value
                ));
            }
        }

        s.push_str(&format!("{}    }},\n", " ".repeat(i * 4)));

        // Write rflags as 64-bit hex value with leading 0x AND also stringify them using the FLAG_TO_NAMES hashmap
        s.push_str(&format!(
            "{}    rflags_raw: 0x{:#016x},\n",
            " ".repeat(i * 4),
            self.rflags
        ));

        s.push_str(&format!("{}    rflags: [\n", " ".repeat(i * 4)));
        for (flag, name) in FLAG_TO_NAMES.iter() {
            if self.rflags & flag != 0 {
                s.push_str(&format!("{}        {},\n", " ".repeat(i * 4), name));
            }
        }

        s.push_str(&format!("{}    ],\n", " ".repeat(i * 4)));

        s.push_str(&format!(
            "{}    fs: {:#018x},\n",
            " ".repeat(i * 4),
            self.fs
        ));
        s.push_str(&format!(
            "{}    finished: {},\n",
            " ".repeat(i * 4),
            self.finished
        ));
        s.push_str(&format!(
            "{}    executed_instructions_count: {},\n",
            " ".repeat(i * 4),
            self.executed_instructions_count
        ));

        s.push_str(&format!("{}}}", " ".repeat(i * 4)));

        s
    }
}

#[wasm_bindgen]
impl Axecutor {
    #[wasm_bindgen(constructor)]
    /// Creates a new Axecutor instance from the given x86-64 instruction bytes, writing the code to memory at `code_start_addr` and setting the initial RIP to `initial_rip`.
    pub fn new(code: &[u8], code_start_addr: u64, initial_rip: u64) -> Result<Axecutor, AxError> {
        debug_log!("Calling Axecutor::new");

        // In case of panics, we want more info in console.error
        #[cfg(all(target_arch = "wasm32", not(test)))]
        {
            console_error_panic_hook::set_once();
            debug_log!("Panic hook set");
        }

        debug_log!("Creating Axecutor");
        Ok(Self {
            code_start_address: code_start_addr,
            code_length: code.len() as u64,
            code: code.to_vec(),
            stack_top: 0,
            hooks: HookProcessor::default(),
            state: MachineState {
                finished: false,
                executed_instructions_count: 0,
                memory: Vec::new(),
                registers: randomized_register_set(initial_rip),
                // Intel SDM 3.4.3 EFLAGS Register mentions "0x00000002" as default value, but this conflicts with some test cases.
                // Also the initial value shouldn't matter much
                rflags: 0,
                fs: 0,
            },
        })
    }

    #[wasm_bindgen(js_name = toString)]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        debug_log!("Calling Axecutor::to_string");
        format!(
            "Axecutor {{
    code_start_address: {:#0x},
    code_length: {:#x},
    hooks: {},
    state: {},
}}",
            self.code_start_address,
            self.code_length,
            self.prefix_each_line(self.hooks.to_string().as_str(), "    "),
            self.state.to_string_ident(1),
        )
    }

    fn prefix_each_line(&self, s: &str, prefix: &str) -> String {
        let mut result = String::new();

        // Prefix each line except for the first
        for (idx, line) in s.lines().enumerate() {
            if idx != 0 {
                result.push('\n');
                result.push_str(prefix);
            }

            result.push_str(line);
        }

        result
    }

    /// This function should be returned from a hook to indicate the state should be kept and execution should continue.
    #[cfg(all(target_arch = "wasm32", not(test)))]
    pub fn commit(&self) -> Result<JsValue, AxError> {
        debug_log!(
            "Calling Axecutor::commit, finished: {}, hooks_running: {}",
            self.state.finished,
            self.hooks.running
        );
        if !self.hooks.running {
            return Err(AxError::from("Cannot call commit() outside of a hook"));
        }

        debug_log!("FS value in commit: {:#x}", self.state.fs);

        let s = serde_wasm_bindgen::Serializer::new().serialize_large_number_types_as_bigints(true);

        self.state
            .serialize(&s)
            .map_err(|e| AxError::from(&*format!("Failed to serialize: {}", e)))
    }

    /// This function should be returned from a hook to indicate the state should be kept and execution should stop.
    /// You can also call this function from ouside of a hook, it will however not stop any running hooks.
    #[cfg(all(target_arch = "wasm32", not(test)))]
    pub fn stop(&mut self) -> Result<JsValue, AxError> {
        debug_log!(
            "Calling Axecutor::stop, finished: {}, hooks_running: {}",
            self.state.finished,
            self.hooks.running
        );

        self.state.finished = true;
        if self.hooks.running {
            self.commit()
        } else {
            Ok(JsValue::UNDEFINED)
        }
    }

    /// This function can be called from a hook to indicate the state should be kept and execution should stop.
    #[cfg(not(all(target_arch = "wasm32", not(test))))]
    pub fn stop(&mut self) {
        debug_log!(
            "Calling Axecutor::stop, finished: {}, hooks_running: {}",
            self.state.finished,
            self.hooks.running
        );

        self.state.finished = true;
    }

    /// This function should be returned from a hook to indicate the state should *not* be kept, but execution should continue.
    #[cfg(all(target_arch = "wasm32", not(test)))]
    pub fn unchanged(&self) -> JsValue {
        debug_log!(
            "Calling Axecutor::unchanged, finished: {}, hooks_running: {}",
            self.state.finished,
            self.hooks.running
        );
        JsValue::NULL
    }

    #[cfg(all(target_arch = "wasm32", not(test)))]
    pub(crate) fn state_from_committed(&mut self, value: JsValue) -> Result<(), AxError> {
        debug_log!(
            "Calling Axecutor::state_from_committed, finished: {}, hooks_running: {}",
            self.state.finished,
            self.hooks.running
        );

        if !self.hooks.running {
            return Err(AxError::from(
                "Cannot call state_from_committed() outside of a hook",
            ));
        }

        if value.is_falsy() {
            return Err(AxError::from("Cannot call state_from_committed() with falsy value. Note that you *must* return either null or Axecutor.commit() from your hook"));
        }

        let state: MachineState = serde_wasm_bindgen::from_value(value).map_err(|e| {
            AxError::from(&*format!(
                "state_from_committed: failed to deserialize state: {}\nNote that you *must* return either Axecutor.unchanged(), Axecutor.stop() or Axecutor.commit() from your hook",
                e,
            ))
        })?;

        self.state = state;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::tests::test_async;

    use iced_x86::Register;

    test_async![test_rip; async {
        let code = [0x48, 0xc7, 0xc0, 0x3, 0x0, 0x0, 0x0];
        let ax = Axecutor::new(&code, 0x1000, 0x1000).unwrap();
        let instruction = ax.decode_next().expect("Failed to get instruction");
        assert_eq!(instruction.ip(), 0x1000);
        assert_eq!(instruction.next_ip(), 0x1000 + code.len() as u64);
        assert_eq!(ax.reg_read_64(Register::RIP.into()).unwrap(), 0x1000);
    }];

    test_async![top_lvl_return_with_stack_setup; async {
        // Basically if we ret from the top of the stack, we want to finish without error
        let code = [0xc3];
        let mut ax = Axecutor::new(&code, 0x1000, 0x1000).unwrap();
        ax.init_stack(0).expect("Failed to init stack");
        if let Err(e) = ax.execute().await {
            crate::helpers::macros::fatal_error!("Failed to execute: {:?}", e);
        }
        assert!(ax.state.finished);
    }];
}
