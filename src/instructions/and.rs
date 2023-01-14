use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::And;

use super::axecutor::Axecutor;
use super::errors::AxError;

use super::macros::calculate_rm_imm;
use crate::instructions::flags::*;
use crate::instructions::macros::calculate_r_rm;
use crate::instructions::macros::calculate_rm_r;
use crate::instructions::macros::fatal_error;

impl Axecutor {
    pub fn mnemonic_and(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), And);

        match i.code() {
            And_rm8_r8 => self.instr_and_rm8_r8(i),
            And_rm16_r16 => self.instr_and_rm16_r16(i),
            And_rm32_r32 => self.instr_and_rm32_r32(i),
            And_rm64_r64 => self.instr_and_rm64_r64(i),
            And_r8_rm8 => self.instr_and_r8_rm8(i),
            And_r16_rm16 => self.instr_and_r16_rm16(i),
            And_r32_rm32 => self.instr_and_r32_rm32(i),
            And_r64_rm64 => self.instr_and_r64_rm64(i),
            And_AL_imm8 => self.instr_and_al_imm8(i),
            And_AX_imm16 => self.instr_and_ax_imm16(i),
            And_EAX_imm32 => self.instr_and_eax_imm32(i),
            And_RAX_imm32 => self.instr_and_rax_imm32(i),
            And_rm8_imm8 => self.instr_and_rm8_imm8(i),
            And_rm16_imm16 => self.instr_and_rm16_imm16(i),
            And_rm32_imm32 => self.instr_and_rm32_imm32(i),
            And_rm64_imm32 => self.instr_and_rm64_imm32(i),
            And_rm8_imm8_82 => self.instr_and_rm8_imm8_82(i),
            And_rm16_imm8 => self.instr_and_rm16_imm8(i),
            And_rm32_imm8 => self.instr_and_rm32_imm8(i),
            And_rm64_imm8 => self.instr_and_rm64_imm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic And", i.code()),
        }
    }

    /// AND r/m8, r8
    ///
    /// 20 /r
    fn instr_and_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_rm8_r8);

        calculate_rm_r![u8f; self; i; |s: u8, d: u8| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r/m16, r16
    ///
    /// o16 21 /r
    fn instr_and_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_rm16_r16);

        calculate_rm_r![u16f; self; i; |s: u16, d: u16| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r/m32, r32
    ///
    /// o32 21 /r
    fn instr_and_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_rm32_r32);

        calculate_rm_r![u32f; self; i; |s: u32, d: u32| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r/m64, r64
    ///
    /// o64 21 /r
    fn instr_and_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_rm64_r64);

        calculate_rm_r![u64f; self; i; |s: u64, d: u64| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r8, r/m8
    ///
    /// 22 /r
    fn instr_and_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_r8_rm8);

        calculate_r_rm![u8f; self; i; |s: u8, d: u8| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r16, r/m16
    ///
    /// o16 23 /r
    fn instr_and_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_r16_rm16);

        calculate_r_rm![u16f; self; i; |s: u16, d: u16| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r32, r/m32
    ///
    /// o32 23 /r
    fn instr_and_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_r32_rm32);

        calculate_r_rm![u32f; self; i; |s: u32, d: u32| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r64, r/m64
    ///
    /// o64 23 /r
    fn instr_and_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_r64_rm64);

        calculate_r_rm![u64f; self; i; |s: u64, d: u64| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND AL, imm8
    ///
    /// 24 ib
    fn instr_and_al_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_AL_imm8);

        self.instr_and_rm8_imm8(i)
    }

    /// AND AX, imm16
    ///
    /// o16 25 iw
    fn instr_and_ax_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_AX_imm16);

        self.instr_and_rm16_imm16(i)
    }

    /// AND EAX, imm32
    ///
    /// o32 25 id
    fn instr_and_eax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_EAX_imm32);

        self.instr_and_rm32_imm32(i)
    }

    /// AND RAX, imm32
    ///
    /// o64 25 id
    fn instr_and_rax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_RAX_imm32);

        self.instr_and_rm64_imm32(i)
    }

    /// AND r/m8, imm8
    ///
    /// 80 /4 ib
    fn instr_and_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u8f; self; i; |s: u8, d: u8| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r/m16, imm16
    ///
    /// o16 81 /4 iw
    fn instr_and_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u16f; self; i; |s: u16, d: u16| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r/m32, imm32
    ///
    /// o32 81 /4 id
    fn instr_and_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u32f; self; i; |s: u32, d: u32| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r/m64, imm32
    ///
    /// o64 81 /4 id
    fn instr_and_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u64f; self; i; |s: u64, d: u64| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r/m8, imm8
    ///
    /// 82 /4 ib
    fn instr_and_rm8_imm8_82(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_rm8_imm8_82);

        self.instr_and_rm8_imm8(i)
    }

    /// AND r/m16, imm8
    ///
    /// o16 83 /4 ib
    fn instr_and_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_rm16_imm8);

        calculate_rm_imm![u16f; self; i; |s: u16, d: u16| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r/m32, imm8
    ///
    /// o32 83 /4 ib
    fn instr_and_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_rm32_imm8);

        calculate_rm_imm![u32f; self; i; |s: u32, d: u32| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// AND r/m64, imm8
    ///
    /// o64 83 /4 ib
    fn instr_and_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), And_rm64_imm8);

        calculate_rm_imm![u64f; self; i; |s: u64, d: u64| {
            (s & d, 0)
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::instructions::tests::{
        assert_mem_value, assert_reg_value, ax_test, write_reg_value,
    };
    use iced_x86::Register::*;

    // and al, bl
    ax_test![and_al_bl; 0x20, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and al, bl
    ax_test![and_al_bl_pf; 0x20, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
            write_reg_value!(b; a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_reg_value!(b; a; BL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and al, bl
    ax_test![and_al_bl_pf_sf; 0x20, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // and al, bl
    ax_test![and_al_bl_pf_zf; 0x20, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and al, bl
    ax_test![and_al_bl_sf; 0x20, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // and ax, bx
    ax_test![and_ax_bx; 0x66, 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(w; a; BX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
            assert_reg_value!(w; a; BX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, bx
    ax_test![and_ax_bx_pf; 0x66, 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xf);
            write_reg_value!(w; a; BX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf);
            assert_reg_value!(w; a; BX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, bx
    ax_test![and_ax_bx_pf_sf; 0x66, 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // and ax, bx
    ax_test![and_ax_bx_pf_zf; 0x66, 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; BX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and eax, ebx
    ax_test![and_eax_ebx; 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(d; a; EBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_reg_value!(d; a; EBX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and eax, ebx
    ax_test![and_eax_ebx_pf; 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xf);
            write_reg_value!(d; a; EBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf);
            assert_reg_value!(d; a; EBX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and eax, ebx
    ax_test![and_eax_ebx_pf_sf; 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // and eax, ebx
    ax_test![and_eax_ebx_pf_zf; 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and rax, rbx
    ax_test![and_rax_rbx; 0x48, 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_reg_value!(q; a; RBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
            assert_reg_value!(q; a; RBX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, rbx
    ax_test![and_rax_rbx_pf; 0x48, 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xf);
            write_reg_value!(q; a; RBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xf);
            assert_reg_value!(q; a; RBX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, rbx
    ax_test![and_rax_rbx_pf_sf; 0x48, 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // and rax, rbx
    ax_test![and_rax_rbx_pf_zf; 0x48, 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and rax, rbx
    ax_test![and_rax_rbx_sf; 0x48, 0x21, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xe37ccc3fabadfdcdu64);
            write_reg_value!(q; a; RBX; 0x91c3b8a3b1058f67u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x81408823a1058d45u64);
            assert_reg_value!(q; a; RBX; 0x91c3b8a3b1058f67u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // and al, byte ptr [rbx]
    ax_test![and_al_byte_ptr_rbx; 0x22, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and al, byte ptr [rbx]
    ax_test![and_al_byte_ptr_rbx_pf; 0x22, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_mem_value!(b; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and al, byte ptr [rbx]
    ax_test![and_al_byte_ptr_rbx_pf_sf; 0x22, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // and al, byte ptr [rbx]
    ax_test![and_al_byte_ptr_rbx_pf_zf; 0x22, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and al, byte ptr [rbx]
    ax_test![and_al_byte_ptr_rbx_sf; 0x22, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // and ax, word ptr [rbx]
    ax_test![and_ax_word_ptr_rbx; 0x66, 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, word ptr [rbx]
    ax_test![and_ax_word_ptr_rbx_pf; 0x66, 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xf);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf);
            assert_mem_value!(w; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, word ptr [rbx]
    ax_test![and_ax_word_ptr_rbx_pf_sf; 0x66, 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // and ax, word ptr [rbx]
    ax_test![and_ax_word_ptr_rbx_pf_zf; 0x66, 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and eax, dword ptr [rbx]
    ax_test![and_eax_dword_ptr_rbx; 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and eax, dword ptr [rbx]
    ax_test![and_eax_dword_ptr_rbx_pf; 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xf);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf);
            assert_mem_value!(d; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and eax, dword ptr [rbx]
    ax_test![and_eax_dword_ptr_rbx_pf_sf; 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // and eax, dword ptr [rbx]
    ax_test![and_eax_dword_ptr_rbx_pf_zf; 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and rax, qword ptr [rbx]
    ax_test![and_rax_qword_ptr_rbx; 0x48, 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, qword ptr [rbx]
    ax_test![and_rax_qword_ptr_rbx_pf; 0x48, 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xf);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xf);
            assert_mem_value!(q; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, qword ptr [rbx]
    ax_test![and_rax_qword_ptr_rbx_pf_zf; 0x48, 0x23, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and al, 0xf
    ax_test![and_al_0xf; 0x24, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and al, 0xf
    ax_test![and_al_0xf_pf; 0x24, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and al, 0xf
    ax_test![and_al_0xf_pf_zf; 0x24, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and ax, 0xf
    ax_test![and_ax_0xf; 0x66, 0x83, 0xe0, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, 0xf
    ax_test![and_ax_0xf_pf; 0x66, 0x83, 0xe0, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, 0xf
    ax_test![and_ax_0xf_pf_zf; 0x66, 0x83, 0xe0, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and eax, 0xf
    ax_test![and_eax_0xf; 0x83, 0xe0, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and eax, 0xf
    ax_test![and_eax_0xf_pf; 0x83, 0xe0, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and eax, 0xf
    ax_test![and_eax_0xf_pf_zf; 0x83, 0xe0, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0xf
    ax_test![and_rax_0xf; 0x48, 0x83, 0xe0, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0xf
    ax_test![and_rax_0xf_pf; 0x48, 0x83, 0xe0, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0xf
    ax_test![and_rax_0xf_pf_zf; 0x48, 0x83, 0xe0, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0xffffffffffff0000
    ax_test![and_rax_0xffffffffffff0000_pf; 0x48, 0x25, 0x0, 0x0, 0xff, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x10000);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0xffffffffffff0000
    ax_test![and_rax_0xffffffffffff0000_pf_sf; 0x48, 0x25, 0x0, 0x0, 0xff, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000050u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // and rax, 0xffffffffffff0000
    ax_test![and_rax_0xffffffffffff0000_pf_zf; 0x48, 0x25, 0x0, 0x0, 0xff, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and al, 0xd
    ax_test![and_al_0xd; 0x24, 0xd;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and al, 0xd
    ax_test![and_al_0xd_pf; 0x24, 0xd;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x5);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and al, 0xd
    ax_test![and_al_0xd_pf_zf; 0x24, 0xd;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and dword ptr [rax], 0xd
    ax_test![and_dword_ptr_rax_0xd; 0x83, 0x20, 0xd;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and dword ptr [rax], 0xd
    ax_test![and_dword_ptr_rax_0xd_pf; 0x83, 0x20, 0xd;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x5);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and dword ptr [rax], 0xd
    ax_test![and_dword_ptr_rax_0xd_pf_zf; 0x83, 0x20, 0xd;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and qword ptr [rax], 0xe380d
    ax_test![and_qword_ptr_rax_0xe380d; 0x48, 0x81, 0x20, 0xd, 0x38, 0xe, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and qword ptr [rax], 0xe380d
    ax_test![and_qword_ptr_rax_0xe380d_pf; 0x48, 0x81, 0x20, 0xd, 0x38, 0xe, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x5);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and qword ptr [rax], 0xe380d
    ax_test![and_qword_ptr_rax_0xe380d_pf_zf; 0x48, 0x81, 0x20, 0xd, 0x38, 0xe, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0x3
    ax_test![and_rax_0x3; 0x48, 0x83, 0xe0, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0x3
    ax_test![and_rax_0x3_pf; 0x48, 0x83, 0xe0, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0x3
    ax_test![and_rax_0x3_pf_zf; 0x48, 0x83, 0xe0, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and rbx, 0x27
    ax_test![and_rbx_0x27; 0x48, 0x83, 0xe3, 0x27;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rbx, 0x27
    ax_test![and_rbx_0x27_pf; 0x48, 0x83, 0xe3, 0x27;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x21);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rbx, 0x27
    ax_test![and_rbx_0x27_pf_zf; 0x48, 0x83, 0xe3, 0x27;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and ax, 0x3
    ax_test![and_ax_0x3; 0x66, 0x83, 0xe0, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, 0x3
    ax_test![and_ax_0x3_pf; 0x66, 0x83, 0xe0, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, 0x3
    ax_test![and_ax_0x3_pf_zf; 0x66, 0x83, 0xe0, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and ax, 0x7cd1
    ax_test![and_ax_0x7cd1; 0x66, 0x25, 0xd1, 0x7c;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, 0x7cd1
    ax_test![and_ax_0x7cd1_pf; 0x66, 0x25, 0xd1, 0x7c;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x11);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and ax, 0x7cd1
    ax_test![and_ax_0x7cd1_pf_zf; 0x66, 0x25, 0xd1, 0x7c;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and eax, 0x4d2
    ax_test![and_eax_0x4d2; 0x25, 0xd2, 0x4, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and eax, 0x4d2
    ax_test![and_eax_0x4d2_pf; 0x25, 0xd2, 0x4, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x12);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and eax, 0x4d2
    ax_test![and_eax_0x4d2_pf_zf; 0x25, 0xd2, 0x4, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0x499602d2
    ax_test![and_rax_0x499602d2; 0x48, 0x25, 0xd2, 0x2, 0x96, 0x49;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0x499602d2
    ax_test![and_rax_0x499602d2_pf; 0x48, 0x25, 0xd2, 0x2, 0x96, 0x49;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x12);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and rax, 0x499602d2
    ax_test![and_rax_0x499602d2_pf_zf; 0x48, 0x25, 0xd2, 0x2, 0x96, 0x49;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and byte ptr [rax], 0x7b
    ax_test![and_byte_ptr_rax_0x7b; 0x80, 0x20, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and byte ptr [rax], 0x7b
    ax_test![and_byte_ptr_rax_0x7b_pf; 0x80, 0x20, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and byte ptr [rax], 0x7b
    ax_test![and_byte_ptr_rax_0x7b_pf_zf; 0x80, 0x20, 0x7b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and word ptr [rax], 0x302b
    ax_test![and_word_ptr_rax_0x302b; 0x66, 0x81, 0x20, 0x2b, 0x30;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and word ptr [rax], 0x302b
    ax_test![and_word_ptr_rax_0x302b_pf; 0x66, 0x81, 0x20, 0x2b, 0x30;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and word ptr [rax], 0x302b
    ax_test![and_word_ptr_rax_0x302b_pf_zf; 0x66, 0x81, 0x20, 0x2b, 0x30;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // and dword ptr [rax], 0x7599daf
    ax_test![and_dword_ptr_rax_0x7599daf; 0x81, 0x20, 0xaf, 0x9d, 0x59, 0x7;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and dword ptr [rax], 0x7599daf
    ax_test![and_dword_ptr_rax_0x7599daf_pf; 0x81, 0x20, 0xaf, 0x9d, 0x59, 0x7;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // and dword ptr [rax], 0x7599daf
    ax_test![and_dword_ptr_rax_0x7599daf_pf_zf; 0x81, 0x20, 0xaf, 0x9d, 0x59, 0x7;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
