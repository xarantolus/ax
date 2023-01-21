use iced_x86::Instruction;
use iced_x86::Mnemonic::Cwd;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;

use crate::state::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_cwd(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cwd);

        match i.code() {
            iced_x86::Code::Cwd => self.instr_cwd(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Cwd", i.code()),
        }
    }

    /// CWD
    ///
    /// o16 99
    fn instr_cwd(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Cwd);

        let ax_value = self.reg_read_16(AX)?;
        let dx_value = if ax_value & 0x8000 == 0x8000 {
            0xFFFF
        } else {
            0x0000
        };

        self.reg_write_16(DX, dx_value)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // cwd
    ax_test![cwd_ax_16384_dx_1; 0x66, 0x99; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x4000);
            write_reg_value!(w; a; DX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4000);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cwd
    ax_test![cwd_ax_32767_dx_33; 0x66, 0x99; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7fff);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cwd
    ax_test![cwd_ax_16_dx_31; 0x66, 0x99; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x10);
            write_reg_value!(w; a; DX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x10);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cwd
    ax_test![cwd_ax_128_dx_1024; 0x66, 0x99; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x80);
            write_reg_value!(w; a; DX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x80);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cwd
    ax_test![cwd_ax_32768_dx_32767; 0x66, 0x99; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(w; a; DX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_reg_value!(w; a; DX; 0xffff);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cwd
    ax_test![cwd_ax_32768_dx_4096; 0x66, 0x99; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(w; a; DX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_reg_value!(w; a; DX; 0xffff);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
