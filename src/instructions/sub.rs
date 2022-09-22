use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Sub;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister;
use crate::{calculate_r_rm, calculate_rm_imm, calculate_rm_r};

impl Axecutor {
    pub fn mnemonic_sub(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Sub);

        match i.code() {
            Sub_rm8_r8 => self.instr_sub_rm8_r8(i),
            Sub_rm16_r16 => self.instr_sub_rm16_r16(i),
            Sub_rm32_r32 => self.instr_sub_rm32_r32(i),
            Sub_rm64_r64 => self.instr_sub_rm64_r64(i),
            Sub_r8_rm8 => self.instr_sub_r8_rm8(i),
            Sub_r16_rm16 => self.instr_sub_r16_rm16(i),
            Sub_r32_rm32 => self.instr_sub_r32_rm32(i),
            Sub_r64_rm64 => self.instr_sub_r64_rm64(i),
            Sub_AL_imm8 => self.instr_sub_al_imm8(i),
            Sub_AX_imm16 => self.instr_sub_ax_imm16(i),
            Sub_EAX_imm32 => self.instr_sub_eax_imm32(i),
            Sub_RAX_imm32 => self.instr_sub_rax_imm32(i),
            Sub_rm8_imm8 => self.instr_sub_rm8_imm8(i),
            Sub_rm16_imm16 => self.instr_sub_rm16_imm16(i),
            Sub_rm32_imm32 => self.instr_sub_rm32_imm32(i),
            Sub_rm64_imm32 => self.instr_sub_rm64_imm32(i),
            Sub_rm8_imm8_82 => self.instr_sub_rm8_imm8_82(i),
            Sub_rm16_imm8 => self.instr_sub_rm16_imm8(i),
            Sub_rm32_imm8 => self.instr_sub_rm32_imm8(i),
            Sub_rm64_imm8 => self.instr_sub_rm64_imm8(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Sub", i.code()),
        }
    }

