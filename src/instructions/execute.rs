use std::{cmp::min, convert::TryInto};

use iced_x86::{Decoder, DecoderOptions, Instruction, Register};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsError;

use crate::{
    debug_log,
    instructions::{generated::SupportedMnemonic, registers::SupportedRegister},
};

use super::{axecutor::Axecutor, errors::AxError};

#[wasm_bindgen]
impl Axecutor {
    pub(crate) fn decode_next(&self) -> Result<Instruction, AxError> {
        let rip = self.reg_read_64(SupportedRegister::RIP);

        if rip < self.code_start_address || rip >= self.code_start_address + self.code.len() as u64
        {
            return Err(AxError::from(format!(
                "Instruction pointer is out of bounds: RIP value {:#x} is not in [{:#x}, {:#x})",
                rip,
                self.code_start_address,
                self.code_start_address + self.code.len() as u64
            )));
        }

        let code_offset = (rip - self.code_start_address) as usize;

        // x86 instructions are at most 15 bytes long; make sure we don't read past the end of the code
        let code = &self.code[code_offset..min(code_offset + 15, self.code.len())];

        let mut dec = Decoder::with_ip(64, code, rip, DecoderOptions::NONE);
        if !dec.can_decode() {
            return Err(AxError::from(format!(
                "Cannot decode instruction at offset {}, decoder says there's no more data left to decode",
                dec.position()
            )));
        }

        let instr = dec.decode();

        if instr.is_invalid() {
            return Err(AxError::from(format!(
                "Invalid instruction at offset {}",
                dec.position() - instr.len()
            )));
        }

        Ok(instr)
    }

    // step executes the next instruction, returning if the execution can continue running
    #[wasm_bindgen]
    pub async fn step(&mut self) -> Result<bool, JsError> {
        debug_log!(
            "Calling Axecutor::step, finished={}, rip={:#x}",
            self.state.finished,
            self.reg_read_64(Register::RIP.into())
        );

        if self.state.finished {
            return Err(
                AxError::from("Cannot advance after execution has already finished").into(),
            );
        }

        // Fetch the next instruction
        let instr = self
            .decode_next()
            .map_err(|e| AxError::from(format!("decoding instruction: {}", e)))?;
        debug_log!("Fetched instruction {}", instr);

        self.reg_write_64(SupportedRegister::RIP, instr.next_ip());

        let mnem: SupportedMnemonic = instr.mnemonic().try_into()?;

        let hooks = self.mnemonic_hooks(mnem);
        if let Some(ref h) = hooks {
            debug_log!("Calling before hooks for mnemonic {:?}", mnem);
            h.run_before(self, mnem)
                .await
                .map_err(|e| AxError::from(format!("running before hooks for {}: {}", instr, e)))?;
            debug_log!("Finished running before hooks for mnemonic {:?}", mnem);
        }

        debug_log!("Executing instruction {} ({:?})", instr, instr.code());
        if let Err(e) = self.switch_instruction_mnemonic(instr) {
            // This is so e.g. the ret instruction can end execution
            if e.signals_normal_finish {
                self.state.finished = true;
                debug_log!("Marked execution as finished due to instruction indicating so");
            } else {
                debug_log!(
                    "Error executing instruction {} (after {} steps): {}",
                    instr,
                    self.state.executed_instructions_count,
                    e
                );
                let err_info = e.add_detail(format!(
                    "executing instruction {} ({:?}, {}): ",
                    instr,
                    instr.code(),
                    self.state.executed_instructions_count
                ));

                debug_log!("Throwing error: {}", err_info);

                // In tests, `.into` panics with a very non-helpful message, so we just panic before with a helpful message
                #[cfg(test)]
                {
                    crate::fatal_error!("{}", err_info);
                }

                // Throw normal JS exception when running in browser
                #[allow(unreachable_code)]
                {
                    return Err(err_info.into());
                }
            }
        }

        self.state.executed_instructions_count += 1;

        // If we reached the last instruction (and no jump has been performed etc.), we're done
        if self.reg_read_64(Register::RIP.into())
            == self.code_start_address + self.code.len() as u64
        {
            self.state.finished = true;
            debug_log!("Marked execution as finished due to reaching end of instruction sequence");
        }

        if let Some(ref h) = hooks {
            debug_log!("Calling after hooks for mnemonic {:?}", mnem);
            h.run_after(self, mnem)
                .await
                .map_err(|e| AxError::from(format!("running after hooks for {}: {}", instr, e)))?;
            debug_log!("Finished running after hooks for mnemonic {:?}", mnem);
        }

        debug_log!("Finished Axecutor::step, finished={}", self.state.finished);
        Ok(!self.state.finished)
    }

    pub async fn execute(&mut self) -> Result<(), JsError> {
        debug_log!("Calling Axecutor::execute");
        while self.step().await? {}
        Ok(())
    }
}
