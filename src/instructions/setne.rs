use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Setne;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::state::flags::*;

use crate::helpers::macros::calculate_rm;

use crate::helpers::macros::fatal_error;

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

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_reg_value, ax_test, write_flags, write_reg_value};
    use iced_x86::Register::*;

    // setne al
    ax_test![setne_al; 0xf, 0x95, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // setne al
    ax_test![setne_al_zf_zf; 0xf, 0x95, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];
}
