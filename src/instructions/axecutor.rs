use std::collections::HashMap;

use iced_x86::{Decoder, DecoderOptions, Instruction};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::instructions::flags::FLAG_TO_NAMES;

use super::errors::AxError;
use super::hooks::HookProcessor;
use super::memory::MemoryArea;
use super::registers::{randomized_register_set, SupportedRegister};

extern crate console_error_panic_hook;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Axecutor {
    // code_start_address is the memory address/RIP of the first instruction, which isn't necessarily the entrypoint (can be set by writing to RIP)
    pub(crate) code_start_address: u64,
    // code_length is the length of the encoded instructions in bytes
    pub(crate) code_length: u64,

    // finished is true if the execution has finished. State may be mutated or read after execution, but no further step-calls must be made
    pub(crate) finished: bool,
    // stack_initial_rsp is the initial RSP address when starting, this allows top-level `ret`s to finish without errors. It's set by init_stack
    pub(crate) stack_top: u64,

    // instructions holds all instructions decoded from the input code
    pub(crate) instructions: Vec<Instruction>,
    // rip_to_index maps a RIP address to the index of the instruction in the instructions vector
    pub(crate) rip_to_index: HashMap<u64, usize>,

    // state holds the current state of the execution
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
        for register in super::registers::NATURAL_REGISTER_ORDER.iter() {
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

        s.push_str(&format!("{}}}", " ".repeat(i * 4)));

        s
    }
}

#[wasm_bindgen]
impl Axecutor {
    #[wasm_bindgen(constructor)]
    pub fn new(code: &[u8], code_start_addr: u64, initial_rip: u64) -> Result<Axecutor, AxError> {
        // In case of panics, we want more info in console.error
        #[cfg(all(target_arch = "wasm32", not(test)))]
        {
            console_error_panic_hook::set_once();
        }

        let instructions = decode_all(code, code_start_addr)?;

        let mut rti = HashMap::with_capacity(instructions.len());
        for (idx, instr) in instructions.iter().enumerate() {
            rti.insert(instr.ip(), idx);
        }

        Ok(Self {
            finished: false,
            code_start_address: code_start_addr,
            code_length: code.len() as u64,
            instructions,
            stack_top: 0,
            rip_to_index: rti,
            hooks: HookProcessor::default(),
            state: MachineState {
                memory: Vec::new(),
                registers: randomized_register_set(initial_rip),
                // Intel SDM 3.4.3 EFLAGS Register mentions "0x00000002" as default value, but this conflicts with some test cases.
                // Also the initial value shouldn't matter much
                rflags: 0,
            },
        })
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "Axecutor {{
    ran: {},
    state: {},
    instructions: {},
    rip_to_index: {},
}}",
            self.finished,
            self.state.to_string_ident(1),
            self.prefix_each_line(format!("{:#?}", self.instructions).as_str(), "    "),
            self.prefix_each_line(format!("{:#?}", self.rip_to_index).as_str(), "    ")
        )
    }

    fn prefix_each_line(&self, s: &str, prefix: &str) -> String {
        let mut result = String::new();

        // Prefix each line except for the first
        for (idx, line) in s.lines().enumerate() {
            if idx != 0 {
                result.push_str("\n");
                result.push_str(prefix);
            }

            result.push_str(line);
        }

        result
    }

    #[wasm_bindgen]
    pub fn commit(&self) -> Result<JsValue, JsError> {
        if !self.hooks.running {
            return Err(JsError::new("Cannot call commit() outside of a hook"));
        }

        let s = serde_wasm_bindgen::Serializer::new().serialize_large_number_types_as_bigints(true);

        self.state
            .serialize(&s)
            .map_err(|e| JsError::new(&*format!("Failed to serialize: {}", e)))
    }

    pub(crate) fn state_from_committed(&mut self, value: JsValue) -> Result<(), JsError> {
        if !self.hooks.running {
            return Err(JsError::new(
                "Cannot call state_from_committed() outside of a hook",
            ));
        }

        if value.is_falsy() {
            return Err(JsError::new("Cannot call state_from_committed() with falsy value. Note that you *must* return either null or Axecutor.commit() from your hook"));
        }

        let state: MachineState = serde_wasm_bindgen::from_value(value).map_err(|e| {
            JsError::new(&*format!(
                "state_from_committed: failed to deserialize state: {}\nNote that you *must* return either null or Axecutor.commit() from your hook",
                e,
            ))
        })?;

        self.state = state;
        Ok(())
    }
}

fn decode_all(code: &[u8], code_start_addr: u64) -> Result<Vec<Instruction>, AxError> {
    if code.is_empty() {
        return Err(AxError::from("Cannot decode empty code buffer"));
    }

    let mut dec = Decoder::with_ip(64, code, code_start_addr, DecoderOptions::NONE);
    let mut instructions = Vec::new();

    while dec.can_decode() {
        let instr = dec.decode();
        if instr.is_invalid() {
            return Err(AxError::from(format!(
                "Invalid instruction at offset {}",
                dec.position() - instr.len()
            )));
        }
        instructions.push(instr);
    }

    // Assert that instructions are sorted by ip
    for i in 0..instructions.len() - 1 {
        if instructions[i].ip() > instructions[i + 1].ip() {
            return Err(AxError::from(format!(
                "Instructions are not sorted by instruction pointer after decode: {} > {}",
                instructions[i].ip(),
                instructions[i + 1].ip()
            )));
        }
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_async;
    use iced_x86::Register;

    #[test]
    fn test_rip() {
        let code = [0x48, 0xc7, 0xc0, 0x3, 0x0, 0x0, 0x0];
        let ax = Axecutor::new(&code, 0x1000, 0x1000).unwrap();
        assert_eq!(ax.instructions.len(), 1);
        assert_eq!(ax.instructions[0].ip(), 0x1000);
        assert_eq!(ax.instructions[0].next_ip(), 0x1000 + code.len() as u64);
        assert_eq!(ax.reg_read_64(Register::RIP.into()), 0x1000);
    }

    test_async![top_lvl_return_with_stack_setup; async {
        // ret
        let code = [0xc3];
        let mut ax = Axecutor::new(&code, 0x1000, 0x1000).unwrap();
        ax.init_stack(0).expect("Failed to init stack");
        if let Err(e) = ax.execute().await {
            panic!("Failed to execute: {:?}", AxError::from(e));
        }
        assert!(ax.finished);
    }];
}
