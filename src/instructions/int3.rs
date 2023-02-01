use iced_x86::Instruction;
use iced_x86::Mnemonic::Int3;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;
use crate::instructions::generated::SupportedMnemonic;

impl Axecutor {
    pub(crate) fn mnemonic_int3(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Int3);

        match i.code() {
            iced_x86::Code::Int3 => self.instr_int3(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Int3", i.code()),
        }
    }

    /// INT3
    ///
    /// CC
    fn instr_int3(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Int3);

        if self.mnemonic_hooks(SupportedMnemonic::Int3).is_some() {
            return Ok(());
        }

        Err(AxError::from(
            "Int3 encountered, but no hook to handle available. Use `hook_before_mnemonic` or `hook_after_mnemonic` to register a hook for interrupts.",
        ))
    }
}
