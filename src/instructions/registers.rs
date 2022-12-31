extern crate lazy_static;

use iced_x86::{Register, Register::*};

use lazy_static::lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::debug_log;

use super::axecutor::Axecutor;

lazy_static! {
    pub(crate) static ref REGISTER_TO_QWORD: HashMap<SupportedRegister, SupportedRegister> =
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
    ].iter().map(|(a,b)| (SupportedRegister::from(*a), SupportedRegister::from(*b))).collect();

    pub(crate) static ref HIGHER_BYTE_REGISTERS: HashSet<SupportedRegister> = [
        AH, BH, CH, DH
    ].iter().map(|a| SupportedRegister::from(*a)).collect();

    pub(crate) static ref NATURAL_REGISTER_ORDER : Vec<SupportedRegister> = [
        RIP, RAX, RBX, RCX, RDX, RSI, RDI, RSP, RBP, R8, R9, R10, R11, R12, R13, R14, R15
    ].iter().map(|a| SupportedRegister::from(*a)).collect();
}

pub(crate) fn randomized_register_set(rip_value: u64) -> HashMap<SupportedRegister, u64> {
    let mut map = HashMap::new();

    let mut rng = rand::thread_rng();

    let registers = vec![
        RAX, RBX, RCX, RDX, RSI, RDI, RSP, RBP, R8, R9, R10, R11, R12, R13, R14, R15,
    ];

    for register in registers {
        let value = rng.gen::<u64>();
        map.insert(SupportedRegister::from(register), value & 0xffff_ffff);
    }

    map.insert(SupportedRegister::RIP, rip_value);

    return map;
}

#[wasm_bindgen(js_name = Register)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportedRegister {
    // 64-bit registers
    RIP,
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    // 32-bit registers
    EIP,
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
    R15D,
    // 16-bit registers
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    SP,
    BP,
    R8W,
    R9W,
    R10W,
    R11W,
    R12W,
    R13W,
    R14W,
    R15W,
    // 8-bit registers
    AH,
    AL,
    BH,
    BL,
    CH,
    CL,
    DH,
    DL,
    SIL,
    DIL,
    SPL,
    BPL,
    R8L,
    R9L,
    R10L,
    R11L,
    R12L,
    R13L,
    R14L,
    R15L,
}

