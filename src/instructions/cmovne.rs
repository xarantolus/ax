use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Cmovne;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;

use crate::{calculate_r_rm, fatal_error};

impl Axecutor {
    pub fn mnemonic_cmovne(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cmovne);

        match i.code() {
            Cmovne_r16_rm16 => self.instr_cmovne_r16_rm16(i),
            Cmovne_r32_rm32 => self.instr_cmovne_r32_rm32(i),
            Cmovne_r64_rm64 => self.instr_cmovne_r64_rm64(i),
            _ => fatal_error!(
                "Invalid instruction code {:?} for mnemonic Cmovne",
                i.code()
            ),
        }
    }

    /// CMOVNE r16, r/m16
    ///
    /// o16 0F 45 /r
    fn instr_cmovne_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovne_r16_rm16);

        if self.state.rflags & FLAG_ZF == 0 {
            calculate_r_rm![u16; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVNE r32, r/m32
    ///
    /// o32 0F 45 /r
    fn instr_cmovne_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovne_r32_rm32);

        if self.state.rflags & FLAG_ZF == 0 {
            calculate_r_rm![u32; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVNE r64, r/m64
    ///
    /// o64 0F 45 /r
    fn instr_cmovne_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovne_r64_rm64);

        if self.state.rflags & FLAG_ZF == 0 {
            calculate_r_rm![u64; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }
}
