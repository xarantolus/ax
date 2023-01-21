use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Int;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;
use crate::instructions::generated::SupportedMnemonic;

impl Axecutor {
    pub fn mnemonic_int(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Int);

        match i.code() {
            Int_imm8 => self.instr_int_imm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Int", i.code()),
        }
    }

    /// INT imm8
    ///
    /// CD ib
    fn instr_int_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Int_imm8);

        if self.mnemonic_hooks(SupportedMnemonic::Int).is_some() {
            return Ok(());
        }

        Err(AxError::from(
            "Int encountered, but no hook to handle available. Use `hook_before_mnemonic` or `hook_after_mnemonic` to register a hook for interrupts.",
        ))
    }
}
