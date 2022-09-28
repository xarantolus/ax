use iced_x86::Instruction;
use iced_x86::Mnemonic::Syscall;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::generated::SupportedMnemonic;

impl Axecutor {
    pub fn mnemonic_syscall(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Syscall);

        match i.code() {
            iced_x86::Code::Syscall => self.instr_syscall(i),
            _ => panic!(
                "Invalid instruction code {:?} for mnemonic Syscall",
                i.code()
            ),
        }
    }

    /// SYSCALL
    ///
    /// 0F 05
    fn instr_syscall(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Syscall);

        if let Some(_) = self.mnemonic_hooks(SupportedMnemonic::Syscall) {
            return Ok(());
        }

        Err(AxError::from("Syscall encountered, but no hook to handle available. Use `hook_before_mnemonic` or `hook_after_mnemonic` to register a hook for syscalls."))
    }
}
