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

#[cfg(test)]
mod tests {
    use crate::instructions::axecutor::Axecutor;

    #[test]
    fn test_no_syscall_handler() {
        async_std::task::block_on(async {
            // syscall encoding
            let code = [0xf, 0x5];
            let mut ax = Axecutor::new(&code, 0x1000, 0x1000).unwrap();
            assert_eq!(ax.instructions.len(), 1);

            let result = ax.execute().await;
            assert!(result.is_err());
        });
    }
}
