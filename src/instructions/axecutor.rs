use std::collections::HashMap;

use iced_x86::{Decoder, DecoderOptions, Instruction};
use wasm_bindgen::prelude::*;

use crate::instructions::flags::FLAG_TO_NAMES;

use super::errors::AxError;
use super::hooks::HookProcessor;
use super::memory::MemoryArea;
use super::registers::{randomized_register_set, RegisterWrapper};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Axecutor {
    pub(crate) finished: bool,

    pub(crate) instructions: Vec<Instruction>,
    pub(crate) rip_to_index: HashMap<u64, usize>,

    pub(crate) state: MachineState,

    pub(crate) hooks: HookProcessor,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub(crate) struct MachineState {
    // TODO: memory could better be modeled with some kind of interval tree that allows storing additional data with each interval
    pub(crate) memory: Vec<MemoryArea>,
    pub(crate) registers: HashMap<RegisterWrapper, u64>,
    pub(crate) rflags: u64,
}

#[wasm_bindgen]
impl MachineState {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.to_string_ident(0)
    }

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
        let instructions = decode_all(code, code_start_addr)?;

        let mut rti = HashMap::new();
        for (idx, instr) in instructions.iter().enumerate() {
            rti.insert(instr.ip(), idx);
        }

        Ok(Self {
            finished: false,
            instructions,
            rip_to_index: rti,
            hooks: HookProcessor::new(),
            state: MachineState {
                memory: Vec::new(),
                registers: randomized_register_set(initial_rip),
                // Intel SDM 3.4.3 EFLAGS Register mentions this default value:
                rflags: 0x00000002,
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
}
