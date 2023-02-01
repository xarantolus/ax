use iced_x86::Instruction;
use iced_x86::Mnemonic::Endbr64;

use crate::helpers::macros::fatal_error;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

impl Axecutor {
    pub(crate) fn mnemonic_endbr64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Endbr64);

        match i.code() {
            iced_x86::Code::Endbr64 => self.instr_endbr64(i),
            _ => fatal_error!(
                "Invalid instruction code {:?} for mnemonic Endbr64",
                i.code()
            ),
        }
    }

    /// ENDBR64
    ///
    /// F3 0F 1E FA
    fn instr_endbr64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Endbr64);

        // Essentially a no-op
        Ok(())
    }
}
