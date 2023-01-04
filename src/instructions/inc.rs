use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Inc;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;

use crate::calculate_rm;
use crate::{fatal_error, opcode_unimplemented};

impl Axecutor {
    pub fn mnemonic_inc(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Inc);

        match i.code() {
            Inc_r16 => self.instr_inc_r16(i),
            Inc_r32 => self.instr_inc_r32(i),
            Inc_rm8 => self.instr_inc_rm8(i),
            Inc_rm16 => self.instr_inc_rm16(i),
            Inc_rm32 => self.instr_inc_rm32(i),
            Inc_rm64 => self.instr_inc_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Inc", i.code()),
        }
    }

    /// INC r16
    ///
    /// o16 40+rw
    fn instr_inc_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Inc_r16);

        opcode_unimplemented!("instr_inc_r16 for Inc")
    }

    /// INC r32
    ///
    /// o32 40+rd
    fn instr_inc_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Inc_r32);

        opcode_unimplemented!("instr_inc_r32 for Inc")
    }

    /// INC r/m8
    ///
    /// FE /0
    fn instr_inc_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Inc_rm8);

        calculate_rm![u8f; self; i; |val: u8| {
            let result = val.wrapping_add(1);
            (
                result,
                if val & 0x80 == 0 && result & 0x80 != 0 { FLAG_OF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// INC r/m16
    ///
    /// o16 FF /0
    fn instr_inc_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Inc_rm16);

        calculate_rm![u16f; self; i; |val: u16| {
            let result = val.wrapping_add(1);
            (
                result,
                if val & 0x8000 == 0 && result & 0x8000 != 0 { FLAG_OF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// INC r/m32
    ///
    /// o32 FF /0
    fn instr_inc_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Inc_rm32);

        calculate_rm![u32f; self; i; |val: u32| {
            let result = val.wrapping_add(1);
            (
                result,
                if val & 0x8000_0000 == 0 && result & 0x8000_0000 != 0 { FLAG_OF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// INC r/m64
    ///
    /// o64 FF /0
    fn instr_inc_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Inc_rm64);

        calculate_rm![u64f; self; i; |val: u64| {
            let result = val.wrapping_add(1);
            (
                result,
                if val & 0x8000_0000_0000_0000 == 0 && result & 0x8000_0000_0000_0000 != 0 { FLAG_OF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // inc al
    ax_test![inc_al; 0xfe, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc al
    ax_test![inc_al_pf; 0xfe, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc al
    ax_test![inc_al_pf_sf; 0xfe, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x81);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // inc al
    ax_test![inc_al_pf_zf; 0xfe, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // inc al
    ax_test![inc_al_sf; 0xfe, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x82);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x83);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // inc al
    ax_test![inc_al_sf_of; 0xfe, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // inc ax
    ax_test![inc_ax; 0x66, 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc ax
    ax_test![inc_ax_pf; 0x66, 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc ax
    ax_test![inc_ax_pf_sf_of; 0x66, 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // inc ax
    ax_test![inc_ax_sf; 0x66, 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8001);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // inc eax
    ax_test![inc_eax; 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc eax
    ax_test![inc_eax_pf; 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc eax
    ax_test![inc_eax_pf_sf_of; 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // inc eax
    ax_test![inc_eax_sf; 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000001u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // inc rax
    ax_test![inc_rax; 0x48, 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc rax
    ax_test![inc_rax_pf; 0x48, 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc rax
    ax_test![inc_rax_pf_sf; 0x48, 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xd48a37b76cee1adcu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xd48a37b76cee1addu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // inc rax
    ax_test![inc_rax_pf_sf_of; 0x48, 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // inc rax
    ax_test![inc_rax_sf; 0x48, 0xff, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000001u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // inc word ptr [r12]
    ax_test![inc_word_ptr_r12; 0x66, 0x41, 0xff, 0x4, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R12; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R12; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc word ptr [r12]
    ax_test![inc_word_ptr_r12_pf; 0x66, 0x41, 0xff, 0x4, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R12; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R12; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // inc word ptr [r12]
    ax_test![inc_word_ptr_r12_pf_sf_of; 0x66, 0x41, 0xff, 0x4, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R12; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R12; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x8000);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // inc word ptr [r12]
    ax_test![inc_word_ptr_r12_sf; 0x66, 0x41, 0xff, 0x4, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R12; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R12; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x8001);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];
}
