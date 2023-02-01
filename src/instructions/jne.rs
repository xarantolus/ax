use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jne;
use iced_x86::OpKind;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::state::flags::*;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::state::registers::SupportedRegister::*;

impl Axecutor {
    pub(crate) fn mnemonic_jne(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jne);

        match i.code() {
            Jne_rel8_16 => self.instr_jne_rel8_16(i),
            Jne_rel8_32 => self.instr_jne_rel8_32(i),
            Jne_rel8_64 => self.instr_jne_rel8_64(i),
            Jne_rel16 => self.instr_jne_rel16(i),
            Jne_rel32_32 => self.instr_jne_rel32_32(i),
            Jne_rel32_64 => self.instr_jne_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jne", i.code()),
        }
    }

    /// JNE rel8
    ///
    /// o16 75 cb
    fn instr_jne_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jne_rel8_16);

        opcode_unimplemented!("instr_jne_rel8_16 for Jne")
    }

    /// JNE rel8
    ///
    /// o32 75 cb
    fn instr_jne_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jne_rel8_32);

        opcode_unimplemented!("instr_jne_rel8_32 for Jne")
    }

    /// JNE rel8
    ///
    /// o64 75 cb
    fn instr_jne_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jne_rel8_64);

        if self.state.rflags & FLAG_ZF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JNE rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JNE rel16
    ///
    /// o16 0F 85 cw
    fn instr_jne_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jne_rel16);

        opcode_unimplemented!("instr_jne_rel16 for Jne")
    }

    /// JNE rel32
    ///
    /// o32 0F 85 cd
    fn instr_jne_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jne_rel32_32);

        opcode_unimplemented!("instr_jne_rel32_32 for Jne")
    }

    /// JNE rel32
    ///
    /// o64 0F 85 cd
    fn instr_jne_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jne_rel32_64);

        if self.state.rflags & FLAG_ZF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.trace_jump(i, offset)?;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JNE rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{assert_reg_value, jmp_test};

    jmp_test![mov_rax_1_cmp_rax_1_jne_end_mov_rax_42_end_nop_pf_zf;
        start: 0x401010; end: 0x40102d;
        0x48, 0xc7, 0xc0, 0x1, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x1, 0x75, 0xf; // mov rax, 1; cmp rax, 1; JNE .end
        8; // 8 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
    jmp_test![mov_rax_2_cmp_rax_1_jne_end_mov_rax_42_end_nop;
        start: 0x401010; end: 0x40102d;
        0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x1, 0x75, 0xf; // mov rax, 2; cmp rax, 1; JNE .end
        8; // 8 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rax_1_cmp_rax_1_jne_end_mov_rax_42_end_nop_pf_zf_32bit;
        start: 0x401010; end: 0x4156a8;
        0x48, 0xc7, 0xc0, 0x1, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x1, 0xf, 0x85, 0x86, 0x46, 0x1, 0x0; // mov rax, 1; cmp rax, 1; JNE .end
        83583; // 83583 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rax_2_cmp_rax_1_jne_end_mov_rax_42_end_nop_32bit;
        start: 0x401010; end: 0x4156a8;
        0x48, 0xc7, 0xc0, 0x2, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x1, 0xf, 0x85, 0x86, 0x46, 0x1, 0x0; // mov rax, 2; cmp rax, 1; JNE .end
        83583; // 83583 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
