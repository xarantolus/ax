use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Test;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::operand::Operand;
use crate::instructions::registers::RegisterWrapper;
use crate::{calculate_r_rm, calculate_rm_imm, calculate_rm_r};

impl Axecutor {
    pub fn mnemonic_test(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Test);

        match i.code() {
            Test_rm8_r8 => self.instr_test_rm8_r8(i),
            Test_rm16_r16 => self.instr_test_rm16_r16(i),
            Test_rm32_r32 => self.instr_test_rm32_r32(i),
            Test_rm64_r64 => self.instr_test_rm64_r64(i),
            Test_AL_imm8 => self.instr_test_al_imm8(i),
            Test_AX_imm16 => self.instr_test_ax_imm16(i),
            Test_EAX_imm32 => self.instr_test_eax_imm32(i),
            Test_RAX_imm32 => self.instr_test_rax_imm32(i),
            Test_rm8_imm8 => self.instr_test_rm8_imm8(i),
            Test_rm16_imm16 => self.instr_test_rm16_imm16(i),
            Test_rm32_imm32 => self.instr_test_rm32_imm32(i),
            Test_rm64_imm32 => self.instr_test_rm64_imm32(i),
            Test_rm8_imm8_F6r1 => self.instr_test_rm8_imm8_f6r1(i),
            Test_rm16_imm16_F7r1 => self.instr_test_rm16_imm16_f7r1(i),
            Test_rm32_imm32_F7r1 => self.instr_test_rm32_imm32_f7r1(i),
            Test_rm64_imm32_F7r1 => self.instr_test_rm64_imm32_f7r1(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Test", i.code()),
        }
    }

    /// TEST r/m8, r8
    ///
    /// 84 /r
    fn instr_test_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;

        let src_val = match src {
            Operand::Register(r) => self.reg_read_8(r),
            _ => panic!("Invalid source operand {:?} for TEST r/m8, r8", src),
        };

        let dest_val = match dest {
            Operand::Register(r) => self.reg_read_8(r),
            Operand::Memory(m) => self.mem_read_8(self.mem_addr(m))?,
            _ => panic!("Invalid destination operand {:?} for TEST r/m8, r8", dest),
        };

        let result = dest_val & src_val;

        self.set_flags_u8(FLAG_SF | FLAG_ZF | FLAG_PF, FLAG_OF | FLAG_CF, result);

