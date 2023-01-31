use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Mul;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::state::flags::*;

use crate::helpers::macros::fatal_error;

use crate::helpers::operand::Operand;
use crate::state::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_mul(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Mul);

        match i.code() {
            Mul_rm8 => self.instr_mul_rm8(i),
            Mul_rm16 => self.instr_mul_rm16(i),
            Mul_rm32 => self.instr_mul_rm32(i),
            Mul_rm64 => self.instr_mul_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Mul", i.code()),
        }
    }

    /// MUL r/m8
    ///
    /// F6 /4
    fn instr_mul_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mul_rm8);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_8(r)?,
            Operand::Memory(m) => self.mem_read_8(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Mul_rm8", op),
        };

        let dst_val = self.reg_read_8(AL)?;

        let result = (dst_val as u16).wrapping_mul(src_val as u16);

        let upper = (result >> 8) as u8;

        self.reg_write_16(AX, result as u64)?;

        self.set_flags_u8(
            if upper == 0 { 0 } else { FLAG_CF | FLAG_OF },
            if upper == 0 { FLAG_CF | FLAG_OF } else { 0 },
            0,
        );

        Ok(())
    }

    /// MUL r/m16
    ///
    /// o16 F7 /4
    fn instr_mul_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mul_rm16);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_16(r)?,
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Mul_rm16", op),
        };

        let dst_val = self.reg_read_16(AX)?;

        let result = (dst_val as u32).wrapping_mul(src_val as u32);

        let upper = (result >> 16) as u16;

        self.reg_write_16(AX, result as u16 as u64)?;
        self.reg_write_16(DX, upper as u64)?;

        self.set_flags_u8(
            if upper == 0 { 0 } else { FLAG_CF | FLAG_OF },
            if upper == 0 { FLAG_CF | FLAG_OF } else { 0 },
            0,
        );

        Ok(())
    }

    /// MUL r/m32
    ///
    /// o32 F7 /4
    fn instr_mul_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mul_rm32);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_32(r)?,
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Mul_rm32", op),
        };

        let dst_val = self.reg_read_32(EAX)?;

        let result = dst_val.wrapping_mul(src_val);

        let upper = (result >> 32) as u32;

        self.reg_write_32(EAX, result as u32 as u64)?;
        self.reg_write_32(EDX, upper as u64)?;

        self.set_flags_u8(
            if upper == 0 { 0 } else { FLAG_CF | FLAG_OF },
            if upper == 0 { FLAG_CF | FLAG_OF } else { 0 },
            0,
        );

        Ok(())
    }

    /// MUL r/m64
    ///
    /// o64 F7 /4
    fn instr_mul_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mul_rm64);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_64(r)?,
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Mul_rm64", op),
        };

        let dst_val = self.reg_read_64(RAX)?;

        let result = (dst_val as u128).wrapping_mul(src_val as u128);

        let upper = (result >> 64) as u64;

        self.reg_write_64(RAX, result as u64)?;
        self.reg_write_64(RDX, upper)?;

        self.set_flags_u8(
            if upper == 0 { 0 } else { FLAG_CF | FLAG_OF },
            if upper == 0 { FLAG_CF | FLAG_OF } else { 0 },
            0,
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_mem_value, assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // mul bl
    ax_test![mul_bl_ax_0; 0xf6, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // mul bl
    ax_test![mul_bl_cf_of_ax_63; 0xf6, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0x1b9);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // mul byte ptr [r11]
    ax_test![mul_byte_ptr_r11_ax_0; 0x41, 0xf6, 0x23;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // mul byte ptr [r11]
    ax_test![mul_byte_ptr_r11_cf_of_ax_63; 0x41, 0xf6, 0x23;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x3f);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1b9);
            assert_mem_value!(b; a; 0x1000; 0x7);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // mul bx
    ax_test![mul_bx_ax_0; 0x66, 0xf7, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // mul bx
    ax_test![mul_bx_cf_of_ax_32767; 0x66, 0xf7, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7);
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7);
            assert_reg_value!(w; a; AX; 0x7ff9);
            assert_reg_value!(w; a; DX; 0x3);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // mul word ptr [r11]
    ax_test![mul_word_ptr_r11_ax_0; 0x66, 0x41, 0xf7, 0x23;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; DX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // mul word ptr [r11]
    ax_test![mul_word_ptr_r11_cf_of_ax_32767; 0x66, 0x41, 0xf7, 0x23;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7ff9);
            assert_reg_value!(w; a; DX; 0x3);
            assert_mem_value!(w; a; 0x1000; 0x7);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // mul eax
    ax_test![mul_eax_cf_of_edx_0; 0xf7, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x1);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // mul eax
    ax_test![mul_eax_edx_0; 0xf7, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; EDX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // mul dword ptr [r11]
    ax_test![mul_dword_ptr_r11_cf_of_eax_2147483647; 0x41, 0xf7, 0x23;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            // 0x37ffffff9
            assert_reg_value!(d; a; EAX; 0x7ffffff9);
            assert_reg_value!(d; a; EDX; 0x3);
            assert_mem_value!(d; a; 0x1000; 0x7);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // mul dword ptr [r11]
    ax_test![mul_dword_ptr_r11_eax_0; 0x41, 0xf7, 0x23;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; EDX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // mul rdx
    ax_test![mul_rdx_cf_of_rax_9223372036854775807; 0x48, 0xf7, 0xe2; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x7);
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x3);
            assert_reg_value!(q; a; RAX; 0x7ffffffffffffff9u64);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // mul rdx
    ax_test![mul_rdx_rax_0; 0x48, 0xf7, 0xe2; |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];

    // mul qword ptr [r11]
    ax_test![mul_qword_ptr_r11_cf_of_rax_9223372036854775807; 0x49, 0xf7, 0x23;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7ffffffffffffff9u64);
            assert_mem_value!(q; a; 0x1000; 0x7);
        };
        (FLAG_CF | FLAG_OF; 0)
    ];

    // mul qword ptr [r11]
    ax_test![mul_qword_ptr_r11_rax_0; 0x49, 0xf7, 0x23;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_OF)
    ];
}
