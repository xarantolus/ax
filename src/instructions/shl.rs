use core::panic;

use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Shl;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;

use crate::calculate_rm_imm;

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
            _ => panic!("Invalid instruction code {:?} for mnemonic Shl", i.code()),
        }
    }

    /// SHL r/m8, imm8
    ///
    /// C0 /4 ib
    fn instr_shl_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm8_imm8);

        calculate_rm_imm![u8f; self; i; |d: u8, s: u8| {
            assert_ne!(s, 1, "SHL r/m8, imm8 with immediate 1 should be handled by opcode SHL r/m8, 1");

            match d.checked_shl((s&0x1f) as u32) {
                Some(v) => (
                    v,
                    if d & (0x80u8.wrapping_shr((s&0x1f) as u32 - 1)) == 0 {0} else {FLAG_CF}
                ),
                None => {
                    // Overflow flag is only defined for shifts of 1, which are handled by another opcode
                    (0, if s == 8 && d & 1 == 1 {FLAG_CF} else {0})}
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
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
                Some(v) => (
                    v,
                    if d & (0x8000u16.wrapping_shr((s&0x1f) as u32 - 1)) == 0 {0} else {FLAG_CF}
                ),
                None => {
                    // Overflow flag is only defined for shifts of 1, which are handled by another opcode,
                    // we should however handle the only possible carry
                    (0, if s == 16 && d & 1 == 1 {FLAG_CF} else {0})
                }
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
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
                    if d & (0x80000000u32.wrapping_shr(match s&0x1f {
                        0 => 0,
                        v => v-1
                    } as u32)) == 0 {0} else {FLAG_CF}
                ),
                None => {
                    panic!("u32 s & 0x1f should never be >=32");
                }
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
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
                    if d & (0x8000000000000000u64.wrapping_shr(match s&0x3f {
                        0 => 0,
                        v => v-1
                    } as u32)) == 0 {0} else {FLAG_CF}
                ),
                None => {
                    panic!("u64 s & 0x1f should never be >=64");
                }
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
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
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
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
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
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
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
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
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHL r/m8, CL
    ///
    /// D2 /4
    fn instr_shl_rm8_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm8_CL);

        todo!("instr_shl_rm8_cl for Shl")
    }

    /// SHL r/m16, CL
    ///
    /// o16 D3 /4
    fn instr_shl_rm16_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm16_CL);

        todo!("instr_shl_rm16_cl for Shl")
    }

    /// SHL r/m32, CL
    ///
    /// o32 D3 /4
    fn instr_shl_rm32_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm32_CL);

        todo!("instr_shl_rm32_cl for Shl")
    }

    /// SHL r/m64, CL
    ///
    /// o64 D3 /4
    fn instr_shl_rm64_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shl_rm64_CL);

        todo!("instr_shl_rm64_cl for Shl")
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::RegisterWrapper, write_reg_value,
    };
    use iced_x86::Register::*;

    // shl r11b, 1
    ax_test![shl_r11b_1_cf_of; 0x41, 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; R11L; 0x8b);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; R11L; 0x16);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl r11w, 1
    ax_test![shl_r11w_1_of_pf; 0x66, 0x41, 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x6ea7b6c7ec446be5u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x6ea7b6c7ec44d7cau64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];
    // shl r11w, 1
    ax_test![shl_r11w_1_cf_pf_sf; 0x66, 0x41, 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x2d350ac8bf64d79bu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x2d350ac8bf64af36u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // shl r11w, 1
    ax_test![shl_r11w_1_no_flags; 0x66, 0x41, 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x89ccc6f0e7781c49u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x89ccc6f0e7783892u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl r11w, 1
    ax_test![shl_r11w_1_sf_of; 0x66, 0x41, 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x639025ce570552e2u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x639025ce5705a5c4u64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // shl bl, 1
    ax_test![shl_bl_1_of; 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xb3);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x66);
        };
        (FLAG_CF | FLAG_PF | FLAG_OF; FLAG_ZF | FLAG_SF)
    ];
    // shl bl, 1
    ax_test![shl_bl_1_no_of; 0xd0, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x3);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl bl, 2
    ax_test![shl_bl_2; 0xc0, 0xe3, 0x2;
        |a: &mut Axecutor| {
            // The upper bit will be shifted out, but CF will be set to the last bit shifted out, which is a zero
            write_reg_value!(b; a; BL; 0x81);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x4);
        };
        (0; FLAG_CF)
    ];

    // shl bl, 2
    ax_test![shl_bl_2_cf; 0xc0, 0xe3, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x4);
        };
        (FLAG_CF; 0)
    ];

    // shl r11b, 8
    ax_test![shl_r11b_8; 0x41, 0xc0, 0xe3, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; R11L; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; R11L; 0);
        };
        (FLAG_CF; 0)
    ];

    // shl bl, 16
    ax_test![shl_bl_16; 0xc0, 0xe3, 0x10;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF)
    ];

    // shl r11w, 1
    ax_test![shl_r11w_1_cf_zf; 0x66, 0x41, 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x8000u16);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0u16);
        };
        (FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_PF; 0)
    ];

    // shl r11w, 1
    ax_test![shl_r11w_1_of; 0x66, 0x41, 0xd1, 0xe3;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0xc000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x8000);
        };
        (FLAG_CF|FLAG_PF|FLAG_SF; FLAG_OF)
    ];

    // shl dx, 0 -- flags not affected
    ax_test![shl_dx_0; 0x66, 0xc1, 0xe2, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; DX; 0x56ce);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; DX; 0x56ce);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl r11w, 5
    ax_test![shl_r11w_5_zero; 0x66, 0x41, 0xc1, 0xe3, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0);
        };
        (FLAG_ZF; FLAG_CF)
    ];

    // shl r11w, 5
    ax_test![shl_r11w_5; 0x66, 0x41, 0xc1, 0xe3, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x8001);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x20);
        };
        (0; FLAG_ZF | FLAG_CF)
    ];

    // shl r11w, 35
    ax_test![shl_r11w_35_of; 0x66, 0x41, 0xc1, 0xe3, 0x23;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x8fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x7ff8);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // shl r11w, 15
    ax_test![shl_r11w_15; 0x66, 0x41, 0xc1, 0xe3, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x8000);
        };
        (0; FLAG_CF)
    ];

    // shl r11w, 16
    ax_test![shl_r11w_16; 0x66, 0x41, 0xc1, 0xe3, 0x10;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; 0)
    ];

    // shl cx, 16
    ax_test![shl_cx_16; 0x66, 0xc1, 0xe1, 0x10;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0xda60);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shl r11d, 25
    ax_test![shl_r11d_25; 0x41, 0xc1, 0xe3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; R11D; 0x80000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; R11D; 0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF)
    ];

    // shl r11d, 32
    ax_test![shl_r11d_32; 0x41, 0xc1, 0xe3, 0x20;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; R11D; 1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; R11D; 1);
        };
        (0; FLAG_ZF | FLAG_CF | FLAG_OF | FLAG_PF | FLAG_SF)
    ];

    // shl edx, 0 -- flags not affected
    ax_test![shl_edx_0; 0xc1, 0xe2, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0xbdb406f5u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0xbdb406f5u32);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl eax, 1
    ax_test![shl_eax_1; 0xd1, 0xe0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x347a69aa);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x68f4d354);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl edx, 1
    ax_test![shl_edx_1; 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x4780bbd0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x8f0177a0u32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // shl edx, 1
    ax_test![shl_edx_1_cf; 0xd1, 0xe2;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0xbc97b537u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x792f6a6e);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shl rbx, 5
    ax_test![shl_rbx_5; 0x48, 0xc1, 0xe3, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x4ce597c73d9b9895u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x9cb2f8e7b37312a0u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // shl rbx, 0
    ax_test![shl_rbx_0; 0x48, 0xc1, 0xe3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xe5e58065bec7819fu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xe5e58065bec7819fu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl rbx, 63
    ax_test![shl_rbx_63; 0x48, 0xc1, 0xe3, 0x3f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x2e3f2d1f8d72340bu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // shl rbx, 64
    ax_test![shl_rbx_64; 0x48, 0xc1, 0xe3, 0x40;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8b5cfb2c7e8fc9c3u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8b5cfb2c7e8fc9c3u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shl rax, 1
    ax_test![shl_rax_1; 0x48, 0xd1, 0xe0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x931cc98c02f1d409u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2639931805e3a812u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_OF; FLAG_ZF | FLAG_SF)
    ];

    // shl rax, 1
    ax_test![shl_rax_1_sf; 0x48, 0xd1, 0xe0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xe50f86adda7cad3au64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xca1f0d5bb4f95a74u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // shl rax, 1
    ax_test![shl_rax_1_of; 0x48, 0xd1, 0xe0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x9e8e938742425e0au64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x3d1d270e8484bc14u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_OF; FLAG_ZF | FLAG_SF)
    ];
}
