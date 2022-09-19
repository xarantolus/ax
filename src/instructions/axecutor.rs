use std::collections::HashMap;

use iced_x86::{Decoder, DecoderOptions, Instruction};
use wasm_bindgen::prelude::*;

use super::errors::AxError;
use super::memory::MemoryArea;
use super::registers::{randomized_register_set, RegisterWrapper};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Axecutor {
    pub(crate) finished: bool,

    pub(crate) instructions: Vec<Instruction>,
    pub(crate) rip_to_index: HashMap<u64, usize>,

    pub(crate) state: MachineState,
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
impl Axecutor {
    #[wasm_bindgen(constructor)]
    pub fn new(code: &[u8], initial_rip: u64) -> Result<Axecutor, AxError> {
        let instructions = decode_all(code, initial_rip)?;

        let mut rti = HashMap::new();
        for (idx, instr) in instructions.iter().enumerate() {
            rti.insert(instr.ip(), idx);
        }

        Ok(Self {
            finished: false,
            instructions,
            rip_to_index: rti,
            state: MachineState {
                memory: Vec::new(),
                registers: randomized_register_set(initial_rip),
                // TODO: Think about how to handle flags
                // Intel SDM 3.4.3 EFLAGS Register mentions this default value:
                rflags: 0x00000002,
            },
        })
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "Axecutor {{ ran: {}, instructions: {:#?}, rip_to_index: {:#?}, state: {:#?} }}",
            self.finished, self.instructions, self.rip_to_index, self.state
        )
    }
}

fn decode_all(code: &[u8], initial_rip: u64) -> Result<Vec<Instruction>, AxError> {
    if code.is_empty() {
        return Err(AxError::from("Cannot decode empty code buffer"));
    }

    let mut dec = Decoder::with_ip(64, code, initial_rip, DecoderOptions::NONE);
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
    use iced_x86::Register;

    use super::*;

    #[test]
    fn test_rip() {
        let code = [0x48, 0xc7, 0xc0, 0x3, 0x0, 0x0, 0x0];
        let ax = Axecutor::new(&code, 0x1000).unwrap();
        assert_eq!(ax.instructions.len(), 1);
        assert_eq!(ax.instructions[0].ip(), 0x1000);
        assert_eq!(ax.instructions[0].next_ip(), 0x1000 + code.len() as u64);
        assert_eq!(ax.reg_read_64(Register::RIP.into()), 0x1000);
    }
}
