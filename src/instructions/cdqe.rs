use iced_x86::Code;

use iced_x86::Instruction;
use iced_x86::Mnemonic::Cdqe;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;

use crate::state::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_cdqe(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cdqe);

        match i.code() {
            Code::Cdqe => self.instr_cdqe(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Cdqe", i.code()),
        }
    }

    /// CDQE
    ///
    /// o64 98
    fn instr_cdqe(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Code::Cdqe);

        // Sign-extend EAX into RAX.
        let rax_value = self.reg_read_32(EAX)? as i32 as i64 as u64;

        self.reg_write_64(RAX, rax_value)
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // cdqe
    ax_test![cdqe_rax_176; 0x48, 0x98;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xb0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xb0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cdqe
    ax_test![cdqe_rax_536870912; 0x48, 0x98;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x20000000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x20000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cdqe
    ax_test![cdqe_rax_9223372036854775808; 0x48, 0x98;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cdqe
    ax_test![cdqe_rax_5820217534605131570; 0x48, 0x98;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x50c59102c9a37f32u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xffffffffc9a37f32u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cdqe
    ax_test![cdqe_rax_281474976710656; 0x48, 0x98;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cdqe
    ax_test![cdqe_rax_668; 0x48, 0x98;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x29c);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x29c);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
