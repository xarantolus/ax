use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Cmovne;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::flags::*;

use crate::instructions::macros::calculate_r_rm;

use crate::instructions::macros::fatal_error;

impl Axecutor {
    pub fn mnemonic_cmovne(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cmovne);

        match i.code() {
            Cmovne_r16_rm16 => self.instr_cmovne_r16_rm16(i),
            Cmovne_r32_rm32 => self.instr_cmovne_r32_rm32(i),
            Cmovne_r64_rm64 => self.instr_cmovne_r64_rm64(i),
            _ => fatal_error!(
                "Invalid instruction code {:?} for mnemonic Cmovne",
                i.code()
            ),
        }
    }

    /// CMOVNE r16, r/m16
    ///
    /// o16 0F 45 /r
    fn instr_cmovne_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovne_r16_rm16);

        if self.state.rflags & FLAG_ZF == 0 {
            calculate_r_rm![u16; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVNE r32, r/m32
    ///
    /// o32 0F 45 /r
    fn instr_cmovne_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovne_r32_rm32);

        if self.state.rflags & FLAG_ZF == 0 {
            calculate_r_rm![u32; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVNE r64, r/m64
    ///
    /// o64 0F 45 /r
    fn instr_cmovne_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovne_r64_rm64);

        if self.state.rflags & FLAG_ZF == 0 {
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
    use crate::instructions::axecutor::Axecutor;
    use crate::instructions::tests::{
        assert_mem_value, assert_reg_value, ax_test, write_flags, write_reg_value,
    };
    use iced_x86::Register::*;
    // cmovne dx, word ptr [rcx]
    ax_test![cmovne_dx_word_ptr_rcx; 0x66, 0xf, 0x45, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; DX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; DX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovne dx, word ptr [rcx]
    ax_test![cmovne_dx_word_ptr_rcx_zf_zf; 0x66, 0xf, 0x45, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; DX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; DX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];

    // cmovne edx, dword ptr [rcx]
    ax_test![cmovne_edx_dword_ptr_rcx; 0xf, 0x45, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovne edx, dword ptr [rcx]
    ax_test![cmovne_edx_dword_ptr_rcx_zf_zf; 0xf, 0x45, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];

    // cmovne rdx, qword ptr [rcx]
    ax_test![cmovne_rdx_qword_ptr_rcx; 0x48, 0xf, 0x45, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovne rdx, qword ptr [rcx]
    ax_test![cmovne_rdx_qword_ptr_rcx_zf_zf; 0x48, 0xf, 0x45, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];
}
