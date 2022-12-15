use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Xor;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::flags::*;
use crate::{calculate_r_rm, calculate_rm_imm, calculate_rm_r, fatal_error};

impl Axecutor {
    pub fn mnemonic_xor(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Xor);

        match i.code() {
            Xor_rm8_r8 => self.instr_xor_rm8_r8(i),
            Xor_rm16_r16 => self.instr_xor_rm16_r16(i),
            Xor_rm32_r32 => self.instr_xor_rm32_r32(i),
            Xor_rm64_r64 => self.instr_xor_rm64_r64(i),
            Xor_r8_rm8 => self.instr_xor_r8_rm8(i),
            Xor_r16_rm16 => self.instr_xor_r16_rm16(i),
            Xor_r32_rm32 => self.instr_xor_r32_rm32(i),
            Xor_r64_rm64 => self.instr_xor_r64_rm64(i),
            Xor_AL_imm8 => self.instr_xor_al_imm8(i),
            Xor_AX_imm16 => self.instr_xor_ax_imm16(i),
            Xor_EAX_imm32 => self.instr_xor_eax_imm32(i),
            Xor_RAX_imm32 => self.instr_xor_rax_imm32(i),
            Xor_rm8_imm8 => self.instr_xor_rm8_imm8(i),
            Xor_rm16_imm16 => self.instr_xor_rm16_imm16(i),
            Xor_rm32_imm32 => self.instr_xor_rm32_imm32(i),
            Xor_rm64_imm32 => self.instr_xor_rm64_imm32(i),
            Xor_rm8_imm8_82 => self.instr_xor_rm8_imm8_82(i),
            Xor_rm16_imm8 => self.instr_xor_rm16_imm8(i),
            Xor_rm32_imm8 => self.instr_xor_rm32_imm8(i),
            Xor_rm64_imm8 => self.instr_xor_rm64_imm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Xor", i.code()),
        }
    }

    /// XOR r/m8, r8
    ///
    /// 30 /r
    fn instr_xor_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm8_r8);

        calculate_rm_r![u8; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r/m16, r16
    ///
    /// o16 31 /r
    fn instr_xor_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm16_r16);

        calculate_rm_r![u16; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r/m32, r32
    ///
    /// o32 31 /r
    fn instr_xor_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm32_r32);

        calculate_rm_r![u32; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r/m64, r64
    ///
    /// o64 31 /r
    fn instr_xor_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm64_r64);

        calculate_rm_r![u64; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r8, r/m8
    ///
    /// 32 /r
    fn instr_xor_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r8_rm8);

        calculate_r_rm![u8; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r16, r/m16
    ///
    /// o16 33 /r
    fn instr_xor_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r16_rm16);

        calculate_r_rm![u16; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r32, r/m32
    ///
    /// o32 33 /r
    fn instr_xor_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r32_rm32);

        calculate_r_rm![u32; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r64, r/m64
    ///
    /// o64 33 /r
    fn instr_xor_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r64_rm64);

        calculate_r_rm![u64; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR AL, imm8
    ///
    /// 34 ib
    fn instr_xor_al_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_AL_imm8);

        self.instr_xor_rm8_imm8(i)
    }

    /// XOR AX, imm16
    ///
    /// o16 35 iw
    fn instr_xor_ax_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_AX_imm16);

        self.instr_xor_rm16_imm16(i)
    }

    /// XOR EAX, imm32
    ///
    /// o32 35 id
    fn instr_xor_eax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_EAX_imm32);

        self.instr_xor_rm32_imm32(i)
    }

    /// XOR RAX, imm32
    ///
    /// o64 35 id
    fn instr_xor_rax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_RAX_imm32);

        self.instr_xor_rm64_imm32(i)
    }

    /// XOR r/m8, imm8
    ///
    /// 80 /6 ib
    fn instr_xor_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u8; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r/m16, imm16
    ///
    /// o16 81 /6 iw
    fn instr_xor_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u16; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r/m32, imm32
    ///
    /// o32 81 /6 id
    fn instr_xor_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u32; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r/m64, imm32
    ///
    /// o64 81 /6 id
    fn instr_xor_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u64; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r/m8, imm8
    ///
    /// 82 /6 ib
    fn instr_xor_rm8_imm8_82(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm8_imm8_82);

        self.instr_xor_rm8_imm8(i)
    }

    /// XOR r/m16, imm8
    ///
    /// o16 83 /6 ib
    fn instr_xor_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm16_imm8);

        calculate_rm_imm![u16; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r/m32, imm8
    ///
    /// o32 83 /6 ib
    fn instr_xor_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm32_imm8);

        calculate_rm_imm![u32; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// XOR r/m64, imm8
    ///
    /// o64 83 /6 ib
    fn instr_xor_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm64_imm8);

        calculate_rm_imm![u64; self; i; |d,s| {
            d^s
        }; (set: FLAG_ZF | FLAG_SF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::SupportedRegister, write_reg_value,
    };
    use iced_x86::Register::*;

    // xor al, al
    ax_test![xor_zero; 0x30, 0xc0; |a: Axecutor| {
        assert_reg_value!(b; a; AL; 0);
    }; (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)];
    // xor al, bh
    ax_test![xor_al_bh; 0x30, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BH; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; BH; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor al, bh
    ax_test![xor_al_bh_pf; 0x30, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BH; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_reg_value!(b; a; BH; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor al, bh
    ax_test![xor_al_bh_pf_sf; 0x30, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BH; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(b; a; BH; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // xor al, bh
    ax_test![xor_al_bh_pf_zf; 0x30, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BH; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BH; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // xor al, bh
    ax_test![xor_al_bh_sf; 0x30, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BH; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; BH; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // xor ax, cx
    ax_test![xor_ax_cx; 0x66, 0x31, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; CX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
            assert_reg_value!(w; a; CX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor ax, cx
    ax_test![xor_ax_cx_pf; 0x66, 0x31, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; CX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf);
            assert_reg_value!(w; a; CX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor ax, cx
    ax_test![xor_ax_cx_pf_sf; 0x66, 0x31, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; CX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_reg_value!(w; a; CX; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // xor ax, cx
    ax_test![xor_ax_cx_pf_zf; 0x66, 0x31, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; CX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; CX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // xor ax, cx
    ax_test![xor_ax_cx_sf; 0x66, 0x31, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(w; a; CX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8001);
            assert_reg_value!(w; a; CX; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // xor dword ptr [rsp+8], eax
    ax_test![xor_dword_ptr_rsp_8_eax; 0x31, 0x44, 0x24, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1008, 4).unwrap();
            a.mem_write_32(0x1008, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_eq!(a.mem_read_32(0x1008).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor dword ptr [rsp+8], eax
    ax_test![xor_dword_ptr_rsp_8_eax_pf; 0x31, 0x44, 0x24, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xf);
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1008, 4).unwrap();
            a.mem_write_32(0x1008, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf);
            assert_eq!(a.mem_read_32(0x1008).unwrap(), 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor dword ptr [rsp+8], eax
    ax_test![xor_dword_ptr_rsp_8_eax_pf_sf; 0x31, 0x44, 0x24, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1008, 4).unwrap();
            a.mem_write_32(0x1008, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_eq!(a.mem_read_32(0x1008).unwrap(), 0x80000000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // xor dword ptr [rsp+8], eax
    ax_test![xor_dword_ptr_rsp_8_eax_pf_zf; 0x31, 0x44, 0x24, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xffff);
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1008, 4).unwrap();
            a.mem_write_32(0x1008, 0xffff0000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffff);
            assert_eq!(a.mem_read_32(0x1008).unwrap(), 0xffffffff);
        };
        (FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_CF | FLAG_OF)
    ];

    // xor dword ptr [rsp+8], eax
    ax_test![xor_dword_ptr_rsp_8_eax_sf; 0x31, 0x44, 0x24, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1008, 4).unwrap();
            a.mem_write_32(0x1008, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_eq!(a.mem_read_32(0x1008).unwrap(), 0x80000001);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];
    // xor qword ptr [r8+8], r11
    ax_test![xor_qword_ptr_r8_8_r11; 0x4d, 0x31, 0x58, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1);
            write_reg_value!(q; a; R8; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1);
            assert_eq!(a.mem_read_64(0x1008).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor qword ptr [r8+8], r11
    ax_test![xor_qword_ptr_r8_8_r11_pf; 0x4d, 0x31, 0x58, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0xf);
            write_reg_value!(q; a; R8; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xf);
            assert_eq!(a.mem_read_64(0x1008).unwrap(), 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor qword ptr [r8+8], r11
    ax_test![xor_qword_ptr_r8_8_r11_pf_sf; 0x4d, 0x31, 0x58, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
            write_reg_value!(q; a; R8; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x8000000000000000u64);
            assert_eq!(a.mem_read_64(0x1008).unwrap(), 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // xor qword ptr [r8+8], r11
    ax_test![xor_qword_ptr_r8_8_r11_pf_zf; 0x4d, 0x31, 0x58, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(q; a; R8; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x0);
            assert_eq!(a.mem_read_64(0x1008).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // xor qword ptr [r8+8], r11
    ax_test![xor_qword_ptr_r8_8_r11_sf; 0x4d, 0x31, 0x58, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
            write_reg_value!(q; a; R8; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x8000000000000000u64);
            assert_eq!(a.mem_read_64(0x1008).unwrap(), 0x8000000000000001u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // xor rax, r11
    ax_test![xor_rax_r11_pf; 0x4c, 0x31, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x33312345678u64);
            write_reg_value!(q; a; R11; 0x33387654321u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x33312345678u64 ^ 0x33387654321u64);
            assert_reg_value!(q; a; R11; 0x33387654321u64);
        };
        (FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor rax, r11
    ax_test![xor_rax_r11; 0x4c, 0x31, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; R11; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
            assert_reg_value!(q; a; R11; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor rax, r11
    ax_test![xor_rax_r11_pf_sf; 0x4c, 0x31, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_reg_value!(q; a; R11; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // xor rax, r11
    ax_test![xor_rax_r11_pf_zf; 0x4c, 0x31, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; R11; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_reg_value!(q; a; R11; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // xor rax, r11
    ax_test![xor_rax_r11_sf; 0x4c, 0x31, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000001u64);
            assert_reg_value!(q; a; R11; 0x8000000000000000u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // xor al, [rsp]
    ax_test![xor_al_rsp; 0x32, 0x4, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);

            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
            write_reg_value!(q; a; RSP; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x10^0xf);
            assert_reg_value!(q; a; RSP; 0x1000);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x10);
        };
        (0; FLAG_PF | FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor r11w, [rsp+0x20]
    ax_test![xor_r11w_rsp_0x20; 0x66, 0x44, 0x33, 0x5c, 0x24, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x10);

            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_16(0x1020, 0x20).unwrap();
            write_reg_value!(q; a; RSP; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x20^0x10);
            assert_reg_value!(q; a; RSP; 0x1000);
            assert_eq!(a.mem_read_16(0x1020).unwrap(), 0x20);
        };
        (FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor edx, [rip+0x35353]
    ax_test![xor_edx_rip0x35353; 0x33, 0x15, 0x53, 0x53, 0x3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x10);

            let rip = a.reg_read_64(RIP.into());
            a.mem_init_zero(rip+0x35353 + 6, 4).unwrap();
            a.mem_write_32(rip+0x35353 +6, 0x12345678).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x12345678^0x10);

            // Note that rip has advanced by instruction length 6
            let rip = a.reg_read_64(RIP.into());
            assert_eq!(a.mem_read_32(rip+0x35353).unwrap(), 0x12345678);
        };
        (0; FLAG_PF | FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor rax, [rip+0x35353]
    ax_test![xor_rax_rip0x35353; 0x48, 0x33, 0x5, 0x53, 0x53, 0x3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10);

            let rip = a.reg_read_64(RIP.into());
            a.mem_init_zero(rip+0x35353 + 7, 8).unwrap();
            a.mem_write_64(rip+0x35353 +7, 0x12345678).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x12345678^0x10);

            // Note that rip has advanced by instruction length 7
            let rip = a.reg_read_64(RIP.into());
            assert_eq!(a.mem_read_64(rip+0x35353).unwrap(), 0x12345678);
        };
        (0; FLAG_PF | FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor rax, [rsp+8]
    ax_test![xor_rax_rsp8; 0x48, 0x33, 0x44, 0x24, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10);

            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_64(0x1008, 0x12345678).unwrap();
            write_reg_value!(q; a; RSP; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x12345678^0x10);
            assert_reg_value!(q; a; RSP; 0x1000);
            assert_eq!(a.mem_read_64(0x1008).unwrap(), 0x12345678);
        };
        (0; FLAG_PF | FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor al, 8
    ax_test![xor_al_8; 0x34, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
        };
        (0; FLAG_PF | FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor bl, 15
    ax_test![xor_bl_15; 0x80, 0xf3, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 15);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0);
        };
        (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)
    ];

    // xor ax, 155
    ax_test![xor_ax_155; 0x66, 0x35, 0x9b, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1355u16);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 5070);
        };
        (0; FLAG_PF | FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor r11w, 0x1358
    ax_test![xor_r11w_0x1358; 0x66, 0x41, 0x81, 0xf3, 0x58, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x1358);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0);
        };
        (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)
    ];

    // xor edx, 0x1358
    ax_test![xor_edx_0x1358; 0x81, 0xf2, 0x58, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x1358);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0);
        };
        (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)
    ];

    // xor eax, 0x1358
    ax_test![xor_eax_0x1358; 0x35, 0x58, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1358);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0);
        };
        (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)
    ];

    // xor rax, 0x35183958
    ax_test![xor_rax_0x35183958; 0x48, 0x35, 0x58, 0x39, 0x18, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x35183957);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 15);
        };
        (FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor cx, 0xffff -- should be sign extended
    ax_test![xor_cx_0xffff; 0x66, 0x83, 0xf1, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0xffff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0);
        };
        (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)
    ];

    // xor rcx, 0xfffffff
    ax_test![xor_rcx_0xfffffff; 0x48, 0x81, 0xf1, 0xff, 0xff, 0xff, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0xffffff0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0xf);
        };
        (FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor ecx, 0xffffffff
    ax_test![xor_ecx_0xffffffff; 0x83, 0xf1, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0xffffffff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0);
        };
        (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)
    ];

    // xor rcx, 0xffffffffffffffff
    ax_test![xor_rcx_0xffffffffffffffff; 0x48, 0x83, 0xf1, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0xffffffffffffffff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0);
        };
        (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)
    ];

    // xor rcx, 0xffffffffffffffff
    ax_test![xor_rcx_0xffffffffffffffff_nonzero; 0x48, 0x83, 0xf1, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0xe759a4c219c95a1bu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x18a65b3de636a5e4u64);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor cl, byte ptr [rax]
    ax_test![xor_cl_byte_ptr_rax_pf_zf_cf_sf_of; 0x32, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x0);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // xor cl, byte ptr [rax]
    ax_test![xor_cl_byte_ptr_rax_cf_pf_zf_sf_of; 0x32, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x1);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor cl, byte ptr [rax]
    ax_test![xor_cl_byte_ptr_rax_pf_cf_zf_sf_of; 0x32, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0xf);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor cl, byte ptr [rax]
    ax_test![xor_cl_byte_ptr_rax_sf_cf_pf_zf_of; 0x32, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x80);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // xor cl, byte ptr [rax]
    ax_test![xor_cl_byte_ptr_rax_pf_sf_cf_zf_of; 0x32, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0xff);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // xor byte ptr [rax], cl
    ax_test![xor_byte_ptr_rax_cl_pf_zf_cf_sf_of; 0x30, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x0);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // xor byte ptr [rax], cl
    ax_test![xor_byte_ptr_rax_cl_cf_pf_zf_sf_of; 0x30, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x1);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor byte ptr [rax], cl
    ax_test![xor_byte_ptr_rax_cl_pf_cf_zf_sf_of; 0x30, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0xf);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0xf);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor byte ptr [rax], cl
    ax_test![xor_byte_ptr_rax_cl_sf_cf_pf_zf_of; 0x30, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x80);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x80);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // xor byte ptr [rax], cl
    ax_test![xor_byte_ptr_rax_cl_pf_sf_cf_zf_of; 0x30, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0xff);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0xff);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // xor al, cl
    ax_test![xor_al_cl_pf_zf_cf_sf_of; 0x30, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // xor al, cl
    ax_test![xor_al_cl_cf_pf_zf_sf_of; 0x30, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor al, cl
    ax_test![xor_al_cl_pf_cf_zf_sf_of; 0x30, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; CL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_reg_value!(b; a; CL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor al, cl
    ax_test![xor_al_cl_sf_cf_pf_zf_of; 0x30, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; CL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; CL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // xor al, cl
    ax_test![xor_al_cl_pf_sf_cf_zf_of; 0x30, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; CL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(b; a; CL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];
    // xor al, byte ptr [rbx]
    ax_test![xor_al_byte_ptr_rbx_pf_zf_cf_sf_of; 0x32, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // xor al, byte ptr [rbx]
    ax_test![xor_al_byte_ptr_rbx_cf_pf_zf_sf_of; 0x32, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor al, byte ptr [rbx]
    ax_test![xor_al_byte_ptr_rbx_pf_cf_zf_sf_of; 0x32, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor al, byte ptr [rbx]
    ax_test![xor_al_byte_ptr_rbx_sf_cf_pf_zf_of; 0x32, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // xor al, byte ptr [rbx]
    ax_test![xor_al_byte_ptr_rbx_pf_sf_cf_zf_of; 0x32, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // xor r11w, word ptr [rbx]
    ax_test![xor_r11w_word_ptr_rbx_pf_zf_cf_sf_of; 0x66, 0x44, 0x33, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x0);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // xor r11w, word ptr [rbx]
    ax_test![xor_r11w_word_ptr_rbx_cf_pf_zf_sf_of; 0x66, 0x44, 0x33, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x1);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor r11w, word ptr [rbx]
    ax_test![xor_r11w_word_ptr_rbx_pf_cf_zf_sf_of; 0x66, 0x44, 0x33, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0xf);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // xor r11w, word ptr [rbx]
    ax_test![xor_r11w_word_ptr_rbx_pf_sf_cf_zf_of; 0x66, 0x44, 0x33, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x8000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // xor r11w, word ptr [rbx]
    ax_test![xor_r11w_word_ptr_rbx_sf_cf_pf_zf_of; 0x66, 0x44, 0x33, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x8001);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];
}
