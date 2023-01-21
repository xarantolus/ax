use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Nop;

use crate::helpers::macros::fatal_error;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

impl Axecutor {
    pub fn mnemonic_nop(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Nop);

        match i.code() {
            Nopw => self.instr_nopw(i),
            Nopd => self.instr_nopd(i),
            Nopq => self.instr_nopq(i),
            Nop_rm16 => self.instr_nop_rm16(i),
            Nop_rm32 => self.instr_nop_rm32(i),
            Nop_rm64 => self.instr_nop_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Nop", i.code()),
        }
    }

    /// NOP
    ///
    /// o16 90
    fn instr_nopw(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Nopw);

        Ok(())
    }

    /// NOP
    ///
    /// o32 90
    fn instr_nopd(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Nopd);

        Ok(())
    }

    /// NOP
    ///
    /// o64 90
    fn instr_nopq(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Nopq);

        // TODO: This isn't covered by tests, not sure which byte sequence is required
        Ok(())
    }

    /// NOP r/m16
    ///
    /// o16 0F 1F /0
    fn instr_nop_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Nop_rm16);

        Ok(())
    }

    /// NOP r/m32
    ///
    /// o32 0F 1F /0
    fn instr_nop_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Nop_rm32);

        Ok(())
    }

    /// NOP r/m64
    ///
    /// o64 0F 1F /0
    fn instr_nop_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Nop_rm64);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_mem_value, assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // nop
    ax_test![nopd; 0x90; |_: Axecutor| {};
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // nop rax
    ax_test![nop_rax; 0x48, 0xf, 0x1f, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // nop eax
    ax_test![nop_eax; 0xf, 0x1f, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // nop ax
    ax_test![nop_ax; 0x66, 0xf, 0x1f, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // nop word ptr [rax]
    ax_test![nop_word_ptr_rax; 0x66, 0xf, 0x1f, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xchg ax, ax
    ax_test![xchg_ax_ax; 0x66, 0x90; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    // xchg rax, rax
    // interestingly, "xchg eax, eax" is en/decoded to an xchg instruction instead of nop, but rax is nop
    ax_test![xchg_rax_rax; 0x90; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // Tests for "Table 4-12. Recommended Multi-Byte Sequence of NOP Instruction" (Intel Manual)
    ax_test![nop_2byte; 0x66, 0x90; |_: Axecutor| {};
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    ax_test![nop_3byte; 0x0f, 0x1f, 0x0; |_: Axecutor| {};
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    ax_test![nop_4byte; 0x0f, 0x1f, 0x40, 0x0; |_: Axecutor| {};
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    ax_test![nop_5byte; 0xf, 0x1f, 0x44, 0x0, 0x0; |_: Axecutor| {};
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    ax_test![nop_6byte; 0x66, 0x0f, 0x1f, 0x44, 0x0, 0x0; |_: Axecutor| {};
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    ax_test![nop_7byte; 0x0f, 0x1f, 0x80, 0x0, 0x0, 0x0, 0x0; |_: Axecutor| {};
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    ax_test![nop_8byte; 0x0f, 0x1f, 0x84, 0x0, 0x0, 0x0, 0x0, 0x0; |_: Axecutor| {};
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    ax_test![nop_9byte; 0x66, 0x0f, 0x1f, 0x84, 0x0, 0x0, 0x0, 0x0, 0x0; |_: Axecutor| {};
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
