use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jrcxz;
use iced_x86::OpKind;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::state::registers::SupportedRegister;

impl Axecutor {
    pub fn mnemonic_jrcxz(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jrcxz);

        match i.code() {
            Jrcxz_rel8_16 => self.instr_jrcxz_rel8_16(i),
            Jrcxz_rel8_64 => self.instr_jrcxz_rel8_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jrcxz", i.code()),
        }
    }

    /// JRCXZ rel8
    ///
    /// a64 o16 E3 cb
    fn instr_jrcxz_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jrcxz_rel8_16);

        opcode_unimplemented!("instr_jrcxz_rel8_16 for Jrcxz")
    }

    /// JRCXZ rel8
    ///
    /// a64 o64 E3 cb
    fn instr_jrcxz_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jrcxz_rel8_64);

        if self.reg_read_64(SupportedRegister::RCX)? == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(SupportedRegister::RIP, offset)?;

                    return Ok(());
                }
                _ => fatal_error!("Invalid op0_kind for JMP rel32: {:?}", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{assert_reg_value, jmp_test};

    jmp_test![mov_rcx_0_jrcxz_end_mov_rcx_42_end_nop;
        start: 0x401010; end: 0x401022;
        0x48, 0xc7, 0xc1, 0x0, 0x0, 0x0, 0x0, 0xe3, 0x8; // mov rcx, 0; jrcxz .end
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc1, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rcx, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rax_0_mov_rcx_100_loop_jrcxz_end_add_rax_2_sub_rcx_1_jmp_loop_end_nop_pf_zf;
        start: 0x401010; end: 0x40102c;
        0x48, 0xc7, 0xc0, 0x0, 0x0, 0x0, 0x0, 0x48, 0xc7, 0xc1, 0x64, 0x0, 0x0, 0x0, 0xe3, 0xb; // mov rax, 0; mov rcx, 100; .loop: jrcxz .end
        1; // 1 bytes of 0x90 (nop) as padding
        0x48, 0x83, 0xc0, 0x2, 0x48, 0x83, 0xe9, 0x1, 0xeb, 0xf3, 0x90; // add rax, 2; sub rcx, 1; JMP .loop; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 200);
            assert_reg_value!(q; a; RCX; 0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
