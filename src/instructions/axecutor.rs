use std::collections::HashMap;

use iced_x86::{Decoder, DecoderOptions, Instruction};
use wasm_bindgen::prelude::*;

use super::errors::AxError;
use super::memory::MemoryArea;
use super::registers::{randomized_register_set, RegisterWrapper};

const EXAMPLE_CODE_RIP: u64 = 0x0000_7FFA_C46A_CDA4;
static EXAMPLE_CODE: &[u8] = &[
    0x48, 0x89, 0x5C, 0x24, 0x10, 0x48, 0x89, 0x74, 0x24, 0x18, 0x55, 0x57, 0x41, 0x56, 0x48, 0x8D,
    0xAC, 0x24, 0x00, 0xFF, 0xFF, 0xFF, 0x48, 0x81, 0xEC, 0x00, 0x02, 0x00, 0x00, 0x48, 0x8B, 0x05,
    0x18, 0x57, 0x0A, 0x00, 0x48, 0x33, 0xC4, 0x48, 0x89, 0x85, 0xF0, 0x00, 0x00, 0x00, 0x4C, 0x8B,
    0x05, 0x2F, 0x24, 0x0A, 0x00, 0x48, 0x8D, 0x05, 0x78, 0x7C, 0x04, 0x00, 0x33, 0xFF,
];

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
                // TODO: Figure out correct default value for rflags
                rflags: 0,
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
                dec.position()
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
