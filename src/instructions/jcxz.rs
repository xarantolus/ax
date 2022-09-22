use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jcxz;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister;
use crate::{calculate_r_rm, calculate_rm_imm, calculate_rm_r};

impl Axecutor {
    pub fn mnemonic_jcxz(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jcxz);

        match i.code() {
            Jcxz_rel8_16 => self.instr_jcxz_rel8_16(i),
            Jcxz_rel8_32 => self.instr_jcxz_rel8_32(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Jcxz", i.code()),
        }
    }

    /// JCXZ rel8
    ///
    /// a16 o16 E3 cb
    fn instr_jcxz_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jcxz_rel8_16);

        todo!("instr_jcxz_rel8_16 for Jcxz")
    }

    /// JCXZ rel8
    ///
    /// a16 o32 E3 cb
    fn instr_jcxz_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jcxz_rel8_32);

        todo!("instr_jcxz_rel8_32 for Jcxz")
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::SupportedRegister, jmp_test,
        write_reg_value,
    };
    use iced_x86::Register::*;
}
