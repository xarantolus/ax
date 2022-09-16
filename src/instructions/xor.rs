use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Xor;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::flags::*;
use crate::{calculate_r_rm, calculate_rm_r};

// TODO: Flags
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
            _ => panic!("Invalid instruction code {:?} for mnemonic Xor", i.code()),
        }
    }

    /// XOR r/m8, r8
    ///
    /// 30 /r
    fn instr_xor_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm8_r8);

        calculate_rm_r![u8; self; i; |d,s| {
            d^s
        }; FLAG_ZF | FLAG_SF | FLAG_PF]
    }

    /// XOR r/m16, r16
    ///
    /// o16 31 /r
    fn instr_xor_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm16_r16);

        calculate_rm_r![u16; self; i; |d,s| {
            d^s
        }; FLAG_ZF | FLAG_SF | FLAG_PF]
    }

    /// XOR r/m32, r32
    ///
    /// o32 31 /r
    fn instr_xor_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm32_r32);

        calculate_rm_r![u32; self; i; |d,s| {
            d^s
        }; FLAG_ZF | FLAG_SF | FLAG_PF]
    }

    /// XOR r/m64, r64
    ///
    /// o64 31 /r
    fn instr_xor_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm64_r64);

        calculate_rm_r![u64; self; i; |d,s| {
            d^s
        }; FLAG_ZF | FLAG_SF | FLAG_PF]
    }

    /// XOR r8, r/m8
    ///
    /// 32 /r
    fn instr_xor_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r8_rm8);

        calculate_r_rm![u8; self; i; |d,s| {
            d^s
        }; FLAG_ZF | FLAG_SF | FLAG_PF]
    }

    /// XOR r16, r/m16
    ///
    /// o16 33 /r
    fn instr_xor_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r16_rm16);

        calculate_r_rm![u16; self; i; |d,s| {
            d^s
        }; FLAG_ZF | FLAG_SF | FLAG_PF]
    }

    /// XOR r32, r/m32
    ///
    /// o32 33 /r
    fn instr_xor_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r32_rm32);

        calculate_r_rm![u32; self; i; |d,s| {
            d^s
        }; FLAG_ZF | FLAG_SF | FLAG_PF]
    }

    /// XOR r64, r/m64
    ///
    /// o64 33 /r
    fn instr_xor_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r64_rm64);

        calculate_r_rm![u64; self; i; |d,s| {
            d^s
        }; FLAG_ZF | FLAG_SF | FLAG_PF]
    }

    /// XOR AL, imm8
    ///
    /// 34 ib
    fn instr_xor_al_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_AL_imm8);

        todo!("instr_xor_al_imm8 for Xor")
    }

    /// XOR AX, imm16
    ///
    /// o16 35 iw
    fn instr_xor_ax_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_AX_imm16);

        todo!("instr_xor_ax_imm16 for Xor")
    }

    /// XOR EAX, imm32
    ///
    /// o32 35 id
    fn instr_xor_eax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_EAX_imm32);

        todo!("instr_xor_eax_imm32 for Xor")
    }

    /// XOR RAX, imm32
    ///
    /// o64 35 id
    fn instr_xor_rax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_RAX_imm32);

        todo!("instr_xor_rax_imm32 for Xor")
    }

    /// XOR r/m8, imm8
    ///
    /// 80 /6 ib
    fn instr_xor_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm8_imm8);

        todo!("instr_xor_rm8_imm8 for Xor")
    }

    /// XOR r/m16, imm16
    ///
    /// o16 81 /6 iw
    fn instr_xor_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm16_imm16);

        todo!("instr_xor_rm16_imm16 for Xor")
    }

    /// XOR r/m32, imm32
    ///
    /// o32 81 /6 id
    fn instr_xor_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm32_imm32);

        todo!("instr_xor_rm32_imm32 for Xor")
    }

    /// XOR r/m64, imm32
    ///
    /// o64 81 /6 id
    fn instr_xor_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm64_imm32);

        todo!("instr_xor_rm64_imm32 for Xor")
    }

    /// XOR r/m8, imm8
    ///
    /// 82 /6 ib
    fn instr_xor_rm8_imm8_82(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm8_imm8_82);

        todo!("instr_xor_rm8_imm8_82 for Xor")
    }

    /// XOR r/m16, imm8
    ///
    /// o16 83 /6 ib
    fn instr_xor_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm16_imm8);

        todo!("instr_xor_rm16_imm8 for Xor")
    }

    /// XOR r/m32, imm8
    ///
    /// o32 83 /6 ib
    fn instr_xor_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm32_imm8);

        todo!("instr_xor_rm32_imm8 for Xor")
    }

    /// XOR r/m64, imm8
    ///
    /// o64 83 /6 ib
    fn instr_xor_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm64_imm8);

        todo!("instr_xor_rm64_imm8 for Xor")
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::RegisterWrapper, write_reg_value,
    };
    use iced_x86::Register::*;

    // xor al, al
    ax_test![xor_zero; 0x30, 0xc0; |a: Axecutor| {
        assert_reg_value!(b; a; AL; 0);
    }; (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)];
    // xor al, bl
    ax_test![xor_same_value; 0x30, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
            write_reg_value!(b; a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0);
            assert_reg_value!(b; a; BL; 0xf);
        };
        (FLAG_ZF | FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF)
    ];
    // xor al, cl
    ax_test![xor_different_value; 0x30, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0b1010);
            write_reg_value!(b; a; CL; 0b0101);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0b1111);
            assert_reg_value!(b; a; CL; 0b0101);
        };
        (FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];
    // xor al, cl
    ax_test![xor_sign_flag; 0x30, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0b10000000);
            write_reg_value!(b; a; CL; 0b00000000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0b10000000);
            assert_reg_value!(b; a; CL; 0b00000000);
        };
        (FLAG_SF; FLAG_PF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];
    // xor ax, cx
    ax_test![xor_ax_cx; 0x66, 0x31, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xffff);
            write_reg_value!(w; a; CX; 0xf0f0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0f0f);
            assert_reg_value!(w; a; CX; 0xf0f0);
        };
        (FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];
    // xor [rsp+8], eax
    ax_test![xor_rsp8_eax; 0x31, 0x44, 0x24, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x12345678);
            // Create a small stack
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_32(0x1008, 0x87654321).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RSP; 0x1000);
            assert_reg_value!(d; a; EAX; 0x12345678);
            assert_eq!(
                a.mem_read_32(0x1008).unwrap(),
                0x12345678 ^ 0x87654321
            );
        };
        (FLAG_PF | FLAG_SF; FLAG_OF | FLAG_CF | FLAG_ZF)
    ];
    // xor [rsp-8], r11
    ax_test![xor_rsp8_r11; 0x4c, 0x31, 0x5c, 0x24, 0xf8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x33312345678u64);
            // Create a small stack
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x800, 0x1000).unwrap();
            a.mem_write_64(0xff8, 0x87654321).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RSP; 0x1000);
            assert_reg_value!(q; a; R11; 0x33312345678u64);
            assert_eq!(
                a.mem_read_64(0xff8).unwrap(),
                0x33312345678 ^ 0x87654321
            );
        };
        (FLAG_PF; FLAG_SF | FLAG_OF | FLAG_CF | FLAG_ZF)
    ];

    // xor rax, r11
    ax_test![xor_rax_r11; 0x4c, 0x31, 0xd8;
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
}
