use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Setne;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;

use crate::{calculate_rm, fatal_error};

impl Axecutor {
    pub fn mnemonic_setne(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Setne);

        match i.code() {
            Setne_rm8 => self.instr_setne_rm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Setne", i.code()),
        }
    }

    /// SETNE r/m8
    ///
    /// 0F 95 /r
    fn instr_setne_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Setne_rm8);

        if self.state.rflags & FLAG_ZF == 0 {
            calculate_rm![u8f; self; i; |_: u8| {
                (1, 0)
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            calculate_rm![u8f; self; i; |_: u8| {
                (0, 0)
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        }
    }
}