    /// SUB r/m8, r8
    ///
    /// 28 /r
    fn instr_sub_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_rm8_r8);

        calculate_rm_r![u8f; self; i; |d:u8, s:u8| {
            let result = (d as i8).wrapping_sub(s as i8) as u8;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i16 ^ s as i16) & (d as i16 ^ result as i16) & 0x80) != 0 { FLAG_OF } else { 0 } |
                if ((d as i16 | 0x100).wrapping_sub(s as i16)) & 0x100 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r/m16, r16
    ///
    /// o16 29 /r
    fn instr_sub_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_rm16_r16);

        calculate_rm_r![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_sub(s as i16) as u16;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i32 ^ s as i32) & (d as i32 ^ result as i32) & 0x8000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i32 | 0x10000).wrapping_sub(s as i32)) & 0x10000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r/m32, r32
    ///
    /// o32 29 /r
    fn instr_sub_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_rm32_r32);

        calculate_rm_r![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_sub(s as i32) as u32;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i64 ^ s as i64) & (d as i64 ^ result as i64) & 0x80000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i64 | 0x100000000).wrapping_sub(s as i64)) & 0x100000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r/m64, r64
    ///
    /// o64 29 /r
    fn instr_sub_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_rm64_r64);

        calculate_rm_r![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_sub(s as i64) as u64;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i128 ^ s as i128) & (d as i128 ^ result as i128) & 0x8000000000000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i128 | 0x10000000000000000).wrapping_sub(s as i128)) & 0x10000000000000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r8, r/m8
    ///
    /// 2A /r
    fn instr_sub_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_r8_rm8);

        calculate_r_rm![u8f; self; i; |d:u8, s:u8| {
            let result = (d as i8).wrapping_sub(s as i8) as u8;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i16 ^ s as i16) & (d as i16 ^ result as i16) & 0x80) != 0 { FLAG_OF } else { 0 } |
                if ((d as i16 | 0x100).wrapping_sub(s as i16)) & 0x100 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r16, r/m16
    ///
    /// o16 2B /r
    fn instr_sub_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_r16_rm16);

        calculate_r_rm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_sub(s as i16) as u16;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i32 ^ s as i32) & (d as i32 ^ result as i32) & 0x8000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i32 | 0x10000).wrapping_sub(s as i32)) & 0x10000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r32, r/m32
    ///
    /// o32 2B /r
    fn instr_sub_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_r32_rm32);

        calculate_r_rm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_sub(s as i32) as u32;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i64 ^ s as i64) & (d as i64 ^ result as i64) & 0x80000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i64 | 0x100000000).wrapping_sub(s as i64)) & 0x100000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r64, r/m64
    ///
    /// o64 2B /r
    fn instr_sub_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_r64_rm64);

        calculate_r_rm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_sub(s as i64) as u64;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i128 ^ s as i128) & (d as i128 ^ result as i128) & 0x8000000000000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i128 | 0x10000000000000000).wrapping_sub(s as i128)) & 0x10000000000000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB AL, imm8
    ///
    /// 2C ib
    fn instr_sub_al_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_AL_imm8);

        self.instr_sub_rm8_imm8(i)
    }

    /// SUB AX, imm16
    ///
    /// o16 2D iw
    fn instr_sub_ax_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_AX_imm16);

        self.instr_sub_rm16_imm16(i)
    }

    /// SUB EAX, imm32
    ///
    /// o32 2D id
    fn instr_sub_eax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_EAX_imm32);

        self.instr_sub_rm32_imm32(i)
    }

    /// SUB RAX, imm32
    ///
    /// o64 2D id
    fn instr_sub_rax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_RAX_imm32);

        self.instr_sub_rm64_imm32(i)
    }

    /// SUB r/m8, imm8
    ///
    /// 80 /5 ib
    fn instr_sub_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u8f; self; i; |d:u8, s:u8| {
            let result = (d as i8).wrapping_sub(s as i8) as u8;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i16 ^ s as i16) & (d as i16 ^ result as i16) & 0x80) != 0 { FLAG_OF } else { 0 } |
                if ((d as i16 | 0x100).wrapping_sub(s as i16)) & 0x100 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r/m16, imm16
    ///
    /// o16 81 /5 iw
    fn instr_sub_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_sub(s as i16) as u16;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i32 ^ s as i32) & (d as i32 ^ result as i32) & 0x8000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i32 | 0x10000).wrapping_sub(s as i32)) & 0x10000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r/m32, imm32
    ///
    /// o32 81 /5 id
    fn instr_sub_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_sub(s as i32) as u32;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i64 ^ s as i64) & (d as i64 ^ result as i64) & 0x80000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i64 | 0x100000000).wrapping_sub(s as i64)) & 0x100000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r/m64, imm32
    ///
    /// o64 81 /5 id
    fn instr_sub_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_sub(s as i64) as u64;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i128 ^ s as i128) & (d as i128 ^ result as i128) & 0x8000000000000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i128 | 0x10000000000000000).wrapping_sub(s as i128)) & 0x10000000000000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r/m8, imm8
    ///
    /// 82 /5 ib
    fn instr_sub_rm8_imm8_82(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_rm8_imm8_82);

        self.instr_sub_rm8_imm8(i)
    }

    /// SUB r/m16, imm8
    ///
    /// o16 83 /5 ib
    fn instr_sub_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_rm16_imm8);

        calculate_rm_imm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_sub(s as i16) as u16;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i32 ^ s as i32) & (d as i32 ^ result as i32) & 0x8000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i32 | 0x10000).wrapping_sub(s as i32)) & 0x10000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r/m32, imm8
    ///
    /// o32 83 /5 ib
    fn instr_sub_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_rm32_imm8);

        calculate_rm_imm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_sub(s as i32) as u32;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i64 ^ s as i64) & (d as i64 ^ result as i64) & 0x80000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i64 | 0x100000000).wrapping_sub(s as i64)) & 0x100000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// SUB r/m64, imm8
    ///
    /// o64 83 /5 ib
    fn instr_sub_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Sub_rm64_imm8);

        calculate_rm_imm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_sub(s as i64) as u64;

            (
                result,
                if ((d & 0xf | 0x10) - (s & 0xf)) & 0x10 == 0 { FLAG_AF } else { 0 } |
                if ((d as i128 ^ s as i128) & (d as i128 ^ result as i128) & 0x8000000000000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i128 | 0x10000000000000000).wrapping_sub(s as i128)) & 0x10000000000000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::SupportedRegister, write_reg_value,
    };
    use iced_x86::Register::*;

    // sub al, 0x5
    ax_test![sub_al_0x5; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_af; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xb);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_cf_pf_sf_af; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xfc);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_cf_sf_af; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xfb);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_of_af; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x81);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7c);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_pf; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_pf_af; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xc);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_pf_of_af; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7b);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_pf_sf; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xfa);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_pf_sf_af; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x90);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8b);
        };
        (FLAG_PF | FLAG_SF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_pf_zf; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x5);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_sf; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x85);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5
    ax_test![sub_al_0x5_sf_af; 0x2c, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x91);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8c);
        };
        (FLAG_SF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x5e);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_af; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x61);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x4);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_cf_pf_sf; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xb2);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_cf_pf_sf_af; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xa3);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_cf_sf; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xc2);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_cf_sf_af; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xa4);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_of; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8e);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x31);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_of_af; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x23);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_pf; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x22);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_pf_af; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x60);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x3);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_pf_of; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8d);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x30);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_pf_of_af; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x81);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x24);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_pf_sf; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xde);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x81);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_pf_sf_af; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xe1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x84);
        };
        (FLAG_PF | FLAG_SF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_pf_zf; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x5d);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_sf; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xa2);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, 0x5d
    ax_test![sub_al_0x5d_sf_af; 0x2c, 0x5d;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xe0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x83);
        };
        (FLAG_SF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub al, bl
    ax_test![sub_al_bl; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];
    // sub al, bl
    ax_test![sub_al_bl_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(b; a; BL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8);
            assert_reg_value!(b; a; BL; 0x8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x90);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x70);
            assert_reg_value!(b; a; BL; 0x90);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_pf; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0xa0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x60);
            assert_reg_value!(b; a; BL; 0xa0);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_pf_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x82);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7e);
            assert_reg_value!(b; a; BL; 0x82);
        };
        (FLAG_CF | FLAG_PF | FLAG_AF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_pf_sf; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf0);
            assert_reg_value!(b; a; BL; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_pf_sf_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_pf_sf_of; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x81);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_pf_sf_of_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(b; a; BL; 0x82);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8e);
            assert_reg_value!(b; a; BL; 0x82);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF | FLAG_AF; FLAG_ZF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_sf; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xe0);
            assert_reg_value!(b; a; BL; 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_sf_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf8);
            assert_reg_value!(b; a; BL; 0x8);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_sf_of; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_cf_sf_of_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(b; a; BL; 0x81);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8f);
            assert_reg_value!(b; a; BL; 0x81);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF; FLAG_PF | FLAG_ZF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_of; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x70);
            assert_reg_value!(b; a; BL; 0x10);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_of_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_pf; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x6);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_pf_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_pf_of; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x60);
            assert_reg_value!(b; a; BL; 0x20);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_pf_of_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x78);
            assert_reg_value!(b; a; BL; 0x8);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_pf_sf; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_pf_sf_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x90);
            write_reg_value!(b; a; BL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x88);
            assert_reg_value!(b; a; BL; 0x8);
        };
        (FLAG_PF | FLAG_SF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_pf_zf; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_sf; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, bl
    ax_test![sub_al_bl_sf_af; 0x28, 0xd8; |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x90);
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8f);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (FLAG_SF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(w; a; R11W; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x1);
            assert_reg_value!(w; a; R11W; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_af; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x10);
            write_reg_value!(w; a; R11W; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8);
            assert_reg_value!(w; a; R11W; 0x8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_cf_pf_sf; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; R11W; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xfff0);
            assert_reg_value!(w; a; R11W; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_cf_pf_sf_af; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; R11W; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xffff);
            assert_reg_value!(w; a; R11W; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_cf_pf_sf_of; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; R11W; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
            assert_reg_value!(w; a; R11W; 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF | FLAG_AF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_cf_sf; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; R11W; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xffe0);
            assert_reg_value!(w; a; R11W; 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_cf_sf_af; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; R11W; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xfff8);
            assert_reg_value!(w; a; R11W; 0x8);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_cf_sf_of; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(w; a; R11W; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8001);
            assert_reg_value!(w; a; R11W; 0x8000);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_AF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_of; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(w; a; R11W; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7fe0);
            assert_reg_value!(w; a; R11W; 0x20);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_of_af; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(w; a; R11W; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7ff8);
            assert_reg_value!(w; a; R11W; 0x8);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_pf; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7);
            write_reg_value!(w; a; R11W; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x6);
            assert_reg_value!(w; a; R11W; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_pf_af; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x10);
            write_reg_value!(w; a; R11W; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xf);
            assert_reg_value!(w; a; R11W; 0x1);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_pf_of; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(w; a; R11W; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7ff0);
            assert_reg_value!(w; a; R11W; 0x10);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_pf_of_af; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(w; a; R11W; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7fff);
            assert_reg_value!(w; a; R11W; 0x1);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_pf_sf; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(w; a; R11W; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
            assert_reg_value!(w; a; R11W; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub bx, r11w
    ax_test![sub_bx_r11w_pf_zf; 0x66, 0x44, 0x29, 0xdb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; R11W; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(w; a; R11W; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(d; a; R12D; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1);
            assert_reg_value!(d; a; R12D; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_af; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x10);
            write_reg_value!(d; a; R12D; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x8);
            assert_reg_value!(d; a; R12D; 0x8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_cf_pf_sf; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xfffffff0u32);
            assert_reg_value!(d; a; R12D; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_cf_pf_sf_af; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xffffffffu32);
            assert_reg_value!(d; a; R12D; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_cf_pf_sf_of; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u32);
            assert_reg_value!(d; a; R12D; 0x80000000u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_cf_sf; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xffffffe0u32);
            assert_reg_value!(d; a; R12D; 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_cf_sf_af; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xfffffff8u32);
            assert_reg_value!(d; a; R12D; 0x8);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_cf_sf_of; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(d; a; R12D; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000001u32);
            assert_reg_value!(d; a; R12D; 0x80000000u32);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_of; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_reg_value!(d; a; R12D; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x7fffffe0);
            assert_reg_value!(d; a; R12D; 0x20);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_of_af; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_reg_value!(d; a; R12D; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x7ffffff8);
            assert_reg_value!(d; a; R12D; 0x8);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_pf; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7);
            write_reg_value!(d; a; R12D; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x6);
            assert_reg_value!(d; a; R12D; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_pf_af; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x10);
            write_reg_value!(d; a; R12D; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xf);
            assert_reg_value!(d; a; R12D; 0x1);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_pf_of; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_reg_value!(d; a; R12D; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x7ffffff0);
            assert_reg_value!(d; a; R12D; 0x10);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_pf_of_af; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_reg_value!(d; a; R12D; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x7fffffffu32);
            assert_reg_value!(d; a; R12D; 0x1);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_pf_sf; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_reg_value!(d; a; R12D; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u32);
            assert_reg_value!(d; a; R12D; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub ebx, r12d
    ax_test![sub_ebx_r12d_pf_zf; 0x44, 0x29, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_reg_value!(d; a; R12D; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_af; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x91cd4a18fad40360u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x91cd4a18fad40360u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x6e32b5e7052bfca1u64);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf_af; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xdaa5b64c30262321u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xdaa5b64c30262321u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x255a49b3cfd9dcdfu64);
        };
        (FLAG_CF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf_pf; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xbe2001ffcd2422c7u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x3f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xbe2001ffcd2422c7u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x41dffe0032dbdd78u64);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf_pf_af; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xa8d77020859f619fu64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xa8d77020859f619fu64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x57288fdf7a609e71u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_AF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf_pf_sf; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x10);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xfffffffffffffff0u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf_pf_sf_af; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xffffffffffffffffu64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf_pf_sf_of; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x8000000000000000u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF | FLAG_AF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf_sf; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x20);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x20);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xffffffffffffffe0u64);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf_sf_af; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xfffffffffffffff8u64);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_cf_sf_of; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x8000000000000001u64);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_AF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_pf; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_pf_af; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xf);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub qword ptr [rbx], rax
    ax_test![sub_qword_ptr_rbx_rax_pf_zf; 0x48, 0x29, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_af; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_cf; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x10);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_cf_af; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xff);
        };
        (FLAG_CF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_cf_pf_af; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x9);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_AF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_cf_pf_sf; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf0);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_cf_pf_sf_af; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_cf_pf_sf_of; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x81);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x80);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_cf_sf; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xe0);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_cf_sf_af; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf8);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x8);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_cf_sf_of; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x80);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_of; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x70);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x10);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_of_af; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x1);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_pf; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x6);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_pf_af; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_pf_of; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x60);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x20);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_pf_of_af; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x78);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x8);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_pf_sf; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_pf_zf; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub al, byte ptr [rbx]
    ax_test![sub_al_byte_ptr_rbx_sf; 0x2a, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x0);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x1);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_af; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_cf_pf_sf; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0xfff0);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_cf_pf_sf_af; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0xffff);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_cf_pf_sf_of; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_cf_sf; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0xffe0);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_cf_sf_af; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0xfff8);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x8);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_cf_sf_of; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8001);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x8000);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_of; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x7fe0);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x20);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_of_af; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x7ff8);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x8);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_pf; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x7);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x6);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_pf_af; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0xf);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_pf_of; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x7ff0);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x10);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_pf_of_af; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x7fff);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_pf_sf; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub cx, word ptr [rbx]
    ax_test![sub_cx_word_ptr_rbx_pf_zf; 0x66, 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x1);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_af; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x8);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_cf_pf_sf; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0xfffffff0u32);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_cf_pf_sf_af; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0xffffffffu32);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_cf_pf_sf_of; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u32).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x80000000u32);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x80000000u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_cf_sf; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0xffffffe0u32);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_cf_sf_af; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0xfffffff8u32);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x8);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_cf_sf_of; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u32).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x80000001u32);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x80000000u32);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_of; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x80000000u32);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x7fffffe0);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x20);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_of_af; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x80000000u32);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x7ffffff8);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x8);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_pf; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x7);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x6);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_pf_af; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0xf);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_pf_of; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x80000000u32);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x7ffffff0);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x10);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_pf_of_af; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x80000000u32);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x7fffffffu32);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_pf_sf; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x80000000u32);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x80000000u32);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub ecx, dword ptr [rbx]
    ax_test![sub_ecx_dword_ptr_rbx_pf_zf; 0x2b, 0xb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x0);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_af; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x8);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_cf_pf_sf; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xfffffffffffffff0u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_cf_pf_sf_af; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xffffffffffffffffu64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_cf_sf; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xffffffffffffffe0u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_cf_sf_af; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xfffffffffffffff8u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x8);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_of; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x7fffffffffffffe0u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x20);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_of_af; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x7ffffffffffffff8u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x8);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_pf; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x7);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x6);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_pf_af; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xf);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_pf_of; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x7ffffffffffffff0u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x10);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_pf_of_af; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x7fffffffffffffffu64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x1);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_pf_sf; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x8000000000000000u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_pf_sf_af; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0xcf6d86fccfbbd28bu64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xcf6d86fccfbbd26cu64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x1f);
        };
        (FLAG_PF | FLAG_SF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_pf_zf; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x0);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_sf; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0xc4142648cbf91786u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xc4142648cbf91776u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x10);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub r11, qword ptr [rbx]
    ax_test![sub_r11_qword_ptr_rbx_sf_af; 0x4c, 0x2b, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0xef3e2c7ce6fa5534u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0xef3e2c7ce6fa54b5u64);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x7f);
        };
        (FLAG_SF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub ax, 0x1358
    ax_test![sub_ax_0x1358; 0x66, 0x2d, 0x58, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x6ca7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub ax, 0x1358
    ax_test![sub_ax_0x1358_af; 0x66, 0x2d, 0x58, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xca8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub ax, 0x1358
    ax_test![sub_ax_0x1358_cf_pf_sf; 0x66, 0x2d, 0x58, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xecb7);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub ax, 0x1358
    ax_test![sub_ax_0x1358_cf_pf_sf_af; 0x66, 0x2d, 0x58, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xeca9);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub ax, 0x1358
    ax_test![sub_ax_0x1358_cf_sf; 0x66, 0x2d, 0x58, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xecb0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub ax, 0x1358
    ax_test![sub_ax_0x1358_cf_sf_af; 0x66, 0x2d, 0x58, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xeca8);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub ax, 0x1358
    ax_test![sub_ax_0x1358_of_af; 0x66, 0x2d, 0x58, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x6ca8);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub eax, 0x1358
    ax_test![sub_eax_0x1358; 0x2d, 0x58, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x6ca7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub eax, 0x1358
    ax_test![sub_eax_0x1358_af; 0x2d, 0x58, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x6ca8);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub eax, 0x1358
    ax_test![sub_eax_0x1358_cf_pf_sf; 0x2d, 0x58, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffecb7u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub eax, 0x1358
    ax_test![sub_eax_0x1358_cf_pf_sf_af; 0x2d, 0x58, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffeca9u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub eax, 0x1358
    ax_test![sub_eax_0x1358_cf_sf; 0x2d, 0x58, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffecb0u32);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub eax, 0x1358
    ax_test![sub_eax_0x1358_cf_sf_af; 0x2d, 0x58, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffeca8u32);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub eax, 0x1358
    ax_test![sub_eax_0x1358_of_af; 0x2d, 0x58, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7fffeca8);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x2125a5885c6bd01au64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2125a5882b18bee5u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_af; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x80000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4eaceecb);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_cf_pf_sf; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xffffffffceaceed2u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_cf_pf_sf_af; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xffffffffceaceeccu64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_cf_sf; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xffffffffceaceed3u64);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_cf_sf_af; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xffffffffceaceecbu64);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_of_af; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7fffffffceaceecbu64);
        };
        (FLAG_OF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_pf; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4eaceeca);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_pf_af; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x275471f0319c03b2u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x275471f00048f27du64);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_pf_sf; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xa6dde50937588068u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xa6dde50906056f33u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_pf_sf_af; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xf3d087a86921fb94u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xf3d087a837ceea5fu64);
        };
        (FLAG_PF | FLAG_SF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_sf; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xc8389d6b5e07cebbu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xc8389d6b2cb4bd86u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub rax, 0x31531135
    ax_test![sub_rax_0x31531135_sf_af; 0x48, 0x2d, 0x35, 0x11, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xc36e5031e6770281u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xc36e5031b523f14cu64);
        };
        (FLAG_SF | FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x3f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0xe);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31_af; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x4f);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31_cf_pf_sf; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0xffd7);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31_cf_pf_sf_af; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0xffcf);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31_cf_sf; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0xffd0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31_cf_sf_af; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0xffdf);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31_pf; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x4e);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31_pf_af; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x40).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0xf);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31_pf_of_af; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x7fcf);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub word ptr [rax], 0x31
    ax_test![sub_word_ptr_rax_0x31_pf_zf; 0x66, 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x31).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x3f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0xe);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31_af; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x4f);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31_cf_pf_sf; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0xffffffd7u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31_cf_pf_sf_af; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0xffffffcfu32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31_cf_sf; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0xffffffd0u32);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31_cf_sf_af; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0xffffffdfu32);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31_pf; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x4e);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31_pf_af; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x40).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0xf);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31_pf_of_af; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u32).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x7fffffcf);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub dword ptr [rax], 0x31
    ax_test![sub_dword_ptr_rax_0x31_pf_zf; 0x83, 0x28, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x31).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rax], 0x35
    ax_test![sub_qword_ptr_rax_0x35; 0x48, 0x83, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x4a);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rax], 0x35
    ax_test![sub_qword_ptr_rax_0x35_af; 0x48, 0x83, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x40).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xb);
        };
        (FLAG_AF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub qword ptr [rax], 0x35
    ax_test![sub_qword_ptr_rax_0x35_cf_pf_sf; 0x48, 0x83, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xffffffffffffffd2u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rax], 0x35
    ax_test![sub_qword_ptr_rax_0x35_cf_pf_sf_af; 0x48, 0x83, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xffffffffffffffccu64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_AF; FLAG_ZF | FLAG_OF)
    ];

    // sub qword ptr [rax], 0x35
    ax_test![sub_qword_ptr_rax_0x35_cf_sf; 0x48, 0x83, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xffffffffffffffd3u64);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rax], 0x35
    ax_test![sub_qword_ptr_rax_0x35_cf_sf_af; 0x48, 0x83, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xffffffffffffffcbu64);
        };
        (FLAG_CF | FLAG_SF | FLAG_AF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // sub qword ptr [rax], 0x35
    ax_test![sub_qword_ptr_rax_0x35_pf; 0x48, 0x83, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x3f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xa);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub qword ptr [rax], 0x35
    ax_test![sub_qword_ptr_rax_0x35_pf_af; 0x48, 0x83, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x41).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0xc);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // sub qword ptr [rax], 0x35
    ax_test![sub_qword_ptr_rax_0x35_pf_zf; 0x48, 0x83, 0x28, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x35).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub byte ptr [rax], 0xf
    ax_test![sub_byte_ptr_rax_0xf; 0x80, 0x28, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_8(0x1000).unwrap(), 0x10);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    // sub word ptr [rax], 0x13b
    ax_test![sub_word_ptr_rax_0x13b_pf_of_af; 0x66, 0x81, 0x28, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_16(0x1000).unwrap(), 0x7ec5);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub dword ptr [rax], 0x4c8ab
    ax_test![sub_dword_ptr_rax_0x4c8ab_pf_of_af; 0x81, 0x28, 0xab, 0xc8, 0x4, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u32).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x7ffb3755);
        };
        (FLAG_PF | FLAG_OF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // sub qword ptr [rax], 0x1de62ef
    ax_test![sub_qword_ptr_rax_0x1de62ef_pf_af; 0x48, 0x81, 0x28, 0xef, 0x62, 0xde, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x2000000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_64(0x1000).unwrap(), 0x219d11);
        };
        (FLAG_PF | FLAG_AF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
