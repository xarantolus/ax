use iced_x86::{Instruction, Register};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsError;

use crate::{debug_log, instructions::registers::SupportedRegister};

use super::{axecutor::Axecutor, errors::AxError};

#[wasm_bindgen]
impl Axecutor {
    fn get_next_instruction(&self) -> Result<Instruction, AxError> {
        // TODO: create new decoder instead of decoding all instructions in advance

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
        debug_log!(
            "Calling Axecutor::step, finished={}, rip={:#x}",
            self.finished,
            self.reg_read_64(Register::RIP.into())
        );

        if self.finished {
            return Err(
                AxError::from("Cannot advance after execution has already finished").into(),
            );
        }

        // Fetch the next instruction
        let instr = self.get_next_instruction()?;
        debug_log!("Fetched instruction {}", instr);

        self.reg_write_64(SupportedRegister::RIP, instr.next_ip());

        let mnem = instr.mnemonic().into();

        let hooks = self.mnemonic_hooks(mnem);
        if let Some(ref h) = hooks {
            debug_log!("Calling before hooks for mnemonic {:?}", mnem);
            h.run_before(self, mnem).await?;
            debug_log!("Finished running before hooks for mnemonic {:?}", mnem);
        }

        debug_log!("Executing instruction {} ({:?})", instr, instr.code());
        if let Err(e) = self.switch_instruction_mnemonic(instr) {
            // This is so e.g. the ret instruction can end execution
            if e.signals_normal_finish {
                self.finished = true;
                debug_log!("Marked execution as finished due to instruction indicating so");
            } else {
                debug_log!("Error executing instruction: {}", e);
                return Err(e
                    .add_detail(format!(
                        "while executing instruction {:?}",
                        instr.mnemonic()
                    ))
                    .into());
            }
        }

        // If we reached the last instruction (and no jump has been performed etc.), we're done
        if self.reg_read_64(Register::RIP.into()) == self.instructions.last().unwrap().next_ip() {
            self.finished = true;
            debug_log!("Marked execution as finished due to reaching end of instruction sequence");
        }

        if let Some(ref h) = hooks {
            debug_log!("Calling after hooks for mnemonic {:?}", mnem);
            h.run_after(self, mnem).await?;
            debug_log!("Finished running after hooks for mnemonic {:?}", mnem);
        }

        debug_log!("Finished Axecutor::step, finished={}", self.finished);
        Ok(!self.finished)
    }

    pub async fn execute(&mut self) -> Result<(), JsError> {
        debug_log!("Calling Axecutor::execute");
        while self.step().await? {}
        Ok(())
    }
}
