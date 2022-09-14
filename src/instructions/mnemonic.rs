use iced_x86::Instruction;
use iced_x86::Mnemonic::*;

use super::axecutor::Axecutor;
use super::errors::AxError;

impl Axecutor {
    pub(crate) fn execute_instruction(&mut self, i: Instruction) -> Result<(), AxError> {
        match i.mnemonic() {
            Push => self.mnemonic_push(i),
            _ => Err(AxError::from(format!(
                "cannot execute instruction with mnemonic {:?}: mnemonic not implemented",
                i.mnemonic()
            ))),
        }
    }
}
