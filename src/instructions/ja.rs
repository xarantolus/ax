use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Ja;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_ja(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Ja);

        match i.code() {
            Ja_rel8_16 => self.instr_ja_rel8_16(i),
            Ja_rel8_32 => self.instr_ja_rel8_32(i),
            Ja_rel8_64 => self.instr_ja_rel8_64(i),
            Ja_rel16 => self.instr_ja_rel16(i),
            Ja_rel32_32 => self.instr_ja_rel32_32(i),
            Ja_rel32_64 => self.instr_ja_rel32_64(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Ja", i.code()),
        }
    }

    /// JA rel8
    ///
    /// o16 77 cb
    fn instr_ja_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Ja_rel8_16);

        todo!("instr_ja_rel8_16 for Ja")
    }

    /// JA rel8
    ///
    /// o32 77 cb
    fn instr_ja_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Ja_rel8_32);

        todo!("instr_ja_rel8_32 for Ja")
    }

    /// JA rel8
    ///
    /// o64 77 cb
    fn instr_ja_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Ja_rel8_64);

        if self.state.rflags & FLAG_CF == 0 && self.state.rflags & FLAG_ZF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => panic!("Invalid op0_kind {:?} for JA rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JA rel16
    ///
    /// o16 0F 87 cw
    fn instr_ja_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Ja_rel16);

        todo!("instr_ja_rel16 for Ja")
    }

    /// JA rel32
    ///
    /// o32 0F 87 cd
    fn instr_ja_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Ja_rel32_32);

        todo!("instr_ja_rel32_32 for Ja")
    }

    /// JA rel32
    ///
    /// o64 0F 87 cd
    fn instr_ja_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Ja_rel32_64);

        if self.state.rflags & FLAG_CF == 0 && self.state.rflags & FLAG_ZF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => panic!("Invalid op0_kind {:?} for JA rel32_64", i.op0_kind()),
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

    jmp_test![mov_rax_4_cmp_rax_3_ja_end_mov_rax_42_end_nop;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc0, 0x4, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x3, 0x77, 0x8; // mov rax, 4; cmp rax, 3; ja .end
        1; // 1 byte of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rax_2_cmp_rax_3_ja_end_mov_rax_42_end_nop_cf_pf_sf;
        start: 0x401010; end: 0x401026;
        0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x3, 0x77, 0x8; // mov rax, 2; cmp rax, 3; ja .end
        1; // 1 byte of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_rax_2_cmp_rax_3_ja_end_mov_rax_42_end_nop_cf_pf_sf_32bit;
        start: 0x401010; end: 0x5d9a96;
        0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x3, 0xf, 0x87, 0x74, 0x8a, 0x1d, 0x0; // mov rax, 2; cmp rax, 3; ja .end
        1935981; // 1935981 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_rax_4_cmp_rax_3_ja_end_mov_rax_42_end_nop_32bit;
        start: 0x401010; end: 0x5d9a96;
        0x48, 0xc7, 0xc0, 0x4, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x3, 0xf, 0x87, 0x74, 0x8a, 0x1d, 0x0; // mov rax, 4; cmp rax, 3; ja .end
        1935981; // 1935981 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_lstart_sub_rcx_1_lcond_cmp_rcx_3_ja_lstart_pf_zf;
        start: 0x401010; end: 0x401024;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xe9, 0x1; // mov rcx, 5; .Lstart: sub rcx, 1;
        3; // 3 bytes of 0x90 (nop) as padding
        0x48, 0x83, 0xf9, 0x3, 0x77, 0xf3; // .Lcond: cmp rcx, 3; JA .Lstart
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 3);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rcx_5_lstart_sub_rcx_1_lcond_cmp_rcx_3_ja_lstart_pf_zf_32bit;
        start: 0x401010; end: 0x408555;
        0x48, 0xc7, 0xc1, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xe9, 0x1; // mov rcx, 5; .Lstart: sub rcx, 1;
        30000; // 30000 bytes of 0x90 (nop) as padding
        0x48, 0x83, 0xf9, 0x3, 0xf, 0x87, 0xc2, 0x8a, 0xff, 0xff; // .Lcond: cmp rcx, 3; JA .Lstart
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 3);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