impl From<Register> for SupportedRegister {
    fn from(register: Register) -> Self {
        match register {
            Register::RIP => SupportedRegister::RIP,
            Register::RAX => SupportedRegister::RAX,
            Register::RBX => SupportedRegister::RBX,
            Register::RCX => SupportedRegister::RCX,
            Register::RDX => SupportedRegister::RDX,
            Register::RSI => SupportedRegister::RSI,
            Register::RDI => SupportedRegister::RDI,
            Register::RSP => SupportedRegister::RSP,
            Register::RBP => SupportedRegister::RBP,
            Register::R8 => SupportedRegister::R8,
            Register::R9 => SupportedRegister::R9,
            Register::R10 => SupportedRegister::R10,
            Register::R11 => SupportedRegister::R11,
            Register::R12 => SupportedRegister::R12,
            Register::R13 => SupportedRegister::R13,
            Register::R14 => SupportedRegister::R14,
            Register::R15 => SupportedRegister::R15,

            Register::EIP => SupportedRegister::EIP,
            Register::EAX => SupportedRegister::EAX,
            Register::EBX => SupportedRegister::EBX,
            Register::ECX => SupportedRegister::ECX,
            Register::EDX => SupportedRegister::EDX,
            Register::ESI => SupportedRegister::ESI,
            Register::EDI => SupportedRegister::EDI,
            Register::ESP => SupportedRegister::ESP,
            Register::EBP => SupportedRegister::EBP,
            Register::R8D => SupportedRegister::R8D,
            Register::R9D => SupportedRegister::R9D,
            Register::R10D => SupportedRegister::R10D,
            Register::R11D => SupportedRegister::R11D,
            Register::R12D => SupportedRegister::R12D,
            Register::R13D => SupportedRegister::R13D,
            Register::R14D => SupportedRegister::R14D,
            Register::R15D => SupportedRegister::R15D,

            Register::AX => SupportedRegister::AX,
            Register::BX => SupportedRegister::BX,
            Register::CX => SupportedRegister::CX,
            Register::DX => SupportedRegister::DX,
            Register::SI => SupportedRegister::SI,
            Register::DI => SupportedRegister::DI,
            Register::SP => SupportedRegister::SP,
            Register::BP => SupportedRegister::BP,
            Register::R8W => SupportedRegister::R8W,
            Register::R9W => SupportedRegister::R9W,
            Register::R10W => SupportedRegister::R10W,
            Register::R11W => SupportedRegister::R11W,
            Register::R12W => SupportedRegister::R12W,
            Register::R13W => SupportedRegister::R13W,
            Register::R14W => SupportedRegister::R14W,
            Register::R15W => SupportedRegister::R15W,

            Register::AH => SupportedRegister::AH,
            Register::AL => SupportedRegister::AL,
            Register::BH => SupportedRegister::BH,
            Register::BL => SupportedRegister::BL,
            Register::CH => SupportedRegister::CH,
            Register::CL => SupportedRegister::CL,
            Register::DH => SupportedRegister::DH,
            Register::DL => SupportedRegister::DL,
            Register::SIL => SupportedRegister::SIL,
            Register::DIL => SupportedRegister::DIL,
            Register::SPL => SupportedRegister::SPL,
            Register::BPL => SupportedRegister::BPL,
            Register::R8L => SupportedRegister::R8L,
            Register::R9L => SupportedRegister::R9L,
            Register::R10L => SupportedRegister::R10L,
            Register::R11L => SupportedRegister::R11L,
            Register::R12L => SupportedRegister::R12L,
            Register::R13L => SupportedRegister::R13L,
            Register::R14L => SupportedRegister::R14L,
            Register::R15L => SupportedRegister::R15L,

            _ => panic!("Unsupported register"),
        }
    }
}

impl From<SupportedRegister> for Register {
    fn from(register: SupportedRegister) -> Self {
        match register {
            SupportedRegister::RIP => Register::RIP,
            SupportedRegister::RAX => Register::RAX,
            SupportedRegister::RBX => Register::RBX,
            SupportedRegister::RCX => Register::RCX,
            SupportedRegister::RDX => Register::RDX,
            SupportedRegister::RSI => Register::RSI,
            SupportedRegister::RDI => Register::RDI,
            SupportedRegister::RSP => Register::RSP,
            SupportedRegister::RBP => Register::RBP,
            SupportedRegister::R8 => Register::R8,
            SupportedRegister::R9 => Register::R9,
            SupportedRegister::R10 => Register::R10,
            SupportedRegister::R11 => Register::R11,
            SupportedRegister::R12 => Register::R12,
            SupportedRegister::R13 => Register::R13,
            SupportedRegister::R14 => Register::R14,
            SupportedRegister::R15 => Register::R15,
            SupportedRegister::EAX => Register::EAX,
            SupportedRegister::EBX => Register::EBX,
            SupportedRegister::ECX => Register::ECX,
            SupportedRegister::EDX => Register::EDX,
            SupportedRegister::ESI => Register::ESI,
            SupportedRegister::EDI => Register::EDI,
            SupportedRegister::ESP => Register::ESP,
            SupportedRegister::EBP => Register::EBP,
            SupportedRegister::AX => Register::AX,
            SupportedRegister::BX => Register::BX,
            SupportedRegister::CX => Register::CX,
            SupportedRegister::DX => Register::DX,
            SupportedRegister::SI => Register::SI,
            SupportedRegister::DI => Register::DI,
            SupportedRegister::SP => Register::SP,
            SupportedRegister::BP => Register::BP,
            SupportedRegister::AH => Register::AH,
            SupportedRegister::BH => Register::BH,
            SupportedRegister::CH => Register::CH,
            SupportedRegister::DH => Register::DH,
            SupportedRegister::SIL => Register::SIL,
            SupportedRegister::DIL => Register::DIL,
            SupportedRegister::SPL => Register::SPL,
            SupportedRegister::BPL => Register::BPL,
            SupportedRegister::R8D => Register::R8D,
            SupportedRegister::R9D => Register::R9D,
            SupportedRegister::R10D => Register::R10D,
            SupportedRegister::R11D => Register::R11D,
            SupportedRegister::R12D => Register::R12D,
            SupportedRegister::R13D => Register::R13D,
            SupportedRegister::R14D => Register::R14D,
            SupportedRegister::R15D => Register::R15D,
            SupportedRegister::R8W => Register::R8W,
            SupportedRegister::R9W => Register::R9W,
            SupportedRegister::R10W => Register::R10W,
            SupportedRegister::R11W => Register::R11W,
            SupportedRegister::R12W => Register::R12W,
            SupportedRegister::R13W => Register::R13W,
            SupportedRegister::R14W => Register::R14W,
            SupportedRegister::R15W => Register::R15W,
            SupportedRegister::R8L => Register::R8L,
            SupportedRegister::R9L => Register::R9L,
            SupportedRegister::R10L => Register::R10L,
            SupportedRegister::R11L => Register::R11L,
            SupportedRegister::R12L => Register::R12L,
            SupportedRegister::R13L => Register::R13L,
            SupportedRegister::R14L => Register::R14L,
            SupportedRegister::R15L => Register::R15L,
            SupportedRegister::AL => Register::AL,
            SupportedRegister::BL => Register::BL,
            SupportedRegister::CL => Register::CL,
            SupportedRegister::DL => Register::DL,
            _ => panic!("Unsupported register"),
        }
    }
}

