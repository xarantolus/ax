use std::collections::HashMap;

use iced_x86::Register;
use wasm_bindgen::prelude::*;

use super::memory::MemoryArea;
use super::*;

const EXAMPLE_CODE_RIP: u64 = 0x0000_7FFA_C46A_CDA4;
static EXAMPLE_CODE: &[u8] = &[
    0x48, 0x89, 0x5C, 0x24, 0x10, 0x48, 0x89, 0x74, 0x24, 0x18, 0x55, 0x57, 0x41, 0x56, 0x48, 0x8D,
    0xAC, 0x24, 0x00, 0xFF, 0xFF, 0xFF, 0x48, 0x81, 0xEC, 0x00, 0x02, 0x00, 0x00, 0x48, 0x8B, 0x05,
    0x18, 0x57, 0x0A, 0x00, 0x48, 0x33, 0xC4, 0x48, 0x89, 0x85, 0xF0, 0x00, 0x00, 0x00, 0x4C, 0x8B,
    0x05, 0x2F, 0x24, 0x0A, 0x00, 0x48, 0x8D, 0x05, 0x78, 0x7C, 0x04, 0x00, 0x33, 0xFF,
];

#[wasm_bindgen]
pub struct Axecutor {
    pub(crate) ran: bool,

    // TODO: memory could better be modeled with some kind of interval tree that allows storing additional data with each interval
    pub(crate) memory: Vec<MemoryArea>,

    pub(crate) registers: HashMap<Register, u64>,
    pub(crate) rflags: u64,
}

#[wasm_bindgen]
impl Axecutor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            ran: false,
            registers: registers::empty_register_set(),
            // TODO: Figure out correct default value for rflags
            // TODO: Think about how to handle flags
            rflags: 0,
            memory: Vec::new(),
        }
    }

    fn next_instruction() {
        // Read 15 bytes of Memory from current RIP, then decode & execute
    }
}
