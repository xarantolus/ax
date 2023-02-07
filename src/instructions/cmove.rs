use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Cmove;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::state::flags::*;

use crate::helpers::macros::calculate_r_rm;

use crate::helpers::macros::fatal_error;

impl Axecutor {
    pub(crate) fn mnemonic_cmove(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cmove);

        match i.code() {
            Cmove_r16_rm16 => self.instr_cmove_r16_rm16(i),
            Cmove_r32_rm32 => self.instr_cmove_r32_rm32(i),
            Cmove_r64_rm64 => self.instr_cmove_r64_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Cmove", i.code()),
        }
    }

    /// CMOVE r16, r/m16
    ///
    /// o16 0F 44 /r
    fn instr_cmove_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmove_r16_rm16);

        if self.state.rflags & FLAG_ZF != 0 {
            calculate_r_rm![u16; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVE r32, r/m32
    ///
    /// o32 0F 44 /r
    fn instr_cmove_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmove_r32_rm32);

        if self.state.rflags & FLAG_ZF != 0 {
            calculate_r_rm![u32; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVE r64, r/m64
    ///
    /// o64 0F 44 /r
    fn instr_cmove_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmove_r64_rm64);

        if self.state.rflags & FLAG_ZF != 0 {
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
    // cmove dx, word ptr [rcx]
    ax_test![cmove_dx_word_ptr_rcx; 0x66, 0xf, 0x44, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; DX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; DX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmove dx, word ptr [rcx]
    ax_test![cmove_dx_word_ptr_rcx_zf_zf; 0x66, 0xf, 0x44, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; DX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x0);
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; DX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];

    // cmove edx, dword ptr [rcx]
    ax_test![cmove_edx_dword_ptr_rcx; 0xf, 0x44, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmove edx, dword ptr [rcx]
    ax_test![cmove_edx_dword_ptr_rcx_zf_zf; 0xf, 0x44, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x0);
            write_flags!(a; FLAG_ZF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_ZF; FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF)
    ];

    // cmove rdx, qword ptr [rcx]
    ax_test![cmove_rdx_qword_ptr_rcx; 0x48, 0xf, 0x44, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(q; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmove rdx, qword ptr [rcx]
    ax_test![cmove_rdx_qword_ptr_rcx_zf_zf; 0x48, 0xf, 0x44, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(q; a; 0x1000; 0x0);
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
