use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jg;
use iced_x86::OpKind;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::state::flags::*;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::state::registers::SupportedRegister::*;

impl Axecutor {
    pub(crate) fn mnemonic_jg(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jg);

        match i.code() {
            Jg_rel8_16 => self.instr_jg_rel8_16(i),
            Jg_rel8_32 => self.instr_jg_rel8_32(i),
            Jg_rel8_64 => self.instr_jg_rel8_64(i),
            Jg_rel16 => self.instr_jg_rel16(i),
            Jg_rel32_32 => self.instr_jg_rel32_32(i),
            Jg_rel32_64 => self.instr_jg_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jg", i.code()),
        }
    }

    /// JG rel8
    ///
    /// o16 7F cb
    fn instr_jg_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jg_rel8_16);

        opcode_unimplemented!("instr_jg_rel8_16 for Jg")
    }

    /// JG rel8
    ///
    /// o32 7F cb
    fn instr_jg_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jg_rel8_32);

        opcode_unimplemented!("instr_jg_rel8_32 for Jg")
    }

    /// JG rel8
    ///
    /// o64 7F cb
    fn instr_jg_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jg_rel8_64);

        if self.state.rflags & FLAG_ZF == 0
            && (self.state.rflags & FLAG_SF == 0) == (self.state.rflags & FLAG_OF == 0)
        {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JG rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JG rel16
    ///
    /// o16 0F 8F cw
    fn instr_jg_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jg_rel16);

        opcode_unimplemented!("instr_jg_rel16 for Jg")
    }

    /// JG rel32
    ///
    /// o32 0F 8F cd
    fn instr_jg_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jg_rel32_32);

        opcode_unimplemented!("instr_jg_rel32_32 for Jg")
    }

    /// JG rel32
    ///
    /// o64 0F 8F cd
    fn instr_jg_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jg_rel32_64);

        if self.state.rflags & FLAG_ZF == 0
            && (self.state.rflags & FLAG_SF == 0) == (self.state.rflags & FLAG_OF == 0)
        {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JG rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{assert_reg_value, jmp_test};

    jmp_test![mov_rcx_6_cmp_rcx_5_jg_end_mov_rcx_42_end_nop;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x6, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x7f, 0x8; // mov rcx, 6; cmp rcx, 5; JG .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 6);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_6_cmp_rcx_5_jg_end_mov_rcx_42_end_nop_32bit;
        start: 0x401010; end: 0x412199;
        0x48, 0xc7, 0xc1, 0x6, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x8f, 0x77, 0x11, 0x1, 0x0; // mov rcx, 6; cmp rcx, 5; JG .end;
        70000; // 70000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 6);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_cmp_rcx_5_jg_end_mov_rcx_42_end_nop_pf_zf;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x7f, 0x8; // mov rcx, 5; cmp rcx, 5; JG .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 42);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_cmp_rcx_5_jg_end_mov_rcx_42_end_nop_pf_zf_32bit;
        start: 0x401010; end: 0x4197f5;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x8f, 0xd3, 0x87, 0x1, 0x0; // mov rcx, 5; cmp rcx, 5; JG .end;
        100300; // 100300 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 42);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
