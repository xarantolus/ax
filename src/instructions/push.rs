use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Push;

use super::axecutor::Axecutor;
use super::errors::AxError;

impl Axecutor {
    fn run_mnemonic_push(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Push);

        match i.code() {
            Push_imm16 => self.instr_push_imm16(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic push", i.code()),
        }
    }

    fn instr_push_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Push_imm16);

        todo!("unimplemented")
    }
}
