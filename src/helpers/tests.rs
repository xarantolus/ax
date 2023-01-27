// A common theme in this file is that we have to differentiate between "normal" and "wasm" tests:
// WASM tests run via wasm-pack test, which does not work with the normal #[test] macro.
// The other tests just use the normal #[test] macro that works with cargo test.
// To only write one test case that works with both, most macros here use #[test] or #[wasm_bindgen_test] depending on the target.

#[cfg(test)]
use crate::axecutor::Axecutor;

#[inline(never)]
#[cfg(test)]
pub(crate) fn ax_test_runner<S, A>(
    bytes: &[u8],
    mut setup: S,
    mut asserts: A,
    flags_to_set: u64,
    flags_not_to_set: u64,
) where
    S: FnMut(&mut Axecutor) + 'static,
    A: FnMut(Axecutor) + 'static,
{
    let copy = bytes.to_vec();

    async_std::task::block_on(async move {
        use rand::Rng;

        // Always use a random rip, but make sure it doesn't overlap with memory that is often allocated at 0x1000 in tests
        let random_rip = rand::thread_rng().gen::<u64>() & 0x0000_ffff_ffff_ffff | 0xf0000;

        let mut ax =
            Axecutor::new(&copy, random_rip, random_rip).expect("Failed to create axecutor");

        setup(&mut ax);

        if let Err(e) = ax.execute().await {
            panic!("Failed to execute: {:?}", e)
        };

        let flags = ax.state.rflags;

        asserts(ax);

        // Check flags
        use crate::state::flags::*;
        for flag in FLAG_LIST {
            // If the flag should be set, it must be != 0
            if flags_to_set & flag != 0 {
                assert!(
                    flags & flag != 0,
                    "FLAG_{} should be set, but wasn't",
                    FLAG_TO_NAMES.get(&flag).expect("Flag not found")
                );
            }

            if flags_not_to_set & flag != 0 {
                assert!(
                    flags & flag == 0,
                    "FLAG_{} should not be set, but was",
                    FLAG_TO_NAMES.get(&flag).expect("Flag not found")
                );
            }
        }
    });
}

#[cfg(test)]
macro_rules! ax_test {
    [$test_name:ident; $($bytes:expr),*; $asserts:expr; ($flags_to_set:expr; $flags_not_to_set:expr)] => {
        // Call the other macro with empty setup code
        ax_test!($test_name; $($bytes),*; |_: &mut Axecutor| {}; $asserts; ($flags_to_set; $flags_not_to_set));
    };
    [$test_name:ident; $($bytes:expr),*; $setup:expr; $asserts:expr; ($flags_to_set:expr; $flags_not_to_set:expr)] => {
        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen_test::wasm_bindgen_test]
        #[cfg(target_arch = "wasm32")]
        fn $test_name () {
            #[allow(unused_imports)] // Some tests use these, some don't
            use $crate::state::flags::*;
            $crate::helpers::tests::ax_test_runner(&[$($bytes),*], $setup, $asserts, $flags_to_set, $flags_not_to_set);
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[test]
        #[cfg(not(target_arch = "wasm32"))]
        fn $test_name () {
            #[allow(unused_imports)] // Some tests use these, some don't
            use $crate::state::flags::*;
            $crate::helpers::tests::ax_test_runner(&[$($bytes),*], $setup, $asserts, $flags_to_set, $flags_not_to_set);
        }
    };
    [$test_name:ident; $($bytes:expr),*; $asserts:expr] => {
        // Call the other macro with empty setup code
        ax_test!($test_name; $($bytes),*; |_: &mut Axecutor| {}; $asserts; (0; 0));
    };
    [$test_name:ident; $($bytes:expr),*; $setup:expr; $asserts:expr] => {
        // Call the other macro with empty flags
        ax_test!($test_name; $($bytes),*; $setup; $asserts; (0; 0));
    };
}

#[cfg(test)]
pub(crate) use ax_test;

