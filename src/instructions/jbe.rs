use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jbe;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister::*;
use crate::{fatal_error, opcode_unimplemented};

impl Axecutor {
    pub fn mnemonic_jbe(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jbe);

        match i.code() {
            Jbe_rel8_16 => self.instr_jbe_rel8_16(i),
            Jbe_rel8_32 => self.instr_jbe_rel8_32(i),
            Jbe_rel8_64 => self.instr_jbe_rel8_64(i),
            Jbe_rel16 => self.instr_jbe_rel16(i),
            Jbe_rel32_32 => self.instr_jbe_rel32_32(i),
            Jbe_rel32_64 => self.instr_jbe_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jbe", i.code()),
        }
    }

    /// JBE rel8
    ///
    /// o16 76 cb
    fn instr_jbe_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jbe_rel8_16);

        opcode_unimplemented!("instr_jbe_rel8_16 for Jbe")
    }

    /// JBE rel8
    ///
    /// o32 76 cb
    fn instr_jbe_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jbe_rel8_32);

        opcode_unimplemented!("instr_jbe_rel8_32 for Jbe")
    }

    /// JBE rel8
    ///
    /// o64 76 cb
    fn instr_jbe_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jbe_rel8_64);

        if self.state.rflags & FLAG_CF != 0 || self.state.rflags & FLAG_ZF != 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JBE rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JBE rel16
    ///
    /// o16 0F 86 cw
    fn instr_jbe_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jbe_rel16);

        opcode_unimplemented!("instr_jbe_rel16 for Jbe")
    }

    /// JBE rel32
    ///
    /// o32 0F 86 cd
    fn instr_jbe_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jbe_rel32_32);

        opcode_unimplemented!("instr_jbe_rel32_32 for Jbe")
    }

    /// JBE rel32
    ///
    /// o64 0F 86 cd
    fn instr_jbe_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jbe_rel32_64);

        if self.state.rflags & FLAG_CF != 0 || self.state.rflags & FLAG_ZF != 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JBE rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::jmp_test;

    jmp_test![mov_rcx_5_cmp_rcx_5_jbe_end_mov_rcx_42_end_nop_pf_zf;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x76, 0x8; // mov rcx, 5; cmp rcx, 5; Jbe .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 5);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_cmp_rcx_5_jbe_end_mov_rcx_42_end_nop_pf_zf_32bit;
        start: 0x401010; end: 0x408b38;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x86, 0x16, 0x7b, 0x0, 0x0; // mov rcx, 5; cmp rcx, 5; Jbe .end;
        31503; // 31503 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 5);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_6_cmp_rcx_5_jbe_end_mov_rcx_42_end_nop;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x6, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x76, 0x8; // mov rcx, 6; cmp rcx, 5; Jbe .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
