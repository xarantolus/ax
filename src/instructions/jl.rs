use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jl;
use iced_x86::OpKind;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::state::flags::*;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::state::registers::SupportedRegister::*;

impl Axecutor {
    pub(crate) fn mnemonic_jl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jl);

        match i.code() {
            Jl_rel8_16 => self.instr_jl_rel8_16(i),
            Jl_rel8_32 => self.instr_jl_rel8_32(i),
            Jl_rel8_64 => self.instr_jl_rel8_64(i),
            Jl_rel16 => self.instr_jl_rel16(i),
            Jl_rel32_32 => self.instr_jl_rel32_32(i),
            Jl_rel32_64 => self.instr_jl_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jl", i.code()),
        }
    }

    /// JL rel8
    ///
    /// o16 7C cb
    fn instr_jl_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jl_rel8_16);

        opcode_unimplemented!("instr_jl_rel8_16 for Jl")
    }

    /// JL rel8
    ///
    /// o32 7C cb
    fn instr_jl_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jl_rel8_32);

        opcode_unimplemented!("instr_jl_rel8_32 for Jl")
    }

    /// JL rel8
    ///
    /// o64 7C cb
    fn instr_jl_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jl_rel8_64);

        if (self.state.rflags & FLAG_SF == 0) != (self.state.rflags & FLAG_OF == 0) {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JL rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JL rel16
    ///
    /// o16 0F 8C cw
    fn instr_jl_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jl_rel16);

        opcode_unimplemented!("instr_jl_rel16 for Jl")
    }

    /// JL rel32
    ///
    /// o32 0F 8C cd
    fn instr_jl_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jl_rel32_32);

        opcode_unimplemented!("instr_jl_rel32_32 for Jl")
    }

    /// JL rel32
    ///
    /// o64 0F 8C cd
    fn instr_jl_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jl_rel32_64);

        if (self.state.rflags & FLAG_SF == 0) != (self.state.rflags & FLAG_OF == 0) {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JL rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{assert_reg_value, jmp_test};

    jmp_test![mov_rcx_5_cmp_rcx_5_jl_end_mov_rcx_42_end_nop_pf_zf;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x7c, 0x8; // mov rcx, 5; cmp rcx, 5; JL .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 42);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_3_cmp_rcx_5_jl_end_mov_rcx_42_end_nop_cf_sf;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x3, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x7c, 0x8; // mov rcx, 3; cmp rcx, 5; JL .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 3);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_cmp_rcx_5_jl_end_mov_rcx_42_end_nop_pf_zf_32bit;
        start: 0x401010; end: 0x403739;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x8c, 0x17, 0x27, 0x0, 0x0; // mov rcx, 5; cmp rcx, 5; JL .end;
        10000; // 10000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 42);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_3_cmp_rcx_5_jl_end_mov_rcx_42_end_nop_cf_sf_32bit;
        start: 0x401010; end: 0x403739;
        0x48, 0xc7, 0xc1, 0x3, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x8c, 0x17, 0x27, 0x0, 0x0; // mov rcx, 3; cmp rcx, 5; JL .end;
        10000; // 10000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 3);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];
}
