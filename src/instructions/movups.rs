use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Movups;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;

use crate::helpers::operand::Operand;

use crate::state::registers::SupportedRegister;

impl Axecutor {
    pub(crate) fn mnemonic_movups(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Movups);

        match i.code() {
            Movups_xmm_xmmm128 => self.instr_movups_xmm_xmmm128(i),
            Movups_xmmm128_xmm => self.instr_movups_xmmm128_xmm(i),
            _ => fatal_error!(
                "Invalid instruction code {:?} for mnemonic Movups",
                i.code()
            ),
        }
    }

    /// MOVUPS xmm1, xmm2/m128
    ///
    /// NP 0F 10 /r
    fn instr_movups_xmm_xmmm128(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movups_xmm_xmmm128);

        let (dest, src) = self.instruction_operands_2(i)?;

        let dest_reg: SupportedRegister = dest.into();

        let src_value = match src {
            Operand::Memory(m) => self.internal_mem_read_128(self.mem_addr(m))?,
            Operand::Register(r) => self.internal_reg_read_128(r)?,
            _ => fatal_error!("Invalid operand for Movups_xmm_xmmm128"),
        };

        self.internal_reg_write_128(dest_reg, src_value)
    }

    /// MOVUPS xmm2/m128, xmm1
    ///
    /// NP 0F 11 /r
    fn instr_movups_xmmm128_xmm(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movups_xmmm128_xmm);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_reg: SupportedRegister = src.into();

        let src_value = self.internal_reg_read_128(src_reg)?;

        match dest {
            Operand::Memory(m) => self.internal_mem_write_128(self.mem_addr(m), src_value),
            Operand::Register(r) => self.internal_reg_write_128(r, src_value),
            _ => fatal_error!("Invalid operand for Movups_xmmm128_xmm"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // movups xmm0, xmm1
    ax_test![movups_xmm0_xmm1; 0xf, 0x10, 0xc1; |a: &mut Axecutor| {
        write_reg_value!(x; a; XMM0; 0x1234_5678_9abc_def0_u128);
        write_reg_value!(x; a; XMM1; 0x1234_5678_9abc_def0_u128);
    }; |a: Axecutor| {
        assert_reg_value!(x; a; XMM0; 0x1234_5678_9abc_def0_u128);
        assert_reg_value!(x; a; XMM1; 0x1234_5678_9abc_def0_u128);
    }];
}
