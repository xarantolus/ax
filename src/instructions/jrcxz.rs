use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jrcxz;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister;
use crate::{calculate_r_rm, calculate_rm_imm, calculate_rm_r};

impl Axecutor {
    pub fn mnemonic_jrcxz(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jrcxz);

        match i.code() {
            Jrcxz_rel8_16 => self.instr_jrcxz_rel8_16(i),
            Jrcxz_rel8_64 => self.instr_jrcxz_rel8_64(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Jrcxz", i.code()),
        }
    }

    /// JRCXZ rel8
    ///
    /// a64 o16 E3 cb
    fn instr_jrcxz_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jrcxz_rel8_16);

        todo!("instr_jrcxz_rel8_16 for Jrcxz")
    }

    /// JRCXZ rel8
    ///
    /// a64 o64 E3 cb
    fn instr_jrcxz_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jrcxz_rel8_64);

        todo!("instr_jrcxz_rel8_64 for Jrcxz")
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
