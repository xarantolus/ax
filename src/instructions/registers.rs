extern crate lazy_static;

use iced_x86::{Register, Register::*};
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

use super::axecutor::Axecutor;

lazy_static! {
    pub(crate) static ref REGISTER_TO_QWORD: HashMap<Register, Register> =
        [
            // 8-bit registers
            (AH, RAX),
            (AL, RAX),
            (BH, RBX),
            (BL, RBX),
            (CH, RCX),
            (CL, RCX),
            (DH, RDX),
            (DL, RDX),
            (SIL, RSI),
            (DIL, RDI),
            (SPL, RSP),
            (BPL, RBP),
            (R8L, R8),
            (R9L, R9),
            (R10L, R10),
            (R11L, R11),
            (R12L, R12),
            (R13L, R13),
            (R14L, R14),
            (R15L, R15),
            // 16-bit registers
            (AX, RAX),
            (BX, RBX),
            (CX, RCX),
            (DX, RDX),
            (SI, RSI),
            (DI, RDI),
            (SP, RSP),
            (BP, RBP),
            (R8W, R8),
            (R9W, R9),
            (R10W, R10),
            (R11W, R11),
            (R12W, R12),
            (R13W, R13),
            (R14W, R14),
            (R15W, R15),
            // 32-bit registers
            (EAX, RAX),
            (EBX, RBX),
            (ECX, RCX),
            (EDX, RDX),
            (ESI, RSI),
            (EDI, RDI),
            (ESP, RSP),
            (EBP, RBP),
            (R8D, R8),
            (R9D, R9),
            (R10D, R10),
            (R11D, R11),
            (R12D, R12),
            (R13D, R13),
            (R14D, R14),
            (R15D, R15),
            // 64-bit registers, of course somewhat unnecessary to keep them in here.
            (RAX, RAX),
            (RBX, RBX),
            (RCX, RCX),
            (RDX, RDX),
            (RSI, RSI),
            (RDI, RDI),
            (RSP, RSP),
            (RBP, RBP),
            (R8, R8),
            (R9, R9),
            (R10, R10),
            (R11, R11),
            (R12, R12),
            (R13, R13),
            (R14, R14),
            (R15, R15),
    ].iter().copied().collect();

    pub(crate) static ref HIGHER_BYTE_REGISTERS: HashSet<Register> = [
        AH, BH, CH, DH
    ].iter().copied().collect();
}

pub(crate) fn empty_register_set() -> HashMap<Register, u64> {
    let mut map = HashMap::new();

    map.insert(Register::RAX, 0);
    map.insert(Register::RBX, 0);
    map.insert(Register::RCX, 0);
    map.insert(Register::RDX, 0);
    map.insert(Register::RSI, 0);
    map.insert(Register::RDI, 0);
    map.insert(Register::RBP, 0);
    map.insert(Register::RSP, 0);
    map.insert(Register::R8, 0);
    map.insert(Register::R9, 0);
    map.insert(Register::R10, 0);
    map.insert(Register::R11, 0);
    map.insert(Register::R12, 0);
    map.insert(Register::R13, 0);
    map.insert(Register::R14, 0);
    map.insert(Register::R15, 0);
    map.insert(Register::RIP, 0);

    return map;
}

impl Axecutor {
    pub fn reg_write_8(&mut self, reg: Register, value: u8) {
        assert!(reg.is_gpr8());

        // Map 8-bit register to 64-bit register that it is part of
        let qword_register = REGISTER_TO_QWORD.get(&reg).unwrap();

        // Depending on the register, we either set the lowest or second lowest byte
        let is_high = HIGHER_BYTE_REGISTERS.contains(&qword_register);
        let reg_value = self.registers.get(&qword_register).unwrap().clone();

        let result_value: u64 = if is_high {
            (reg_value & 0xFFFF_FFFF_FFFF_00FF) | ((value as u64) << 8)
        } else {
            (reg_value & 0xFFFF_FFFF_FFFF_FF00) | (value as u64)
        };

        self.registers.insert(*qword_register, result_value);
    }

    pub fn reg_write_16(&mut self, reg: Register, value: u16) {
        assert!(reg.is_gpr16());

        // Map 16-bit register to 64-bit register that it is part of
        let qword_register = REGISTER_TO_QWORD.get(&reg).unwrap();

        let reg_value = self.registers.get(&qword_register).unwrap().clone();

        let result_value = (reg_value & 0xFFFF_FFFF_FFFF_0000) | (value as u64);
        self.registers.insert(*qword_register, result_value);
    }

    pub fn reg_write_32(&mut self, reg: Register, value: u32) {
        assert!(reg.is_gpr32());

        // Map 32-bit register to 64-bit register that it is part of
        let qword_register = REGISTER_TO_QWORD.get(&reg).unwrap();

        // Intentionally cut off the upper 32bit, setting them to zero
        let result_value = value as u64;
        self.registers.insert(*qword_register, result_value);
    }

    pub fn reg_write_64(&mut self, reg: Register, value: u64) {
        assert!(reg.is_gpr64());

        self.registers.insert(reg, value);
    }
}
