use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Or;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;
use crate::helpers::macros::calculate_r_rm;
use crate::helpers::macros::calculate_rm_imm;
use crate::helpers::macros::calculate_rm_r;
use crate::helpers::macros::fatal_error;

use crate::state::flags::*;

impl Axecutor {
    pub(crate) fn mnemonic_or(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Or);

        match i.code() {
            Or_rm8_r8 => self.instr_or_rm8_r8(i),
            Or_rm16_r16 => self.instr_or_rm16_r16(i),
            Or_rm32_r32 => self.instr_or_rm32_r32(i),
            Or_rm64_r64 => self.instr_or_rm64_r64(i),
            Or_r8_rm8 => self.instr_or_r8_rm8(i),
            Or_r16_rm16 => self.instr_or_r16_rm16(i),
            Or_r32_rm32 => self.instr_or_r32_rm32(i),
            Or_r64_rm64 => self.instr_or_r64_rm64(i),
            Or_AL_imm8 => self.instr_or_al_imm8(i),
            Or_AX_imm16 => self.instr_or_ax_imm16(i),
            Or_EAX_imm32 => self.instr_or_eax_imm32(i),
            Or_RAX_imm32 => self.instr_or_rax_imm32(i),
            Or_rm8_imm8 => self.instr_or_rm8_imm8(i),
            Or_rm16_imm16 => self.instr_or_rm16_imm16(i),
            Or_rm32_imm32 => self.instr_or_rm32_imm32(i),
            Or_rm64_imm32 => self.instr_or_rm64_imm32(i),
            Or_rm8_imm8_82 => self.instr_or_rm8_imm8_82(i),
            Or_rm16_imm8 => self.instr_or_rm16_imm8(i),
            Or_rm32_imm8 => self.instr_or_rm32_imm8(i),
            Or_rm64_imm8 => self.instr_or_rm64_imm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Or", i.code()),
        }
    }

    /// OR r/m8, r8
    ///
    /// 08 /r
    fn instr_or_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_rm8_r8);

        calculate_rm_r![u8; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r/m16, r16
    ///
    /// o16 09 /r
    fn instr_or_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_rm16_r16);

        calculate_rm_r![u16; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r/m32, r32
    ///
    /// o32 09 /r
    fn instr_or_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_rm32_r32);

        calculate_rm_r![u32; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r/m64, r64
    ///
    /// o64 09 /r
    fn instr_or_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_rm64_r64);

        calculate_rm_r![u64; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r8, r/m8
    ///
    /// 0A /r
    fn instr_or_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_r8_rm8);

        calculate_r_rm![u8; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r16, r/m16
    ///
    /// o16 0B /r
    fn instr_or_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_r16_rm16);

        calculate_r_rm![u16; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r32, r/m32
    ///
    /// o32 0B /r
    fn instr_or_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_r32_rm32);

        calculate_r_rm![u32; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r64, r/m64
    ///
    /// o64 0B /r
    fn instr_or_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_r64_rm64);

        calculate_r_rm![u64; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR AL, imm8
    ///
    /// 0C ib
    fn instr_or_al_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_AL_imm8);

        self.instr_or_rm8_imm8(i)
    }

    /// OR AX, imm16
    ///
    /// o16 0D iw
    fn instr_or_ax_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_AX_imm16);

        self.instr_or_rm16_imm16(i)
    }

    /// OR EAX, imm32
    ///
    /// o32 0D id
    fn instr_or_eax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_EAX_imm32);

        self.instr_or_rm32_imm32(i)
    }

    /// OR RAX, imm32
    ///
    /// o64 0D id
    fn instr_or_rax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_RAX_imm32);

        self.instr_or_rm64_imm32(i)
    }

    /// OR r/m8, imm8
    ///
    /// 80 /1 ib
    fn instr_or_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u8; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r/m16, imm16
    ///
    /// o16 81 /1 iw
    fn instr_or_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u16; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r/m32, imm32
    ///
    /// o32 81 /1 id
    fn instr_or_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u32; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r/m64, imm32
    ///
    /// o64 81 /1 id
    fn instr_or_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u64; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r/m8, imm8
    ///
    /// 82 /1 ib
    fn instr_or_rm8_imm8_82(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_rm8_imm8_82);

        self.instr_or_rm8_imm8(i)
    }

    /// OR r/m16, imm8
    ///
    /// o16 83 /1 ib
    fn instr_or_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_rm16_imm8);

        calculate_rm_imm![u16; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r/m32, imm8
    ///
    /// o32 83 /1 ib
    fn instr_or_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_rm32_imm8);

        calculate_rm_imm![u32; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// OR r/m64, imm8
    ///
    /// o64 83 /1 ib
    fn instr_or_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Or_rm64_imm8);

        calculate_rm_imm![u64; self; i; |d,s| {
            d|s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{
        assert_mem_value, assert_reg_value, ax_test, init_mem_value, write_reg_value,
    };
    use iced_x86::Register::*;

    // or al, bl
    ax_test![or_al_bl; 0x8, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or al, bl
    ax_test![or_al_bl_pf; 0x8, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_reg_value!(b; a; BL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or al, bl
    ax_test![or_al_bl_pf_sf; 0x8, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or al, bl
    ax_test![or_al_bl_pf_zf; 0x8, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or al, bl
    ax_test![or_al_bl_sf; 0x8, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or ax, bx
    ax_test![or_ax_bx; 0x66, 0x9, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; BX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
            assert_reg_value!(w; a; BX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or ax, bx
    ax_test![or_ax_bx_pf; 0x66, 0x9, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; BX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf);
            assert_reg_value!(w; a; BX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or ax, bx
    ax_test![or_ax_bx_pf_sf; 0x66, 0x9, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or ax, bx
    ax_test![or_ax_bx_pf_zf; 0x66, 0x9, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; BX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or ax, bx
    ax_test![or_ax_bx_sf; 0x66, 0x9, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8001);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or eax, ecx
    ax_test![or_eax_ecx; 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; ECX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_reg_value!(d; a; ECX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or eax, ecx
    ax_test![or_eax_ecx_pf; 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; ECX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf);
            assert_reg_value!(d; a; ECX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or eax, ecx
    ax_test![or_eax_ecx_pf_sf; 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; ECX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_reg_value!(d; a; ECX; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or eax, ecx
    ax_test![or_eax_ecx_pf_zf; 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; ECX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; ECX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or eax, ecx
    ax_test![or_eax_ecx_sf; 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(d; a; ECX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000001u32);
            assert_reg_value!(d; a; ECX; 0x80000000u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or rax, rcx
    ax_test![or_rax_rcx; 0x48, 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or rax, rcx
    ax_test![or_rax_rcx_pf; 0x48, 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RCX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xf);
            assert_reg_value!(q; a; RCX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or rax, rcx
    ax_test![or_rax_rcx_pf_sf; 0x48, 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RCX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_reg_value!(q; a; RCX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or rax, rcx
    ax_test![or_rax_rcx_pf_zf; 0x48, 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RCX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_reg_value!(q; a; RCX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or rax, rcx
    ax_test![or_rax_rcx_sf; 0x48, 0x9, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_reg_value!(q; a; RCX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000001u64);
            assert_reg_value!(q; a; RCX; 0x8000000000000000u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or al, byte ptr [rbx]
    ax_test![or_al_byte_ptr_rbx; 0xa, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or al, byte ptr [rbx]
    ax_test![or_al_byte_ptr_rbx_pf; 0xa, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or al, byte ptr [rbx]
    ax_test![or_al_byte_ptr_rbx_pf_sf; 0xa, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or al, byte ptr [rbx]
    ax_test![or_al_byte_ptr_rbx_pf_zf; 0xa, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or al, byte ptr [rbx]
    ax_test![or_al_byte_ptr_rbx_sf; 0xa, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or cx, word ptr [rbx]
    ax_test![or_cx_word_ptr_rbx; 0x66, 0xb, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x1);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or cx, word ptr [rbx]
    ax_test![or_cx_word_ptr_rbx_pf; 0x66, 0xb, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0xf);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or cx, word ptr [rbx]
    ax_test![or_cx_word_ptr_rbx_pf_sf; 0x66, 0xb, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8000);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or cx, word ptr [rbx]
    ax_test![or_cx_word_ptr_rbx_pf_zf; 0x66, 0xb, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or cx, word ptr [rbx]
    ax_test![or_cx_word_ptr_rbx_sf; 0x66, 0xb, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8001);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or edx, dword ptr [rbx]
    ax_test![or_edx_dword_ptr_rbx; 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x1);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or edx, dword ptr [rbx]
    ax_test![or_edx_dword_ptr_rbx_pf; 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0xf);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or edx, dword ptr [rbx]
    ax_test![or_edx_dword_ptr_rbx_pf_sf; 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x80000000u32);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or edx, dword ptr [rbx]
    ax_test![or_edx_dword_ptr_rbx_pf_zf; 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x0);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or edx, dword ptr [rbx]
    ax_test![or_edx_dword_ptr_rbx_sf; 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x80000001u32);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or rdx, qword ptr [rbx]
    ax_test![or_rdx_qword_ptr_rbx; 0x48, 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(q; a; 0x1000; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x1);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or rdx, qword ptr [rbx]
    ax_test![or_rdx_qword_ptr_rbx_pf; 0x48, 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(q; a; 0x1000; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xf);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or rdx, qword ptr [rbx]
    ax_test![or_rdx_qword_ptr_rbx_pf_sf; 0x48, 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(q; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x8000000000000000u64);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or rdx, qword ptr [rbx]
    ax_test![or_rdx_qword_ptr_rbx_pf_zf; 0x48, 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(q; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or rdx, qword ptr [rbx]
    ax_test![or_rdx_qword_ptr_rbx_sf; 0x48, 0xb, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(q; a; 0x1000; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x8000000000000001u64);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or byte ptr [rbx], cl
    ax_test![or_byte_ptr_rbx_cl; 0x8, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or byte ptr [rbx], cl
    ax_test![or_byte_ptr_rbx_cl_pf; 0x8, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0x0);
            write_reg_value!(b; a; CL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xf);
            assert_reg_value!(b; a; CL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or byte ptr [rbx], cl
    ax_test![or_byte_ptr_rbx_cl_pf_sf; 0x8, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0x0);
            write_reg_value!(b; a; CL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
            assert_reg_value!(b; a; CL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or byte ptr [rbx], cl
    ax_test![or_byte_ptr_rbx_cl_pf_zf; 0x8, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or byte ptr [rbx], cl
    ax_test![or_byte_ptr_rbx_cl_sf; 0x8, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(b; a; 0x1000; 0x0);
            write_reg_value!(b; a; CL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
            assert_reg_value!(b; a; CL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or word ptr [rbx], cx
    ax_test![or_word_ptr_rbx_cx; 0x66, 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x0);
            write_reg_value!(w; a; CX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
            assert_reg_value!(w; a; CX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or word ptr [rbx], cx
    ax_test![or_word_ptr_rbx_cx_pf; 0x66, 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x0);
            write_reg_value!(w; a; CX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0xf);
            assert_reg_value!(w; a; CX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or word ptr [rbx], cx
    ax_test![or_word_ptr_rbx_cx_pf_sf; 0x66, 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x0);
            write_reg_value!(w; a; CX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
            assert_reg_value!(w; a; CX; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or word ptr [rbx], cx
    ax_test![or_word_ptr_rbx_cx_pf_zf; 0x66, 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x0);
            write_reg_value!(w; a; CX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
            assert_reg_value!(w; a; CX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or word ptr [rbx], cx
    ax_test![or_word_ptr_rbx_cx_sf; 0x66, 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(w; a; 0x1000; 0x1);
            write_reg_value!(w; a; CX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8001);
            assert_reg_value!(w; a; CX; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or dword ptr [rbx], ecx
    ax_test![or_dword_ptr_rbx_ecx; 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x0);
            write_reg_value!(d; a; ECX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
            assert_reg_value!(d; a; ECX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or dword ptr [rbx], ecx
    ax_test![or_dword_ptr_rbx_ecx_pf; 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x0);
            write_reg_value!(d; a; ECX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0xf);
            assert_reg_value!(d; a; ECX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or dword ptr [rbx], ecx
    ax_test![or_dword_ptr_rbx_ecx_pf_sf; 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x0);
            write_reg_value!(d; a; ECX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
            assert_reg_value!(d; a; ECX; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or dword ptr [rbx], ecx
    ax_test![or_dword_ptr_rbx_ecx_pf_zf; 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x0);
            write_reg_value!(d; a; ECX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
            assert_reg_value!(d; a; ECX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // or dword ptr [rbx], ecx
    ax_test![or_dword_ptr_rbx_ecx_sf; 0x9, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x1);
            write_reg_value!(d; a; ECX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000001u32);
            assert_reg_value!(d; a; ECX; 0x80000000u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or dword ptr [rbx], 0x7c77c41
    ax_test![or_dword_ptr_rbx_0x7c77c41; 0x81, 0xb, 0x41, 0x7c, 0xc7, 0x7;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x7c77c49);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or dword ptr [rbx], 0x7c77c41
    ax_test![or_dword_ptr_rbx_0x7c77c41_pf; 0x81, 0xb, 0x41, 0x7c, 0xc7, 0x7;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x7c77c41);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or dword ptr [rbx], 0x7c77c41
    ax_test![or_dword_ptr_rbx_0x7c77c41_pf_sf; 0x81, 0xb, 0x41, 0x7c, 0xc7, 0x7;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x87c77c41u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or al, 0xf
    ax_test![or_al_0xf; 0xc, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or al, 0xf
    ax_test![or_al_0xf_pf; 0xc, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or al, 0xf
    ax_test![or_al_0xf_pf_sf; 0xc, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or al, 0xf
    ax_test![or_al_0xf_sf; 0xc, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8f);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or ax, 0xff38
    ax_test![or_ax_0xff38_pf_sf; 0x66, 0xd, 0x38, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xff39);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or ax, 0xff38
    ax_test![or_ax_0xff38_sf; 0x66, 0xd, 0x38, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xff38);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or eax, 0x154c65
    ax_test![or_eax_0x154c65; 0xd, 0x65, 0x4c, 0x15, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x154c67);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or eax, 0x154c65
    ax_test![or_eax_0x154c65_pf; 0xd, 0x65, 0x4c, 0x15, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x154c65);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or eax, 0x154c65
    ax_test![or_eax_0x154c65_pf_sf; 0xd, 0x65, 0x4c, 0x15, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80154c65u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or rax, 0x154c65
    ax_test![or_rax_0x154c65; 0x48, 0xd, 0x65, 0x4c, 0x15, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x154c67);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or rax, 0x154c65
    ax_test![or_rax_0x154c65_pf; 0x48, 0xd, 0x65, 0x4c, 0x15, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x154c65);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or rax, 0x154c65
    ax_test![or_rax_0x154c65_pf_sf; 0x48, 0xd, 0x65, 0x4c, 0x15, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000154c65u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or rax, 0x154c65
    ax_test![or_rax_0x154c65_sf; 0x48, 0xd, 0x65, 0x4c, 0x15, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8b2481a59a2f9cc4u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8b2481a59a3fdce5u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or bl, 0x7b
    ax_test![or_bl_0x7b; 0x80, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or bl, 0x7b
    ax_test![or_bl_0x7b_pf; 0x80, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7b);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or bl, 0x7b
    ax_test![or_bl_0x7b_pf_sf; 0x80, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or bl, 0x7b
    ax_test![or_bl_0x7b_sf; 0x80, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xfb);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // or bx, 0x7b
    ax_test![or_bx_0x7b; 0x66, 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or bx, 0x7b
    ax_test![or_bx_0x7b_pf; 0x66, 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7b);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or bx, 0x7b
    ax_test![or_bx_0x7b_pf_sf; 0x66, 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x807b);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or ebx, 0x7b
    ax_test![or_ebx_0x7b; 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or ebx, 0x7b
    ax_test![or_ebx_0x7b_pf; 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x7b);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or ebx, 0x7b
    ax_test![or_ebx_0x7b_pf_sf; 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x8000007bu32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or rbx, 0x7b
    ax_test![or_rbx_0x7b; 0x48, 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or rbx, 0x7b
    ax_test![or_rbx_0x7b_pf; 0x48, 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x7b);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // or rbx, 0x7b
    ax_test![or_rbx_0x7b_pf_sf; 0x48, 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x800000000000007bu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // or rbx, 0x7b
    ax_test![or_rbx_0x7b_sf; 0x48, 0x83, 0xcb, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x89d1bb3c5f343245u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x89d1bb3c5f34327fu64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];
}
