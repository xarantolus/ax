use crate::{
    axecutor::Axecutor, helpers::debug::debug_log, state::registers::SupportedRegister::RSP,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::errors::AxError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct LastRSP {
    pub(crate) value: u64,
    pub(crate) hook: bool,
    pub(crate) set_instr_ip: u64,
    pub(crate) set_func_addr: u64,
}

macro_rules! symbol {
    ($self:ident, $value:expr) => {
        format!(
            "{}@{:#x}",
            $self
                .resolve_symbol($value)
                .unwrap_or_else(|| "<unknown>".to_string()),
            $value
        )
    };
}

#[wasm_bindgen]
impl Axecutor {
    pub fn mem_dump(&mut self, address: u64, range: u64) -> Result<String, AxError> {
        debug_log!(
            "Calling Axecutor::mem_dump with addr={:#x} and range={}",
            address,
            range
        );

        let mut all_inaccessible = true;

        let mut buf = String::new();
        // Basically the address of the line in the middle of the dump
        let aligned_addr = address & !0xf;

        // Make sure there are no underflows, otherwise this would panic
        let start_addr = if aligned_addr > (range * 16) {
            aligned_addr - (range * 16)
        } else {
            0
        };

        let max_width = format!("{:#x}", start_addr + (range * 2) * 16).len();

        for i in 0..(range * 2) {
            let line_addr = start_addr + (i * 16);
            buf.push_str(&format!("{:#width$x}  ", line_addr, width = max_width));

            let mut display_buf = String::new();

            for j in 0..16 {
                let addr = line_addr + j;
                let byte = self.mem_read_8(addr);

                match byte {
                    Ok(b) => {
                        all_inaccessible = false;
                        buf.push_str(&format!("{:02x} ", b));
                        let b = b as u8 as char;
                        if b.is_ascii_graphic() {
                            display_buf.push(b);
                        } else {
                            display_buf.push('.');
                        }
                    }
                    Err(_) => {
                        buf.push_str("?? ");
                        display_buf.push(' ');
                    }
                }

                if j == 7 {
                    buf.push(' ');
                }
            }

            buf.push_str(" |");
            buf.push_str(&display_buf);
            buf.push('|');
            // If our address was in the middle of this line, highlight it
            if address >= line_addr && address < line_addr + 16 {
                buf.push_str(
                    format!(" <-- {:#x} at index {}", address, address - line_addr).as_str(),
                );
            }
            buf.push('\n');
        }

        if all_inaccessible {
            return Err(AxError::from(
                "All bytes in the specified range are inaccessible",
            ));
        }

        Ok(buf)
    }

    pub fn stack_dump(&mut self) -> Result<String, AxError> {
        let rsp = self.reg_read_64(RSP)?;

        match self.mem_dump(rsp, 10) {
            Ok(dump) => Ok(format!("Stack dump around {:#x} (RSP):\n{}", rsp, dump,)),
            Err(_) => {
                // No byte was accessible, so try to use the previous RSP value instead
                let last_rsp = self.state.last_rsp.value;
                match self.mem_dump(last_rsp, 10) {
                    Ok(dump) => {
                        let help = match self.state.last_rsp {
							LastRSP { hook: true, .. } => format!(
								"Using the previous RSP value {:#x} that was set during a hook executing at instruction {:#x} in function {}",
								last_rsp, self.state.last_rsp.set_instr_ip, symbol!(self, self.state.last_rsp.set_func_addr)
							),
							_ => format!(
								"Using the previous RSP value {:#x} that was set at instruction {:#x} in function {}",
								last_rsp, self.state.last_rsp.set_instr_ip, symbol!(self, self.state.last_rsp.set_func_addr)
							),
						};
                        Ok(format!(
                            "Stack dump around {:#x} (RSP) failed!\n{}:\n{}",
                            rsp, help, dump,
                        ))
                    }
                    Err(_) => Err(AxError::from("All bytes around RSP are inaccessible!")),
                }
            }
        }
    }
}
