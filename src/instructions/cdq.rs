use iced_x86::Instruction;
use iced_x86::Mnemonic::Cdq;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::macros::fatal_error;

use crate::instructions::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_cdq(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cdq);

        match i.code() {
            iced_x86::Code::Cdq => self.instr_cdq(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Cdq", i.code()),
        }
    }

    /// CDQ
    ///
    /// o32 99
    fn instr_cdq(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Cdq);

        let eax = self.reg_read_32(EAX)?;
        let edx = if eax & 0x8000_0000 == 0 {
            0
        } else {
            0xFFFF_FFFF
        };

        self.reg_write_32(EDX, edx)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::axecutor::Axecutor;
    use crate::instructions::tests::{assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // cdq
    ax_test![cdq_eax_2048_edx_1; 0x99; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x800);
            write_reg_value!(d; a; EDX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x800);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cdq
    ax_test![cdq_eax_2147483648_edx_4; 0x99; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(d; a; EDX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_reg_value!(d; a; EDX; 0xffffffffu32);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
