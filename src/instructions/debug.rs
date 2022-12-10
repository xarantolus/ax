use iced_x86::Register;
use wasm_bindgen::prelude::wasm_bindgen;

use super::axecutor::Axecutor;

impl Axecutor {
    fn current_stack(&self) -> String {
        let mut output = String::new();
        let rsp = self.reg_read_64(Register::RSP.into());

        output.push_str("Stack:");

        // Print some items around current RSP in size of 8 bytes
        // e.g.
        // Stack:
        //        Offsets     +0 +1 +2 +3 +4 +5 +6 +7
        //        RSP-32      00 00 00 00 00 00 00 00
        //        RSP-24      00 00 00 00 00 00 00 00
        //        RSP-16      00 00 00 00 00 00 00 00
        //        RSP-8       00 00 00 00 00 00 00 00
        //        RSP         00 00 00 00 00 00 00 00
        //        RSP+8       00 00 00 00 00 00 00 00
        //        RSP+16      00 00 00 00 00 00 00 00
        //        RSP+24      00 00 00 00 00 00 00 00
        //        RSP+32      00 00 00 00 00 00 00 00

        const MARGIN: i8 = 4;

        for i in MARGIN..-MARGIN {
            let offset = i * 8;
            let addr = rsp.wrapping_add(offset as u64);
            let bytes = self.mem_read_bytes(addr, 8);

            if let Ok(bytes) = bytes {
                let mut line = format!("\n    0x{:016x}    ", addr);
                for b in bytes.iter() {
                    line.push_str(&format!("{:02x} ", b));
                }
                output.push_str(&line);
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_log() {
        let mut ax = Axecutor::new(&[0xc3], 0x5000, 0x5000).expect("Failed to create axecutor");

        ax.init_stack(0x1000).expect("Failed to init stack");

        let current_stack = ax.current_stack();

        assert_eq!(
            current_stack,
            r#"Stack:
0x0000000000001000    00 00 00 00 00 00 00 00
0x0000000000001008    00 00 00 00 00 00 00 00
0x0000000000001010    00 00 00 00 00 00 00 00
0x0000000000001018    00 00 00 00 00 00 00 00
0x0000000000001020    00 00 00 00 00 00 00 00
"#
        );
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    pub(crate) fn js_debug_log(a: &str);
}

#[macro_export]
macro_rules! debug_log {
    ($str:expr) => {
        #[cfg(all(target_arch = "wasm32", debug_assertions))]
        {
            use crate::instructions::debug::js_debug_log;
            js_debug_log(&*format!("{}:{}: {}", file!(), line!(), $str));
        }

        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
        {
            println!("{}:{}: {}", file!(), line!(), $str);
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[cfg(all(target_arch = "wasm32", debug_assertions))]
        {
            use crate::instructions::debug::js_debug_log;
            js_debug_log(&*format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg)*)));
        }

        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
        {
            println!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg)*));
        }
    };
}
