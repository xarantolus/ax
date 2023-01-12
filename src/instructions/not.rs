use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Not;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::fatal_error;
use crate::instructions::flags::*;

use crate::calculate_rm;

impl Axecutor {
    pub fn mnemonic_not(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Not);

        match i.code() {
            Not_rm8 => self.instr_not_rm8(i),
            Not_rm16 => self.instr_not_rm16(i),
            Not_rm32 => self.instr_not_rm32(i),
            Not_rm64 => self.instr_not_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Not", i.code()),
        }
    }

    /// NOT r/m8
    ///
    /// F6 /2
    fn instr_not_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Not_rm8);

        calculate_rm![u8f; self; i; |s: u8| {
            (!s, 0)
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// NOT r/m16
    ///
    /// o16 F7 /2
    fn instr_not_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Not_rm16);

        calculate_rm![u16f; self; i; |s: u16| {
            (!s, 0)
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// NOT r/m32
    ///
    /// o32 F7 /2
    fn instr_not_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Not_rm32);

        calculate_rm![u32f; self; i; |s: u32| {
            (!s, 0)
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// NOT r/m64
    ///
    /// o64 F7 /2
    fn instr_not_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Not_rm64);

        calculate_rm![u64f; self; i; |s: u64| {
            (!s, 0)
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_mem_value, assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // not al
    ax_test![not_al_zero; 0xf6, 0xd0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // not al
    ax_test![not_al; 0xf6, 0xd0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x3);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xfc);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // not word ptr [rax]
    ax_test![not_word_ptr_rax; 0x66, 0xf7, 0x10;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0xffff);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // not eax
    ax_test![not_eax; 0xf7, 0xd0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffffffu32);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // not r11
    ax_test![not_r11; 0x49, 0xf7, 0xd3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xffffffffffffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
