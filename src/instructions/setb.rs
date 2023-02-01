use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Setb;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;
use crate::state::flags::*;

use crate::helpers::macros::fatal_error;

use crate::helpers::macros::calculate_rm;

impl Axecutor {
    pub(crate) fn mnemonic_setb(&mut self, i: Instruction) -> Result<(), AxError> {
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
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_reg_value, ax_test, write_flags, write_reg_value};
    use iced_x86::Register::*;

    // setb al
    ax_test![setb_al; 0xf, 0x92, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // setb al
    ax_test![setb_al_cf_cf; 0xf, 0x92, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
