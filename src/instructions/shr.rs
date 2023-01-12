use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Shr;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::fatal_error;
use crate::instructions::flags::*;

use crate::{calculate_rm_imm, calculate_rm_r};

impl Axecutor {
    pub fn mnemonic_shr(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Shr);

        match i.code() {
            Shr_rm8_imm8 => self.instr_shr_rm8_imm8(i),
            Shr_rm16_imm8 => self.instr_shr_rm16_imm8(i),
            Shr_rm32_imm8 => self.instr_shr_rm32_imm8(i),
            Shr_rm64_imm8 => self.instr_shr_rm64_imm8(i),
            Shr_rm8_1 => self.instr_shr_rm8_1(i),
            Shr_rm16_1 => self.instr_shr_rm16_1(i),
            Shr_rm32_1 => self.instr_shr_rm32_1(i),
            Shr_rm64_1 => self.instr_shr_rm64_1(i),
            Shr_rm8_CL => self.instr_shr_rm8_cl(i),
            Shr_rm16_CL => self.instr_shr_rm16_cl(i),
            Shr_rm32_CL => self.instr_shr_rm32_cl(i),
            Shr_rm64_CL => self.instr_shr_rm64_cl(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Shr", i.code()),
        }
    }

    /// SHR r/m8, imm8
    ///
    /// C0 /5 ib
    fn instr_shr_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm8_imm8);

        calculate_rm_imm![u8f; self; i; |d: u8, s:u8| {
            assert_ne!(s, 1, "SHR r/m8, 1 should be handled by opcode SHR r/m8, 1");

            if s == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shr((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (1 << ((s-1)&0x1f)) != 0 { FLAG_CF } else {0};

                    (v, cf)
                }
                None => (0, if s == 8 && d & 0x80 != 0 { FLAG_CF } else {0})
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m16, imm8
    ///
    /// o16 C1 /5 ib
    fn instr_shr_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm16_imm8);

        calculate_rm_imm![u16f; u8; self; i; |d: u16, s:u8| {
            assert_ne!(s, 1, "SHR r/m16, 1 should be handled by opcode SHR r/m16, 1");

            if s == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shr((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (1 << ((s-1)&0x1f)) != 0 { FLAG_CF } else {0};

                    (v, cf)
                }
                None => (0, if s == 16 && d & 0x8000 != 0 { FLAG_CF } else {0})
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m32, imm8
    ///
    /// o32 C1 /5 ib
    fn instr_shr_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm32_imm8);

        calculate_rm_imm![u32f; u8; self; i; |d: u32, s:u8| {
            assert_ne!(s, 1, "SHR r/m32, 1 should be handled by opcode SHR r/m32, 1");

            if s == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shr((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (1 << ((s-1)&0x1f)) != 0 { FLAG_CF } else {0};

                    (v, cf)
                }
                None => (0, if s == 32 && d & 0x8000_0000 != 0 { FLAG_CF } else {0})
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m64, imm8
    ///
    /// o64 C1 /5 ib
    fn instr_shr_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm64_imm8);

        calculate_rm_imm![u64f; u8; self; i; |d: u64, s:u8| {
            assert_ne!(s, 1, "SHR r/m64, 1 should be handled by opcode SHR r/m64, 1");

            if s == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shr((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (1 << ((s-1)&0x1f)) != 0 { FLAG_CF } else {0};

                    (v, cf)
                }
                None => (0, if s == 64 && d & 0x8000_0000_0000_0000 != 0 { FLAG_CF } else {0})
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m8, 1
    ///
    /// D0 /5
    fn instr_shr_rm8_1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm8_1);

        calculate_rm_imm![u8f; self; i; |d: u8, s: u8| {
            debug_assert_eq!(s, 1, "SHL r/m8, 1: src is not 1");

            let cf = if d & 0x01 != 0 { FLAG_CF } else {0};
            // "OF flag is set to the most-significant bit of the original operand"
            let of = if d & 0x80 != 0 { FLAG_OF } else {0};

            (d.wrapping_shr(1), cf | of)
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m16, 1
    ///
    /// o16 D1 /5
    fn instr_shr_rm16_1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm16_1);

        calculate_rm_imm![u16f; u8; self; i; |d: u16, s: u8| {
            debug_assert_eq!(s, 1, "SHL r/m16, 1: src is not 1");

            let cf = if d & 0x01 != 0 { FLAG_CF } else {0};
            // "OF flag is set to the most-significant bit of the original operand"
            let of = if d & 0x8000 != 0 { FLAG_OF } else {0};

            (d.wrapping_shr(1), cf | of)
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m32, 1
    ///
    /// o32 D1 /5
    fn instr_shr_rm32_1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm32_1);

        calculate_rm_imm![u32f; u8; self; i; |d: u32, s: u8| {
            debug_assert_eq!(s, 1, "SHL r/m32, 1: src is not 1");

            let cf = if d & 0x01 != 0 { FLAG_CF } else {0};
            // "OF flag is set to the most-significant bit of the original operand"
            let of = if d & 0x8000_0000 != 0 { FLAG_OF } else {0};

            (d.wrapping_shr(1), cf | of)
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m64, 1
    ///
    /// o64 D1 /5
    fn instr_shr_rm64_1(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm64_1);

        calculate_rm_imm![u64f; u8; self; i; |d: u64, s: u8| {
            debug_assert_eq!(s, 1, "SHL r/m64, 1: src is not 1");

            let cf = if d & 0x01 != 0 { FLAG_CF } else {0};
            // "OF flag is set to the most-significant bit of the original operand"
            let of = if d & 0x8000_0000_0000_0000 != 0 { FLAG_OF } else {0};

            (d.wrapping_shr(1), cf | of)
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m8, CL
    ///
    /// D2 /5
    fn instr_shr_rm8_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm8_CL);

        calculate_rm_r![u8f; self; i; |d: u8, s: u8| {
            if s == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shr((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (1 << ((s-1)&0x1f)) != 0 { FLAG_CF } else {0};
                    let of = if s == 1 && d & 0x80 != 0 { FLAG_OF } else {0};

                    (v, cf|of)
                }
                None => (0, if s == 8 && d & 0x80 != 0 { FLAG_CF } else {0})
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m16, CL
    ///
    /// o16 D3 /5
    fn instr_shr_rm16_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm16_CL);

        calculate_rm_r![u16f; u8; self; i; |d: u16, s: u8| {
            if s == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shr((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (1 << ((s-1)&0x1f)) != 0 { FLAG_CF } else {0};
                    let of = if s == 1 && d & 0x8000 != 0 { FLAG_OF } else {0};

                    (v, cf|of)
                }
                None => (0, if s == 16 && d & 0x8000 != 0 { FLAG_CF } else {0})
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m32, CL
    ///
    /// o32 D3 /5
    fn instr_shr_rm32_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm32_CL);

        calculate_rm_r![u32f; u8; self; i; |d: u32, s: u8| {
            if s == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shr((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (1 << ((s-1)&0x1f)) != 0 { FLAG_CF } else {0};
                    let of = if s == 1 && d & 0x8000_0000 != 0 { FLAG_OF } else {0};

                    (v, cf|of)
                }
                None => (0, if s == 32 && d & 0x8000_0000 != 0 { FLAG_CF } else {0})
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }

    /// SHR r/m64, CL
    ///
    /// o64 D3 /5
    fn instr_shr_rm64_cl(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Shr_rm64_CL);

        calculate_rm_r![u64f; u8; self; i; |d: u64, s: u8| {
            if s == 0 {
                return (d, FLAGS_UNAFFECTED);
            }

            match d.checked_shr((s&0x1f) as u32) {
                Some(v) => {
                    let cf = if d & (1 << ((s-1)&0x1f)) != 0 { FLAG_CF } else {0};
                    let of = if s == 1 && d & 0x8000_0000_0000_0000 != 0 { FLAG_OF } else {0};

                    (v, cf|of)
                }
                None => (0, if s == 64 && d & 0x8000_0000_0000_0000 != 0 { FLAG_CF } else {0})
            }
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: 0)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_mem_value, assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // shr byte ptr [rax], 0x3
    ax_test![shr_byte_ptr_rax_0x3; 0xc0, 0x28, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr byte ptr [rax], 0x3
    ax_test![shr_byte_ptr_rax_0x3_cf; 0xc0, 0x28, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr byte ptr [rax], 0x3
    ax_test![shr_byte_ptr_rax_0x3_cf_pf; 0xc0, 0x28, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF)
    ];

    // shr byte ptr [rax], 0x3
    ax_test![shr_byte_ptr_rax_0x3_cf_pf_zf; 0xc0, 0x28, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shr byte ptr [rax], 0x3
    ax_test![shr_byte_ptr_rax_0x3_pf; 0xc0, 0x28, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x18).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr byte ptr [rax], 0x3
    ax_test![shr_byte_ptr_rax_0x3_pf_zf; 0xc0, 0x28, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shr byte ptr [rax], 0x0
    ax_test![shr_byte_ptr_rax_0x0; 0xc0, 0x28, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr word ptr [rax], 0x0
    ax_test![shr_word_ptr_rax_0x0; 0x66, 0xc1, 0x28, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    // shr word ptr [rax], 0x2
    ax_test![shr_word_ptr_rax_0x2; 0x66, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr word ptr [rax], 0x2
    ax_test![shr_word_ptr_rax_0x2_cf; 0x66, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr word ptr [rax], 0x2
    ax_test![shr_word_ptr_rax_0x2_cf_pf; 0x66, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF)
    ];

    // shr word ptr [rax], 0x2
    ax_test![shr_word_ptr_rax_0x2_cf_pf_zf; 0x66, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x2).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shr word ptr [rax], 0x2
    ax_test![shr_word_ptr_rax_0x2_pf; 0x66, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x2000);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr word ptr [rax], 0x2
    ax_test![shr_word_ptr_rax_0x2_pf_zf; 0x66, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];
    // shr word ptr [rax], 0x5
    ax_test![shr_word_ptr_rax_0x5; 0x66, 0xc1, 0x28, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr word ptr [rax], 0x5
    ax_test![shr_word_ptr_rax_0x5_cf; 0x66, 0xc1, 0x28, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x3f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr word ptr [rax], 0x5
    ax_test![shr_word_ptr_rax_0x5_cf_pf; 0x66, 0xc1, 0x28, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF)
    ];

    // shr word ptr [rax], 0x5
    ax_test![shr_word_ptr_rax_0x5_cf_pf_zf; 0x66, 0xc1, 0x28, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shr word ptr [rax], 0x5
    ax_test![shr_word_ptr_rax_0x5_pf; 0x66, 0xc1, 0x28, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x400);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr word ptr [rax], 0x5
    ax_test![shr_word_ptr_rax_0x5_pf_zf; 0x66, 0xc1, 0x28, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shr dword ptr [rax], 0x0
    ax_test![shr_dword_ptr_rax_0x0; 0xc1, 0x28, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr dword ptr [rax], 0x2
    ax_test![shr_dword_ptr_rax_0x2; 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr dword ptr [rax], 0x2
    ax_test![shr_dword_ptr_rax_0x2_cf; 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr dword ptr [rax], 0x2
    ax_test![shr_dword_ptr_rax_0x2_cf_pf; 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF)
    ];

    // shr dword ptr [rax], 0x2
    ax_test![shr_dword_ptr_rax_0x2_cf_pf_zf; 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x2).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shr dword ptr [rax], 0x2
    ax_test![shr_dword_ptr_rax_0x2_pf; 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x2000);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr dword ptr [rax], 0x2
    ax_test![shr_dword_ptr_rax_0x2_pf_zf; 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shr dword ptr [rax], 0x1f
    ax_test![shr_dword_ptr_rax_0x1f; 0xc1, 0x28, 0x1f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr dword ptr [rax], 0x1f
    ax_test![shr_dword_ptr_rax_0x1f_cf_pf_zf; 0xc1, 0x28, 0x1f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shr dword ptr [rax], 0x1f
    ax_test![shr_dword_ptr_rax_0x1f_pf_zf; 0xc1, 0x28, 0x1f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shr qword ptr [rax], 0x0
    ax_test![shr_qword_ptr_rax_0x0; 0x48, 0xc1, 0x28, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr qword ptr [rax], 0x2
    ax_test![shr_qword_ptr_rax_0x2; 0x48, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr qword ptr [rax], 0x2
    ax_test![shr_qword_ptr_rax_0x2_cf; 0x48, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr qword ptr [rax], 0x2
    ax_test![shr_qword_ptr_rax_0x2_cf_pf; 0x48, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF)
    ];

    // shr qword ptr [rax], 0x2
    ax_test![shr_qword_ptr_rax_0x2_cf_pf_zf; 0x48, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x2).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF)
    ];

    // shr qword ptr [rax], 0x2
    ax_test![shr_qword_ptr_rax_0x2_pf; 0x48, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x2000);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr qword ptr [rax], 0x2
    ax_test![shr_qword_ptr_rax_0x2_pf_zf; 0x48, 0xc1, 0x28, 0x2;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shr qword ptr [rax], 0x35
    ax_test![shr_qword_ptr_rax_0x35_pf_zf; 0x48, 0xc1, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1_cf; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1_cf_of; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1_cf_pf; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1_cf_pf_of; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x83);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x41);
        };
        (FLAG_CF | FLAG_PF | FLAG_OF; FLAG_ZF | FLAG_SF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1_cf_pf_zf; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1_of; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x40);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1_pf; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x6);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1_pf_of; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x82);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x41);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr al, 0x1
    ax_test![shr_al_0x1_pf_zf; 0xd0, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shr ax, 0x1
    ax_test![shr_ax_0x1; 0x66, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr ax, 0x1
    ax_test![shr_ax_0x1_cf; 0x66, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr ax, 0x1
    ax_test![shr_ax_0x1_cf_pf; 0x66, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr ax, 0x1
    ax_test![shr_ax_0x1_cf_pf_zf; 0x66, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // shr ax, 0x1
    ax_test![shr_ax_0x1_pf; 0x66, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x100);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr ax, 0x1
    ax_test![shr_ax_0x1_pf_of; 0x66, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4000);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr ax, 0x1
    ax_test![shr_ax_0x1_pf_zf; 0x66, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shr eax, 0x1
    ax_test![shr_eax_0x1; 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr eax, 0x1
    ax_test![shr_eax_0x1_cf; 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr eax, 0x1
    ax_test![shr_eax_0x1_cf_pf; 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr eax, 0x1
    ax_test![shr_eax_0x1_cf_pf_zf; 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // shr eax, 0x1
    ax_test![shr_eax_0x1_pf; 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x4000);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr eax, 0x1
    ax_test![shr_eax_0x1_pf_of; 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x40000000);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr eax, 0x1
    ax_test![shr_eax_0x1_pf_zf; 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1_cf; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1_cf_of; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x884a7374a519494fu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x442539ba528ca4a7u64);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1_cf_pf; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1_cf_pf_of; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8421bb25d5371e85u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4210dd92ea9b8f42u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_OF; FLAG_ZF | FLAG_SF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1_cf_pf_zf; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1_of; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xaf67a6c3d3d21fd4u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x57b3d361e9e90feau64);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1_pf; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4000);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1_pf_of; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4000000000000000u64);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr rax, 0x1
    ax_test![shr_rax_0x1_pf_zf; 0x48, 0xd1, 0xe8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shr al, cl
    ax_test![shr_al_cl; 0xd2, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr al, cl
    ax_test![shr_al_cl_cf; 0xd2, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
            write_reg_value!(b; a; CL; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; CL; 0x2);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr al, cl
    ax_test![shr_al_cl_cf_of; 0xd2, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr al, cl
    ax_test![shr_al_cl_cf_pf; 0xd2, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x3);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr al, cl
    ax_test![shr_al_cl_cf_pf_zf; 0xd2, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // shr al, cl
    ax_test![shr_al_cl_of; 0xd2, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x40);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr al, cl
    ax_test![shr_al_cl_pf_zf; 0xd2, 0xe8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shr bx, cl
    ax_test![shr_bx_cl; 0x66, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr bx, cl
    ax_test![shr_bx_cl_cf; 0x66, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7);
            write_reg_value!(b; a; CL; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x1);
            assert_reg_value!(b; a; CL; 0x2);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr bx, cl
    ax_test![shr_bx_cl_cf_pf; 0x66, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x3);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr bx, cl
    ax_test![shr_bx_cl_cf_pf_zf; 0x66, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // shr bx, cl
    ax_test![shr_bx_cl_pf; 0x66, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(b; a; CL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x100);
            assert_reg_value!(b; a; CL; 0x7);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr bx, cl
    ax_test![shr_bx_cl_pf_of; 0x66, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x4000);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr bx, cl
    ax_test![shr_bx_cl_pf_zf; 0x66, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shr ebx, cl
    ax_test![shr_ebx_cl; 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr ebx, cl
    ax_test![shr_ebx_cl_cf; 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7);
            write_reg_value!(b; a; CL; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1);
            assert_reg_value!(b; a; CL; 0x2);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr ebx, cl
    ax_test![shr_ebx_cl_cf_pf; 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x3);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr ebx, cl
    ax_test![shr_ebx_cl_cf_pf_zf; 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // shr ebx, cl
    ax_test![shr_ebx_cl_pf; 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x8000);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x4000);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr ebx, cl
    ax_test![shr_ebx_cl_pf_of; 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x40000000);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr ebx, cl
    ax_test![shr_ebx_cl_pf_zf; 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // shr r11, cl
    ax_test![shr_r11_cl; 0x49, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(b; a; CL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x0);
            assert_reg_value!(b; a; CL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr r11, cl
    ax_test![shr_r11_cl_cf; 0x49, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x7);
            write_reg_value!(b; a; CL; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1);
            assert_reg_value!(b; a; CL; 0x2);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr r11, cl
    ax_test![shr_r11_cl_cf_of; 0x49, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0xfc3e4790dbc9ff8fu64);
            write_reg_value!(b; a; CL; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x7e1f23c86de4ffc7u64);
            assert_reg_value!(b; a; CL; 0x41);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // shr r11, cl
    ax_test![shr_r11_cl_cf_pf; 0x49, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x7);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x3);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr r11, cl
    ax_test![shr_r11_cl_cf_pf_of; 0x49, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0xf8546923ce4658adu64);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x7c2a3491e7232c56u64);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_OF; FLAG_ZF | FLAG_SF)
    ];

    // shr r11, cl
    ax_test![shr_r11_cl_cf_pf_zf; 0x49, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // shr r11, cl
    ax_test![shr_r11_cl_pf; 0x49, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x4000);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // shr r11, cl
    ax_test![shr_r11_cl_pf_of; 0x49, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x4000000000000000u64);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // shr r11, cl
    ax_test![shr_r11_cl_pf_zf; 0x49, 0xd3, 0xeb; |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(b; a; CL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x0);
            assert_reg_value!(b; a; CL; 0x1);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
