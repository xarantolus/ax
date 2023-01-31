use std::convert::TryInto;

use iced_x86::{Decoder, DecoderOptions, Instruction, Register};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::debug::debug_log;

use crate::{instructions::generated::SupportedMnemonic, state::registers::SupportedRegister};

use crate::{axecutor::Axecutor, helpers::errors::AxError};

#[wasm_bindgen]
impl Axecutor {
    pub(crate) fn decode_at(&self, rip: u64) -> Result<Instruction, AxError> {
        // x86 instructions are at most 15 bytes long; make sure we don't read past the end of the code
        let code = &self.mem_read_executable_bytes(rip)?;

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

    pub(crate) fn decode_next(&self) -> Result<Instruction, AxError> {
        let rip = self.reg_read_64(Register::RIP.into())?;
        self.decode_at(rip)
    }

    /// Execute the next instruction (including all registered hooks), returning if execution has stopped
    pub async fn step(&mut self) -> Result<bool, AxError> {
        debug_log!(
            "Calling Axecutor::step, finished={}, rip={:#x}",
            self.state.finished,
            self.reg_read_64(Register::RIP.into())?
        );

        if self.state.finished {
            return Err(AxError::from(
                "Cannot advance after execution has already finished",
            ));
        }

        // Fetch the next instruction
        let instr = self.decode_next().map_err(|e| {
            e.add_detail(
                "fetching next instruction".to_string(),
                self.call_stack().unwrap_or_else(|e| e.to_string()),
                self.trace().unwrap_or_else(|e| e.to_string()),
            )
        })?;

        debug_log!("Fetched instruction {}", instr);

        let rip = instr.next_ip();
        self.reg_write_64(SupportedRegister::RIP, rip)?;

        let mnem: SupportedMnemonic = instr.mnemonic().try_into().map_err(|e: AxError| {
            e.add_detail(
                "".to_string(),
                self.call_stack().unwrap_or_else(|e| e.to_string()),
                self.trace().unwrap_or_else(|e| e.to_string()),
            )
        })?;

        let hooks = self.mnemonic_hooks(mnem);
        if let Some(ref h) = hooks {
            debug_log!("Calling before hooks for mnemonic {:?}", mnem);
            h.run_before(self, mnem).await.map_err(|e| {
                AxError::from(format!("running before hooks for {}: {}", instr, e)).add_detail(
                    format!(
                        "executing syscall after executing {} instructions: ",
                        self.state.executed_instructions_count
                    ),
                    self.call_stack().unwrap_or_else(|e| e.to_string()),
                    self.trace().unwrap_or_else(|e| e.to_string()),
                )
            })?;
            debug_log!("Finished running before hooks for mnemonic {:?}", mnem);
        }

        debug_log!(
            "Executing instruction {} ({:?}) @ {:#x}",
            instr,
            instr.code(),
            rip
        );
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

                let err_info = e.add_detail(
                    format!(
                        "executing instruction {} ({:?}) after executing {} instructions: ",
                        instr,
                        instr.code(),
                        self.state.executed_instructions_count
                    ),
                    self.call_stack().unwrap_or_else(|e| e.to_string()),
                    self.trace().unwrap_or_else(|e| e.to_string()),
                );

                debug_log!("Throwing error: {}", err_info);

                return Err(err_info);
            }
        }

        self.state.executed_instructions_count += 1;

        // If we reached the last instruction (and no jump has been performed etc.), we're done
        if self.reg_read_64(Register::RIP.into())? == self.code_end_addr {
            self.state.finished = true;
            debug_log!("Marked execution as finished due to reaching end of instruction sequence");
        }

        if let Some(ref h) = hooks {
            debug_log!("Calling after hooks for mnemonic {:?}", mnem);
            h.run_after(self, mnem).await.map_err(|e| {
                AxError::from(format!("running after hooks for {}: {}", instr, e)).add_detail(
                    format!(
                        "executing syscall after executing {} instructions: ",
                        self.state.executed_instructions_count
                    ),
                    self.call_stack().unwrap_or_else(|e| e.to_string()),
                    self.trace().unwrap_or_else(|e| e.to_string()),
                )
            })?;
            debug_log!("Finished running after hooks for mnemonic {:?}", mnem);
        }

        debug_log!("Finished Axecutor::step, finished={}", self.state.finished);
        Ok(!self.state.finished)
    }

    /// Execute all instructions until execution has stopped.
    /// Execution might stop due to hooks stopping emulation via the stop() method, execution reaching the end of the code, or an error.
    /// This is the same as calling `step` in a loop, but staying in WASM should be more efficient.
    pub async fn execute(&mut self) -> Result<(), AxError> {
        debug_log!("Calling Axecutor::execute");
        while self.step().await? {}
        Ok(())
    }
}
