use iced_x86::{Instruction, Register};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::instructions::registers::RegisterWrapper;

use super::{axecutor::Axecutor, errors::AxError};

#[wasm_bindgen]
impl Axecutor {
    fn advance_next_instruction(&mut self) -> Result<Instruction, AxError> {
        let rip_register = RegisterWrapper::from(Register::RIP);

        let rip = self.reg_read_64(rip_register);

        // Fetch the next instruction
        let idx = self.rip_to_index.get(&rip).ok_or(AxError::from(format!(
            "invalid rip {:#x} does not map to an instruction index",
            rip,
        )))?;

        let instr = self.instructions.get(*idx).ok_or(AxError::from(format!(
            "invalid index {} for rip={:#x} does not map to an instruction",
            *idx, rip,
        )))?;

        // Set rip to the instruction after this one
        // TODO: Maybe should use self.write_reg_64 instead, but that threw borrow checker errors
        self.state.registers.insert(rip_register, instr.next_ip());

        if instr.next_ip() == self.instructions.last().unwrap().next_ip() {
            self.finished = true;
        }

        Ok(*instr)
    }

    // step executes the next instruction, returning if the execution can continue running
    #[wasm_bindgen]
    pub fn step(&mut self) -> Result<bool, AxError> {
        if self.finished {
            return Err(AxError::from(
                "Cannot advance after execution has already finished",
            ));
        }

        // Fetch the next instruction
        let instr = self.advance_next_instruction()?;

        self.switch_instruction_mnemonic(instr)?;

        // TODO: Actually execute the instruction
        // TODO: Figure out what to do with flags, when to reset e.g. carry etc.
        // Maybe create a bitmask that is xored after the next instruction is executed

        // If we reached the last instruction (and no jump has been performed etc.), we're done
        if self.reg_read_64(Register::RIP.into()) == self.instructions.last().unwrap().next_ip() {
            self.finished = true;
        }

        Ok(!self.finished)
    }

    pub fn execute(&mut self) -> Result<(), AxError> {
        while self.step()? {}
        Ok(())
    }
}
