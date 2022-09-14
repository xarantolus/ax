use super::axecutor::MachineState;

mod test {
    use crate::instructions::axecutor::{Axecutor, MachineState};
    use crate::instructions::registers::RegisterWrapper;
    use iced_x86::{Register, Register::*};

    impl Axecutor {
        pub(crate) fn run_bytes(bytes: &[u8]) -> MachineState {
            // Create new axecutor
            let mut ax = Axecutor::new(bytes, 0x1000).unwrap();

            ax.execute().unwrap();

            ax.state
        }
    }
}

#[macro_export]
macro_rules! ax_test {
    [$test_name:ident; $($bytes:expr),*; $asserts:expr] => {
		#[test]
		fn $test_name () {
			let bytes = vec![$($bytes),*];
			let state = Axecutor::run_bytes(&bytes);

			$asserts(state);
		}
    }
}

#[macro_export]
macro_rules! assert_reg_value {
        [$state:expr; $reg:expr; $value:expr] => {
            let val = $state.registers.get(&RegisterWrapper::from($reg)).unwrap();
            assert_eq!(
                val, &$value,
                "expected register $reg to have value $value, but got {}",
                val
            );
        };
    }
