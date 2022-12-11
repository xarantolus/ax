use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Imul;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::fatal_error;
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
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Imul", i.code()),
        }
    }

    /// IMUL r16, r/m16, imm16
    ///
    /// o16 69 /r iw
    fn instr_imul_r16_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r16_rm16_imm16);

        let (dest_op, src_op) = self.instruction_operands_2(i)?;
        let imm_op = self.instruction_operand(i, 2)?;

        let src_value = match src_op {
            Operand::Register(r) => self.reg_read_16(r),
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand for IMUL r16, r/m16, imm16"),
        } as i16;

        let imm_value = match imm_op {
            Operand::Immediate { data, size: _ } => data as i16,
            _ => {
                fatal_error!("Invalid operand for IMUL r16, r/m16, imm16; expected immediate value")
            }
        };

        let (result, overflow) = src_value.overflowing_mul(imm_value);

        self.reg_write_16(dest_op.into(), result as u16);

        if overflow {
            self.set_flags_u8(FLAG_CF | FLAG_OF, 0, 0);
        } else {
            self.set_flags_u8(0, FLAG_CF | FLAG_OF, 0);
        }

        Ok(())
    }

    /// IMUL r32, r/m32, imm32
    ///
    /// o32 69 /r id
    fn instr_imul_r32_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r32_rm32_imm32);

        let (dest_op, src_op) = self.instruction_operands_2(i)?;
        let imm_op = self.instruction_operand(i, 2)?;

        let src_value = match src_op {
            Operand::Register(r) => self.reg_read_32(r),
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand for IMUL r32, r/m32, imm32"),
        } as i32;

        let imm_value = match imm_op {
            Operand::Immediate { data, size: _ } => data as i32,
            _ => {
                fatal_error!("Invalid operand for IMUL r32, r/m32, imm32; expected immediate value")
            }
        };

        let (result, overflow) = src_value.overflowing_mul(imm_value);

        self.reg_write_32(dest_op.into(), result as u32);

        if overflow {
            self.set_flags_u8(FLAG_CF | FLAG_OF, 0, 0);
        } else {
            self.set_flags_u8(0, FLAG_CF | FLAG_OF, 0);
        }

        Ok(())
    }

    /// IMUL r64, r/m64, imm32
    ///
    /// o64 69 /r id
    fn instr_imul_r64_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r64_rm64_imm32);

        let (dest_op, src_op) = self.instruction_operands_2(i)?;
        let imm_op = self.instruction_operand(i, 2)?;

        let src_value = match src_op {
            Operand::Register(r) => self.reg_read_64(r),
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand for IMUL r64, r/m64, imm32"),
        } as i64;

        let imm_value = match imm_op {
            Operand::Immediate { data, size: _ } => data as i64,
            _ => {
                fatal_error!("Invalid operand for IMUL r64, r/m64, imm32; expected immediate value")
            }
        };

        let (result, overflow) = src_value.overflowing_mul(imm_value);

        self.reg_write_64(dest_op.into(), result as u64);

        if overflow {
            self.set_flags_u8(FLAG_CF | FLAG_OF, 0, 0);
        } else {
            self.set_flags_u8(0, FLAG_CF | FLAG_OF, 0);
        }

        Ok(())
    }

    /// IMUL r16, r/m16, imm8
    ///
    /// o16 6B /r ib
    fn instr_imul_r16_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r16_rm16_imm8);

        let (dest_op, src_op) = self.instruction_operands_2(i)?;
        let imm_op = self.instruction_operand(i, 2)?;

        let src_value = match src_op {
            Operand::Register(r) => self.reg_read_16(r),
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand for IMUL r16, r/m16, imm8"),
        } as i16;

        let imm_value = match imm_op {
            Operand::Immediate { data, size: _ } => data as i16,
            _ => {
                fatal_error!("Invalid operand for IMUL r16, r/m16, imm8; expected immediate value")
            }
        };

        let (result, overflow) = src_value.overflowing_mul(imm_value);

        self.reg_write_16(dest_op.into(), result as u16);

        if overflow {
            self.set_flags_u8(FLAG_CF | FLAG_OF, 0, 0);
        } else {
            self.set_flags_u8(0, FLAG_CF | FLAG_OF, 0);
        }

        Ok(())
    }

    /// IMUL r32, r/m32, imm8
    ///
    /// o32 6B /r ib
    fn instr_imul_r32_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r32_rm32_imm8);

        let (dest_op, src_op) = self.instruction_operands_2(i)?;
        let imm_op = self.instruction_operand(i, 2)?;

        let src_value = match src_op {
            Operand::Register(r) => self.reg_read_32(r),
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand for IMUL r32, r/m32, imm8"),
        } as i32;

        let imm_value = match imm_op {
            Operand::Immediate { data, size: _ } => data as i32,
            _ => {
                fatal_error!("Invalid operand for IMUL r32, r/m32, imm8; expected immediate value")
            }
        };

        let (result, overflow) = src_value.overflowing_mul(imm_value);

        self.reg_write_32(dest_op.into(), result as u32);

        if overflow {
            self.set_flags_u8(FLAG_CF | FLAG_OF, 0, 0);
        } else {
            self.set_flags_u8(0, FLAG_CF | FLAG_OF, 0);
        }

        Ok(())
    }

    /// IMUL r64, r/m64, imm8
    ///
    /// o64 6B /r ib
    fn instr_imul_r64_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r64_rm64_imm8);

        let (dest_op, src_op) = self.instruction_operands_2(i)?;
        let imm_op = self.instruction_operand(i, 2)?;

        let src_value = match src_op {
            Operand::Register(r) => self.reg_read_64(r),
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand for IMUL r64, r/m64, imm8"),
        } as i64;

        let imm_value = match imm_op {
            Operand::Immediate { data, size: _ } => data as i64,
            _ => {
                fatal_error!("Invalid operand for IMUL r64, r/m64, imm8; expected immediate value")
            }
        };

        let (result, overflow) = src_value.overflowing_mul(imm_value);

        self.reg_write_64(dest_op.into(), result as u64);

        if overflow {
            self.set_flags_u8(FLAG_CF | FLAG_OF, 0, 0);
        } else {
            self.set_flags_u8(0, FLAG_CF | FLAG_OF, 0);
        }

        Ok(())
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
            _ => fatal_error!("Invalid operand {:?} for Imul_rm8", op),
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

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_16(r),
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Imul_rm16", op),
        } as i16;

        let dst_val = self.reg_read_16(AX) as i16;

        let result = (dst_val as i32).wrapping_mul(src_val as i32);

        self.reg_write_16(AX, result as u16);
        self.reg_write_16(DX, (result >> 16) as u16);

        self.set_flags_u16(
            if result >> 15 != 0 && result >> 15 != -1 {
                FLAG_CF | FLAG_OF
            } else {
                0
            },
            if result >> 15 != 0 && result >> 15 != -1 {
                0
            } else {
                FLAG_CF | FLAG_OF
            },
            0,
        );

        Ok(())
    }

    /// IMUL r/m32
    ///
    /// o32 F7 /5
    fn instr_imul_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_rm32);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_32(r),
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Imul_rm32", op),
        } as i32;

        let dst_val = self.reg_read_32(EAX) as i32;

        let result = (dst_val as i64).wrapping_mul(src_val as i64);

        self.reg_write_32(EAX, result as u32);
        self.reg_write_32(EDX, (result >> 32) as u32);

        self.set_flags_u32(
            if result >> 31 != 0 && result >> 31 != -1 {
                FLAG_CF | FLAG_OF
            } else {
                0
            },
            if result >> 31 != 0 && result >> 31 != -1 {
                0
            } else {
                FLAG_CF | FLAG_OF
            },
            0,
        );

        Ok(())
    }

    /// IMUL r/m64
    ///
    /// o64 F7 /5
    fn instr_imul_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_rm64);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_64(r),
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Imul_rm64", op),
        } as i64;

        let dst_val = self.reg_read_64(RAX) as i64;

        let result = (dst_val as i128).wrapping_mul(src_val as i128);

        self.reg_write_64(RAX, result as u64);
        self.reg_write_64(RDX, (result >> 64) as u64);

        self.set_flags_u64(
            if result >> 63 != 0 && result >> 63 != -1 {
                FLAG_CF | FLAG_OF
            } else {
                0
            },
            if result >> 63 != 0 && result >> 63 != -1 {
                0
            } else {
                FLAG_CF | FLAG_OF
            },
            0,
        );

        Ok(())
    }

    /// IMUL r16, r/m16
    ///
    /// o16 0F AF /r
    fn instr_imul_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r16_rm16);

        let (dest_op, src_op) = self.instruction_operands_2(i)?;

        let src_val = match src_op {
            Operand::Register(r) => self.reg_read_16(r),
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Imul_r16_rm16", src_op),
        } as i16;

        let dst_val = match dest_op {
            Operand::Register(r) => self.reg_read_16(r),
            _ => fatal_error!("Invalid operand {:?} for Imul_r16_rm16", dest_op),
        } as i16;

        let result = (dst_val as i32).wrapping_mul(src_val as i32);

        self.reg_write_16(dest_op.into(), result as u16);

        self.set_flags_u16(
            if result >> 15 != 0 && result >> 15 != -1 {
                FLAG_CF | FLAG_OF
            } else {
                0
            },
            if result >> 15 != 0 && result >> 15 != -1 {
                0
            } else {
                FLAG_CF | FLAG_OF
            },
            0,
        );

        Ok(())
    }

    /// IMUL r32, r/m32
    ///
    /// o32 0F AF /r
    fn instr_imul_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r32_rm32);

        let (dest_op, src_op) = self.instruction_operands_2(i)?;

        let src_val = match src_op {
            Operand::Register(r) => self.reg_read_32(r),
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Imul_r32_rm32", src_op),
        } as i32;

        let dst_val = match dest_op {
            Operand::Register(r) => self.reg_read_32(r),
            _ => fatal_error!("Invalid operand {:?} for Imul_r32_rm32", dest_op),
        } as i32;

        let result = (dst_val as i64).wrapping_mul(src_val as i64);

        self.reg_write_32(dest_op.into(), result as u32);

        self.set_flags_u32(
            if result >> 31 != 0 && result >> 31 != -1 {
                FLAG_CF | FLAG_OF
            } else {
                0
            },
            if result >> 31 != 0 && result >> 31 != -1 {
                0
            } else {
                FLAG_CF | FLAG_OF
            },
            0,
        );

        Ok(())
    }

    /// IMUL r64, r/m64
    ///
    /// o64 0F AF /r
    fn instr_imul_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Imul_r64_rm64);

        let (dest_op, src_op) = self.instruction_operands_2(i)?;

        let src_val = match src_op {
            Operand::Register(r) => self.reg_read_64(r),
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Imul_r64_rm64", src_op),
        } as i64;

        let dst_val = match dest_op {
            Operand::Register(r) => self.reg_read_64(r),
            _ => fatal_error!("Invalid operand {:?} for Imul_r64_rm64", dest_op),
        } as i64;

        let result = (dst_val as i128).wrapping_mul(src_val as i128);

        self.reg_write_64(dest_op.into(), result as u64);

        self.set_flags_u64(
            if result >> 63 != 0 && result >> 63 != -1 {
                FLAG_CF | FLAG_OF
            } else {
                0
            },
            if result >> 63 != 0 && result >> 63 != -1 {
                0
            } else {
                FLAG_CF | FLAG_OF
            },
            0,
        );

        Ok(())
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

    // imul ax
    ax_test![imul_ax_dx_16384; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_7; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_65; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_8; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_32; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_17; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_127; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_64; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x40);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_16; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_32767; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_7; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_128; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_1; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_16384; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_0; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_1; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_256; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_4096; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_64; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x40);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_127; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_2; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_32768; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_31; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_63; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_1024; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_4; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_65; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_16; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_2048; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_32767; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_1024; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_32768; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_8192; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_32; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_255; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_15; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_4096; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_8192; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_0; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_255; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_512; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_63; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_33; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_512; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_31; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_2048; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_dx_17; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_2; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_cf_of_dx_15; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ax
    ax_test![imul_ax_dx_256; 0x66, 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; DX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_32768; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_65536; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x10000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_1073741824; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x40000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_255; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_2048; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_127; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_8192; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_63; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_edx_67108864; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x4000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_262144; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x40000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_127; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_7; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_2; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_256; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_33; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_4194304; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x400000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_edx_65; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_32768; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_edx_32; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_4096; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_17; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_536870912; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x20000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_edx_32767; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_edx_33554432; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x2000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_1024; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_edx_536870912; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x20000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_512; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_edx_16384; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_262144; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x40000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_65536; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x10000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_4; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_32767; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax
    ax_test![imul_eax_cf_of_edx_15; 0xf7, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_rax_36028797018963968; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x80000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_6212733718593131064; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x587d7480d2262a2u64);
            write_reg_value!(q; a; RAX; 0x563810668cd5d238u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x1dcd86400cb9c7bu64);
            assert_reg_value!(q; a; RAX; 0x8bbf5aa1bd387770u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_rax_256; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_17792937989460827356; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x3a57a0d2f7a35c3fu64);
            write_reg_value!(q; a; RAX; 0xf6ed36d4945d9cdcu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xfdeea35693f08e22u64);
            assert_reg_value!(q; a; RAX; 0x662dda0d4b7caa24u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_6384137965328039445; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x800000000u64);
            write_reg_value!(q; a; RAX; 0x589903a7a4767e15u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x2c4c81d3du64);
            assert_reg_value!(q; a; RAX; 0x23b3f0a800000000u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_9266472426817105366; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x861e2e59bf21f332u64);
            write_reg_value!(q; a; RAX; 0x80991f938962add6u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x3ca801d43d9ae8f7u64);
            assert_reg_value!(q; a; RAX; 0x7e056f0a92de15ccu64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_2251799813685248; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x7fff);
            write_reg_value!(q; a; RAX; 0x8000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x3);
            assert_reg_value!(q; a; RAX; 0xfff8000000000000u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_rax_281474976710656; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_5387174685143504005; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x461599b0318a7c07u64);
            write_reg_value!(q; a; RAX; 0x4ac316d56af49485u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x1477a72147fdaf13u64);
            assert_reg_value!(q; a; RAX; 0xf4bc1059e9527ba3u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_34359738368; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x7fffffffu64);
            write_reg_value!(q; a; RAX; 0x800000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x3);
            assert_reg_value!(q; a; RAX; 0xfffffff800000000u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_12061242800591253505; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x10000);
            write_reg_value!(q; a; RAX; 0xa762246c86fab001u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xffffffffffffa762u64);
            assert_reg_value!(q; a; RAX; 0x246c86fab0010000u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_7127888912088746134; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x19f5e13a14f80701u64);
            write_reg_value!(q; a; RAX; 0x62eb59334b7ec896u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xa07fdf9ab492691u64);
            assert_reg_value!(q; a; RAX; 0x648612bdcc4ae296u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_268435456; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RAX; 0x10000000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x7ffffff);
            assert_reg_value!(q; a; RAX; 0xfffffffff0000000u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_rax_70368744177664; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x400000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_12855651112178859905; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x80000000000u64);
            write_reg_value!(q; a; RAX; 0xb268729309c6bf81u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xfffffd934394984eu64);
            assert_reg_value!(q; a; RAX; 0x35fc080000000000u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_1048576; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RAX; 0x100000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x7ffff);
            assert_reg_value!(q; a; RAX; 0xfffffffffff00000u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_rax_2305843009213693952; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x2000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rdx
    ax_test![imul_rdx_rax_2147483647; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rdx
    ax_test![imul_rdx_rax_9223372036854775808; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_11725129329379829068; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0xf88320e7b1ef5f7au64);
            write_reg_value!(q; a; RAX; 0xa2b80700af32e54cu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x2ba77f623f87b41u64);
            assert_reg_value!(q; a; RAX; 0x428cbff8ff4c7a38u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_8192; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RAX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xfff);
            assert_reg_value!(q; a; RAX; 0xffffffffffffe000u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_rax_4611686018427387904; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x4000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rdx
    ax_test![imul_rdx_rax_33; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_288230376151711744; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x20);
            write_reg_value!(q; a; RAX; 0x400000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rdx
    ax_test![imul_rdx_cf_of_rax_17271153243854462235; 0x48, 0xf7, 0xea; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x4a2cac6ba2f5ebb7u64);
            write_reg_value!(q; a; RAX; 0xefaf765e6947dd1bu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xfb45df62aa242344u64);
            assert_reg_value!(q; a; RAX; 0x874c121ceb2dd74du64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul cx, bx
    ax_test![imul_cx_bx; 0x66, 0xf, 0xaf, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
            assert_reg_value!(w; a; BX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul cx, bx
    ax_test![imul_cx_bx_cf_of; 0x66, 0xf, 0xaf, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x7);
            write_reg_value!(w; a; BX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x7ff9);
            assert_reg_value!(w; a; BX; 0x7fff);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul cx, bx
    ax_test![imul_cx_bx_cf_of_2; 0x66, 0xf, 0xaf, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x7);
            write_reg_value!(w; a; BX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x7ff9);
            assert_reg_value!(w; a; BX; 0x7fff);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul cx, bx
    ax_test![imul_cx_bx_1; 0x66, 0xf, 0xaf, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x1);
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
            assert_reg_value!(w; a; BX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ecx, ebx
    ax_test![imul_ecx_ebx; 0xf, 0xaf, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x0);
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ecx, ebx
    ax_test![imul_ecx_ebx_cf_of; 0xf, 0xaf, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x7);
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x7ffffff9);
            assert_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rcx, rbx
    ax_test![imul_rcx_rbx; 0x48, 0xf, 0xaf, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x0);
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x0);
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rcx, rbx
    ax_test![imul_rcx_rbx_cf_of; 0x48, 0xf, 0xaf, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x7);
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x7ffffffffffffff9u64);
            assert_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul eax, 0x5 -- this is encoded as imul eax,eax,0x5
    ax_test![imul_eax_0x5; 0x6b, 0xc0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax, 0x5 -- this is encoded as imul eax,eax,0x5
    ax_test![imul_eax_0x5_2; 0x6b, 0xc0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x5);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x19);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax, 0x5 -- this is encoded as imul eax,eax,0x5
    ax_test![imul_eax_0x5_negative; 0x6b, 0xc0, 0x5;
        |a: &mut Axecutor| {
            // -20
            write_reg_value!(d; a; EAX; 0xffffffec);
        };
        |a: Axecutor| {
            // -20 * 5 = -100
            assert_reg_value!(d; a; EAX; 0xffffff9cu32);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul eax, 0x5 -- this is encoded as imul eax,eax,0x5
    ax_test![imul_eax_0x5_cf_of; 0x6b, 0xc0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7ffffffb);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rbx, 0x3
    ax_test![imul_rbx_0x3; 0x48, 0x6b, 0xdb, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rbx, 0x3
    ax_test![imul_rbx_0x3_cf_of; 0x48, 0x6b, 0xdb, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x7ffffffffffffffdu64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rbx, -0x35
    ax_test![imul_rbx_0x35; 0x48, 0x6b, 0xdb, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rbx, -0x35
    ax_test![imul_rbx_0x35_cf_of; 0x48, 0x6b, 0xdb, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000035u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul cx, -0x14d7
    ax_test![imul_cx_0x14d7; 0x66, 0x69, 0xc9, 0x29, 0xeb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul cx, -0x14d7
    ax_test![imul_cx_0x14d7_cf_of; 0x66, 0x69, 0xc9, 0x29, 0xeb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x6e1f);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul ebx, -0x8241f
    ax_test![imul_ebx_0x8241f; 0x69, 0xdb, 0xe1, 0xdb, 0xf7, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul ebx, -0x8241f
    ax_test![imul_ebx_0x8241f_cf_of; 0x69, 0xdb, 0xe1, 0xdb, 0xf7, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xedf8a41fu32);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul rbx, -0x8241f
    ax_test![imul_rbx_0x8241f; 0x48, 0x69, 0xdb, 0xe1, 0xdb, 0xf7, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul rbx, -0x8241f
    ax_test![imul_rbx_0x8241f_cf_of; 0x48, 0x69, 0xdb, 0xe1, 0xdb, 0xf7, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x800000000008241fu64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // imul edx, ebx, 0x5
    ax_test![imul_edx_ebx_0x5; 0x6b, 0xd3, 0x5; |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x0);
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // imul edx, ebx, 0x5
    ax_test![imul_edx_ebx_0x5_cf_of; 0x6b, 0xd3, 0x5; |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x0);
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x7ffffffb);
            assert_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];
}
