use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jno;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister::*;
use crate::{fatal_error, opcode_unimplemented};

impl Axecutor {
    pub fn mnemonic_jno(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jno);

        match i.code() {
            Jno_rel8_16 => self.instr_jno_rel8_16(i),
            Jno_rel8_32 => self.instr_jno_rel8_32(i),
            Jno_rel8_64 => self.instr_jno_rel8_64(i),
            Jno_rel16 => self.instr_jno_rel16(i),
            Jno_rel32_32 => self.instr_jno_rel32_32(i),
            Jno_rel32_64 => self.instr_jno_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jno", i.code()),
        }
    }

    /// JNO rel8
    ///
    /// o16 71 cb
    fn instr_jno_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jno_rel8_16);

        opcode_unimplemented!("instr_jno_rel8_16 for Jno")
    }

    /// JNO rel8
    ///
    /// o32 71 cb
    fn instr_jno_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jno_rel8_32);

        opcode_unimplemented!("instr_jno_rel8_32 for Jno")
    }

    /// JNO rel8
    ///
    /// o64 71 cb
    fn instr_jno_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jno_rel8_64);

        if self.state.rflags & FLAG_OF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JNO rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JNO rel16
    ///
    /// o16 0F 81 cw
    fn instr_jno_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jno_rel16);

        opcode_unimplemented!("instr_jno_rel16 for Jno")
    }

    /// JNO rel32
    ///
    /// o32 0F 81 cd
    fn instr_jno_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jno_rel32_32);

        opcode_unimplemented!("instr_jno_rel32_32 for Jno")
    }

    /// JNO rel32
    ///
    /// o64 0F 81 cd
    fn instr_jno_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jno_rel32_64);

        if self.state.rflags & FLAG_OF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JNO rel32_64", i.op0_kind()),
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

    jmp_test![mov_al_0x7f_add_al_1_jno_end_mov_al_42_end_nop_sf_of;
        start: 0x401010; end: 0x401021;
        0xb0, 0x7f, 0x4, 0x1, 0x71, 0xa; // mov al, 0x7f; add al, 1; JNO .end
        8; // 8 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 42);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    jmp_test![mov_al_0x7f_add_al_1_jno_end_mov_al_42_end_nop_sf_of_32bit;
        start: 0x401010; end: 0x41489d;
        0xb0, 0x7f, 0x4, 0x1, 0xf, 0x81, 0x82, 0x38, 0x1, 0x0; // mov al, 0x7f; add al, 1; JNO .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 42);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    jmp_test![mov_al_0x7e_add_al_1_jno_end_mov_al_42_end_nop;
        start: 0x401010; end: 0x401021;
        0xb0, 0x7e, 0x4, 0x1, 0x71, 0xa; // mov al, 0x7e; add al, 1; JNO .end
        8; // 8 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_al_0x7e_add_al_1_jno_end_mov_al_42_end_nop_32bit;
        start: 0x401010; end: 0x41489d;
        0xb0, 0x7e, 0x4, 0x1, 0xf, 0x81, 0x82, 0x38, 0x1, 0x0; // mov al, 0x7e; add al, 1; JNO .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
