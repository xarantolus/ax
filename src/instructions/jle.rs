use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jle;
use iced_x86::OpKind;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::state::flags::*;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::state::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_jle(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jle);

        match i.code() {
            Jle_rel8_16 => self.instr_jle_rel8_16(i),
            Jle_rel8_32 => self.instr_jle_rel8_32(i),
            Jle_rel8_64 => self.instr_jle_rel8_64(i),
            Jle_rel16 => self.instr_jle_rel16(i),
            Jle_rel32_32 => self.instr_jle_rel32_32(i),
            Jle_rel32_64 => self.instr_jle_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jle", i.code()),
        }
    }

    /// JLE rel8
    ///
    /// o16 7E cb
    fn instr_jle_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jle_rel8_16);

        opcode_unimplemented!("instr_jle_rel8_16 for Jle")
    }

    /// JLE rel8
    ///
    /// o32 7E cb
    fn instr_jle_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jle_rel8_32);

        opcode_unimplemented!("instr_jle_rel8_32 for Jle")
    }

    /// JLE rel8
    ///
    /// o64 7E cb
    fn instr_jle_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jle_rel8_64);

        if (self.state.rflags & FLAG_ZF != 0)
            || ((self.state.rflags & FLAG_SF == 0) != (self.state.rflags & FLAG_OF == 0))
        {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JLE rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JLE rel16
    ///
    /// o16 0F 8E cw
    fn instr_jle_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jle_rel16);

        opcode_unimplemented!("instr_jle_rel16 for Jle")
    }

    /// JLE rel32
    ///
    /// o32 0F 8E cd
    fn instr_jle_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jle_rel32_32);

        opcode_unimplemented!("instr_jle_rel32_32 for Jle")
    }

    /// JLE rel32
    ///
    /// o64 0F 8E cd
    fn instr_jle_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jle_rel32_64);

        if (self.state.rflags & FLAG_ZF != 0)
            || ((self.state.rflags & FLAG_SF == 0) != (self.state.rflags & FLAG_OF == 0))
        {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JLE rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{assert_reg_value, jmp_test};

    jmp_test![mov_rcx_4_cmp_rcx_5_jle_end_mov_rcx_42_end_nop_cf_pf_sf;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x4, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x7e, 0x8; // mov rcx, 4; cmp rcx, 5; JLE .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 4);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_rcx_4_cmp_rcx_5_jle_end_mov_rcx_42_end_nop_cf_pf_sf_32;
        start: 0x401010; end: 0x403739;
        0x48, 0xc7, 0xc1, 0x4, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x8e, 0x17, 0x27, 0x0, 0x0; // mov rcx, 4; cmp rcx, 5; JLE .end;
        10000; // 10000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; //  mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 4);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_cmp_rcx_5_jle_end_mov_rcx_42_end_nop_pf_zf;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x7e, 0x8; // mov rcx, 5; cmp rcx, 5; JLE .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 5);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_cmp_rcx_5_jle_end_mov_rcx_42_end_nop_pf_zf_32;
        start: 0x401010; end: 0x4196c9;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x8e, 0xa7, 0x86, 0x1, 0x0; // mov rcx, 5; cmp rcx, 5; JLE .end;
        100000; // 100000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; //  mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 5);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_6_cmp_rcx_5_jle_end_mov_rcx_42_end_nop;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x6, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x7e, 0x8; // mov rcx, 6; cmp rcx, 5; JLE .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_6_cmp_rcx_5_jle_end_mov_rcx_42_end_nop_32;
        start: 0x401010; end: 0x4196c9;
        0x48, 0xc7, 0xc1, 0x6, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x8e, 0xa7, 0x86, 0x1, 0x0; // mov rcx, 6; cmp rcx, 5; JLE .end;
        100000; // 100000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; //  mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
