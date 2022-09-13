use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};
use wasm_bindgen::prelude::*;
use iced_x86::Mnemonic::*;


pub(crate) fn run_program(assembled_program: &[u8], rip: u64)  {
    let mut decoder = Decoder::with_ip(
        64,
        assembled_program,
        rip,
        DecoderOptions::NONE,
    );

    // Initialize this outside the loop because decode_out() writes to every field
    let mut decoded_instruction = Instruction::default();

    while decoder.can_decode() {
        // decode_out overwrites all fields, we can reuse decoded_instruction without problems
        decoder.decode_out(&mut decoded_instruction);

        // TODO: If instruction matches criteria for hooks, call hooks before or after or both (this should be configurable)

        // Match the instruction mnemonic, e.g. "mov" or "add"
        match decoded_instruction.mnemonic() {
            Push => {

            }
            _ => {}
        }
    }
}

const HEXBYTES_COLUMN_BYTE_LENGTH: usize = 10;
const EXAMPLE_CODE_BITNESS: u32 = 64;
const EXAMPLE_CODE_RIP: u64 = 0x0000_7FFA_C46A_CDA4;
static EXAMPLE_CODE: &[u8] = &[
    0x48, 0x89, 0x5C, 0x24, 0x10, 0x48, 0x89, 0x74, 0x24, 0x18, 0x55, 0x57, 0x41, 0x56, 0x48, 0x8D,
    0xAC, 0x24, 0x00, 0xFF, 0xFF, 0xFF, 0x48, 0x81, 0xEC, 0x00, 0x02, 0x00, 0x00, 0x48, 0x8B, 0x05,
    0x18, 0x57, 0x0A, 0x00, 0x48, 0x33, 0xC4, 0x48, 0x89, 0x85, 0xF0, 0x00, 0x00, 0x00, 0x4C, 0x8B,
    0x05, 0x2F, 0x24, 0x0A, 0x00, 0x48, 0x8D, 0x05, 0x78, 0x7C, 0x04, 0x00, 0x33, 0xFF,
];

#[wasm_bindgen]
#[derive(Clone)]
pub struct Axecutor {
    pub(crate) ran: bool,
}

#[wasm_bindgen]
impl Axecutor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { ran: false }
    }

    pub fn disassemble(&self) {
        run_program(EXAMPLE_CODE, EXAMPLE_CODE_RIP);
    }
}