impl SupportedRegister {
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

#[wasm_bindgen]
impl Axecutor {
    pub fn reg_write_8(&mut self, reg: SupportedRegister, value: u64) {
        assert!(
            value <= 0xFF,
            "reg_write_8: value {:x} is too large to fit in 8 bits",
            value
        );

        let r: Register = reg.into();
        assert!(r.is_gpr8(), "{:?} is not a valid 8-bit register", r);

        // Map 8-bit register to 64-bit register that it is part of
        let qword_register = REGISTER_TO_QWORD.get(&reg).unwrap();

        // Depending on the register, we either set the lowest or second lowest byte
        let is_high = HIGHER_BYTE_REGISTERS.contains(&reg);
        let reg_value = self.state.registers.get(&qword_register).unwrap().clone();

        let result_value: u64 = if is_high {
            (reg_value & 0xFFFF_FFFF_FFFF_00FF) | (value << 8)
        } else {
            (reg_value & 0xFFFF_FFFF_FFFF_FF00) | value
        };

        self.state.registers.insert(*qword_register, result_value);

        debug_log!(
            "Wrote 0x{:x} to {:?}, setting {:?} to 0x{:x} (previously 0x{:x})",
            value,
            reg,
            qword_register,
            result_value,
            reg_value
        );
    }

    pub fn reg_write_16(&mut self, reg: SupportedRegister, value: u64) {
        assert!(
            value <= 0xFFFF,
            "reg_write_16: value {:x} is too large to fit in 16 bits",
            value
        );

        let r: Register = reg.into();
        assert!(r.is_gpr16(), "{:?} is not a valid 16-bit register", r);

        // Map 16-bit register to 64-bit register that it is part of
        let qword_register = REGISTER_TO_QWORD.get(&reg).unwrap();

        let reg_value = self.state.registers.get(&qword_register).unwrap().clone();

        let result_value = (reg_value & 0xFFFF_FFFF_FFFF_0000) | value;
        self.state.registers.insert(*qword_register, result_value);

        debug_log!(
            "Wrote 0x{:x} to {:?}, setting {:?} to 0x{:x} (previously 0x{:x})",
            value,
            reg,
            qword_register,
            result_value,
            reg_value
        );
    }

