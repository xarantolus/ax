use iced_x86::Instruction;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{axecutor::Axecutor, state::registers::SupportedRegister};

use super::errors::AxError;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TraceEntry {
    pub(crate) instr_ip: u64,
    pub(crate) target: u64,
    pub(crate) variant: TraceVariant,
    pub(crate) level: i16,
    pub(crate) count: u64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(crate) enum TraceVariant {
    Call,
    Return,
    Jump,
}

// This is for the full tracing functionality, not to be confused with call_stack
#[wasm_bindgen]
impl Axecutor {
    fn add_trace(
        &mut self,
        i: Instruction,
        target: u64,
        variant: TraceVariant,
    ) -> Result<(), AxError> {
        let instr_ip = self.reg_read_64(SupportedRegister::RIP)? - i.len() as u64;
        let mut lvl = 0;

        if let Some(last) = self.state.trace.last_mut() {
            lvl = last.level;
            match last.variant {
                TraceVariant::Call => lvl += 1,
                TraceVariant::Return => lvl -= 1,
                TraceVariant::Jump => {
                    // If we have seen this exact jump before, we just increment the count
                    if last.instr_ip == instr_ip
                        && last.target == target
                        && last.variant == variant
                        && last.level == lvl
                    {
                        last.count += 1;
                        return Ok(());
                    }
                }
            }
        }

        self.state.trace.push(TraceEntry {
            level: lvl,
            instr_ip,
            target,
            variant,
            count: 1,
        });

        Ok(())
    }

    pub(crate) fn trace_call(&mut self, i: Instruction, target: u64) -> Result<(), AxError> {
        self.add_trace(i, target, TraceVariant::Call {})
    }

    pub(crate) fn trace_return(&mut self, i: Instruction, target: u64) -> Result<(), AxError> {
        self.add_trace(i, target, TraceVariant::Return {})
    }

    pub(crate) fn trace_jump(&mut self, i: Instruction, target: u64) -> Result<(), AxError> {
        self.add_trace(i, target, TraceVariant::Jump {})
    }

    /// Generate a trace of the current execution state. This trace is a list of all executed jumps, calls and returns.
    /// This works best when a symbol table has been provided, which is currently only the case for ELF binaries.
    pub fn trace(&mut self) -> Result<String, AxError> {
        let mut trace = String::new();

        let mut i = 0;
        while i < self.state.trace.len() {
            let entry = &self.state.trace[i];

            let instruction = if entry.instr_ip == 0 {
                "entrypoint".to_string()
            } else {
                match self.decode_at(entry.instr_ip) {
                    Ok(instr) => format!("{instr}"),
                    Err(_) => "<decoding error>".to_string(),
                }
            };
            let instruction_symbol = if entry.instr_ip == 0 {
                "<emulator_start>".to_string()
            } else {
                match self.symbol_table.get(&entry.instr_ip) {
                    Some(sym) => format!("{}@{:#x}", sym, entry.instr_ip),
                    None => format!("{:#x}", entry.instr_ip),
                }
            };

            let target_symbol = match self.symbol_table.get(&entry.target) {
                Some(sym) => format!("{}@{:#x}", sym, entry.target),
                None => format!("{:#x}", entry.target),
            };

            // If we have a jump, we count how many of the next are equal and then write e.g. x10 instead of 10 times the same line
            if entry.count > 1 {
                trace.push_str(&format!(
                    "{}{}: {} => {} ({} times)\n",
                    "  ".repeat(entry.level as usize),
                    instruction_symbol,
                    instruction,
                    target_symbol,
                    entry.count,
                ));
            } else {
                trace.push_str(&format!(
                    "{}{}: {} => {}\n",
                    "  ".repeat(entry.level as usize),
                    instruction_symbol,
                    instruction,
                    target_symbol,
                ));
            }
            i += 1;
        }

        Ok(trace)
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::syscalls::Syscall;
    use crate::helpers::{debug::debug_log, tests::test_async};
    use crate::instructions::generated::SupportedMnemonic;

    test_async![simple_trace; async {
        let binary = include_bytes!("../../testdata/trace.bin");

        let mut ax = Axecutor::from_binary(binary).expect("Failed to parse binary");

        ax.handle_syscalls(vec![Syscall::Exit]).expect("Failed to add syscall handlers");

        ax.init_stack(0x1000).expect("Failed to setup stack");

        ax.execute().await.expect("Failed to execute");

        let trace = ax.trace().expect("Failed to generate trace");

        debug_log!("Trace:\n{}", trace);

        assert_eq!(trace, r#"<emulator_start>: entrypoint => _start@0x401000
  _start@0x401000: call 0000000000401015h => first_level@0x401015
    first_level@0x401015: jmp short 000000000040101Eh => 0x40101e
    0x40101e: call 0000000000401031h => second_level@0x401031
      second_level@0x401031: call 000000000040103Ch => third_level@0x40103c
        0x401052: jne short 000000000040104Ah => 0x40104a (9 times)
        0x401054: ret => 0x401036
      0x401036: call 000000000040103Ch => third_level@0x40103c
        0x401052: jne short 000000000040104Ah => 0x40104a (9 times)
        0x401054: ret => 0x40103b
      0x40103b: ret => 0x401023
    0x401023: call 0000000000401029h => second_level_two@0x401029
      0x401030: ret => 0x401028
    0x401028: ret => 0x401005
"#);
    }];

    test_async![c_loop; async {
        let binary = include_bytes!("../../testdata/c_loop.bin");

        let mut ax = Axecutor::from_binary(binary).expect("Failed to parse binary");

        ax.handle_syscalls(vec![Syscall::Exit]).expect("Failed to add syscall handlers");

        #[allow(non_upper_case_globals)]
        static mut jle_count: u64 = 0;
        #[allow(non_upper_case_globals)]
        static mut jle_rip: u64 = 0;
        ax.hook_before_mnemonic_native(SupportedMnemonic::Jle, &move |ax: &mut Axecutor, _| {
            unsafe {
                jle_rip = ax.reg_read_64(crate::state::registers::SupportedRegister::RIP)?;
            };

            Ok(crate::state::hooks::HookResult::Handled)
        }).expect("Failed to add hook");

        ax.hook_after_mnemonic_native(SupportedMnemonic::Jle, &move |ax: &mut Axecutor, _| {
            unsafe {
                // Did we jump somewhere else?
                if jle_rip != ax.reg_read_64(crate::state::registers::SupportedRegister::RIP)? {
                    jle_count += 1;
                }
            };

            Ok(crate::state::hooks::HookResult::Handled)
        }).expect("Failed to add hook");

        ax.init_stack(0x1000).expect("Failed to setup stack");

        ax.execute().await.expect("Failed to execute");

        let trace = ax.trace().expect("Failed to generate trace");

        debug_log!("Trace:\n{}", trace);

        assert_eq!(trace, format!(r#"<emulator_start>: entrypoint => _start@0x40101a
  0x401034: jmp short 000000000040103Eh => 0x40103e
  0x401042: jle short 0000000000401036h => 0x401036 ({} times)
  0x401049: call 0000000000401000h => sys_exit@0x401000
"#, unsafe { jle_count }));
    }];
}

// This is only the call stack, which uses different data than the full tracing functionality
#[wasm_bindgen]
impl Axecutor {
    /// Give an overview of the current call stack.
    /// This works best when a symbol table has been provided, which is currently only the case for ELF binaries.
    pub fn call_stack(&self) -> Result<String, AxError> {
        let mut trace = String::new();

        for (i, addr) in self.state.call_stack.iter().enumerate() {
            let formatted = match self.symbol_table.get(addr) {
                Some(sym) => format!("{sym}@{addr:#x}"),
                None => format!("{addr:#x}"),
            };

            if i == self.state.call_stack.len() - 1 {
                trace.push_str(&format!(
                    "{}=> {}            <------------ in this function\n",
                    "  ".repeat(i),
                    formatted
                ));
            } else {
                trace.push_str(&format!("{}-> {}\n", "  ".repeat(i), formatted));
            }
        }

        let rip = self.reg_read_64(SupportedRegister::RIP)?;
        if let Ok(instr) = self.decode_at(rip) {
            trace.push_str(&format!(
                "{}  rip@{:#x}            <------------ at or before this instruction pointer\n{}  {} ({:#?})            <------------ at this or the previous instruction",
                "  ".repeat(self.state.call_stack.len()),
                rip,
                "  ".repeat(self.state.call_stack.len()),
                instr,
                instr.code()
            ));
        }

        Ok(trace)
    }
}
