use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Sete;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::flags::*;

use crate::instructions::macros::calculate_rm;

use crate::instructions::macros::fatal_error;

impl Axecutor {
    pub fn mnemonic_sete(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Sete);

        match i.code() {
            Sete_rm8 => self.instr_sete_rm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Sete", i.code()),
        }
    }

    /// SETE r/m8
    ///
    /// 0F 94 /r
    fn instr_sete_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sete_rm8);

        if self.state.rflags & FLAG_ZF == 0 {
            calculate_rm![u8f; self; i; |_: u8| {
                (0, 0)
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            calculate_rm![u8f; self; i; |_: u8| {
                (1, 0)
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::axecutor::Axecutor;
    use crate::instructions::tests::{assert_reg_value, ax_test, write_flags, write_reg_value};
    use iced_x86::Register::*;

    // sete al
    ax_test![sete_al; 0xf, 0x94, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sete al
    ax_test![sete_al_zf_zf; 0xf, 0x94, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];
}
