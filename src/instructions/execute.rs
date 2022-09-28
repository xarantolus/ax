use iced_x86::{Instruction, Register};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsError;

use crate::instructions::registers::SupportedRegister;

use super::{axecutor::Axecutor, errors::AxError};

#[wasm_bindgen]
impl Axecutor {
    fn get_next_instruction(&self) -> Result<Instruction, AxError> {
        let rip_register: SupportedRegister = Register::RIP.into();

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

        Ok(*instr)
    }

    // step executes the next instruction, returning if the execution can continue running
    #[wasm_bindgen]
    pub async fn step(&mut self) -> Result<bool, JsError> {
        if self.finished {
            return Err(
                AxError::from("Cannot advance after execution has already finished").into(),
            );
        }

        // Fetch the next instruction
        let instr = self.get_next_instruction()?;

        self.reg_write_64(Register::RIP.into(), instr.next_ip());

        let mnem = instr.mnemonic().into();

        let hooks = self.mnemonic_hooks(mnem);
        if let Some(ref h) = hooks {
            h.run_before(self, mnem).await?;
        }

        self.switch_instruction_mnemonic(instr).map_err(|e| {
            e.add_detail(format!(
                "while executing instruction {:?}",
                instr.mnemonic()
            ))
        })?;

        // If we reached the last instruction (and no jump has been performed etc.), we're done
        if self.reg_read_64(Register::RIP.into()) == self.instructions.last().unwrap().next_ip() {
            self.finished = true;
        }

        if let Some(ref h) = hooks {
            h.run_after(self, mnem).await?;
        }

        Ok(!self.finished)
    }

    pub async fn execute(&mut self) -> Result<(), JsError> {
        while self.step().await? {}
        Ok(())
    }
}