#[cfg(test)]
macro_rules! test_async {
    ($test_name:ident; $test:expr) => {
        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen_test::wasm_bindgen_test]
        #[cfg(target_arch = "wasm32")]
        fn $test_name() {
            async_std::task::block_on(async {
                $test.await;
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[test]
        #[cfg(not(target_arch = "wasm32"))]
        fn $test_name() {
            async_std::task::block_on(async {
                $test.await;
            });
        }
    };
}

#[cfg(test)]
pub(crate) use test_async;

#[cfg(test)]
macro_rules! assert_reg_value {
    [b; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(iced_x86::Register::from($reg).is_gpr8(), "Register must be 8 bit wide");
        let val = $axecutor.reg_read_8(wrap).expect("could not read 8-bit register") as u8;
        assert_eq!(
            val as u8, $value as u8,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, val
        );
    };
    [w; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(iced_x86::Register::from($reg).is_gpr16(), "Register must be 16 bit wide");
        let val = $axecutor.reg_read_16(wrap).expect("could not read 16-bit register") as u16;
        assert_eq!(
            val, $value as u16,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, val
        );
    };
    [d; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(iced_x86::Register::from($reg).is_gpr32(), "Register must be 32 bit wide");
        let val = $axecutor.reg_read_32(wrap).expect("could not read 32-bit register") as u32;
        assert_eq!(
            val, $value as u32,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, val
        );
    };
    [q; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(iced_x86::Register::from($reg).is_gpr64() || iced_x86::Register::from($reg).is_ip(), "Register must be 64 bit wide");
        let val = $axecutor.reg_read_64(wrap).expect("could not read 64-bit register");
        assert_eq!(
            val, $value as u64,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, val
        );
    };
    [x; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(iced_x86::Register::from($reg).is_xmm(), "Register must be 128 bit wide");
        let val = $axecutor.reg_read_128(wrap).expect("could not read 128-bit register");
        assert_eq!(
            val, $value as u128,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, val
        );
    };
}

#[cfg(test)]
pub(crate) use assert_reg_value;

#[cfg(test)]
macro_rules! assert_mem_value {
    [b; $axecutor:expr; $addr:expr; $value:expr] => {
        let val = $axecutor.mem_read_8($addr).expect("could not read 8-bit memory") as u8;
        assert_eq!(
            val, $value as u8,
            "expected memory at {:#x} to have value {:#x}, but got {:#x}",
            $addr, $value, val
        );
    };
    [w; $axecutor:expr; $addr:expr; $value:expr] => {
        let val = $axecutor.mem_read_16($addr).expect("could not read 16-bit memory") as u16;
        assert_eq!(
            val, $value as u16,
            "expected memory at {:#x} to have value {:#x}, but got {:#x}",
            $addr, $value, val
        );
    };
    [d; $axecutor:expr; $addr:expr; $value:expr] => {
        let val = $axecutor.mem_read_32($addr).expect("could not read 32-bit memory") as u32;
        assert_eq!(
            val, $value as u32,
            "expected memory at {:#x} to have value {:#x}, but got {:#x}",
            $addr, $value, val
        );
    };
    [q; $axecutor:expr; $addr:expr; $value:expr] => {
        let val = $axecutor.mem_read_64($addr).expect("could not read 64-bit memory");
        assert_eq!(
            val, $value as u64,
            "expected memory at {:#x} to have value {:#x}, but got {:#x}",
            $addr, $value, val
        );
    };
    [x; $axecutor:expr; $addr:expr; $value:expr] => {
        let val : u128 = $axecutor.mem_read_128($addr).expect("could not read 128-bit memory");
        assert_eq!(
            val, $value as u128,
            "expected memory at {:#x} to have value {:#x}, but got {:#x}",
            $addr, $value, val
        );
    };
}

#[cfg(test)]
pub(crate) use assert_mem_value;

#[cfg(test)]
macro_rules! write_reg_value {
    (b; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(
            iced_x86::Register::from($reg).is_gpr8(),
            "Register must be 8 bit wide"
        );
        $axecutor
            .reg_write_8(wrap, $value as u64)
            .expect("could not write 8-bit register");
    };
    (w; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(
            iced_x86::Register::from($reg).is_gpr16(),
            "Register must be 16 bit wide"
        );
        $axecutor
            .reg_write_16(wrap, $value as u64)
            .expect("could not write 16-bit register");
    };
    (d; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(
            iced_x86::Register::from($reg).is_gpr32(),
            "Register must be 32 bit wide"
        );
        $axecutor
            .reg_write_32(wrap, $value as u64)
            .expect("could not write 32-bit register");
    };
    (q; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(
            iced_x86::Register::from($reg).is_gpr64(),
            "Register must be 64 bit wide"
        );
        $axecutor
            .reg_write_64(wrap, $value as u64)
            .expect("could not write 64-bit register");
    };
    (x; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = $crate::state::registers::SupportedRegister::from($reg);
        assert!(
            iced_x86::Register::from($reg).is_xmm(),
            "Register must be 128 bit wide"
        );
        $axecutor
            .reg_write_128(wrap, $value)
            .expect("could not write 128-bit register");
    };
}

#[cfg(test)]
pub(crate) use write_reg_value;

#[cfg(test)]
macro_rules! write_flags {
    ($axecutor:expr; $flags:expr) => {
        $axecutor.state.rflags = $flags;
    };
}

#[cfg(test)]
pub(crate) use write_flags;

#[cfg(test)]
macro_rules! code_with_nops {
    ($($bytes:expr),*; $count:expr; $($bytes2:expr),*) => {
        {
            // Concatenate bytes, then add count times 0x90 (nop), then the rest of bytes 2
            let mut bytes = vec![$($bytes),*];
            for _ in 0..$count {
                bytes.push(0x90);
            }
            bytes.extend(vec![$($bytes2),*]);
            bytes
        }
    };
}

#[cfg(test)]
pub(crate) use code_with_nops;

#[cfg(test)]
macro_rules! jmp_test {
    [$name:ident; start: $initial_rip:expr; end: $final_rip:expr; $($bytes_start:expr),*; $count:expr; $($bytes_end:expr),*; ($flags_to_set:expr; $flags_not_to_set:expr)] => {
        jmp_test![$name; start: $initial_rip; end: $final_rip;
        $($bytes_start),*; $count; $($bytes_end),*;
        |_ax| {}; |_ax| {};
        ($flags_to_set; $flags_not_to_set)];
    };
    [$name:ident; start: $initial_rip:expr; end: $final_rip:expr; $($bytes_start:expr),*; $count:expr; $($bytes_end:expr),*; $asserts:expr; ($flags_to_set:expr; $flags_not_to_set:expr)] => {
        jmp_test![$name; start: $initial_rip; end: $final_rip;
        $($bytes_start),*; $count; $($bytes_end),*;
        |_ax| {}; $asserts;
        ($flags_to_set; $flags_not_to_set)];
    };
    [$name:ident; start: $initial_rip:expr; end: $final_rip:expr; $($bytes_start:expr),*; $count:expr; $($bytes_end:expr),*; $setup:expr; $asserts:expr; ($flags_to_set:expr; $flags_not_to_set:expr)] => {
        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen_test::wasm_bindgen_test]
        #[cfg(target_arch = "wasm32")]
        fn $name() {
            async_std::task::block_on(async {
                use $crate::helpers::errors::AxError;
                use $crate::helpers::tests::code_with_nops;
                use $crate::helpers::macros::fatal_error;
                use $crate::axecutor::Axecutor;
                use $crate::helpers::tests::assert_reg_value;
                use iced_x86::Register::*;

                let bytes = code_with_nops!($($bytes_start),*; $count; $($bytes_end),*);

                let mut ax = Axecutor::new(&bytes, $initial_rip, $initial_rip).expect("Failed to create axecutor");
                $setup(&mut ax);

                assert_reg_value!(q; ax; RIP; $initial_rip);

                if let Err(e) = ax.execute().await {
                    fatal_error!("Failed to execute: {:?}", AxError::from(e));
                }

                assert_reg_value!(q; ax; RIP; $final_rip);

                let flags = ax.state.rflags;

                $asserts(ax);

                // Check flags
                use $crate::state::flags::*;
                for flag in FLAG_LIST {
                    // If the flag should be set, it must be != 0
                    #[allow(clippy::bad_bit_mask)]
                    if $flags_to_set & flag != 0 {
                        assert!(flags & flag != 0, "FLAG_{} should be set, but wasn't", FLAG_TO_NAMES.get(&flag).expect("Flag not found"));
                    }

                    #[allow(clippy::bad_bit_mask)]
                    if $flags_not_to_set & flag != 0 {
                        assert!(flags & flag == 0, "FLAG_{} should not be set, but was", FLAG_TO_NAMES.get(&flag).expect("Flag not found"));
                    }
                }


                #[cfg(all(target_arch = "wasm32", test))]
                return Ok::<(), AxError>(());
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[test]
        #[cfg(not(target_arch = "wasm32"))]
        fn $name() {
            async_std::task::block_on(async {
                use $crate::helpers::errors::AxError;
                use $crate::helpers::tests::code_with_nops;
                use $crate::helpers::macros::fatal_error;
                use $crate::axecutor::Axecutor;
                use $crate::helpers::tests::assert_reg_value;
                use iced_x86::Register::*;

                let bytes = code_with_nops!($($bytes_start),*; $count; $($bytes_end),*);

                let mut ax = Axecutor::new(&bytes, $initial_rip, $initial_rip).expect("Failed to create axecutor");
                $setup(&mut ax);

                assert_reg_value!(q; ax; RIP; $initial_rip);

                if let Err(e) = ax.execute().await {
                    fatal_error!("Failed to execute: {:?}", AxError::from(e));
                }

                assert_reg_value!(q; ax; RIP; $final_rip);

                let flags = ax.state.rflags;

                $asserts(ax);

                // Check flags
                use $crate::state::flags::*;
                for flag in FLAG_LIST {
                    // If the flag should be set, it must be != 0
                    #[allow(clippy::bad_bit_mask)]
                    if $flags_to_set & flag != 0 {
                        assert!(flags & flag != 0, "FLAG_{} should be set, but wasn't", FLAG_TO_NAMES.get(&flag).expect("Flag not found"));
                    }

                    #[allow(clippy::bad_bit_mask)]
                    if $flags_not_to_set & flag != 0 {
                        assert!(flags & flag == 0, "FLAG_{} should not be set, but was", FLAG_TO_NAMES.get(&flag).expect("Flag not found"));
                    }
                }


                #[cfg(all(target_arch = "wasm32", test))]
                return Ok(());
            });
        }
    };
}

#[cfg(test)]
pub(crate) use jmp_test;
