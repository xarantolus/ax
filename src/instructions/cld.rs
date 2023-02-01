use iced_x86::Code;

use iced_x86::Instruction;
use iced_x86::Mnemonic::Cld;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;

use crate::state::flags::*;

impl Axecutor {
    pub(crate) fn mnemonic_cld(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cld);

        match i.code() {
            Code::Cld => self.instr_cld(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Cld", i.code()),
        }
    }

    /// CLD
    ///
    /// FC
    fn instr_cld(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Code::Cld);

        self.state.rflags &= !FLAG_DF;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{ax_test, write_flags};

    // cld
    ax_test![cld_clear; 0xfc; |a: &mut Axecutor| {
        write_flags!(a; FLAG_DF);
    }; |_| {};
        (0; FLAG_DF)
    ];

    // cld
    ax_test![cld_noclear; 0xfc; |_| {}; |_| {};
        (0; FLAG_DF)
    ];
}
