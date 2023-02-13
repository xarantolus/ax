use iced_x86::Code;

use iced_x86::Instruction;
use iced_x86::Mnemonic::Cpuid;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;

use crate::state::registers::SupportedRegister;

impl Axecutor {
    pub(crate) fn mnemonic_cpuid(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cpuid);

        match i.code() {
            Code::Cpuid => self.instr_cpuid(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Cpuid", i.code()),
        }
    }

    /// CPUID
    ///
    /// 0F A2
    fn instr_cpuid(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Code::Cpuid);

        // TODO: CPUID; https://github.com/aengelke/instrew/blob/master/client/emulate.c#L214-L271

        // let eax = self.reg_read_32(SupportedRegister::EAX)?;
        // let ecx = self.reg_read_32(SupportedRegister::ECX)?;

        // just write dummy values for now. Not sure if this even makes sense
        self.reg_write_32(SupportedRegister::EAX, 0)?;
        self.reg_write_32(SupportedRegister::EBX, 0)?;
        self.reg_write_32(SupportedRegister::ECX, 0)?;
        self.reg_write_32(SupportedRegister::EDX, 0)?;

        Ok(())
    }
}
