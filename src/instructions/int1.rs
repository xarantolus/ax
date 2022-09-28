use iced_x86::Instruction;
use iced_x86::Mnemonic::Int1;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::generated::SupportedMnemonic;

impl Axecutor {
    pub fn mnemonic_int1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Int1);

        match i.code() {
            iced_x86::Code::Int1 => self.instr_int1(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Int1", i.code()),
        }
    }

    /// INT1
    ///
    /// F1
    fn instr_int1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Int1);

        if let Some(_) = self.mnemonic_hooks(SupportedMnemonic::Int1) {
            return Ok(());
        }

        Err(AxError::from(
            "Int1 encountered, but no hook to handle available. Use `hook_before_mnemonic` or `hook_after_mnemonic` to register a hook for interrupts.",
        ))
    }
}