        Ok(())
    }

    /// TEST r/m16, r16
    ///
    /// o16 85 /r
    fn instr_test_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_rm16_r16);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_val = match src {
            Operand::Register(r) => self.reg_read_16(r),
            _ => panic!("Invalid source operand {:?} for TEST r/m16, r16", src),
        };

        let dest_val = match dest {
            Operand::Register(r) => self.reg_read_16(r),
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => panic!("Invalid destination operand {:?} for TEST r/m16, r16", dest),
        };

        let result = dest_val & src_val;

        self.set_flags_u16(FLAG_SF | FLAG_ZF | FLAG_PF, FLAG_OF | FLAG_CF, result);

        Ok(())
    }

    /// TEST r/m32, r32
    ///
    /// o32 85 /r
    fn instr_test_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_rm32_r32);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_val = match src {
            Operand::Register(r) => self.reg_read_32(r),
            _ => panic!("Invalid source operand {:?} for TEST r/m32, r32", src),
        };

        let dest_val = match dest {
            Operand::Register(r) => self.reg_read_32(r),
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => panic!("Invalid destination operand {:?} for TEST r/m32, r32", dest),
        };

        let result = dest_val & src_val;

        self.set_flags_u32(FLAG_SF | FLAG_ZF | FLAG_PF, FLAG_OF | FLAG_CF, result);

        Ok(())
    }

    /// TEST r/m64, r64
    ///
    /// o64 85 /r
    fn instr_test_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_rm64_r64);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_val = match src {
            Operand::Register(r) => self.reg_read_64(r),
            _ => panic!("Invalid source operand {:?} for TEST r/m64, r64", src),
        };

        let dest_val = match dest {
            Operand::Register(r) => self.reg_read_64(r),
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            _ => panic!("Invalid destination operand {:?} for TEST r/m64, r64", dest),
        };

        let result = dest_val & src_val;

        self.set_flags_u64(FLAG_SF | FLAG_ZF | FLAG_PF, FLAG_OF | FLAG_CF, result);

        Ok(())
    }

    /// TEST AL, imm8
    ///
    /// A8 ib
    fn instr_test_al_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_AL_imm8);

        self.instr_test_rm8_imm8(i)
    }

    /// TEST AX, imm16
    ///
    /// o16 A9 iw
    fn instr_test_ax_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_AX_imm16);

        self.instr_test_rm16_imm16(i)
    }

    /// TEST EAX, imm32
    ///
    /// o32 A9 id
    fn instr_test_eax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_EAX_imm32);

        self.instr_test_rm32_imm32(i)
    }

    /// TEST RAX, imm32
    ///
    /// o64 A9 id
    fn instr_test_rax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_RAX_imm32);

        self.instr_test_rm64_imm32(i)
    }

    /// TEST r/m8, imm8
    ///
    /// F6 /0 ib
    fn instr_test_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;

        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(size, 1);

                data as u8
            }
            _ => panic!("Invalid source operand {:?} for TEST r/m8, imm8", src),
        };

        let dest_val = match dest {
            Operand::Register(r) => self.reg_read_8(r),
            Operand::Memory(m) => self.mem_read_8(self.mem_addr(m))?,
            _ => panic!("Invalid destination operand {:?} for TEST r/m8, imm8", dest),
        };

        let result = dest_val & src_val;

        self.set_flags_u8(FLAG_SF | FLAG_ZF | FLAG_PF, FLAG_OF | FLAG_CF, result);

        Ok(())
    }

    /// TEST r/m8, imm8
    ///
    /// F6 /1 ib
    fn instr_test_rm8_imm8_f6r1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_rm8_imm8_F6r1);

        // This opcode doesn't seem to be mentioned in the Intel manual
        todo!("instr_test_rm8_imm8_f6r1 for Test")
    }

    /// TEST r/m16, imm16
    ///
    /// o16 F7 /0 iw
    fn instr_test_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;

        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(size, 2);

                data as u16
            }
            _ => panic!("Invalid source operand {:?} for TEST r/m16, imm16", src),
        };

        let dest_val = match dest {
            Operand::Register(r) => self.reg_read_16(r),
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => panic!(
                "Invalid destination operand {:?} for TEST r/m16, imm16",
                dest
            ),
        };

        let result = dest_val & src_val;

        self.set_flags_u16(FLAG_SF | FLAG_ZF | FLAG_PF, FLAG_OF | FLAG_CF, result);

        Ok(())
    }

    /// TEST r/m32, imm32
    ///
    /// o32 F7 /0 id
    fn instr_test_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;

        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(size, 4);

                data as u32
            }
            _ => panic!("Invalid source operand {:?} for TEST r/m32, imm32", src),
        };

        let dest_val = match dest {
            Operand::Register(r) => self.reg_read_32(r),
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => panic!(
                "Invalid destination operand {:?} for TEST r/m32, imm32",
                dest
            ),
        };

        let result = dest_val & src_val;

        self.set_flags_u32(FLAG_SF | FLAG_ZF | FLAG_PF, FLAG_OF | FLAG_CF, result);

        Ok(())
    }

    /// TEST r/m64, imm32
    ///
    /// o64 F7 /0 id
    fn instr_test_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;

        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(size, 8);

                data
            }
            _ => panic!("Invalid source operand {:?} for TEST r/m64, imm32", src),
        };

        let dest_val = match dest {
            Operand::Register(r) => self.reg_read_64(r),
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            _ => panic!(
                "Invalid destination operand {:?} for TEST r/m64, imm32",
                dest
            ),
        };

        let result = dest_val & src_val;

        self.set_flags_u64(FLAG_SF | FLAG_ZF | FLAG_PF, FLAG_OF | FLAG_CF, result);

        Ok(())
    }

    /// TEST r/m16, imm16
    ///
    /// o16 F7 /1 iw
    fn instr_test_rm16_imm16_f7r1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_rm16_imm16_F7r1);

        todo!("instr_test_rm16_imm16_f7r1 for Test")
    }

    /// TEST r/m32, imm32
    ///
    /// o32 F7 /1 id
    fn instr_test_rm32_imm32_f7r1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_rm32_imm32_F7r1);

        todo!("instr_test_rm32_imm32_f7r1 for Test")
    }

    /// TEST r/m64, imm32
    ///
    /// o64 F7 /1 id
    fn instr_test_rm64_imm32_f7r1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Test_rm64_imm32_F7r1);

        todo!("instr_test_rm64_imm32_f7r1 for Test")
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::RegisterWrapper, write_reg_value,
    };
    use iced_x86::Register::*;

    // test al, 0x7f
    ax_test![test_al_0x7f_pf_zf; 0xa8, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test al, 0x7f
    ax_test![test_al_0x7f; 0xa8, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test al, 0x7f
    ax_test![test_al_0x7f_pf; 0xa8, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test bl, 0x7f
    ax_test![test_bl_0x7f_pf_zf; 0xf6, 0xc3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test bl, 0x7f
    ax_test![test_bl_0x7f; 0xf6, 0xc3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test bl, 0x7f
    ax_test![test_bl_0x7f_pf; 0xf6, 0xc3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test bl, 0x0
    ax_test![test_bl_0x0_pf_zf; 0xf6, 0xc3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test byte ptr [rbx], r11b
    ax_test![test_byte_ptr_rbx_r11b_pf_zf; 0x44, 0x84, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; R11L; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; R11L; 0x0);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test byte ptr [rbx], r11b
    ax_test![test_byte_ptr_rbx_r11b; 0x44, 0x84, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; R11L; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; R11L; 0x1);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test byte ptr [rbx], r11b
    ax_test![test_byte_ptr_rbx_r11b_pf; 0x44, 0x84, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; R11L; 0xf);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; R11L; 0xf);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test byte ptr [rbx], r11b
    ax_test![test_byte_ptr_rbx_r11b_sf; 0x44, 0x84, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; R11L; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; R11L; 0x80);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // test byte ptr [rbx], r11b
    ax_test![test_byte_ptr_rbx_r11b_pf_sf; 0x44, 0x84, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; R11L; 0xff);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; R11L; 0xff);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // test bx, 0x5
    ax_test![test_bx_0x5_pf_zf; 0x66, 0xf7, 0xc3, 0x5, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test bx, 0x5
    ax_test![test_bx_0x5; 0x66, 0xf7, 0xc3, 0x5, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test bx, 0x5
    ax_test![test_bx_0x5_pf; 0x66, 0xf7, 0xc3, 0x5, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test r11d, 0x5
    ax_test![test_r11d_0x5_pf_zf; 0x41, 0xf7, 0xc3, 0x5, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; R11D; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; R11D; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test r11d, 0x5
    ax_test![test_r11d_0x5; 0x41, 0xf7, 0xc3, 0x5, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; R11D; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; R11D; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test r11d, 0x5
    ax_test![test_r11d_0x5_pf; 0x41, 0xf7, 0xc3, 0x5, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; R11D; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; R11D; 0x7);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, r11
    ax_test![test_rbx_r11_pf_zf; 0x4c, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; R11; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
            assert_reg_value!(q; a; R11; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, r11
    ax_test![test_rbx_r11; 0x4c, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
            write_reg_value!(q; a; R11; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1);
            assert_reg_value!(q; a; R11; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, r11
    ax_test![test_rbx_r11_pf; 0x4c, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xf);
            write_reg_value!(q; a; R11; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xf);
            assert_reg_value!(q; a; R11; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, r11
    ax_test![test_rbx_r11_pf_sf; 0x4c, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
            assert_reg_value!(q; a; R11; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // test rbx, r11
    ax_test![test_rbx_r11_sf; 0x4c, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xe1ad15757e111c98u64);
            write_reg_value!(q; a; R11; 0x92b4bf2fd1c0b312u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xe1ad15757e111c98u64);
            assert_reg_value!(q; a; R11; 0x92b4bf2fd1c0b312u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // test rbx, 0x7fffffff
    ax_test![test_rbx_0x7fffffff_pf_zf; 0x48, 0xf7, 0xc3, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, 0x7fffffff
    ax_test![test_rbx_0x7fffffff; 0x48, 0xf7, 0xc3, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, 0x7fffffff
    ax_test![test_rbx_0x7fffffff_pf; 0x48, 0xf7, 0xc3, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, 0x7f
    ax_test![test_rbx_0x7f_pf_zf; 0x48, 0xf7, 0xc3, 0x7f, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, 0x7f
    ax_test![test_rbx_0x7f; 0x48, 0xf7, 0xc3, 0x7f, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, 0x7f
    ax_test![test_rbx_0x7f_pf; 0x48, 0xf7, 0xc3, 0x7f, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, rbx
    ax_test![test_rbx_rbx_pf_zf; 0x48, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, rbx
    ax_test![test_rbx_rbx; 0x48, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; RBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1);
            assert_reg_value!(q; a; RBX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, rbx
    ax_test![test_rbx_rbx_pf; 0x48, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; RBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xf);
            assert_reg_value!(q; a; RBX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rbx, rbx
    ax_test![test_rbx_rbx_pf_sf; 0x48, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // test rbx, rbx
    ax_test![test_rbx_rbx_sf; 0x48, 0x85, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7);
            write_reg_value!(q; a; RBX; 0xbcb1679b2fdbb851u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xbcb1679b2fdbb851u64);
            assert_reg_value!(q; a; RBX; 0xbcb1679b2fdbb851u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // test al, al
    ax_test![test_al_al_pf_zf; 0x84, 0xc0; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test al, al
    ax_test![test_al_al; 0x84, 0xc0; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test al, al
    ax_test![test_al_al_pf; 0x84, 0xc0; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; AL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_reg_value!(b; a; AL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test al, al
    ax_test![test_al_al_sf; 0x84, 0xc0; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // test al, al
    ax_test![test_al_al_pf_sf; 0x84, 0xc0; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // test ax, cx
    ax_test![test_ax_cx_pf_zf; 0x66, 0x85, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(w; a; CX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(w; a; CX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test ax, cx
    ax_test![test_ax_cx; 0x66, 0x85, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(w; a; CX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
            assert_reg_value!(w; a; CX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test ax, cx
    ax_test![test_ax_cx_pf; 0x66, 0x85, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xf);
            write_reg_value!(w; a; CX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf);
            assert_reg_value!(w; a; CX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test ax, cx
    ax_test![test_ax_cx_pf_sf; 0x66, 0x85, 0xc8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(w; a; CX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_reg_value!(w; a; CX; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // test eax, r12d
    ax_test![test_eax_r12d_pf_zf; 0x44, 0x85, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(d; a; R12D; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(d; a; R12D; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test eax, r12d
    ax_test![test_eax_r12d; 0x44, 0x85, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(d; a; R12D; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_reg_value!(d; a; R12D; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test eax, r12d
    ax_test![test_eax_r12d_pf; 0x44, 0x85, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xf);
            write_reg_value!(d; a; R12D; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf);
            assert_reg_value!(d; a; R12D; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test eax, r12d
    ax_test![test_eax_r12d_pf_sf; 0x44, 0x85, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(d; a; R12D; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_reg_value!(d; a; R12D; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // test ax, 0x1234
    ax_test![test_ax_0x1234_pf_zf; 0x66, 0xa9, 0x34, 0x12;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test ax, 0x1234
    ax_test![test_ax_0x1234; 0x66, 0xa9, 0x34, 0x12;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test ax, 0x1234
    ax_test![test_ax_0x1234_pf; 0x66, 0xa9, 0x34, 0x12;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1f);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test eax, 0x1234
    ax_test![test_eax_0x1234_pf_zf; 0xa9, 0x34, 0x12, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test eax, 0x1234
    ax_test![test_eax_0x1234; 0xa9, 0x34, 0x12, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test eax, 0x1234
    ax_test![test_eax_0x1234_pf; 0xa9, 0x34, 0x12, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1f);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    // test rax, 0x1234
    ax_test![test_rax_0x1234_pf_zf; 0x48, 0xa9, 0x34, 0x12, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // test rax, 0x1234
    ax_test![test_rax_0x1234; 0x48, 0xa9, 0x34, 0x12, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // test rax, 0x1234
    ax_test![test_rax_0x1234_pf; 0x48, 0xa9, 0x34, 0x12, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1f);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
