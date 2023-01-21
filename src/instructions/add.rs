use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Add;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;
use crate::helpers::macros::calculate_r_rm;
use crate::helpers::macros::calculate_rm_imm;
use crate::helpers::macros::calculate_rm_r;
use crate::helpers::macros::fatal_error;
use crate::state::flags::*;

impl Axecutor {
    pub fn mnemonic_add(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Add);

        match i.code() {
            Add_rm8_r8 => self.instr_add_rm8_r8(i),
            Add_rm16_r16 => self.instr_add_rm16_r16(i),
            Add_rm32_r32 => self.instr_add_rm32_r32(i),
            Add_rm64_r64 => self.instr_add_rm64_r64(i),
            Add_r8_rm8 => self.instr_add_r8_rm8(i),
            Add_r16_rm16 => self.instr_add_r16_rm16(i),
            Add_r32_rm32 => self.instr_add_r32_rm32(i),
            Add_r64_rm64 => self.instr_add_r64_rm64(i),
            Add_AL_imm8 => self.instr_add_al_imm8(i),
            Add_AX_imm16 => self.instr_add_ax_imm16(i),
            Add_EAX_imm32 => self.instr_add_eax_imm32(i),
            Add_RAX_imm32 => self.instr_add_rax_imm32(i),
            Add_rm8_imm8 => self.instr_add_rm8_imm8(i),
            Add_rm16_imm16 => self.instr_add_rm16_imm16(i),
            Add_rm32_imm32 => self.instr_add_rm32_imm32(i),
            Add_rm64_imm32 => self.instr_add_rm64_imm32(i),
            Add_rm8_imm8_82 => self.instr_add_rm8_imm8_82(i),
            Add_rm16_imm8 => self.instr_add_rm16_imm8(i),
            Add_rm32_imm8 => self.instr_add_rm32_imm8(i),
            Add_rm64_imm8 => self.instr_add_rm64_imm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Add", i.code()),
        }
    }

    /// ADD r/m8, r8
    ///
    /// 00 /r
    fn instr_add_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_rm8_r8);

        calculate_rm_r![u8f; self; i; |d:u8, s:u8| {
            let result = d.wrapping_add(s);

            (
                result,
                if (result & 0x80 != d & 0x80) && (result & 0x80 != s & 0x80) { FLAG_OF } else { 0 } |
                if ((d as u16) + (s as u16)) & 0x100 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r/m16, r16
    ///
    /// o16 01 /r
    fn instr_add_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_rm16_r16);

        calculate_rm_r![u16f; self; i; |d:u16, s:u16| {
            let result = d.wrapping_add(s);

            (
                result,
                if (result & 0x8000 != d & 0x8000) && (result & 0x8000 != s & 0x8000) { FLAG_OF } else { 0 } |
                if ((d as u32) + (s as u32)) & 0x10000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r/m32, r32
    ///
    /// o32 01 /r
    fn instr_add_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_rm32_r32);

        calculate_rm_r![u32f; self; i; |d:u32, s:u32| {
            let result = d.wrapping_add(s);

            (
                result,
                if (result & 0x80000000 != d & 0x80000000) && (result & 0x80000000 != s & 0x80000000) { FLAG_OF } else { 0 } |
                if ((d as u64) + (s as u64)) & 0x100000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r/m64, r64
    ///
    /// o64 01 /r
    fn instr_add_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_rm64_r64);

        calculate_rm_r![u64f; self; i; |d:u64, s:u64| {
            let result = d.wrapping_add(s);

            (
                result,
                if (result & 0x8000000000000000 != d & 0x8000000000000000) && (result & 0x8000000000000000 != s & 0x8000000000000000) { FLAG_OF } else { 0 } |
                if ((d as u128) + (s as u128)) & 0x10000000000000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r8, r/m8
    ///
    /// 02 /r
    fn instr_add_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_r8_rm8);

        calculate_r_rm![u8f; self; i; |d:u8, s:u8| {
            let result = (d as i8).wrapping_add(s as i8);

            (
                result as u8,
                if ((result as u8 & 0x80) != d & 0x80) && (result as u8 & 0x80 != s & 0x80) { FLAG_OF } else { 0 } |
                if ((d as u16) + (s as u16)) & 0x100 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r16, r/m16
    ///
    /// o16 03 /r
    fn instr_add_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_r16_rm16);

        calculate_r_rm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_add(s as i16);

            (
                result as u16,
                if ((result as u16 & 0x8000) != d & 0x8000) && (result as u16 & 0x8000 != s & 0x8000) { FLAG_OF } else { 0 } |
                if ((d as u32) + (s as u32)) & 0x10000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r32, r/m32
    ///
    /// o32 03 /r
    fn instr_add_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_r32_rm32);

        calculate_r_rm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_add(s as i32);

            (
                result as u32,
                if ((result as u32 & 0x80000000) != d & 0x80000000) && (result as u32 & 0x80000000 != s & 0x80000000) { FLAG_OF } else { 0 } |
                if ((d as u64) + (s as u64)) & 0x100000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r64, r/m64
    ///
    /// o64 03 /r
    fn instr_add_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_r64_rm64);

        calculate_r_rm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_add(s as i64);

            (
                result as u64,
                if ((result as u64 & 0x8000000000000000) != d & 0x8000000000000000) && (result as u64 & 0x8000000000000000 != s & 0x8000000000000000) { FLAG_OF } else { 0 } |
                if ((d as u128) + (s as u128)) & 0x10000000000000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD AL, imm8
    ///
    /// 04 ib
    fn instr_add_al_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_AL_imm8);

        self.instr_add_rm8_imm8(i)
    }

    /// ADD AX, imm16
    ///
    /// o16 05 iw
    fn instr_add_ax_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_AX_imm16);

        self.instr_add_rm16_imm16(i)
    }

    /// ADD EAX, imm32
    ///
    /// o32 05 id
    fn instr_add_eax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_EAX_imm32);

        self.instr_add_rm32_imm32(i)
    }

    /// ADD RAX, imm32
    ///
    /// o64 05 id
    fn instr_add_rax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_RAX_imm32);

        self.instr_add_rm64_imm32(i)
    }

    /// ADD r/m8, imm8
    ///
    /// 80 /0 ib
    fn instr_add_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u8f; self; i; |d:u8, s:u8| {
            let result = (d as i8).wrapping_add(s as i8);

            (
                result as u8,
                if ((result as u8 & 0x80) != d & 0x80) && (result as u8 & 0x80 != s & 0x80) { FLAG_OF } else { 0 } |
                if ((d as u16) + (s as u16)) & 0x100 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r/m16, imm16
    ///
    /// o16 81 /0 iw
    fn instr_add_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_add(s as i16);

            (
                result as u16,
                if ((result as u16 & 0x8000) != d & 0x8000) && (result as u16 & 0x8000 != s & 0x8000) { FLAG_OF } else { 0 } |
                if ((d as u32) + (s as u32)) & 0x10000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r/m32, imm32
    ///
    /// o32 81 /0 id
    fn instr_add_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_add(s as i32);

            (
                result as u32,
                if ((result as u32 & 0x80000000) != d & 0x80000000) && (result as u32 & 0x80000000 != s & 0x80000000) { FLAG_OF } else { 0 } |
                if ((d as u64) + (s as u64)) & 0x100000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r/m64, imm32
    ///
    /// o64 81 /0 id
    fn instr_add_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_add(s as i64);

            (
                result as u64,
                if ((result as u64 & 0x8000000000000000) != d & 0x8000000000000000) && (result as u64 & 0x8000000000000000 != s & 0x8000000000000000) { FLAG_OF } else { 0 } |
                if ((d as u128) + (s as u128)) & 0x10000000000000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r/m8, imm8
    ///
    /// 82 /0 ib
    fn instr_add_rm8_imm8_82(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_rm8_imm8_82);

        self.instr_add_rm8_imm8(i)
    }

    /// ADD r/m16, imm8
    ///
    /// o16 83 /0 ib
    fn instr_add_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_rm16_imm8);

        calculate_rm_imm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_add(s as i16);

            (
                result as u16,
                if ((result as u16 & 0x8000) != d & 0x8000) && (result as u16 & 0x8000 != s & 0x8000) { FLAG_OF } else { 0 } |
                if ((d as u32) + (s as u32)) & 0x10000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r/m32, imm8
    ///
    /// o32 83 /0 ib
    fn instr_add_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_rm32_imm8);

        calculate_rm_imm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_add(s as i32);

            (
                result as u32,
                if ((result as u32 & 0x80000000) != d & 0x80000000) && (result as u32 & 0x80000000 != s & 0x80000000) { FLAG_OF } else { 0 } |
                if ((d as u64) + (s as u64)) & 0x100000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADD r/m64, imm8
    ///
    /// o64 83 /0 ib
    fn instr_add_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Add_rm64_imm8);

        calculate_rm_imm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_add(s as i64);

            (
                result as u64,
                if ((result as u64 & 0x8000000000000000) != d & 0x8000000000000000) && (result as u64 & 0x8000000000000000 != s & 0x8000000000000000) { FLAG_OF } else { 0 } |
                if ((d as u128) + (s as u128)) & 0x10000000000000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{
        assert_mem_value, assert_reg_value, ax_test, write_flags, write_reg_value,
    };

    use crate::axecutor::Axecutor;
    use iced_x86::Register::*;
    // add al, bl
    ax_test![add_al_bl; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_75; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_of; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_of_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_pf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x6);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_pf_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x6);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_pf_zf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_pf_zf_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_pf_zf_of; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_pf_zf_of_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(b; a; BL; 0x80);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_sf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xfe);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_cf_sf_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xfe);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_pf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_reg_value!(b; a; BL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_pf_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0xf);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_reg_value!(b; a; BL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_pf_sf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_pf_sf_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_pf_sf_of; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
            write_reg_value!(b; a; BL; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x87);
            assert_reg_value!(b; a; BL; 0x7f);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add al, bl
    ax_test![add_al_bl_pf_sf_of_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
            write_reg_value!(b; a; BL; 0x7f);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x87);
            assert_reg_value!(b; a; BL; 0x7f);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add al, bl
    ax_test![add_al_bl_pf_zf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_pf_zf_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_sf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_sf_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(b; a; BL; 0x80);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add al, bl
    ax_test![add_al_bl_sf_of; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(b; a; BL; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; BL; 0x7f);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add al, bl
    ax_test![add_al_bl_sf_of_cf; 0x0, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(b; a; BL; 0x7f);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_reg_value!(b; a; BL; 0x7f);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_pf_zf; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; CX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(w; a; CX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add bx, cx
    ax_test![add_bx_cx; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; CX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x1);
            assert_reg_value!(w; a; CX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_pf; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; CX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xf);
            assert_reg_value!(w; a; CX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_pf_sf; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(w; a; CX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
            assert_reg_value!(w; a; CX; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_af; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(w; a; CX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x10);
            assert_reg_value!(w; a; CX; 0xf);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_pf_af; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(w; a; CX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x100);
            assert_reg_value!(w; a; CX; 0xff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_pf_sf_of_af; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(w; a; CX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
            assert_reg_value!(w; a; CX; 0x7fff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_sf; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(w; a; CX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8001);
            assert_reg_value!(w; a; CX; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_sf_of_af; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8);
            write_reg_value!(w; a; CX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8007);
            assert_reg_value!(w; a; CX; 0x7fff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_pf_sf_of; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x10);
            write_reg_value!(w; a; CX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x800f);
            assert_reg_value!(w; a; CX; 0x7fff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_sf_of; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x20);
            write_reg_value!(w; a; CX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x801f);
            assert_reg_value!(w; a; CX; 0x7fff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add bx, cx
    ax_test![add_bx_cx_cf_pf_zf_of; 0x66, 0x1, 0xcb; |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(w; a; CX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(w; a; CX; 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_pf_zf; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_reg_value!(d; a; R12D; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1);
            assert_reg_value!(d; a; R12D; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_pf; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xf);
            assert_reg_value!(d; a; R12D; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_pf_sf; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(d; a; R12D; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u32);
            assert_reg_value!(d; a; R12D; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_af; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(d; a; R12D; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x10);
            assert_reg_value!(d; a; R12D; 0xf);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_pf_af; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(d; a; R12D; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x100);
            assert_reg_value!(d; a; R12D; 0xff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_pf_sf_of_af; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(d; a; R12D; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u32);
            assert_reg_value!(d; a; R12D; 0x7fffffffu32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_sf; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(d; a; R12D; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000001u32);
            assert_reg_value!(d; a; R12D; 0x80000000u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_sf_of_af; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x8);
            write_reg_value!(d; a; R12D; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000007u32);
            assert_reg_value!(d; a; R12D; 0x7fffffffu32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_pf_sf_of; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x10);
            write_reg_value!(d; a; R12D; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x8000000fu32);
            assert_reg_value!(d; a; R12D; 0x7fffffffu32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_sf_of; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x20);
            write_reg_value!(d; a; R12D; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x8000001fu32);
            assert_reg_value!(d; a; R12D; 0x7fffffffu32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add ebx, r12d
    ax_test![add_ebx_r12d_cf_pf_zf_of; 0x44, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_reg_value!(d; a; R12D; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_reg_value!(d; a; R12D; 0x80000000u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_pf_zf; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; R12; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
            assert_reg_value!(q; a; R12; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; R12; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1);
            assert_reg_value!(q; a; R12; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_pf; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; R12; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xf);
            assert_reg_value!(q; a; R12; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_pf_sf; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; R12; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
            assert_reg_value!(q; a; R12; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
            write_reg_value!(q; a; R12; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x10);
            assert_reg_value!(q; a; R12; 0xf);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_pf_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
            write_reg_value!(q; a; R12; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x100);
            assert_reg_value!(q; a; R12; 0xff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_pf_sf_of_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
            write_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
            assert_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_sf; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
            write_reg_value!(q; a; R12; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000001u64);
            assert_reg_value!(q; a; R12; 0x8000000000000000u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_sf_of_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8);
            write_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000007u64);
            assert_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_pf_sf_of; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x10);
            write_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x800000000000000fu64);
            assert_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_sf_of; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x20);
            write_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x800000000000001fu64);
            assert_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_pf_zf_of; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_reg_value!(q; a; R12; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
            assert_reg_value!(q; a; R12; 0x8000000000000000u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_pf_sf_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xb8ee6d8e0a605a18u64);
            write_reg_value!(q; a; R12; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xb8ee6d8e0a605a27u64);
            assert_reg_value!(q; a; R12; 0xf);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_sf_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xd6f719afa070031bu64);
            write_reg_value!(q; a; R12; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xd6f719afa070041au64);
            assert_reg_value!(q; a; R12; 0xff);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_pf_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x819adf7b94f0f67fu64);
            write_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x19adf7b94f0f67eu64);
            assert_reg_value!(q; a; R12; 0x7fffffffffffffffu64);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xf8596daedffee74au64);
            write_reg_value!(q; a; R12; 0x800000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x596daedffee74au64);
            assert_reg_value!(q; a; R12; 0x800000000000000u64);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_pf_of; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_reg_value!(q; a; R12; 0xeb282f8c3d53cd63u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x6b282f8c3d53cd63u64);
            assert_reg_value!(q; a; R12; 0xeb282f8c3d53cd63u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_OF; FLAG_ZF | FLAG_SF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_sf; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xedba2e2a9e40fac6u64);
            write_reg_value!(q; a; R12; 0xe3844983ddad0778u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xd13e77ae7bee023eu64);
            assert_reg_value!(q; a; R12; 0xe3844983ddad0778u64);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_pf_of_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xdba42e7d650d88bdu64);
            write_reg_value!(q; a; R12; 0x99393a717aa9c025u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x74dd68eedfb748e2u64);
            assert_reg_value!(q; a; R12; 0x99393a717aa9c025u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_OF; FLAG_ZF | FLAG_SF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xcc645e9253e008fau64);
            write_reg_value!(q; a; R12; 0x788b3a728af1af2fu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x44ef9904ded1b829u64);
            assert_reg_value!(q; a; R12; 0x788b3a728af1af2fu64);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_pf; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x6b57ec33023fb536u64);
            write_reg_value!(q; a; R12; 0xabc5904853bcd699u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x171d7c7b55fc8bcfu64);
            assert_reg_value!(q; a; R12; 0xabc5904853bcd699u64);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_pf_sf; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xb44a247e7767e457u64);
            write_reg_value!(q; a; R12; 0xe0a1c94c31a2d675u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x94ebedcaa90abaccu64);
            assert_reg_value!(q; a; R12; 0xe0a1c94c31a2d675u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_of; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x845fd19e36a92e10u64);
            write_reg_value!(q; a; R12; 0x83797ec4fbeea67fu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x7d950633297d48fu64);
            assert_reg_value!(q; a; R12; 0x83797ec4fbeea67fu64);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_pf_sf_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xfd45279b6172b587u64);
            write_reg_value!(q; a; R12; 0x928de6dc52b2766eu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8fd30e77b4252bf5u64);
            assert_reg_value!(q; a; R12; 0x928de6dc52b2766eu64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // add rbx, r12
    ax_test![add_rbx_r12_cf_of_af; 0x4c, 0x1, 0xe3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xd6d13b2cc00c7b26u64);
            write_reg_value!(q; a; R12; 0x97da2ca88d689e1au64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x6eab67d54d751940u64);
            assert_reg_value!(q; a; R12; 0x97da2ca88d689e1au64);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_pf_zf; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_pf; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_mem_value!(b; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_sf; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_pf_sf; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_af; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x10);
            assert_mem_value!(b; a; 0x1000; 0xf);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_sf_of_af; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x7f);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_cf_pf_zf_af; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_cf_pf_af; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x6);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_pf_af; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x17);
            assert_mem_value!(b; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_pf_sf_of_af; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x87);
            assert_mem_value!(b; a; 0x1000; 0x7f);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_cf_af; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_sf_of; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8f);
            assert_mem_value!(b; a; 0x1000; 0x7f);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_cf_pf; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_pf_sf_of; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x20);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x9f);
            assert_mem_value!(b; a; 0x1000; 0x7f);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_cf; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x20);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1f);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_cf_pf_zf_of; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_cf_of; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // add al, byte ptr [rbx]
    ax_test![add_al_byte_ptr_rbx_cf_sf_af; 0x2, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xfe);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_pf_zf; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x1);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_pf; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xf);
            assert_mem_value!(w; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_pf_sf; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_af; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x10);
            assert_mem_value!(w; a; 0x1000; 0xf);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_pf_af; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x100);
            assert_mem_value!(w; a; 0x1000; 0xff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_pf_sf_of_af; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x7fff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_sf; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8001);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_sf_of_af; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8007);
            assert_mem_value!(w; a; 0x1000; 0x7fff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_pf_sf_of; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x10);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x800f);
            assert_mem_value!(w; a; 0x1000; 0x7fff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_sf_of; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x20);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x801f);
            assert_mem_value!(w; a; 0x1000; 0x7fff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add bx, word ptr [r11]
    ax_test![add_bx_word_ptr_r11_cf_pf_zf_of; 0x66, 0x41, 0x3, 0x1b;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_pf_zf; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_pf; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xf);
            assert_mem_value!(d; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_pf_sf; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_af; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x10);
            assert_mem_value!(d; a; 0x1000; 0xf);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_pf_af; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x100);
            assert_mem_value!(d; a; 0x1000; 0xff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_pf_sf_of_af; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x7fffffff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_sf; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000001u32);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_sf_of_af; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x8);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000007u32);
            assert_mem_value!(d; a; 0x1000; 0x7fffffff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_pf_sf_of; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x8000000fu32);
            assert_mem_value!(d; a; 0x1000; 0x7fffffff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_sf_of; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x20);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x8000001fu32);
            assert_mem_value!(d; a; 0x1000; 0x7fffffff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add ebx, dword ptr [rcx]
    ax_test![add_ebx_dword_ptr_rcx_cf_pf_zf_of; 0x3, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_pf_zf; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
            assert_mem_value!(q; a; 0x1008; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1);
            assert_mem_value!(q; a; 0x1008; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_pf; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xf);
            assert_mem_value!(q; a; 0x1008; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_af; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x10);
            assert_mem_value!(q; a; 0x1008; 0xf);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_pf_af; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x100);
            assert_mem_value!(q; a; 0x1008; 0xff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_pf_sf_of_af; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
            assert_mem_value!(q; a; 0x1008; 0x1);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_sf_of_af; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000007u64);
            assert_mem_value!(q; a; 0x1008; 0x8);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_pf_sf_of; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x800000000000000fu64);
            assert_mem_value!(q; a; 0x1008; 0x10);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_sf_of; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x800000000000001fu64);
            assert_mem_value!(q; a; 0x1008; 0x20);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_pf_sf; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
            assert_mem_value!(q; a; 0x1008; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_sf; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000001u64);
            assert_mem_value!(q; a; 0x1008; 0x1);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_sf_af; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x998bd641bc70c4d7u64);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x998bd641bc70c4e6u64);
            assert_mem_value!(q; a; 0x1008; 0xf);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rbx, qword ptr [r11+8]
    ax_test![add_rbx_qword_ptr_r11_8_pf_sf_af; 0x49, 0x3, 0x5b, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xe17fb3d9bfface1fu64);
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x1008, 8).unwrap();
            a.mem_write_64(0x1008, 0x41).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xe17fb3d9bfface60u64);
            assert_mem_value!(q; a; 0x1008; 0x41);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add al, 0x0
    ax_test![add_al_0x0_pf_zf; 0x4, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0x0
    ax_test![add_al_0x0; 0x4, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0x0
    ax_test![add_al_0x0_pf; 0x4, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0x0
    ax_test![add_al_0x0_sf; 0x4, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add al, 0x0
    ax_test![add_al_0x0_pf_sf; 0x4, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add al, 0x1
    ax_test![add_al_0x1; 0x4, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0x1
    ax_test![add_al_0x1_pf; 0x4, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0x1
    ax_test![add_al_0x1_af; 0x4, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x10);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0x1
    ax_test![add_al_0x1_sf_of_af; 0x4, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add al, 0x1
    ax_test![add_al_0x1_pf_sf; 0x4, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x81);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add al, 0x1
    ax_test![add_al_0x1_cf_pf_zf_af; 0x4, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // add al, 0x5
    ax_test![add_al_0x5_pf; 0x4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x5);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0x5
    ax_test![add_al_0x5; 0x4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xd);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0x5
    ax_test![add_al_0x5_pf_af; 0x4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x14);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0x5
    ax_test![add_al_0x5_pf_sf_of_af; 0x4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x84);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add al, 0x5
    ax_test![add_al_0x5_sf; 0x4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x85);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add al, 0x5
    ax_test![add_al_0x5_cf_af; 0x4, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x4);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0xff
    ax_test![add_al_0xff_pf_sf; 0x4, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add al, 0xff
    ax_test![add_al_0xff_cf_pf_zf_af; 0x4, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // add al, 0xff
    ax_test![add_al_0xff_cf_pf_af; 0x4, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x6);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0xff
    ax_test![add_al_0xff_cf_af; 0x4, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0xff
    ax_test![add_al_0xff_cf_pf; 0x4, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0xff
    ax_test![add_al_0xff_cf; 0x4, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1f);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add al, 0xff
    ax_test![add_al_0xff_cf_of; 0x4, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7f);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // add al, 0xff
    ax_test![add_al_0xff_cf_sf_af; 0x4, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xfe);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add ax, 0xff
    ax_test![add_ax_0xff_pf; 0x66, 0x5, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ax, 0xff
    ax_test![add_ax_0xff_pf_af; 0x66, 0x5, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x100);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ax, 0xff
    ax_test![add_ax_0xff_af; 0x66, 0x5, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x107);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ax, 0xff
    ax_test![add_ax_0xff; 0x66, 0x5, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x11f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ax, 0xff
    ax_test![add_ax_0xff_sf_of_af; 0x66, 0x5, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x80fe);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add ax, 0xff
    ax_test![add_ax_0xff_pf_sf; 0x66, 0x5, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x80ff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add ax, 0x7f
    ax_test![add_ax_0x7f; 0x66, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ax, 0x7f
    ax_test![add_ax_0x7f_af; 0x66, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x80);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ax, 0x7f
    ax_test![add_ax_0x7f_pf_af; 0x66, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x87);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ax, 0x7f
    ax_test![add_ax_0x7f_pf; 0x66, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x9f);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ax, 0x7f
    ax_test![add_ax_0x7f_pf_sf_of_af; 0x66, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x807e);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add ax, 0x7f
    ax_test![add_ax_0x7f_sf; 0x66, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x807f);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add eax, 0x7f
    ax_test![add_eax_0x7f; 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add eax, 0x7f
    ax_test![add_eax_0x7f_af; 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add eax, 0x7f
    ax_test![add_eax_0x7f_pf_af; 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x87);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add eax, 0x7f
    ax_test![add_eax_0x7f_pf; 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x9f);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add eax, 0x7f
    ax_test![add_eax_0x7f_pf_sf_of_af; 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8000007eu32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add eax, 0x7f
    ax_test![add_eax_0x7f_sf; 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8000007fu32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add ebx, 0xf
    ax_test![add_ebx_0xf_pf; 0x83, 0xc3, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, 0xf
    ax_test![add_ebx_0xf_af; 0x83, 0xc3, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x10);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, 0xf
    ax_test![add_ebx_0xf_pf_af; 0x83, 0xc3, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x17);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, 0xf
    ax_test![add_ebx_0xf; 0x83, 0xc3, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add ebx, 0xf
    ax_test![add_ebx_0xf_sf_of_af; 0x83, 0xc3, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x8000000eu32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add ebx, 0xf
    ax_test![add_ebx_0xf_pf_sf; 0x83, 0xc3, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x8000000fu32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add rax, 0x7f
    ax_test![add_rax_0x7f; 0x48, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rax, 0x7f
    ax_test![add_rax_0x7f_af; 0x48, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x80);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rax, 0x7f
    ax_test![add_rax_0x7f_pf_af; 0x48, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x87);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rax, 0x7f
    ax_test![add_rax_0x7f_pf; 0x48, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x9f);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rax, 0x7f
    ax_test![add_rax_0x7f_pf_sf_of_af; 0x48, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x800000000000007eu64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add rax, 0x7f
    ax_test![add_rax_0x7f_sf; 0x48, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x800000000000007fu64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rax, 0x7f
    ax_test![add_rax_0x7f_sf_af; 0x48, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xd7a19a9f0b71a828u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xd7a19a9f0b71a8a7u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rax, 0x7f
    ax_test![add_rax_0x7f_pf_sf_af; 0x48, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xb6fce2b91ad843a3u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xb6fce2b91ad84422u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add rax, 0x7f
    ax_test![add_rax_0x7f_pf_sf; 0x48, 0x83, 0xc0, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xb4f1e952a8acfe50u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xb4f1e952a8acfecfu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add eax, 0x7777f
    ax_test![add_eax_0x7777f; 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7777f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add eax, 0x7777f
    ax_test![add_eax_0x7777f_af; 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x77780);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add eax, 0x7777f
    ax_test![add_eax_0x7777f_pf_af; 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x77787);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add eax, 0x7777f
    ax_test![add_eax_0x7777f_pf; 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7779f);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add eax, 0x7777f
    ax_test![add_eax_0x7777f_pf_sf_of_af; 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8007777eu32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add eax, 0x7777f
    ax_test![add_eax_0x7777f_sf; 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8007777fu32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rax, 0x7777f
    ax_test![add_rax_0x7777f; 0x48, 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7777f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rax, 0x7777f
    ax_test![add_rax_0x7777f_af; 0x48, 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x77780);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rax, 0x7777f
    ax_test![add_rax_0x7777f_pf_af; 0x48, 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x77787);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rax, 0x7777f
    ax_test![add_rax_0x7777f_pf; 0x48, 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7779f);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add rax, 0x7777f
    ax_test![add_rax_0x7777f_pf_sf_of_af; 0x48, 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x800000000007777eu64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add rax, 0x7777f
    ax_test![add_rax_0x7777f_sf; 0x48, 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x800000000007777fu64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rax, 0x7777f
    ax_test![add_rax_0x7777f_sf_af; 0x48, 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xe24439cf2c6a05a1u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xe24439cf2c717d20u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // add rax, 0x7777f
    ax_test![add_rax_0x7777f_pf_sf_af; 0x48, 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xa536ee45dba5510fu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xa536ee45dbacc88eu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add rax, 0x7777f
    ax_test![add_rax_0x7777f_pf_sf; 0x48, 0x5, 0x7f, 0x77, 0x7, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xacc2851d636db830u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xacc2851d63752fafu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add byte ptr [rbx], 0x7f
    ax_test![add_byte_ptr_rbx_0x7f; 0x80, 0x3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add byte ptr [rbx], 0x7f
    ax_test![add_byte_ptr_rbx_0x7f_sf_of_af; 0x80, 0x3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add byte ptr [rbx], 0x7f
    ax_test![add_byte_ptr_rbx_0x7f_pf_sf_of_af; 0x80, 0x3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x87);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add byte ptr [rbx], 0x7f
    ax_test![add_byte_ptr_rbx_0x7f_sf_of; 0x80, 0x3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x8f);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add byte ptr [rbx], 0x7f
    ax_test![add_byte_ptr_rbx_0x7f_pf_sf_of; 0x80, 0x3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x9f);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // add byte ptr [rbx], 0x7f
    ax_test![add_byte_ptr_rbx_0x7f_pf_sf; 0x80, 0x3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add byte ptr [rbx], 0x7f
    ax_test![add_byte_ptr_rbx_0x7f_cf_pf_af; 0x80, 0x3, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x7e);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add word ptr [rbx], 0xff
    ax_test![add_word_ptr_rbx_0xff_pf; 0x66, 0x81, 0x3, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0xff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add word ptr [rbx], 0xff
    ax_test![add_word_ptr_rbx_0xff_pf_af; 0x66, 0x81, 0x3, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x100);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add word ptr [rbx], 0xff
    ax_test![add_word_ptr_rbx_0xff_af; 0x66, 0x81, 0x3, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x107);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add word ptr [rbx], 0xff
    ax_test![add_word_ptr_rbx_0xff; 0x66, 0x81, 0x3, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x11f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add word ptr [rbx], 0xff
    ax_test![add_word_ptr_rbx_0xff_sf_of_af; 0x66, 0x81, 0x3, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x80fe);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add word ptr [rbx], 0xff
    ax_test![add_word_ptr_rbx_0xff_pf_sf; 0x66, 0x81, 0x3, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x80ff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0xff
    ax_test![add_dword_ptr_rbx_0xff_pf; 0x81, 0x3, 0xff, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0xff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0xff
    ax_test![add_dword_ptr_rbx_0xff_pf_af; 0x81, 0x3, 0xff, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x100);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0xff
    ax_test![add_dword_ptr_rbx_0xff_af; 0x81, 0x3, 0xff, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x107);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0xff
    ax_test![add_dword_ptr_rbx_0xff; 0x81, 0x3, 0xff, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x11f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0xff
    ax_test![add_dword_ptr_rbx_0xff_sf_of_af; 0x81, 0x3, 0xff, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x800000feu32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add dword ptr [rbx], 0xff
    ax_test![add_dword_ptr_rbx_0xff_pf_sf; 0x81, 0x3, 0xff, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x800000ffu32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0x3
    ax_test![add_dword_ptr_rbx_0x3_pf; 0x83, 0x3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0x3
    ax_test![add_dword_ptr_rbx_0x3; 0x83, 0x3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0x3
    ax_test![add_dword_ptr_rbx_0x3_pf_af; 0x83, 0x3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x12);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0x3
    ax_test![add_dword_ptr_rbx_0x3_af; 0x83, 0x3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x102);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add dword ptr [rbx], 0x3
    ax_test![add_dword_ptr_rbx_0x3_sf_of_af; 0x83, 0x3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000002u32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // add dword ptr [rbx], 0x3
    ax_test![add_dword_ptr_rbx_0x3_pf_sf; 0x83, 0x3, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000003u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // add qword ptr [r11+15], 0x1
    ax_test![add_qword_ptr_r11_15_0x1; 0x49, 0x83, 0x43, 0xf, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x100f, 8).unwrap();
            a.mem_write_64(0x100f, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1000);
            assert_mem_value!(q; a; 0x100f; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add qword ptr [r11+15], 0x1
    ax_test![add_qword_ptr_r11_15_0x1_pf; 0x49, 0x83, 0x43, 0xf, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x100f, 8).unwrap();
            a.mem_write_64(0x100f, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1000);
            assert_mem_value!(q; a; 0x100f; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add qword ptr [r11+15], 0x1
    ax_test![add_qword_ptr_r11_15_0x1_af; 0x49, 0x83, 0x43, 0xf, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x100f, 8).unwrap();
            a.mem_write_64(0x100f, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1000);
            assert_mem_value!(q; a; 0x100f; 0x10);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add qword ptr [r11+15], 0x1
    ax_test![add_qword_ptr_r11_15_0x1_pf_af; 0x49, 0x83, 0x43, 0xf, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x100f, 8).unwrap();
            a.mem_write_64(0x100f, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1000);
            assert_mem_value!(q; a; 0x100f; 0x100);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add qword ptr [r11+15], 0xffff931
    ax_test![add_qword_ptr_r11_15_0xffff931; 0x49, 0x81, 0x43, 0xf, 0x31, 0xf9, 0xff, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x100f, 8).unwrap();
            a.mem_write_64(0x100f, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1000);
            assert_mem_value!(q; a; 0x100f; 0xffff931);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add qword ptr [r11+15], 0xffff931
    ax_test![add_qword_ptr_r11_15_0xffff931_pf; 0x49, 0x81, 0x43, 0xf, 0x31, 0xf9, 0xff, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x100f, 8).unwrap();
            a.mem_write_64(0x100f, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1000);
            assert_mem_value!(q; a; 0x100f; 0xffff939);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add qword ptr [r11+15], 0xffff931
    ax_test![add_qword_ptr_r11_15_0xffff931_af; 0x49, 0x81, 0x43, 0xf, 0x31, 0xf9, 0xff, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x100f, 8).unwrap();
            a.mem_write_64(0x100f, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1000);
            assert_mem_value!(q; a; 0x100f; 0xffff940);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // add qword ptr [r11+15], 0xffff931
    ax_test![add_qword_ptr_r11_15_0xffff931_pf_af; 0x49, 0x81, 0x43, 0xf, 0x31, 0xf9, 0xff, 0xf;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x1000);
            a.mem_init_zero(0x100f, 8).unwrap();
            a.mem_write_64(0x100f, 0x1f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x1000);
            assert_mem_value!(q; a; 0x100f; 0xffff950);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
