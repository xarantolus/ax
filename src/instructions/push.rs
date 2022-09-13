use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Push;

use super::axecutor::Axecutor;
use super::errors::AxError;

impl Axecutor {
    fn mnemonic_push(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Push);

        match i.code() {
            Push_r16 => self.instr_push_r16(i),
            Push_r32 => self.instr_push_r32(i),
            Push_r64 => self.instr_push_r64(i),
            Push_imm16 => self.instr_push_imm16(i),
            Push_rm16 => self.instr_push_rm16(i),
            Push_rm32 => self.instr_push_rm32(i),
            Push_rm64 => self.instr_push_rm64(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Push", i.code()),
        }
    }

    /// PUSH r16
    ///
    /// o16 50+rw
    fn instr_push_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Push_r16);

        todo!("unimplemented")
    }

    /// PUSH r32
    ///
    /// o32 50+rd
    fn instr_push_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Push_r32);

        todo!("unimplemented")
    }

    /// PUSH r64
    ///
    /// o64 50+ro
    fn instr_push_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Push_r64);

        todo!("unimplemented")
    }

    /// PUSH imm16
    ///
    /// o16 68 iw
    fn instr_push_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Push_imm16);

        todo!("unimplemented")
    }

    /// PUSH r/m16
    ///
    /// o16 FF /6
    fn instr_push_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Push_rm16);

        todo!("unimplemented")
    }

    /// PUSH r/m32
    ///
    /// o32 FF /6
    fn instr_push_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Push_rm32);

        todo!("unimplemented")
    }

    /// PUSH r/m64
    ///
    /// o64 FF /6
    fn instr_push_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Push_rm64);

        todo!("unimplemented")
    }
}
