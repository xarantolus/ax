use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Xor;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::registers::RegisterWrapper;

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

        let dest_val = match i.op1_kind() {
            OpKind::Memory => todo!("instr_xor_rm8_r8: Memory operands not implemented"),
            OpKind::Register => self.reg_read_8(RegisterWrapper::from(i.op0_register())),
            _ => panic!("Invalid op1_kind for XOR r/m8, r8"),
        };

        let src_val = self.reg_read_8(RegisterWrapper::from(i.op1_register()));

        let result = src_val ^ dest_val;

        match i.op1_kind() {
            OpKind::Memory => todo!("instr_xor_rm8_r8: Memory operands not implemented"),
            OpKind::Register => {
                self.reg_write_8(RegisterWrapper::from(i.op0_register()), result);
                Ok(())
            }
            _ => panic!("Invalid op1_kind for XOR r/m8, r8"),
        }
    }

    /// XOR r/m16, r16
    ///
    /// o16 31 /r
    fn instr_xor_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm16_r16);

        todo!("instr_xor_rm16_r16 for Xor")
    }

    /// XOR r/m32, r32
    ///
    /// o32 31 /r
    fn instr_xor_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm32_r32);

        todo!("instr_xor_rm32_r32 for Xor")
    }

    /// XOR r/m64, r64
    ///
    /// o64 31 /r
    fn instr_xor_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_rm64_r64);

        todo!("instr_xor_rm64_r64 for Xor")
    }

    /// XOR r8, r/m8
    ///
    /// 32 /r
    fn instr_xor_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r8_rm8);

        todo!("instr_xor_r8_rm8 for Xor")
    }

    /// XOR r16, r/m16
    ///
    /// o16 33 /r
    fn instr_xor_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r16_rm16);

        todo!("instr_xor_r16_rm16 for Xor")
    }

    /// XOR r32, r/m32
    ///
    /// o32 33 /r
    fn instr_xor_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r32_rm32);

        todo!("instr_xor_r32_rm32 for Xor")
    }

    /// XOR r64, r/m64
    ///
    /// o64 33 /r
    fn instr_xor_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Xor_r64_rm64);

        todo!("instr_xor_r64_rm64 for Xor")
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
        assert_reg_value!(a; AL; 0);
    }];

    // xor al, bl
    ax_test![xor_same_value; 0x30, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(a; AL; 0xf);
            write_reg_value!(a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(a; AL; 0);
        }
    ];

    // xor al, cl
    ax_test![xor_different_value; 0x30, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(a; AL; 0b1010);
            write_reg_value!(a; CL; 0b0101);
        };
        |a: Axecutor| {
            assert_reg_value!(a; AL; 0b1111);
        }
    ];
}
