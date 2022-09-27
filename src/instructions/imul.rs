use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Imul;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::operand::Operand;
use crate::instructions::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_imul(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Imul);

        match i.code() {
            Imul_r16_rm16_imm16 => self.instr_imul_r16_rm16_imm16(i),
            Imul_r32_rm32_imm32 => self.instr_imul_r32_rm32_imm32(i),
            Imul_r64_rm64_imm32 => self.instr_imul_r64_rm64_imm32(i),
            Imul_r16_rm16_imm8 => self.instr_imul_r16_rm16_imm8(i),
            Imul_r32_rm32_imm8 => self.instr_imul_r32_rm32_imm8(i),
            Imul_r64_rm64_imm8 => self.instr_imul_r64_rm64_imm8(i),
            Imul_rm8 => self.instr_imul_rm8(i),
            Imul_rm16 => self.instr_imul_rm16(i),
            Imul_rm32 => self.instr_imul_rm32(i),
            Imul_rm64 => self.instr_imul_rm64(i),
            Imul_r16_rm16 => self.instr_imul_r16_rm16(i),
            Imul_r32_rm32 => self.instr_imul_r32_rm32(i),
            Imul_r64_rm64 => self.instr_imul_r64_rm64(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Imul", i.code()),
        }
    }

    /// IMUL r16, r/m16, imm16
    ///
    /// o16 69 /r iw
    fn instr_imul_r16_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r16_rm16_imm16);

        todo!("instr_imul_r16_rm16_imm16 for Imul")
    }

    /// IMUL r32, r/m32, imm32
    ///
    /// o32 69 /r id
    fn instr_imul_r32_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r32_rm32_imm32);

        todo!("instr_imul_r32_rm32_imm32 for Imul")
    }

    /// IMUL r64, r/m64, imm32
    ///
    /// o64 69 /r id
    fn instr_imul_r64_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r64_rm64_imm32);

        todo!("instr_imul_r64_rm64_imm32 for Imul")
    }

    /// IMUL r16, r/m16, imm8
    ///
    /// o16 6B /r ib
    fn instr_imul_r16_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r16_rm16_imm8);

        todo!("instr_imul_r16_rm16_imm8 for Imul")
    }

    /// IMUL r32, r/m32, imm8
    ///
    /// o32 6B /r ib
    fn instr_imul_r32_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r32_rm32_imm8);

        todo!("instr_imul_r32_rm32_imm8 for Imul")
    }

    /// IMUL r64, r/m64, imm8
    ///
    /// o64 6B /r ib
    fn instr_imul_r64_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r64_rm64_imm8);

        todo!("instr_imul_r64_rm64_imm8 for Imul")
    }

    /// IMUL r/m8
    ///
    /// F6 /5
    fn instr_imul_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_rm8);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_8(r),
            Operand::Memory(m) => self.mem_read_8(self.mem_addr(m))?,
            _ => panic!("Invalid operand {:?} for Imul_rm8", op),
        } as i8;

        let dst_val = self.reg_read_8(AL) as i8;

        let result = (dst_val as i16).wrapping_mul(src_val as i16);

        self.reg_write_16(AX, result as u16);

        self.set_flags_u8(
            if result >> 7 != 0 && result >> 7 != -1 {
                FLAG_CF | FLAG_OF
            } else {
                0
            },
            if result >> 7 != 0 && result >> 7 != -1 {
                0
            } else {
                FLAG_CF | FLAG_OF
            },
            0,
        );

        Ok(())
    }

    /// IMUL r/m16
    ///
    /// o16 F7 /5
    fn instr_imul_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_rm16);

        todo!("instr_imul_rm16 for Imul")
    }

    /// IMUL r/m32
    ///
    /// o32 F7 /5
    fn instr_imul_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_rm32);

        todo!("instr_imul_rm32 for Imul")
    }

    /// IMUL r/m64
    ///
    /// o64 F7 /5
    fn instr_imul_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_rm64);

        todo!("instr_imul_rm64 for Imul")
    }

    /// IMUL r16, r/m16
    ///
    /// o16 0F AF /r
    fn instr_imul_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r16_rm16);

        todo!("instr_imul_r16_rm16 for Imul")
    }

    /// IMUL r32, r/m32
    ///
    /// o32 0F AF /r
    fn instr_imul_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r32_rm32);

        todo!("instr_imul_r32_rm32 for Imul")
    }

    /// IMUL r64, r/m64
    ///
    /// o64 0F AF /r
    fn instr_imul_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r64_rm64);

        todo!("instr_imul_r64_rm64 for Imul")
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::SupportedRegister, write_reg_value,
    };
    use iced_x86::Register::*;

    // imul bl
    ax_test![imul_bl_ax_0; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_1; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_1024; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_127; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_128; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_15; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_16; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_16384; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_17; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_2; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_2048; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_255; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_256; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_31; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_32; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_32767; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_32768; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_33; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_4; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_4096; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_512; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_63; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_64; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x40);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_65; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_7; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_8; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_ax_8192; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_127; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0x379);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_128; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0xfc80);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_15; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
            write_reg_value!(w; a; AX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf);
            assert_reg_value!(w; a; AX; 0xe1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_16; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_reg_value!(w; a; AX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x8);
            assert_reg_value!(w; a; AX; 0x80);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_17; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_reg_value!(w; a; AX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x8);
            assert_reg_value!(w; a; AX; 0x88);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_2; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x40);
            write_reg_value!(w; a; AX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x40);
            assert_reg_value!(w; a; AX; 0x80);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_255; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(w; a; AX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(w; a; AX; 0x80);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_31; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0xd9);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_32; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0xe0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_32767; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(w; a; AX; 0x80);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_33; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0xe7);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_4; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x20);
            write_reg_value!(w; a; AX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x20);
            assert_reg_value!(w; a; AX; 0x80);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_63; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0x1b9);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_64; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x40);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0x1c0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_65; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0x1c7);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_7; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1f);
            write_reg_value!(w; a; AX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1f);
            assert_reg_value!(w; a; AX; 0xd9);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul bl
    ax_test![imul_bl_cf_of_ax_8; 0xf6, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x10);
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x10);
            assert_reg_value!(w; a; AX; 0x80);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];
}
