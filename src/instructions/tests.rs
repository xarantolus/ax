#[cfg(test)]
use super::axecutor::Axecutor;

#[inline(never)]
#[cfg(test)]
pub(crate) fn ax_test_runner<S, A>(
    bytes: &[u8],
    mut setup: S,
    mut asserts: A,
    flags_to_set: u64,
    flags_not_to_set: u64,
) where
    S: FnMut(&mut Axecutor) -> () + 'static,
    A: FnMut(Axecutor) -> () + 'static,
{
    let copy = bytes.to_vec();

    async_std::task::block_on(async move {
        use crate::instructions::errors::AxError;
        use rand::Rng;

        // Always use a random rip, but make sure it doesn't overlap with memory that is often allocated at 0x1000 in tests
        let random_rip = rand::thread_rng().gen::<u64>() & 0x0000_ffff_ffff_ffff | 0xf0000;

        let mut ax =
            Axecutor::new(&*copy, random_rip, random_rip).expect("Failed to create axecutor");

        setup(&mut ax);

        match ax.execute().await {
            Err(e) => panic!("Failed to execute: {:?}", AxError::from(e)),
            _ => {}
        };

        let flags = ax.state.rflags;

        asserts(ax);

        // Check flags
        use crate::instructions::flags::*;
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

#[macro_export]
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
            use crate::instructions::flags::*;
            crate::instructions::tests::ax_test_runner(&[$($bytes),*], $setup, $asserts, $flags_to_set, $flags_not_to_set);
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[test]
        #[cfg(not(target_arch = "wasm32"))]
        fn $test_name () {
            #[allow(unused_imports)] // Some tests use these, some don't
            use crate::instructions::flags::*;
            crate::instructions::tests::ax_test_runner(&[$($bytes),*], $setup, $asserts, $flags_to_set, $flags_not_to_set);
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

#[macro_export]
#[cfg(test)]
macro_rules! test_async {
    ($test_name:ident; $test:expr) => {
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

#[macro_export]
#[cfg(test)]
macro_rules! assert_reg_value {
    [b; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr8(), "Register must be 8 bit wide");
        let val = $axecutor.reg_read_8(wrap) as u8;
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
    [w; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr16(), "Register must be 16 bit wide");
        let val = $axecutor.reg_read_16(wrap) as u16;
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
    [d; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr32(), "Register must be 32 bit wide");
        let val = $axecutor.reg_read_32(wrap) as u32;
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
    [q; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr64() || $reg.is_ip(), "Register must be 64 bit wide");
        let val = $axecutor.reg_read_64(wrap);
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
}

#[macro_export]
#[cfg(test)]
macro_rules! write_reg_value {
    (b; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr8(), "Register must be 8 bit wide");
        $axecutor.reg_write_8(wrap, $value as u64);
    };
    (w; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr16(), "Register must be 16 bit wide");
        $axecutor.reg_write_16(wrap, $value as u64);
    };
    (d; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr32(), "Register must be 32 bit wide");
        $axecutor.reg_write_32(wrap, $value as u64);
    };
    (q; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr64(), "Register must be 64 bit wide");
        $axecutor.reg_write_64(wrap, $value as u64);
    };
}

#[macro_export]
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

#[macro_export]
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
        #[cfg(all(target_arch = "wasm32", test))]
        #[wasm_bindgen_test]
        #[cfg(not(all(target_arch = "wasm32", test)))]
        #[test]
        fn $name() {
            async_std::task::block_on(async {
                use crate::instructions::errors::AxError;
                use crate::code_with_nops;
                use crate::fatal_error;

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
                use crate::instructions::flags::*;
                for flag in FLAG_LIST {
                    // If the flag should be set, it must be != 0
                    if $flags_to_set & flag != 0 {
                        assert!(flags & flag != 0, "FLAG_{} should be set, but wasn't", FLAG_TO_NAMES.get(&flag).expect("Flag not found"));
                    }

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
