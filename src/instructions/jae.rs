use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jae;
use iced_x86::OpKind;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::state::flags::*;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::state::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_jae(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jae);

        match i.code() {
            Jae_rel8_16 => self.instr_jae_rel8_16(i),
            Jae_rel8_32 => self.instr_jae_rel8_32(i),
            Jae_rel8_64 => self.instr_jae_rel8_64(i),
            Jae_rel16 => self.instr_jae_rel16(i),
            Jae_rel32_32 => self.instr_jae_rel32_32(i),
            Jae_rel32_64 => self.instr_jae_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jae", i.code()),
        }
    }

    /// JAE rel8
    ///
    /// o16 73 cb
    fn instr_jae_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jae_rel8_16);

        opcode_unimplemented!("instr_jae_rel8_16 for Jae")
    }

    /// JAE rel8
    ///
    /// o32 73 cb
    fn instr_jae_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jae_rel8_32);

        opcode_unimplemented!("instr_jae_rel8_32 for Jae")
    }

    /// JAE rel8
    ///
    /// o64 73 cb
    fn instr_jae_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jae_rel8_64);

        if self.state.rflags & FLAG_CF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JAE rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JAE rel16
    ///
    /// o16 0F 83 cw
    fn instr_jae_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jae_rel16);

        opcode_unimplemented!("instr_jae_rel16 for Jae")
    }

    /// JAE rel32
    ///
    /// o32 0F 83 cd
    fn instr_jae_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jae_rel32_32);

        opcode_unimplemented!("instr_jae_rel32_32 for Jae")
    }

    /// JAE rel32
    ///
    /// o64 0F 83 cd
    fn instr_jae_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jae_rel32_64);

        if self.state.rflags & FLAG_CF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JAE rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{assert_reg_value, jmp_test};

    jmp_test![mov_rcx_5_lstart_sub_rcx_1_lcond_cmp_rcx_3_jae_lstart_cf_pf_sf;
        start: 0x401010; end: 0x401022;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xe9, 0x1; // mov rcx, 5; .Lstart: sub rcx, 1;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0x83, 0xf9, 0x3, 0x73, 0xf5; // .Lcond: cmp rcx, 3; JAE .Lstart
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 2);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_lstart_sub_rcx_1_lcond_cmp_rcx_3_jae_lstart_cf_pf_sf_32bit;
        start: 0x401010; end: 0x408555;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xe9, 0x1; // mov rcx, 5; .Lstart: sub rcx, 1;
        30000; // 30000 bytes of 0x90 (nop) as padding
        0x48, 0x83, 0xf9, 0x3, 0xf, 0x83, 0xc2, 0x8a, 0xff, 0xff; // .Lcond: cmp rcx, 3; JAE .Lstart
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 2);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];
    jmp_test![mov_rcx_5_cmp_rcx_5_jae_end_mov_rcx_42_end_nop_pf_zf;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0x73, 0x8; // mov rcx, 5; cmp rcx, 5; JAE .end;
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 5);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_cmp_rcx_5_jae_end_mov_rcx_42_end_nop_pf_zf_32bit;
        start: 0x401010; end: 0x4196c9;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf9, 0x5, 0xf, 0x83, 0xa7, 0x86, 0x1, 0x0; // mov rcx, 5; cmp rcx, 5; JAE .end;
        100000; // 100000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 5);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
