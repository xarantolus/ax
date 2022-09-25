use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jnp;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_jnp(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jnp);

        match i.code() {
            Jnp_rel8_16 => self.instr_jnp_rel8_16(i),
            Jnp_rel8_32 => self.instr_jnp_rel8_32(i),
            Jnp_rel8_64 => self.instr_jnp_rel8_64(i),
            Jnp_rel16 => self.instr_jnp_rel16(i),
            Jnp_rel32_32 => self.instr_jnp_rel32_32(i),
            Jnp_rel32_64 => self.instr_jnp_rel32_64(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Jnp", i.code()),
        }
    }

    /// JNP rel8
    ///
    /// o16 7B cb
    fn instr_jnp_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jnp_rel8_16);

        todo!("instr_jnp_rel8_16 for Jnp")
    }

    /// JNP rel8
    ///
    /// o32 7B cb
    fn instr_jnp_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jnp_rel8_32);

        todo!("instr_jnp_rel8_32 for Jnp")
    }

    /// JNP rel8
    ///
    /// o64 7B cb
    fn instr_jnp_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jnp_rel8_64);

        if self.state.rflags & FLAG_PF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => panic!("Invalid op0_kind {:?} for JNP rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JNP rel16
    ///
    /// o16 0F 8B cw
    fn instr_jnp_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jnp_rel16);

        todo!("instr_jnp_rel16 for Jnp")
    }

    /// JNP rel32
    ///
    /// o32 0F 8B cd
    fn instr_jnp_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jnp_rel32_32);

        todo!("instr_jnp_rel32_32 for Jnp")
    }

    /// JNP rel32
    ///
    /// o64 0F 8B cd
    fn instr_jnp_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jnp_rel32_64);

        if self.state.rflags & FLAG_PF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => panic!("Invalid op0_kind {:?} for JNP rel32_64", i.op0_kind()),
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

    jmp_test![mov_rax_2_cmp_rax_1_jnp_end_mov_rax_42_end_nop;
        start: 0x401010; end: 0x40102d;
        0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x1, 0x7b, 0xf; // mov rax, 2; cmp rax, 1; JNP .end
        8; // 8 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rax_2_cmp_rax_1_jnp_end_mov_rax_42_end_nop_32bit;
        start: 0x401010; end: 0x4148a9;
        0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x1, 0xf, 0x8b, 0x87, 0x38, 0x1, 0x0; // mov rax, 2; cmp rax, 1; JNP .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rax_2_cmp_rax_3_jnp_end_mov_rax_42_end_nop_cf_pf_sf;
        start: 0x401010; end: 0x40102d;
        0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x3, 0x7b, 0xf; // mov rax, 2; cmp rax, 3; JNP .end
        8; // 8 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_rax_2_cmp_rax_3_jnp_end_mov_rax_42_end_nop_cf_pf_sf_32bit;
        start: 0x401010; end: 0x4148a9;
        0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x3, 0xf, 0x8b, 0x87, 0x38, 0x1, 0x0; // mov rax, 2; cmp rax, 3; JNP .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];
}
