use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Cmovae;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;
use crate::state::flags::*;

use crate::helpers::macros::calculate_r_rm;

use crate::helpers::macros::fatal_error;

impl Axecutor {
    pub(crate) fn mnemonic_cmovae(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cmovae);

        match i.code() {
            Cmovae_r16_rm16 => self.instr_cmovae_r16_rm16(i),
            Cmovae_r32_rm32 => self.instr_cmovae_r32_rm32(i),
            Cmovae_r64_rm64 => self.instr_cmovae_r64_rm64(i),
            _ => fatal_error!(
                "Invalid instruction code {:?} for mnemonic Cmovae",
                i.code()
            ),
        }
    }

    /// CMOVAE r16, r/m16
    ///
    /// o16 0F 43 /r
    fn instr_cmovae_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovae_r16_rm16);

        if self.state.rflags & FLAG_CF != 0 {
            calculate_r_rm![u16; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVAE r32, r/m32
    ///
    /// o32 0F 43 /r
    fn instr_cmovae_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovae_r32_rm32);

        if self.state.rflags & FLAG_CF != 0 {
            calculate_r_rm![u32; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVAE r64, r/m64
    ///
    /// o64 0F 43 /r
    fn instr_cmovae_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovae_r64_rm64);

        if self.state.rflags & FLAG_CF != 0 {
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
    // cmovae bx, word ptr [rcx]
    ax_test![cmovae_bx_word_ptr_rcx; 0x66, 0xf, 0x43, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovae bx, word ptr [rcx]
    ax_test![cmovae_bx_word_ptr_rcx_cf_cf; 0x66, 0xf, 0x43, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovae edx, dword ptr [rcx]
    ax_test![cmovae_edx_dword_ptr_rcx; 0xf, 0x43, 0x11;
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

    // cmovae edx, dword ptr [rcx]
    ax_test![cmovae_edx_dword_ptr_rcx_cf_cf; 0xf, 0x43, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmovae rdx, qword ptr [rcx]
    ax_test![cmovae_rdx_qword_ptr_rcx; 0x48, 0xf, 0x43, 0x11;
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

    // cmovae rdx, qword ptr [rcx]
    ax_test![cmovae_rdx_qword_ptr_rcx_cf_cf; 0x48, 0xf, 0x43, 0x11;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            init_mem_value!(q; a; 0x1000; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
