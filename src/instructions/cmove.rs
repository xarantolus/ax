use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Cmove;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::flags::*;

use crate::instructions::macros::calculate_r_rm;

use crate::instructions::macros::fatal_error;

impl Axecutor {
    pub fn mnemonic_cmove(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cmove);

        match i.code() {
            Cmove_r16_rm16 => self.instr_cmove_r16_rm16(i),
            Cmove_r32_rm32 => self.instr_cmove_r32_rm32(i),
            Cmove_r64_rm64 => self.instr_cmove_r64_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Cmove", i.code()),
        }
    }

    /// CMOVE r16, r/m16
    ///
    /// o16 0F 44 /r
    fn instr_cmove_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmove_r16_rm16);

        if self.state.rflags & FLAG_ZF != 0 {
            calculate_r_rm![u16; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVE r32, r/m32
    ///
    /// o32 0F 44 /r
    fn instr_cmove_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmove_r32_rm32);

        if self.state.rflags & FLAG_ZF != 0 {
            calculate_r_rm![u32; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVE r64, r/m64
    ///
    /// o64 0F 44 /r
    fn instr_cmove_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmove_r64_rm64);

        if self.state.rflags & FLAG_ZF != 0 {
            calculate_r_rm![u64; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {}
