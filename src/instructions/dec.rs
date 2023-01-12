use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Dec;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;

use crate::calculate_rm;
use crate::{fatal_error, opcode_unimplemented};

impl Axecutor {
    pub fn mnemonic_dec(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Dec);

        match i.code() {
            Dec_r16 => self.instr_dec_r16(i),
            Dec_r32 => self.instr_dec_r32(i),
            Dec_rm8 => self.instr_dec_rm8(i),
            Dec_rm16 => self.instr_dec_rm16(i),
            Dec_rm32 => self.instr_dec_rm32(i),
            Dec_rm64 => self.instr_dec_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Dec", i.code()),
        }
    }

    /// DEC r16
    ///
    /// o16 48+rw
    fn instr_dec_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Dec_r16);

        opcode_unimplemented!("instr_dec_r16 for Dec")
    }

    /// DEC r32
    ///
    /// o32 48+rd
    fn instr_dec_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Dec_r32);

        opcode_unimplemented!("instr_dec_r32 for Dec")
    }

    /// DEC r/m8
    ///
    /// FE /1
    fn instr_dec_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Dec_rm8);

        calculate_rm![u8f; self; i; |val: u8| {
            let result = val.wrapping_sub(1);
            (
                result,
                if val & 0x80 != 0 && result & 0x80 == 0 { FLAG_OF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// DEC r/m16
    ///
    /// o16 FF /1
    fn instr_dec_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Dec_rm16);

        calculate_rm![u16f; self; i; |val: u16| {
            let result = val.wrapping_sub(1);
            (
                result,
                if val & 0x8000 != 0 && result & 0x8000 == 0 { FLAG_OF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// DEC r/m32
    ///
    /// o32 FF /1
    fn instr_dec_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Dec_rm32);

        calculate_rm![u32f; self; i; |val: u32| {
            let result = val.wrapping_sub(1);
            (
                result,
                if val & 0x8000_0000 != 0 && result & 0x8000_0000 == 0 { FLAG_OF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// DEC r/m64
    ///
    /// o64 FF /1
    fn instr_dec_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Dec_rm64);

        calculate_rm![u64f; self; i; |val: u64| {
            let result = val.wrapping_sub(1);
            (
                result,
                if val & 0x8000_0000_0000_0000 != 0 && result & 0x8000_0000_0000_0000 == 0 { FLAG_OF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_mem_value, assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // dec al
    ax_test![dec_al; 0xfe, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec al
    ax_test![dec_al_of; 0xfe, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // dec al
    ax_test![dec_al_pf; 0xfe, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec al
    ax_test![dec_al_pf_sf; 0xfe, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // dec al
    ax_test![dec_al_pf_zf; 0xfe, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // dec al
    ax_test![dec_al_sf; 0xfe, 0xc8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xfe);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // dec bx
    ax_test![dec_bx; 0x66, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec bx
    ax_test![dec_bx_pf; 0x66, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec bx
    ax_test![dec_bx_pf_of; 0x66, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7fff);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // dec bx
    ax_test![dec_bx_pf_sf; 0x66, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xffff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // dec bx
    ax_test![dec_bx_pf_zf; 0x66, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // dec ebx
    ax_test![dec_ebx; 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec ebx
    ax_test![dec_ebx_pf; 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec ebx
    ax_test![dec_ebx_pf_of; 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // dec ebx
    ax_test![dec_ebx_pf_sf; 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xffffffffu32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // dec ebx
    ax_test![dec_ebx_pf_zf; 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // dec r11
    ax_test![dec_r11; 0x49, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec r11
    ax_test![dec_r11_pf; 0x49, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec r11
    ax_test![dec_r11_pf_of; 0x49, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x7fffffffffffffffu64);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // dec r11
    ax_test![dec_r11_pf_sf; 0x49, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xffffffffffffffffu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // dec r11
    ax_test![dec_r11_pf_zf; 0x49, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // dec r11
    ax_test![dec_r11_sf; 0x49, 0xff, 0xcb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0xbf799dfe56bbb3bdu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xbf799dfe56bbb3bcu64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // dec word ptr [r12]
    ax_test![dec_word_ptr_r12; 0x66, 0x41, 0xff, 0xc, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R12; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R12; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec word ptr [r12]
    ax_test![dec_word_ptr_r12_pf; 0x66, 0x41, 0xff, 0xc, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R12; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R12; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // dec word ptr [r12]
    ax_test![dec_word_ptr_r12_pf_of; 0x66, 0x41, 0xff, 0xc, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R12; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R12; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x7fff);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // dec word ptr [r12]
    ax_test![dec_word_ptr_r12_pf_sf; 0x66, 0x41, 0xff, 0xc, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R12; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R12; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0xffff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // dec word ptr [r12]
    ax_test![dec_word_ptr_r12_pf_zf; 0x66, 0x41, 0xff, 0xc, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R12; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R12; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
