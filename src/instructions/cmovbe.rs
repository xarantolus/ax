use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Cmovbe;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;
use crate::helpers::macros::calculate_r_rm;

use crate::helpers::macros::fatal_error;

use crate::state::flags::*;

impl Axecutor {
    pub(crate) fn mnemonic_cmovbe(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cmovbe);

        match i.code() {
            Cmovbe_r16_rm16 => self.instr_cmovbe_r16_rm16(i),
            Cmovbe_r32_rm32 => self.instr_cmovbe_r32_rm32(i),
            Cmovbe_r64_rm64 => self.instr_cmovbe_r64_rm64(i),
            _ => fatal_error!(
                "Invalid instruction code {:?} for mnemonic Cmovbe",
                i.code()
            ),
        }
    }

    /// CMOVBE r16, r/m16
    ///
    /// o16 0F 46 /r
    fn instr_cmovbe_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovbe_r16_rm16);

        if self.state.rflags & (FLAG_ZF | FLAG_CF) != 0 {
            calculate_r_rm![u16; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVBE r32, r/m32
    ///
    /// o32 0F 46 /r
    fn instr_cmovbe_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovbe_r32_rm32);

        if self.state.rflags & (FLAG_ZF | FLAG_CF) != 0 {
            calculate_r_rm![u32; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVBE r64, r/m64
    ///
    /// o64 0F 46 /r
    fn instr_cmovbe_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovbe_r64_rm64);

        if self.state.rflags & (FLAG_ZF | FLAG_CF) != 0 {
            calculate_r_rm![u64; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{
        assert_mem_value, assert_reg_value, ax_test, init_mem_value, write_flags, write_reg_value,
    };
    use iced_x86::Register::*;

    // cmovbe dx, word ptr [rcx]
    ax_test![cmovbe_dx_word_ptr_rcx; 0x66, 0xf, 0x46, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; DX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; DX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe dx, word ptr [rcx]
    ax_test![cmovbe_dx_word_ptr_rcx_cf_cf; 0x66, 0xf, 0x46, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; DX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x2);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; DX; 0x2);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x2);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe dx, word ptr [rcx]
    ax_test![cmovbe_dx_word_ptr_rcx_cf_zf_cf_zf; 0x66, 0xf, 0x46, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; DX; 0x3);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x4);
            write_flags!(a; FLAG_CF | FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; DX; 0x4);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x4);
        };
        (FLAG_CF | FLAG_ZF; FLAG_PF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe dx, word ptr [rcx]
    ax_test![cmovbe_dx_word_ptr_rcx_zf_zf; 0x66, 0xf, 0x46, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; DX; 0x4);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x5);
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; DX; 0x5);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x5);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe edx, r11d
    ax_test![cmovbe_edx_r11d; 0x41, 0xf, 0x46, 0xd3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x1);
            write_reg_value!(d; a; R11D; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x1);
            assert_reg_value!(d; a; R11D; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe edx, r11d
    ax_test![cmovbe_edx_r11d_cf_cf; 0x41, 0xf, 0x46, 0xd3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x1);
            write_reg_value!(d; a; R11D; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x0);
            assert_reg_value!(d; a; R11D; 0x0);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe edx, r11d
    ax_test![cmovbe_edx_r11d_cf_zf_cf_zf; 0x41, 0xf, 0x46, 0xd3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(d; a; R11D; 0x1);
            write_flags!(a; FLAG_CF | FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x1);
            assert_reg_value!(d; a; R11D; 0x1);
        };
        (FLAG_CF | FLAG_ZF; FLAG_PF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe edx, r11d
    ax_test![cmovbe_edx_r11d_zf_zf; 0x41, 0xf, 0x46, 0xd3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(d; a; R11D; 0x1);
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x1);
            assert_reg_value!(d; a; R11D; 0x1);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe rdx, rcx
    ax_test![cmovbe_rdx_rcx; 0x48, 0xf, 0x46, 0xd1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe rdx, rcx
    ax_test![cmovbe_rdx_rcx_cf_cf; 0x48, 0xf, 0x46, 0xd1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe rdx, rcx
    ax_test![cmovbe_rdx_rcx_cf_zf_cf_zf; 0x48, 0xf, 0x46, 0xd1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x135);
            write_reg_value!(q; a; RCX; 0x1);
            write_flags!(a; FLAG_CF | FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1);
        };
        (FLAG_CF | FLAG_ZF; FLAG_PF | FLAG_SF | FLAG_OF)
    ];

    // cmovbe rdx, rcx
    ax_test![cmovbe_rdx_rcx_zf_zf; 0x48, 0xf, 0x46, 0xd1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x19315);
            write_reg_value!(q; a; RCX; 0x1);
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];
}
