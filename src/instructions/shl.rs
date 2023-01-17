use core::panic;

use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Shl;

use super::axecutor::Axecutor;
use super::errors::AxError;
use super::macros::calculate_rm_imm;
use crate::instructions::flags::*;

use crate::instructions::macros::calculate_rm_r;
use crate::instructions::macros::fatal_error;

impl Axecutor {
    pub fn mnemonic_shl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Shl);

        match i.code() {
            Shl_rm8_imm8 => self.instr_shl_rm8_imm8(i),
            Shl_rm16_imm8 => self.instr_shl_rm16_imm8(i),
            Shl_rm32_imm8 => self.instr_shl_rm32_imm8(i),
            Shl_rm64_imm8 => self.instr_shl_rm64_imm8(i),
            Shl_rm8_1 => self.instr_shl_rm8_1(i),
            Shl_rm16_1 => self.instr_shl_rm16_1(i),
            Shl_rm32_1 => self.instr_shl_rm32_1(i),
            Shl_rm64_1 => self.instr_shl_rm64_1(i),
            Shl_rm8_CL => self.instr_shl_rm8_cl(i),
            Shl_rm16_CL => self.instr_shl_rm16_cl(i),
            Shl_rm32_CL => self.instr_shl_rm32_cl(i),
            Shl_rm64_CL => self.instr_shl_rm64_cl(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Shl", i.code()),
        }
    }

    /// SHL r/m8, imm8
    ///
    /// C0 /4 ib
    fn instr_shl_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm8_imm8);

        calculate_rm_imm![u8f; self; i; |d: u8, s: u8| {
            assert_ne!(s, 1, "SHL r/m8, imm8 with immediate 1 should be handled by opcode SHL r/m8, 1");

            if s == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shl((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (0x80 >> ((s-1) & 0x1f)) == 0 {0} else {FLAG_CF};

                    (v, cf)
                },
                None => {
                    // Overflow flag is only defined for shifts of 1, which are handled by another opcode
                    (0, if s == 8 && d & 1 == 1 {FLAG_CF} else {0})}
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF)]
    }

    /// SHL r/m16, imm8
    ///
    /// o16 C1 /4 ib
    fn instr_shl_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm16_imm8);

        calculate_rm_imm![u16f; u8; self; i; |d: u16, s: u8| {
            assert_ne!(s, 1, "SHL r/m16, imm8 with immediate 1 should be handled by opcode SHL r/m16, 1");

            if (s&0x1f) == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shl((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (0x8000 >> ((s-1) & 0x1f)) == 0 {0} else {FLAG_CF};

                    (v, cf)
                },
                None => {
                    // Overflow flag is only defined for shifts of 1, which are handled by another opcode
                    (0, if s == 16 && d & 1 == 1 {FLAG_CF} else {0})}
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF)]
    }

    /// SHL r/m32, imm8
    ///
    /// o32 C1 /4 ib
    fn instr_shl_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm32_imm8);

        calculate_rm_imm![u32f; u8; self; i; |d: u32, s: u8| {
            assert_ne!(s, 1, "SHL r/m32, imm8 with immediate 1 should be handled by opcode SHL r/m32, 1");

            if (s&0x1f) == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shl((s&0x1f) as u32) {
                Some(v) => (
                    v,
                    if d & (0x80000000u32.wrapping_shr(((s-1) & 0x1f) as u32)) == 0 {0} else {FLAG_CF}
                ),
                None => {
                    panic!("u8 s & 0x1f should never be >=32");
                }
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF)]
    }

    /// SHL r/m64, imm8
    ///
    /// o64 C1 /4 ib
    fn instr_shl_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm64_imm8);

        calculate_rm_imm![u64f; u8; self; i; |d: u64, s: u8| {
            assert_ne!(s, 1, "SHL r/m64, imm8 with immediate 1 should be handled by opcode SHL r/m64, 1");

            if s&0x3f == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shl((s&0x3f) as u32) {
                Some(v) => (
                    v,
                    if d & (0x8000000000000000u64.wrapping_shr(((s-1) & 0x3f) as u32)) == 0 {0} else {FLAG_CF}
                ),
                None => {
                    panic!("u64 s & 0x1f should never be >=64");
                }
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF)]
    }

    /// SHL r/m8, 1
    ///
    /// D0 /4
    fn instr_shl_rm8_1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm8_1);

        calculate_rm_imm![u8f; self; i; |d: u8, s: u8| {
            debug_assert_eq!(s, 1, "SHL r/m8, 1: src is not 1");

            let cf = if d & 0x80 == 0 {0} else {FLAG_CF};
            // OF == 0 <=> Two top bits of rm operand were the same
            let of = if (d & 0x40 == 0) == (cf == 0) {0} else {FLAG_OF};

            (d.wrapping_shl(1), cf | of)
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// SHL r/m16, 1
    ///
    /// o16 D1 /4
    fn instr_shl_rm16_1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm16_1);

        calculate_rm_imm![u16f; u8; self; i; |d: u16, s: u8| {
            debug_assert_eq!(s, 1, "SHL r/m16, 1: src is not 1");

            let cf = if d & 0x8000 == 0 {0} else {FLAG_CF};
            // OF == 0 <=> Two top bits of rm operand were the same
            let of = if (d & 0x4000 == 0) == (cf == 0) {0} else {FLAG_OF};

            (d.wrapping_shl(1), cf | of)
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// SHL r/m32, 1
    ///
    /// o32 D1 /4
    fn instr_shl_rm32_1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm32_1);

        calculate_rm_imm![u32f; u8; self; i; |d: u32, s: u8| {
            debug_assert_eq!(s, 1, "SHL r/m32, 1: src is not 1");

            let cf = if d & 0x80000000 == 0 {0} else {FLAG_CF};
            // OF == 0 <=> Two top bits of rm operand were the same
            let of = if (d & 0x40000000 == 0) == (cf == 0) {0} else {FLAG_OF};

            (d.wrapping_shl(1), cf | of)
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// SHL r/m64, 1
    ///
    /// o64 D1 /4
    fn instr_shl_rm64_1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm64_1);

        calculate_rm_imm![u64f; u8; self; i; |d: u64, s: u8| {
            debug_assert_eq!(s, 1, "SHL r/m64, 1: src is not 1");

            let cf = if d & 0x8000000000000000 == 0 {0} else {FLAG_CF};
            // OF == 0 <=> Two top bits of rm operand were the same
            let of = if (d & 0x8000000000000000) == ((d & 0x4000000000000000)<<1) {0} else {FLAG_OF};

            (d.wrapping_shl(1), cf | of)
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// SHL r/m8, CL
    ///
    /// D2 /4
    fn instr_shl_rm8_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm8_CL);

        calculate_rm_r![u8f; self; i; |d: u8, s: u8| {
            if s&0x1f == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shl((s&0x1f) as u32) {
                Some(v) => (
                    v,
                    if d & (0x80u8.wrapping_shr(((s-1) & 0x1f) as u32)) == 0 {0} else {FLAG_CF} |
                    if (d & 0x40 == 0) == (d & 0x80 == 0) {0} else {FLAG_OF}
                ),
                None => {
                    (0, if s == 8 && d & 1 == 0 {0} else {FLAG_CF})
                }
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// SHL r/m16, CL
    ///
    /// o16 D3 /4
    fn instr_shl_rm16_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm16_CL);

        calculate_rm_r![u16f; u8; self; i; |d: u16, s: u8| {
            if s&0x1f == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shl((s&0x1f) as u32) {
                Some(v) => (
                    v,
                    if d & (0x8000u16.wrapping_shr(((s-1) & 0x1f) as u32)) == 0 {0} else {FLAG_CF} |
                    if (d & 0x4000 == 0) == (d & 0x8000 == 0) {0} else {FLAG_OF}
                ),
                None => {
                    (0, if s == 16 && d & 1 == 0 {0} else {FLAG_CF})
                }
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// SHL r/m32, CL
    ///
    /// o32 D3 /4
    fn instr_shl_rm32_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm32_CL);

        calculate_rm_r![u32f; u8; self; i; |d: u32, s: u8| {
            if s&0x1f == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shl((s&0x1f) as u32) {
                Some(v) => (
                    v,
                    if d & (0x80000000u32.wrapping_shr(((s-1) & 0x1f) as u32)) == 0 {0} else {FLAG_CF} |
                    if (d & 0x40000000 == 0) == (d & 0x80000000 == 0) {0} else {FLAG_OF}
                ),
                None => {
                    panic!("u8 s & 0x1f should never be >=32");
                }
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// SHL r/m64, CL
    ///
    /// o64 D3 /4
    fn instr_shl_rm64_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm64_CL);

        calculate_rm_r![u64f; u8; self; i; |d: u64, s: u8| {
            if s&0x3f == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shl((s&0x3f) as u32) {
                Some(v) => (
                    v,
                    if d & (0x8000000000000000u64.wrapping_shr(((s-1) & 0x3f) as u32)) == 0 {0} else {FLAG_CF} |
                    if (d & 0x8000000000000000) == ((d & 0x4000000000000000)<<1) {0} else {FLAG_OF}
                ),
                None => {
                    panic!("u8 s & 0x3f should never be >=64");
                }
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::axecutor::Axecutor;
    use crate::instructions::tests::{
        assert_mem_value, assert_reg_value, ax_test, write_flags, write_reg_value,
    };
    use iced_x86::Register::*;
    // shl al, cl
    ax_test![shl_al_cl_sf_of_cf; 0xd2, 0xe0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(b; a; CL; 0x7);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; CL; 0x7);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl al, cl
    ax_test![shl_al_cl_pf_zf_cf; 0xd2, 0xe0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; CL; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl al, cl
    ax_test![shl_al_cl_pf_cf; 0xd2, 0xe0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
            write_reg_value!(b; a; CL; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1e);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl bl, 0x0
    ax_test![shl_bl_0x0; 0xc0, 0xe3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl bl, 0x3
    ax_test![shl_bl_0x3_pf_zf; 0xc0, 0xe3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shl bl, 0x3
    ax_test![shl_bl_0x3; 0xc0, 0xe3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x8);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl bl, 0x3
    ax_test![shl_bl_0x3_pf; 0xc0, 0xe3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x78);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shl bl, 0x3
    ax_test![shl_bl_0x3_sf_of; 0xc0, 0xe3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl bl, 0x3
    ax_test![shl_bl_0x3_pf_sf_of; 0xc0, 0xe3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x88);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF)
    ];

    // shl bl, 0x3
    ax_test![shl_bl_0x3_cf_pf_zf_of; 0xc0, 0xe3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shl bl, 0x3
    ax_test![shl_bl_0x3_cf_of; 0xc0, 0xe3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x8);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl bl, 0x3
    ax_test![shl_bl_0x3_cf_sf; 0xc0, 0xe3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf8);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF)
    ];

    // shl bl, 0x1
    ax_test![shl_bl_0x1_pf_zf; 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl bl, 0x1
    ax_test![shl_bl_0x1; 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl bl, 0x1
    ax_test![shl_bl_0x1_pf; 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1e);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl bl, 0x1
    ax_test![shl_bl_0x1_sf_of; 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x40);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl bl, 0x1
    ax_test![shl_bl_0x1_pf_sf_of; 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x82);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // shl bl, 0x1
    ax_test![shl_bl_0x1_cf_pf_zf_of; 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // shl bl, 0x1
    ax_test![shl_bl_0x1_cf_sf; 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xfe);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // shl eax, 0x0
    ax_test![shl_eax_0x0; 0xc1, 0xe0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl r12w, 0x1
    ax_test![shl_r12w_0x1_pf_zf; 0x66, 0x41, 0xd1, 0xe4;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl r12w, 0x1
    ax_test![shl_r12w_0x1; 0x66, 0x41, 0xd1, 0xe4;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl r12w, 0x1
    ax_test![shl_r12w_0x1_pf; 0x66, 0x41, 0xd1, 0xe4;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x1e);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl r12w, 0x1
    ax_test![shl_r12w_0x1_sf_of; 0x66, 0x41, 0xd1, 0xe4;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0xfffe);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl r12w, 0x1
    ax_test![shl_r12w_0x1_cf_pf_zf_of; 0x66, 0x41, 0xd1, 0xe4;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // shl r12w, 0x1
    ax_test![shl_r12w_0x1_pf_sf_of; 0x66, 0x41, 0xd1, 0xe4;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x8000);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // shl r12w, 0x5
    ax_test![shl_r12w_0x5_pf_zf; 0x66, 0x41, 0xc1, 0xe4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shl r12w, 0x5
    ax_test![shl_r12w_0x5; 0x66, 0x41, 0xc1, 0xe4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x20);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl r12w, 0x5
    ax_test![shl_r12w_0x5_pf; 0x66, 0x41, 0xc1, 0xe4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x100);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shl r12w, 0x5
    ax_test![shl_r12w_0x5_cf_sf; 0x66, 0x41, 0xc1, 0xe4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0xffe0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF)
    ];

    // shl r12w, 0x5
    ax_test![shl_r12w_0x5_pf_sf_of; 0x66, 0x41, 0xc1, 0xe4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF)
    ];

    // shl r12w, 0x5
    ax_test![shl_r12w_0x5_cf_pf_zf_of; 0x66, 0x41, 0xc1, 0xe4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R12W; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R12W; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shl ecx, 0x0
    ax_test![shl_ecx_0x0; 0xc1, 0xe1, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl ebx, 0x1
    ax_test![shl_ebx_0x1_pf_zf; 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl ebx, 0x1
    ax_test![shl_ebx_0x1; 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl ebx, 0x1
    ax_test![shl_ebx_0x1_pf; 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1e);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl ebx, 0x1
    ax_test![shl_ebx_0x1_sf_of; 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xfffffffeu32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl ebx, 0x1
    ax_test![shl_ebx_0x1_cf_pf_zf_of; 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // shl ebx, 0x1
    ax_test![shl_ebx_0x1_pf_sf_of; 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x40000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // shl eax, 0x5
    ax_test![shl_eax_0x5_pf_zf; 0xc1, 0xe0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF )
    ];

    // shl eax, 0x5
    ax_test![shl_eax_0x5; 0xc1, 0xe0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x20);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl eax, 0x5
    ax_test![shl_eax_0x5_pf; 0xc1, 0xe0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x100);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shl eax, 0x5
    ax_test![shl_eax_0x5_cf_sf; 0xc1, 0xe0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffffe0u32);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF)
    ];

    // shl eax, 0x5
    ax_test![shl_eax_0x5_pf_sf_of; 0xc1, 0xe0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x4000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF)
    ];

    // shl eax, 0x5
    ax_test![shl_eax_0x5_cf_pf_zf_of; 0xc1, 0xe0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shl rcx, 0x0
    ax_test![shl_rcx_0x0; 0x48, 0xc1, 0xe1, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1_pf_zf; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1_pf; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x1e);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1_sf_of; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xfffffffffffffffeu64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1_cf_pf_zf_of; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1_pf_sf_of; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x4000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1_cf_of; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x8e6aa3e53fec6d75u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x1cd547ca7fd8daeau64);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1_cf_pf_sf; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0xf2409f8266822c91u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xe4813f04cd045922u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1_cf_pf_of; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0xb7a9ea5143efa612u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x6f53d4a287df4c24u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_OF; FLAG_ZF | FLAG_SF)
    ];

    // shl rdx, 0x1
    ax_test![shl_rdx_0x1_cf_sf; 0x48, 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0xf19a6fd26c3079b1u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xe334dfa4d860f362u64);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2_pf_zf; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2_pf; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x3c);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2_cf_pf_sf; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xfffffffffffffffcu64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF)
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2_pf_sf_of; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x2000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF ; FLAG_CF | FLAG_ZF)
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2_cf_pf_zf_of; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x4000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF ; FLAG_SF)
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2_cf_pf_of; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x5fceb2a667be6576u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x7f3aca999ef995d8u64);
        };
        (FLAG_CF | FLAG_PF ; FLAG_ZF | FLAG_SF)
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2_cf_of; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x487407c071154637u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x21d01f01c45518dcu64);
        };
        (FLAG_CF ; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2_cf_sf; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0xf24120afe2e67393u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0xc90482bf8b99ce4cu64);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF )
    ];

    // shl rdx, 0x2
    ax_test![shl_rdx_0x2_sf_of; 0x48, 0xc1, 0xe2, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0xa074c9460a0f977bu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x81d32518283e5decu64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl rdx, 0x0
    ax_test![shl_rdx_0x0; 0x48, 0xc1, 0xe2, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RDX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl bl, cl
    ax_test![shl_bl_cl; 0xd2, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl bl, cl
    ax_test![shl_bl_cl_pf_zf; 0xd2, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl bl, cl
    ax_test![shl_bl_cl_sf_of; 0xd2, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(b; a; CL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(b; a; CL; 0x7);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl bl, cl
    ax_test![shl_bl_cl_cf_pf_zf_of; 0xd2, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(b; a; CL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(b; a; CL; 0x8);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF ; FLAG_SF)
    ];

    // shl bl, cl
    ax_test![shl_bl_cl_cf_sf; 0xd2, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(b; a; CL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(b; a; CL; 0x7);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF)
    ];

    // shl bl, cl
    ax_test![shl_bl_cl_pf; 0xd2, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1e);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl bl, cl
    ax_test![shl_bl_cl_pf_sf_of; 0xd2, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
            write_reg_value!(b; a; CL; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf0);
            assert_reg_value!(b; a; CL; 0x4);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF)
    ];

    // shl bl, cl
    ax_test![shl_bl_cl_cf_of; 0xd2, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x11);
            write_reg_value!(b; a; CL; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x10);
            assert_reg_value!(b; a; CL; 0x4);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl bl, cl
    ax_test![shl_bl_cl_cf_pf_sf; 0xd2, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1f);
            write_reg_value!(b; a; CL; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf0);
            assert_reg_value!(b; a; CL; 0x4);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF)
    ];

    // shl cl, cl
    ax_test![shl_cl_cl; 0xd2, 0xe1; |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl cl, cl
    ax_test![shl_cl_cl_cf_sf; 0xd2, 0xe1; |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x80);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF)
    ];

    // shl cl, cl
    ax_test![shl_cl_cl_pf_zf; 0xd2, 0xe1; |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shl cl, cl
    ax_test![shl_cl_cl_pf; 0xd2, 0xe1; |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x42);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shl cl, cl
    ax_test![shl_cl_cl_pf_sf_of; 0xd2, 0xe1; |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x82);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF)
    ];

    // shl byte ptr [rax], cl
    ax_test![shl_byte_ptr_rax_cl; 0xd2, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl byte ptr [rax], cl
    ax_test![shl_byte_ptr_rax_cl_pf_zf; 0xd2, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl byte ptr [rax], cl
    ax_test![shl_byte_ptr_rax_cl_sf_of; 0xd2, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x7);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x7);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl byte ptr [rax], cl
    ax_test![shl_byte_ptr_rax_cl_cf_pf_zf_of; 0xd2, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x8);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shl byte ptr [rax], cl
    ax_test![shl_byte_ptr_rax_cl_cf_sf; 0xd2, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x7);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x7);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF)
    ];

    // shl byte ptr [rax], cl
    ax_test![shl_byte_ptr_rax_cl_pf; 0xd2, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x1e);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl byte ptr [rax], cl
    ax_test![shl_byte_ptr_rax_cl_pf_sf_of; 0xd2, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x4);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x4);
            assert_mem_value!(b; a; 0x1000; 0xf0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF)
    ];

    // shl byte ptr [rax], cl
    ax_test![shl_byte_ptr_rax_cl_cf_of; 0xd2, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x4);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x11).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x4);
            assert_mem_value!(b; a; 0x1000; 0x10);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl byte ptr [rax], cl
    ax_test![shl_byte_ptr_rax_cl_cf_pf_sf; 0xd2, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; CL; 0x4);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; CL; 0x4);
            assert_mem_value!(b; a; 0x1000; 0xf0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF)
    ];

    // shl ax, cl
    ax_test![shl_ax_cl; 0x66, 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl ax, cl
    ax_test![shl_ax_cl_pf_zf; 0x66, 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl ax, cl
    ax_test![shl_ax_cl_pf; 0x66, 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(b; a; CL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x100);
            assert_reg_value!(b; a; CL; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shl ax, cl
    ax_test![shl_ax_cl_pf_sf_of; 0x66, 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(b; a; CL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_reg_value!(b; a; CL; 0xf);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF)
    ];

    // shl ax, cl
    ax_test![shl_ax_cl_cf_pf_zf_of; 0x66, 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(b; a; CL; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_reg_value!(b; a; CL; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shl ax, cl
    ax_test![shl_ax_cl_cf_pf_sf; 0x66, 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
            write_reg_value!(b; a; CL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_reg_value!(b; a; CL; 0xf);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF )
    ];

    // shl ax, cl
    ax_test![shl_ax_cl_sf_of; 0x66, 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfffe);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl ax, cl
    ax_test![shl_ax_cl_cf_sf; 0x66, 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(b; a; CL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xff80);
            assert_reg_value!(b; a; CL; 0x7);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF)
    ];

    // shl eax, cl
    ax_test![shl_eax_cl; 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl eax, cl
    ax_test![shl_eax_cl_pf_zf; 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl eax, cl
    ax_test![shl_eax_cl_pf; 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(b; a; CL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x100);
            assert_reg_value!(b; a; CL; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shl eax, cl
    ax_test![shl_eax_cl_pf_sf_of; 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(b; a; CL; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_reg_value!(b; a; CL; 0x1f);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF)
    ];

    // shl eax, cl
    ax_test![shl_eax_cl_cf_pf_sf; 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
            write_reg_value!(b; a; CL; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_reg_value!(b; a; CL; 0x1f);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF)
    ];

    // shl eax, cl
    ax_test![shl_eax_cl_cf_pf_zf_of; 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8000);
            write_reg_value!(b; a; CL; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(b; a; CL; 0x11);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shl eax, cl
    ax_test![shl_eax_cl_sf_of; 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xfffffffeu32);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl eax, cl
    ax_test![shl_eax_cl_cf_sf; 0xd3, 0xe0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(b; a; CL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffff80u32);
            assert_reg_value!(b; a; CL; 0x7);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl_pf_zf; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl_pf; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1);
            write_reg_value!(b; a; CL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x100);
            assert_reg_value!(b; a; CL; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl_pf_sf_of; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1);
            write_reg_value!(b; a; CL; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x8000000000000000u64);
            assert_reg_value!(b; a; CL; 0x3f);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl_cf_pf_sf; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x7);
            write_reg_value!(b; a; CL; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x8000000000000000u64);
            assert_reg_value!(b; a; CL; 0x3f);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl_cf_pf_zf_of; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x80000000u64);
            write_reg_value!(b; a; CL; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x0);
            assert_reg_value!(b; a; CL; 0x21);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl_sf_of; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x7fffffffffffffffu64);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xfffffffffffffffeu64);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl_cf_sf; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x7fffffffffffffffu64);
            write_reg_value!(b; a; CL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xffffffffffffff80u64);
            assert_reg_value!(b; a; CL; 0x7);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl_cf_pf_of; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x42a18d2c00fd57d6u64);
            write_reg_value!(b; a; CL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x50c696007eabeb00u64);
            assert_reg_value!(b; a; CL; 0x7);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF)
    ];

    // shl r11, cl
    ax_test![shl_r11_cl_cf_of; 0x49, 0xd3, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0xb5062632bfa18f6du64);
            write_reg_value!(b; a; CL; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x5062632bfa18f6d0u64);
            assert_reg_value!(b; a; CL; 0x4);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];
}
