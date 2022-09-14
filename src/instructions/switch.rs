use iced_x86::{Instruction, Mnemonic::*};

use super::{axecutor::Axecutor, errors::AxError};
use crate::ax_test;
use crate::instructions::registers::RegisterWrapper;

impl Axecutor {
    pub fn switch_instruction_mnemonic(&mut self, i: Instruction) -> Result<(), AxError> {
        match i.mnemonic() {
            Push => self.mnemonic_push(i),
            Xor => self.mnemonic_xor(i),
            _ => Err(AxError::from(format!(
                "unimplemented mnemonic {:?}",
                i.mnemonic()
            ))),
        }
    }
}
