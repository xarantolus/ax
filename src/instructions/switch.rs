// THIS FILE IS AUTOGENERATED, DO NOT EDIT
// You can regenerate it using `make switch` after creating a new instruction file with `python3 generate.py <mneumonic>`

use super::{axecutor::Axecutor, errors::AxError};
use iced_x86::{Instruction, Mnemonic::*};

impl Axecutor {
    pub fn switch_instruction_mnemonic(&mut self, i: Instruction) -> Result<(), AxError> {
        match i.mnemonic() {
            Jmp => self.mnemonic_jmp(i),
            Lea => self.mnemonic_lea(i),
            Mov => self.mnemonic_mov(i),
            Pop => self.mnemonic_pop(i),
            Push => self.mnemonic_push(i),
            Shl => self.mnemonic_shl(i),
            Test => self.mnemonic_test(i),
            Xor => self.mnemonic_xor(i),
            _ => Err(AxError::from(format!(
                "unimplemented mnemonic {:?}",
                i.mnemonic()
            ))),
        }
    }
}
