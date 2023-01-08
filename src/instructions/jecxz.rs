use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jecxz;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::registers::SupportedRegister;
use crate::{fatal_error, opcode_unimplemented};

impl Axecutor {
    pub fn mnemonic_jecxz(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jecxz);

        match i.code() {
            Jecxz_rel8_16 => self.instr_jecxz_rel8_16(i),
            Jecxz_rel8_32 => self.instr_jecxz_rel8_32(i),
            Jecxz_rel8_64 => self.instr_jecxz_rel8_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jecxz", i.code()),
        }
    }

    /// JECXZ rel8
    ///
    /// a32 o16 E3 cb
    fn instr_jecxz_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jecxz_rel8_16);

        // This is not supported in 64-bit mode (according to GNU as), so ignored
        opcode_unimplemented!("instr_jecxz_rel8_16 for Jecxz")
    }

    /// JECXZ rel8
    ///
    /// a32 o32 E3 cb
    fn instr_jecxz_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jecxz_rel8_32);

        opcode_unimplemented!("instr_jecxz_rel8_32 for Jecxz")
    }

    /// JECXZ rel8
    ///
    /// a32 o64 E3 cb
    fn instr_jecxz_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jecxz_rel8_64);

        if self.reg_read_32(SupportedRegister::ECX)? == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(SupportedRegister::RIP, offset)?;

                    return Ok(());
                }
                _ => fatal_error!("Invalid op0_kind for JMP rel32: {:?}", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::jmp_test;

    jmp_test![mov_ecx_0_jecxz_end_mov_ecx_42_end_nop;
        start: 0x401010; end: 0x40101f;
        0xb9, 0x0, 0x0, 0x0, 0x0, 0x67, 0xe3, 0x6; // mov ecx, 0; jecxz .end
        1; // 1 bytes of 0x90 (nop) as padding
        0xb9, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov ecx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
