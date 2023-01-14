use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Setb;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;

use crate::instructions::macros::fatal_error;

use crate::instructions::macros::calculate_rm;

impl Axecutor {
    pub fn mnemonic_setb(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Setb);

        match i.code() {
            Setb_rm8 => self.instr_setb_rm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Setb", i.code()),
        }
    }

    /// SETB r/m8
    ///
    /// 0F 92 /r
    fn instr_setb_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Setb_rm8);

        if self.state.rflags & FLAG_CF != 0 {
            calculate_rm![u8f; self; i; |_: u8| {
                (1, 0)
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {}