    pub fn reg_write_32(&mut self, reg: SupportedRegister, value: u64) {
        assert!(
            value <= 0xFFFF_FFFF,
            "reg_write_32: value {:x} is too large to fit in 32 bits",
            value
        );

        let r: Register = reg.into();
        assert!(r.is_gpr32(), "{:?} is not a valid 32-bit register", r);

        // Map 32-bit register to 64-bit register that it is part of
        let qword_register = REGISTER_TO_QWORD.get(&reg).unwrap();

        // Intentionally cut off the upper 32bit, setting them to zero
        let result_value = value as u32 as u64;
        #[allow(unused_variables)]
        let old = self.state.registers.insert(*qword_register, result_value);

        debug_log!(
            "Wrote 0x{:x} to {:?}, setting {:?} to 0x{:x}{}",
            value,
            reg,
            qword_register,
            result_value,
            match old {
                Some(o) => format!(" (previously 0x{:x})", o),
                Option::None => "".to_string(),
            }
        );
    }

    pub fn reg_write_64(&mut self, reg: SupportedRegister, value: u64) {
        let r: Register = reg.into();
        assert!(
            r.is_gpr64() || r.is_ip(),
            "{:?} is not a valid 64-bit register",
            r
        );

        #[allow(unused_variables)]
        let old = self.state.registers.insert(reg, value);

        debug_log!(
            "Wrote 0x{:x} to {:?}{}",
            value,
            reg,
            match old {
                Some(o) => format!(" (previously 0x{:x})", o),
                Option::None => "".to_string(),
            }
        );
    }

    pub fn reg_read_8(&self, reg: SupportedRegister) -> u64 {
        let r: Register = reg.into();
        assert!(r.is_gpr8(), "{:?} is not a valid 8-bit register", r);

        // Map 8-bit register to 64-bit register that it is part of
        let qword_register = REGISTER_TO_QWORD.get(&reg).unwrap();

        // Depending on the register, we either get the lowest or second lowest byte
        let is_high = HIGHER_BYTE_REGISTERS.contains(&reg);
        let reg_value = self.state.registers.get(&qword_register).unwrap().clone();

        let result_value: u8 = if is_high {
            ((reg_value & 0xFF00) >> 8) as u8
        } else {
            (reg_value & 0xFF) as u8
        };

        debug_log!("Read value 0x{:x} from {:?}", result_value, reg);

        return result_value as u64;
    }

    pub fn reg_read_16(&self, reg: SupportedRegister) -> u64 {
        let r: Register = reg.into();
        assert!(r.is_gpr16(), "{:?} is not a valid 16-bit register", r);

        // Map 16-bit register to 64-bit register that it is part of
        let qword_register = REGISTER_TO_QWORD.get(&reg).unwrap();

        let reg_value = self.state.registers.get(&qword_register).unwrap().clone();

        let result_value = reg_value & 0xFFFF;

        debug_log!("Read value 0x{:x} from {:?}", result_value, reg);

        return result_value;
    }

    pub fn reg_read_32(&self, reg: SupportedRegister) -> u64 {
        let r: Register = reg.into();
        assert!(r.is_gpr32(), "{:?} is not a valid 32-bit register", r);

        // Map 32-bit register to 64-bit register that it is part of
        let qword_register = REGISTER_TO_QWORD.get(&reg).unwrap();

        let reg_value = self.state.registers.get(&qword_register).unwrap().clone();

        let result_value = reg_value & 0xFFFF_FFFF;

        debug_log!("Read value 0x{:x} from {:?}", result_value, reg);

        return result_value;
    }

    pub fn reg_read_64(&self, reg: SupportedRegister) -> u64 {
        let r: Register = reg.into();
        assert!(
            r.is_gpr64() || r.is_ip(),
            "{:?} is not a valid 64-bit register",
            r
        );

        let reg_value = self.state.registers.get(&reg).unwrap().clone();

        debug_log!("Read value 0x{:x} from {:?}", reg_value, reg);

        return reg_value;
    }
}
