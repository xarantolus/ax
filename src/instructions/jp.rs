use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jp;
use iced_x86::OpKind;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::state::flags::*;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::state::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_jp(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jp);

        match i.code() {
            Jp_rel8_16 => self.instr_jp_rel8_16(i),
            Jp_rel8_32 => self.instr_jp_rel8_32(i),
            Jp_rel8_64 => self.instr_jp_rel8_64(i),
            Jp_rel16 => self.instr_jp_rel16(i),
            Jp_rel32_32 => self.instr_jp_rel32_32(i),
            Jp_rel32_64 => self.instr_jp_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jp", i.code()),
        }
    }

    /// JP rel8
    ///
    /// o16 7A cb
    fn instr_jp_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jp_rel8_16);

        opcode_unimplemented!("instr_jp_rel8_16 for Jp")
    }

    /// JP rel8
    ///
    /// o32 7A cb
    fn instr_jp_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jp_rel8_32);

        opcode_unimplemented!("instr_jp_rel8_32 for Jp")
    }

    /// JP rel8
    ///
    /// o64 7A cb
    fn instr_jp_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jp_rel8_64);

        if self.state.rflags & FLAG_PF != 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JP rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JP rel16
    ///
    /// o16 0F 8A cw
    fn instr_jp_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jp_rel16);

        opcode_unimplemented!("instr_jp_rel16 for Jp")
    }

    /// JP rel32
    ///
    /// o32 0F 8A cd
    fn instr_jp_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jp_rel32_32);

        opcode_unimplemented!("instr_jp_rel32_32 for Jp")
    }

    /// JP rel32
    ///
    /// o64 0F 8A cd
    fn instr_jp_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jp_rel32_64);

        if self.state.rflags & FLAG_PF != 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JP rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{assert_reg_value, jmp_test};

    jmp_test![mov_al_0x0_add_al_1_jp_end_mov_al_42_end_nop;
        start: 0x401010; end: 0x401021;
        0xb0, 0x0, 0x4, 0x1, 0x7a, 0xa; // mov al, 0x0; add al, 1; JP .end
        8; // 8 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_al_0x0_add_al_1_jp_end_mov_al_42_end_nop_32bit;
        start: 0x401010; end: 0x41489d;
        0xb0, 0x0, 0x4, 0x1, 0xf, 0x8a, 0x82, 0x38, 0x1, 0x0; // mov al, 0x0; add al, 1; JP .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_al_0x0_add_al_6_jp_end_mov_al_42_end_nop_pf;
        start: 0x401010; end: 0x401021;
        0xb0, 0x0, 0x4, 0x6, 0x7a, 0xa; // mov al, 0x0; add al, 6; JP .end
        8; // 8 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_al_0x0_add_al_6_jp_end_mov_al_42_end_nop_pf_32bit;
        start: 0x401010; end: 0x41489d;
        0xb0, 0x0, 0x4, 0x6, 0xf, 0x8a, 0x82, 0x38, 0x1, 0x0; // mov al, 0x0; add al, 6; JP .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
