use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Xorps;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;

use crate::helpers::operand::Operand;

use crate::state::registers::SupportedRegister;

impl Axecutor {
    pub fn mnemonic_xorps(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Xorps);

        match i.code() {
            Xorps_xmm_xmmm128 => self.instr_xorps_xmm_xmmm128(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Xorps", i.code()),
        }
    }

    /// XORPS xmm1, xmm2/m128
    ///
    /// NP 0F 57 /r
    fn instr_xorps_xmm_xmmm128(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Xorps_xmm_xmmm128);

        let (dest, src) = self.instruction_operands_2(i)?;

        let dest_reg: SupportedRegister = dest.into();

        let src_value = match src {
            Operand::Memory(m) => self.internal_mem_read_128(self.mem_addr(m))?,
            Operand::Register(r) => self.internal_reg_read_128(r)?,
            _ => fatal_error!("Invalid operand for Movups_xmm_xmmm128"),
        };

        let dest_value = self.internal_reg_read_128(dest_reg)?;

        self.internal_reg_write_128(dest_reg, dest_value ^ src_value)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
