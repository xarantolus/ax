use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jns;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::flags::*;

use crate::instructions::macros::fatal_error;
use crate::instructions::macros::opcode_unimplemented;
use crate::instructions::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_jns(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jns);

        match i.code() {
            Jns_rel8_16 => self.instr_jns_rel8_16(i),
            Jns_rel8_32 => self.instr_jns_rel8_32(i),
            Jns_rel8_64 => self.instr_jns_rel8_64(i),
            Jns_rel16 => self.instr_jns_rel16(i),
            Jns_rel32_32 => self.instr_jns_rel32_32(i),
            Jns_rel32_64 => self.instr_jns_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Jns", i.code()),
        }
    }

    /// JNS rel8
    ///
    /// o16 79 cb
    fn instr_jns_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jns_rel8_16);

        opcode_unimplemented!("instr_jns_rel8_16 for Jns")
    }

    /// JNS rel8
    ///
    /// o32 79 cb
    fn instr_jns_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jns_rel8_32);

        opcode_unimplemented!("instr_jns_rel8_32 for Jns")
    }

    /// JNS rel8
    ///
    /// o64 79 cb
    fn instr_jns_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jns_rel8_64);

        if self.state.rflags & FLAG_SF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JNS rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JNS rel16
    ///
    /// o16 0F 89 cw
    fn instr_jns_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jns_rel16);

        opcode_unimplemented!("instr_jns_rel16 for Jns")
    }

    /// JNS rel32
    ///
    /// o32 0F 89 cd
    fn instr_jns_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jns_rel32_32);

        opcode_unimplemented!("instr_jns_rel32_32 for Jns")
    }

    /// JNS rel32
    ///
    /// o64 0F 89 cd
    fn instr_jns_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jns_rel32_64);

        if self.state.rflags & FLAG_SF == 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset)?;
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JNS rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::tests::{assert_reg_value, jmp_test};

    jmp_test![mov_ax_0xffff_cmp_ax_0_jns_end_mov_rax_42_end_nop_pf_sf;
        start: 0x401010; end: 0x40102a;
        0x66, 0xb8, 0xff, 0xff, 0x66, 0x83, 0xf8, 0x0, 0x79, 0xf; // mov ax, 0xffff; cmp ax, 0; JNS .end
        8; // 8 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 42);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_ax_0xffff_cmp_ax_0_jns_end_mov_rax_42_end_nop_pf_sf_32bit;
        start: 0x401010; end: 0x4148a6;
        0x66, 0xb8, 0xff, 0xff, 0x66, 0x83, 0xf8, 0x0, 0xf, 0x89, 0x87, 0x38, 0x1, 0x0; // mov ax, 0xffff; cmp ax, 0; JNS .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 42);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_ax_0xffff_sub_ax_0xf000_jns_end_mov_rax_42_end_nop_pf;
        start: 0x401010; end: 0x40102a;
        0x66, 0xb8, 0xff, 0xff, 0x66, 0x2d, 0x0, 0xf0, 0x79, 0xf; // mov ax, 0xffff; sub ax, 0xf000; JNS .end
        8; // 8 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_ax_0xffff_sub_ax_0xf000_jns_end_mov_rax_42_end_nop_pf_32bit;
        start: 0x401010; end: 0x4148a6;
        0x66, 0xb8, 0xff, 0xff, 0x66, 0x2d, 0x0, 0xf0, 0xf, 0x89, 0x87, 0x38, 0x1, 0x0; // mov ax, 0xffff; sub ax, 0xf000; JNS .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
