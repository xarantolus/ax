use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Cmovae;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;

use crate::instructions::macros::calculate_r_rm;

use crate::instructions::macros::fatal_error;

impl Axecutor {
    pub fn mnemonic_cmovae(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cmovae);

        match i.code() {
            Cmovae_r16_rm16 => self.instr_cmovae_r16_rm16(i),
            Cmovae_r32_rm32 => self.instr_cmovae_r32_rm32(i),
            Cmovae_r64_rm64 => self.instr_cmovae_r64_rm64(i),
            _ => fatal_error!(
                "Invalid instruction code {:?} for mnemonic Cmovae",
                i.code()
            ),
        }
    }

    /// CMOVAE r16, r/m16
    ///
    /// o16 0F 43 /r
    fn instr_cmovae_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovae_r16_rm16);

        if self.state.rflags & FLAG_CF != 0 {
            calculate_r_rm![u16; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVAE r32, r/m32
    ///
    /// o32 0F 43 /r
    fn instr_cmovae_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovae_r32_rm32);

        if self.state.rflags & FLAG_CF != 0 {
            calculate_r_rm![u32; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }

    /// CMOVAE r64, r/m64
    ///
    /// o64 0F 43 /r
    fn instr_cmovae_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmovae_r64_rm64);

        if self.state.rflags & FLAG_CF != 0 {
            calculate_r_rm![u64; self; i; |_, s| {
                s
            }; (set: FLAGS_UNAFFECTED; clear: 0)]
        } else {
            Ok(())
        }
    }
}
