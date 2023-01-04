use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Js;
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister::*;
use crate::{fatal_error, opcode_unimplemented};

impl Axecutor {
    pub fn mnemonic_js(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Js);

        match i.code() {
            Js_rel8_16 => self.instr_js_rel8_16(i),
            Js_rel8_32 => self.instr_js_rel8_32(i),
            Js_rel8_64 => self.instr_js_rel8_64(i),
            Js_rel16 => self.instr_js_rel16(i),
            Js_rel32_32 => self.instr_js_rel32_32(i),
            Js_rel32_64 => self.instr_js_rel32_64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Js", i.code()),
        }
    }

    /// JS rel8
    ///
    /// o16 78 cb
    fn instr_js_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Js_rel8_16);

        opcode_unimplemented!("instr_js_rel8_16 for Js")
    }

    /// JS rel8
    ///
    /// o32 78 cb
    fn instr_js_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Js_rel8_32);

        opcode_unimplemented!("instr_js_rel8_32 for Js")
    }

    /// JS rel8
    ///
    /// o64 78 cb
    fn instr_js_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Js_rel8_64);

        if self.state.rflags & FLAG_SF != 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JS rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JS rel16
    ///
    /// o16 0F 88 cw
    fn instr_js_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Js_rel16);

        opcode_unimplemented!("instr_js_rel16 for Js")
    }

    /// JS rel32
    ///
    /// o32 0F 88 cd
    fn instr_js_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Js_rel32_32);

        opcode_unimplemented!("instr_js_rel32_32 for Js")
    }

    /// JS rel32
    ///
    /// o64 0F 88 cd
    fn instr_js_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Js_rel32_64);

        if self.state.rflags & FLAG_SF != 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP, offset);
                }
                _ => fatal_error!("Invalid op0_kind {:?} for JS rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::jmp_test;

    jmp_test![mov_al_0x0_add_al_0xff_js_end_mov_al_42_end_nop_pf_sf;
        start: 0x401010; end: 0x401021;
        0xb0, 0x0, 0x4, 0xff, 0x78, 0xa; // mov al, 0x0; add al, 0xff; JS .end
        8; // 8 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_al_0x0_add_al_0xff_js_end_mov_al_42_end_nop_pf_sf_32bit;
        start: 0x401010; end: 0x41489d;
        0xb0, 0x0, 0x4, 0xff, 0xf, 0x88, 0x82, 0x38, 0x1, 0x0; // mov al, 0x0; add al, 0xff; JS .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    jmp_test![mov_al_0x0_add_al_0x7f_js_end_mov_al_42_end_nop;
        start: 0x401010; end: 0x401021;
        0xb0, 0x0, 0x4, 0x7f, 0x78, 0xa; // mov al, 0x0; add al, 0x7f; JS .end
        8; // 8 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_al_0x0_add_al_0x7f_js_end_mov_al_42_end_nop_32bit;
        start: 0x401010; end: 0x41489d;
        0xb0, 0x0, 0x4, 0x7f, 0xf, 0x88, 0x82, 0x38, 0x1, 0x0; // mov al, 0x0; add al, 0x7f; JS .end
        80000; // 80000 bytes of 0x90 (nop) as padding
        0xb0, 0x2a, 0x90; // mov al, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
