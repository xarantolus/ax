use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jge;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_jge(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jge);

        match i.code() {
            Jge_rel8_16 => self.instr_jge_rel8_16(i),
            Jge_rel8_32 => self.instr_jge_rel8_32(i),
            Jge_rel8_64 => self.instr_jge_rel8_64(i),
            Jge_rel16 => self.instr_jge_rel16(i),
            Jge_rel32_32 => self.instr_jge_rel32_32(i),
            Jge_rel32_64 => self.instr_jge_rel32_64(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Jge", i.code()),
        }
    }

    /// JGE rel8
    ///
    /// o16 7D cb
    fn instr_jge_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jge_rel8_16);

        todo!("instr_jge_rel8_16 for Jge")
    }

    /// JGE rel8
    ///
    /// o32 7D cb
    fn instr_jge_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jge_rel8_32);

        todo!("instr_jge_rel8_32 for Jge")
    }

    /// JGE rel8
    ///
    /// o64 7D cb
    fn instr_jge_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jge_rel8_64);

        if (self.state.rflags & FLAG_SF == 0) == (self.state.rflags & FLAG_OF == 0) {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => panic!("Invalid op0_kind {:?} for JGE rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JGE rel16
    ///
    /// o16 0F 8D cw
    fn instr_jge_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jge_rel16);

        todo!("instr_jge_rel16 for Jge")
    }

    /// JGE rel32
    ///
    /// o32 0F 8D cd
    fn instr_jge_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jge_rel32_32);

        todo!("instr_jge_rel32_32 for Jge")
    }

    /// JGE rel32
    ///
    /// o64 0F 8D cd
    fn instr_jge_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jge_rel32_64);

        if (self.state.rflags & FLAG_SF == 0) == (self.state.rflags & FLAG_OF == 0) {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => panic!("Invalid op0_kind {:?} for JGE rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_reg_value, instructions::registers::SupportedRegister, jmp_test};
    use iced_x86::Register::*;

    jmp_test![mov_rcx_5_cmp_rcx_5_jge_end_mov_rcx_42_end_nop_pf_zf;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x7d, 0x8; // mov rcx, 5; cmp rcx, 5; JGE .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 5);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_cmp_rcx_5_jge_end_mov_rcx_42_end_nop_pf_zf_32bit;
        start: 0x401010; end: 0x4197f5;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x8d, 0xd3, 0x87, 0x1, 0x0; // mov rcx, 5; cmp rcx, 5; JGE .end;
        100300; // 100300 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 5);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
