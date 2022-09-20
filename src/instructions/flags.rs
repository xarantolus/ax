use std::collections::HashMap;

extern crate lazy_static;
use lazy_static::lazy_static;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::instructions::axecutor::Axecutor;

// See Figure 3-8. EFLAGS Register in Intel SDM
pub const FLAG_CF: u64 = 0x0001; // Carry Flag
pub const FLAG_PF: u64 = 0x0004; // Parity Flag
pub const FLAG_AF: u64 = 0x0010; // Auxiliary Carry Flag
pub const FLAG_ZF: u64 = 0x0040; // Zero Flag
pub const FLAG_SF: u64 = 0x0080; // Sign Flag
pub const FLAG_TF: u64 = 0x0100; // Trap Flag
pub const FLAG_IF: u64 = 0x0200; // Interrupt Enable Flag
pub const FLAG_DF: u64 = 0x0400; // Direction Flag
pub const FLAG_OF: u64 = 0x0800; // Overflow Flag
pub const FLAG_IOPL: u64 = 0x3000; // I/O Privilege Level (2 bits)
pub const FLAG_NT: u64 = 0x4000; // Nested Task
pub const FLAG_RF: u64 = 0x10000; // Resume Flag
pub const FLAG_VM: u64 = 0x20000; // Virtual 8086 Mode
pub const FLAG_AC: u64 = 0x40000; // Alignment Check/Access Control
pub const FLAG_VIF: u64 = 0x80000; // Virtual Interrupt Flag
pub const FLAG_VIP: u64 = 0x100000; // Virtual Interrupt Pending
pub const FLAG_ID: u64 = 0x200000; // ID Flag

pub const FLAGS_UNAFFECTED: u64 = 0x7fffffffffffffff;

#[cfg(test)]
pub(crate) const FLAG_LIST: [u64; 17] = [
    FLAG_CF, FLAG_PF, FLAG_AF, FLAG_ZF, FLAG_SF, FLAG_TF, FLAG_IF, FLAG_DF, FLAG_OF, FLAG_IOPL,
    FLAG_NT, FLAG_RF, FLAG_VM, FLAG_AC, FLAG_VIF, FLAG_VIP, FLAG_ID,
];

lazy_static! {
    pub(crate) static ref FLAG_TO_NAMES: HashMap<u64, &'static str> = [
        (FLAG_CF, "CF"),
        (FLAG_PF, "PF"),
        (FLAG_AF, "AF"),
        (FLAG_ZF, "ZF"),
        (FLAG_SF, "SF"),
        (FLAG_TF, "TF"),
        (FLAG_IF, "IF"),
        (FLAG_DF, "DF"),
        (FLAG_OF, "OF"),
        (FLAG_IOPL, "IOPL"),
        (FLAG_NT, "NT"),
        (FLAG_RF, "RF"),
        (FLAG_VM, "VM"),
        (FLAG_AC, "AC"),
        (FLAG_VIF, "VIF"),
        (FLAG_VIP, "VIP"),
        (FLAG_ID, "ID"),
    ]
    .iter()
    .cloned()
    .collect();
}

macro_rules! set_flags {
	[$type:ident; $type_size:expr] => {
			|a: &mut Axecutor, flags_to_set: u64, flags_to_clear: u64, result: $type| {
				if flags_to_set == FLAGS_UNAFFECTED {
					return;
				}

				// Clear flags we might set now
				let mut new_flags = a.state.rflags & !flags_to_set & !flags_to_clear;

				// Carry Flag is defined by caller
				if flags_to_set & FLAG_CF != 0 {
					new_flags |= FLAG_CF;
				}
				// Overflow also defined by caller
				if flags_to_set & FLAG_OF != 0 {
					new_flags |= FLAG_OF;
				}
				// Auxiliary Carry Flag is defined by caller
				if flags_to_set & FLAG_AF != 0 {
					new_flags |= FLAG_AF;
				}
				// If zero, set ZF
				if result == 0 {
					new_flags |= FLAG_ZF;
				}

				// If signed, set SF
				if result & (1<<($type_size-1)) != 0 {
					new_flags |= FLAG_SF;
				}

				// PF = 1 if number of set bits in lowest byte of result is even
				if flags_to_set & FLAG_PF != 0 {
					let mut count = 0;

					for i in 0..8 {
						if result & (1 << i) != 0 {
							count += 1;
						}
					}

					// If even number of bits set, set PF
					if count % 2 == 0 {
						new_flags |= FLAG_PF;
					}
				}


				// Fail if any other flags should be set
				assert_eq!(
					flags_to_set & FLAG_TF,
					0,
					"flags: set_flags: FLAG_TF not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_IF,
					0,
					"flags: set_flags: FLAG_IF not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_DF,
					0,
					"flags: set_flags: FLAG_DF not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_IOPL,
					0,
					"flags: set_flags: FLAG_IOPL not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_NT,
					0,
					"flags: set_flags: FLAG_NT not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_RF,
					0,
					"flags: set_flags: FLAG_RF not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_VM,
					0,
					"flags: set_flags: FLAG_VM not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_AC,
					0,
					"flags: set_flags: FLAG_AC not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_VIF,
					0,
					"flags: set_flags: FLAG_VIF not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_VIP,
					0,
					"flags: set_flags: FLAG_VIP not implemented"
				);
				assert_eq!(
					flags_to_set & FLAG_ID,
					0,
					"flags: set_flags: FLAG_ID not implemented"
				);

				debug_assert!(
					!(new_flags & FLAG_ZF != 0) || (new_flags & FLAG_PF != 0),
					"flags: set_flags: ZF set (so number of bits set is 0 => even) but PF (\"even number of bits\") not set; this does not make sense"
				);

				a.state.rflags = new_flags;
		}
	};
}

#[wasm_bindgen]
impl Axecutor {
    pub fn set_flags_u8(&mut self, flags_to_set: u64, flags_to_clear: u64, result: u8) {
        set_flags!(u8; 8)(self, flags_to_set, flags_to_clear, result);
    }

    pub fn set_flags_u16(&mut self, flags_to_set: u64, flags_to_clear: u64, result: u16) {
        set_flags!(u16; 16)(self, flags_to_set, flags_to_clear, result);
    }

    pub fn set_flags_u32(&mut self, flags_to_set: u64, flags_to_clear: u64, result: u32) {
        set_flags!(u32; 32)(self, flags_to_set, flags_to_clear, result);
    }

    pub fn set_flags_u64(&mut self, flags_to_set: u64, flags_to_clear: u64, result: u64) {
        set_flags!(u64; 64)(self, flags_to_set, flags_to_clear, result);
    }
}
