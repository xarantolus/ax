use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Cmp;

use super::axecutor::Axecutor;
use super::errors::AxError;
use super::macros::calculate_rm_imm;
use crate::instructions::flags::*;
use crate::instructions::macros::calculate_r_rm;
use crate::instructions::macros::calculate_rm_r;
use crate::instructions::macros::fatal_error;
use crate::instructions::macros::NO_WRITEBACK;

impl Axecutor {
    pub fn mnemonic_cmp(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cmp);

        match i.code() {
            Cmp_rm8_r8 => self.instr_cmp_rm8_r8(i),
            Cmp_rm16_r16 => self.instr_cmp_rm16_r16(i),
            Cmp_rm32_r32 => self.instr_cmp_rm32_r32(i),
            Cmp_rm64_r64 => self.instr_cmp_rm64_r64(i),
            Cmp_r8_rm8 => self.instr_cmp_r8_rm8(i),
            Cmp_r16_rm16 => self.instr_cmp_r16_rm16(i),
            Cmp_r32_rm32 => self.instr_cmp_r32_rm32(i),
            Cmp_r64_rm64 => self.instr_cmp_r64_rm64(i),
            Cmp_AL_imm8 => self.instr_cmp_al_imm8(i),
            Cmp_AX_imm16 => self.instr_cmp_ax_imm16(i),
            Cmp_EAX_imm32 => self.instr_cmp_eax_imm32(i),
            Cmp_RAX_imm32 => self.instr_cmp_rax_imm32(i),
            Cmp_rm8_imm8 => self.instr_cmp_rm8_imm8(i),
            Cmp_rm16_imm16 => self.instr_cmp_rm16_imm16(i),
            Cmp_rm32_imm32 => self.instr_cmp_rm32_imm32(i),
            Cmp_rm64_imm32 => self.instr_cmp_rm64_imm32(i),
            Cmp_rm8_imm8_82 => self.instr_cmp_rm8_imm8_82(i),
            Cmp_rm16_imm8 => self.instr_cmp_rm16_imm8(i),
            Cmp_rm32_imm8 => self.instr_cmp_rm32_imm8(i),
            Cmp_rm64_imm8 => self.instr_cmp_rm64_imm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Cmp", i.code()),
        }
    }

    /// CMP r/m8, r8
    ///
    /// 38 /r
    fn instr_cmp_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_rm8_r8);

        calculate_rm_r![u8f; self; i; |d:u8, s:u8| {
            let result = (d as i8).wrapping_sub(s as i8) as u8;

            (
                result,
                if ((d as i16 ^ s as i16) & (d as i16 ^ result as i16) & 0x80) != 0 { FLAG_OF } else { 0 } |
                if ((d as i16 | 0x100).wrapping_sub(s as i16)) & 0x100 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r/m16, r16
    ///
    /// o16 39 /r
    fn instr_cmp_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_rm16_r16);

        calculate_rm_r![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_sub(s as i16) as u16;

            (
                result,
                if ((d as i32 ^ s as i32) & (d as i32 ^ result as i32) & 0x8000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i32 | 0x10000).wrapping_sub(s as i32)) & 0x10000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r/m32, r32
    ///
    /// o32 39 /r
    fn instr_cmp_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_rm32_r32);

        calculate_rm_r![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_sub(s as i32) as u32;

            (
                result,
                if ((d as i64 ^ s as i64) & (d as i64 ^ result as i64) & 0x80000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i64 | 0x100000000).wrapping_sub(s as i64)) & 0x100000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r/m64, r64
    ///
    /// o64 39 /r
    fn instr_cmp_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_rm64_r64);

        calculate_rm_r![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_sub(s as i64) as u64;

            (
                result,
                if ((d as i128 ^ s as i128) & (d as i128 ^ result as i128) & 0x8000000000000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i128 | 0x10000000000000000).wrapping_sub(s as i128)) & 0x10000000000000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r8, r/m8
    ///
    /// 3A /r
    fn instr_cmp_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_r8_rm8);

        calculate_r_rm![u8f; self; i; |d:u8, s:u8| {
            let result = (d as i8).wrapping_sub(s as i8) as u8;

            (
                result,
                if ((d as i16 ^ s as i16) & (d as i16 ^ result as i16) & 0x80) != 0 { FLAG_OF } else { 0 } |
                if ((d as i16 | 0x100).wrapping_sub(s as i16)) & 0x100 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r16, r/m16
    ///
    /// o16 3B /r
    fn instr_cmp_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_r16_rm16);

        calculate_r_rm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_sub(s as i16) as u16;

            (
                result,
                if ((d as i32 ^ s as i32) & (d as i32 ^ result as i32) & 0x8000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i32 | 0x10000).wrapping_sub(s as i32)) & 0x10000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r32, r/m32
    ///
    /// o32 3B /r
    fn instr_cmp_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_r32_rm32);

        calculate_r_rm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_sub(s as i32) as u32;

            (
                result,
                if ((d as i64 ^ s as i64) & (d as i64 ^ result as i64) & 0x80000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i64 | 0x100000000).wrapping_sub(s as i64)) & 0x100000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r64, r/m64
    ///
    /// o64 3B /r
    fn instr_cmp_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_r64_rm64);

        calculate_r_rm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_sub(s as i64) as u64;

            (
                result,
                if ((d as i128 ^ s as i128) & (d as i128 ^ result as i128) & 0x8000000000000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i128 | 0x10000000000000000).wrapping_sub(s as i128)) & 0x10000000000000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP AL, imm8
    ///
    /// 3C ib
    fn instr_cmp_al_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_AL_imm8);

        self.instr_cmp_rm8_imm8(i)
    }

    /// CMP AX, imm16
    ///
    /// o16 3D iw
    fn instr_cmp_ax_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_AX_imm16);

        self.instr_cmp_rm16_imm16(i)
    }

    /// CMP EAX, imm32
    ///
    /// o32 3D id
    fn instr_cmp_eax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_EAX_imm32);

        self.instr_cmp_rm32_imm32(i)
    }

    /// CMP RAX, imm32
    ///
    /// o64 3D id
    fn instr_cmp_rax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_RAX_imm32);

        self.instr_cmp_rm64_imm32(i)
    }

    /// CMP r/m8, imm8
    ///
    /// 80 /7 ib
    fn instr_cmp_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u8f; self; i; |d:u8, s:u8| {
            let result = (d as i8).wrapping_sub(s as i8) as u8;

            (
                result,
                if ((d as i16 ^ s as i16) & (d as i16 ^ result as i16) & 0x80) != 0 { FLAG_OF } else { 0 } |
                if ((d as i16 | 0x100).wrapping_sub(s as i16)) & 0x100 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r/m16, imm16
    ///
    /// o16 81 /7 iw
    fn instr_cmp_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_sub(s as i16) as u16;

            (
                result,
                if ((d as i32 ^ s as i32) & (d as i32 ^ result as i32) & 0x8000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i32 | 0x10000).wrapping_sub(s as i32)) & 0x10000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r/m32, imm32
    ///
    /// o32 81 /7 id
    fn instr_cmp_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_sub(s as i32) as u32;

            (
                result,
                if ((d as i64 ^ s as i64) & (d as i64 ^ result as i64) & 0x80000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i64 | 0x100000000).wrapping_sub(s as i64)) & 0x100000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r/m64, imm32
    ///
    /// o64 81 /7 id
    fn instr_cmp_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        calculate_rm_imm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_sub(s as i64) as u64;

            (
                result,
                if ((d as i128 ^ s as i128) & (d as i128 ^ result as i128) & 0x8000000000000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i128 | 0x10000000000000000).wrapping_sub(s as i128)) & 0x10000000000000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r/m8, imm8
    ///
    /// 82 /7 ib
    fn instr_cmp_rm8_imm8_82(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_rm8_imm8_82);

        self.instr_cmp_rm8_imm8(i)
    }

    /// CMP r/m16, imm8
    ///
    /// o16 83 /7 ib
    fn instr_cmp_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_rm16_imm8);

        calculate_rm_imm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as i16).wrapping_sub(s as i16) as u16;

            (
                result,
                if ((d as i32 ^ s as i32) & (d as i32 ^ result as i32) & 0x8000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i32 | 0x10000).wrapping_sub(s as i32)) & 0x10000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r/m32, imm8
    ///
    /// o32 83 /7 ib
    fn instr_cmp_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_rm32_imm8);

        calculate_rm_imm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as i32).wrapping_sub(s as i32) as u32;

            (
                result,
                if ((d as i64 ^ s as i64) & (d as i64 ^ result as i64) & 0x80000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i64 | 0x100000000).wrapping_sub(s as i64)) & 0x100000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }

    /// CMP r/m64, imm8
    ///
    /// o64 83 /7 ib
    fn instr_cmp_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Cmp_rm64_imm8);

        calculate_rm_imm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as i64).wrapping_sub(s as i64) as u64;

            (
                result,
                if ((d as i128 ^ s as i128) & (d as i128 ^ result as i128) & 0x8000000000000000) != 0 { FLAG_OF } else { 0 } |
                if ((d as i128 | 0x10000000000000000).wrapping_sub(s as i128)) & 0x10000000000000000 == 0 { FLAG_CF } else { 0 }
            )
        }; (set: NO_WRITEBACK | FLAG_SF | FLAG_ZF | FLAG_PF; clear: 0)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::instructions::tests::{
        assert_mem_value, assert_reg_value, ax_test, write_reg_value,
    };
    use iced_x86::Register::*;
    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x8);
            assert_mem_value!(b; a; 0x1000; 0x10);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x90);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x90);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xff);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xff);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_pf; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xa0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xa0);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_pf_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x82);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x82);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_pf_sf; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x10);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x10);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_pf_sf_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_pf_sf_of; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_pf_sf_of_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x82);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x82);
            assert_mem_value!(b; a; 0x1000; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_sf; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x20);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x20);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_sf_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x8);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_sf_of; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_cf_sf_of_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x81);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x81);
            assert_mem_value!(b; a; 0x1000; 0x10);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_of; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x10);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x10);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_of_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_pf; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x7);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_pf_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x10);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_pf_of; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x20);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x20);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_pf_of_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x8);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_pf_sf; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_pf_sf_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x90).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x8);
            assert_mem_value!(b; a; 0x1000; 0x90);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_pf_zf; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_sf; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp byte ptr [rax], bl
    ax_test![cmp_byte_ptr_rax_bl_sf_af; 0x38, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x90).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x90);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_af; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8);
            assert_mem_value!(w; a; 0x1000; 0x10);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_cf_pf_sf; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x10);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x10);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_cf_pf_sf_af; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x1);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_cf_pf_sf_of; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8000);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_cf_sf; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x20);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x20);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_cf_sf_af; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_cf_sf_of; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8000);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_of; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x20);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x20);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_of_af; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_pf; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x1);
            assert_mem_value!(w; a; 0x1000; 0x7);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_pf_af; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x1);
            assert_mem_value!(w; a; 0x1000; 0x10);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_pf_of; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x10);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x10);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_pf_of_af; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x1);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_pf_sf; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp word ptr [rax], cx
    ax_test![cmp_word_ptr_rax_cx_pf_zf; 0x66, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_af; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x8);
            assert_mem_value!(d; a; 0x1000; 0x10);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_cf_pf_sf; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x10);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x10);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_cf_pf_sf_af; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x1);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_cf_pf_sf_of; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x80000000u32);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_cf_sf; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x20);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x20);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_cf_sf_af; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x8);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_cf_sf_of; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x80000000u32);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_of; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x20);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x20);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_of_af; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x8);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_pf; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x1);
            assert_mem_value!(d; a; 0x1000; 0x7);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_pf_af; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x1);
            assert_mem_value!(d; a; 0x1000; 0x10);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_pf_of; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x10);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x10);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_pf_of_af; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x1);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_pf_sf; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp dword ptr [rax], ecx
    ax_test![cmp_dword_ptr_rax_ecx_pf_zf; 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x0);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_af; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x8);
            assert_mem_value!(q; a; 0x1000; 0x10);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x98c214c8bfa47469u64);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x98c214c8bfa47469u64);
            assert_mem_value!(q; a; 0x1000; 0x1f);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf_af; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0xffea810badfedcb2u64);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0xffea810badfedcb2u64);
            assert_mem_value!(q; a; 0x1000; 0x10);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf_pf; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0xaa65f07cc317b275u64);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0xaa65f07cc317b275u64);
            assert_mem_value!(q; a; 0x1000; 0x7f);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf_pf_af; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0xcc13f17ac7eed496u64);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x11).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0xcc13f17ac7eed496u64);
            assert_mem_value!(q; a; 0x1000; 0x11);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf_pf_sf; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x10);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x10);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf_pf_sf_af; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf_pf_sf_of; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x8000000000000000u64);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x8000000000000000u64);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf_sf; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x20);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x20);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf_sf_af; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x8);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x8);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_cf_sf_of; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x8000000000000000u64);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x8000000000000000u64);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_pf; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x7).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1);
            assert_mem_value!(q; a; 0x1000; 0x7);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_pf_af; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1);
            assert_mem_value!(q; a; 0x1000; 0x10);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp qword ptr [rax], rcx
    ax_test![cmp_qword_ptr_rax_rcx_pf_zf; 0x48, 0x39, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x0);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_af; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x10);
            assert_mem_value!(w; a; 0x1000; 0x8);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_cf_pf_sf; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_cf_pf_sf_af; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_cf_pf_sf_of; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_cf_sf; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_cf_sf_af; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x8);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_cf_sf_of; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_of; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x20);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_of_af; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x8);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_pf; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_pf_af; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x10);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_pf_of; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x10);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_pf_of_af; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_pf_sf; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp ax, word ptr [rcx]
    ax_test![cmp_ax_word_ptr_rcx_pf_zf; 0x66, 0x3b, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_af; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x10);
            assert_mem_value!(d; a; 0x1000; 0x8);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_cf_pf_sf; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_cf_pf_sf_af; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_cf_pf_sf_of; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_cf_sf; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_cf_sf_af; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x8);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_cf_sf_of; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_of; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x20);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_of_af; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x8);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_pf; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_pf_af; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x10);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_pf_of; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x10);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_pf_of_af; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_pf_sf; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp eax, dword ptr [rcx+4*rbx]
    ax_test![cmp_eax_dword_ptr_rcx_4_rbx_pf_zf; 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_af; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x10);
            assert_mem_value!(q; a; 0x1000; 0x8);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_cf_pf_sf; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_mem_value!(q; a; 0x1000; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_cf_pf_sf_af; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_cf_sf; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_mem_value!(q; a; 0x1000; 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_cf_sf_af; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_mem_value!(q; a; 0x1000; 0x8);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_of; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_mem_value!(q; a; 0x1000; 0x20);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_of_af; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_mem_value!(q; a; 0x1000; 0x8);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_pf; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_pf_af; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x10);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_pf_of; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_mem_value!(q; a; 0x1000; 0x10);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_pf_of_af; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_pf_sf; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_pf_sf_af; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x93be148ab21d1e26u64);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x93be148ab21d1e26u64);
            assert_mem_value!(q; a; 0x1000; 0x8);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_pf_zf; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_sf; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xf3432afa4adf67b4u64);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xf3432afa4adf67b4u64);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, qword ptr [rcx+4*rbx]
    ax_test![cmp_rax_qword_ptr_rcx_4_rbx_sf_af; 0x48, 0x3b, 0x4, 0x99;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xdbe407232cb35629u64);
            write_reg_value!(q; a; RCX; 0x1000);
            write_reg_value!(q; a; RBX; 0);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xdbe407232cb35629u64);
            assert_mem_value!(q; a; 0x1000; 0xf);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x10);
            assert_mem_value!(b; a; 0x1000; 0x8);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x90).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x90);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_pf; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xa0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0xa0);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_pf_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x82).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x82);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_pf_sf; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_pf_sf_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_pf_sf_of; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_pf_sf_of_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x82).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x10);
            assert_mem_value!(b; a; 0x1000; 0x82);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_sf; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x20);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_sf_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x8);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_sf_of; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_cf_sf_of_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x81).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x10);
            assert_mem_value!(b; a; 0x1000; 0x81);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_of; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x10).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x10);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_of_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_pf; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_pf_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x10);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_pf_of; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x20).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x20);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_pf_of_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x8);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_pf_sf; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_pf_sf_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x90);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x90);
            assert_mem_value!(b; a; 0x1000; 0x8);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_pf_zf; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_sf; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, byte ptr [rcx]
    ax_test![cmp_al_byte_ptr_rcx_sf_af; 0x3a, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x90);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x90);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_af; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x10);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_cf_pf_sf_af; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x2);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_cf_sf_af; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_of_af; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x82);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x82);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_pf; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_pf_af; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x20);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_pf_of_af; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_pf_sf; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_pf_sf_af; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x90);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x90);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_pf_zf; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x3);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x3);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_sf; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x83);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x83);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp al, 0x3
    ax_test![cmp_al_0x3_sf_af; 0x3c, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x92);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x92);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp bl, 0x81
    ax_test![cmp_bl_0x81_pf_af; 0x80, 0xfb, 0x81;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x90);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x90);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp bx, 0x813
    ax_test![cmp_bx_0x813; 0x66, 0x81, 0xfb, 0x13, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7fff);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp bx, 0x813
    ax_test![cmp_bx_0x813_cf_pf_sf; 0x66, 0x81, 0xfb, 0x13, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp bx, 0x813
    ax_test![cmp_bx_0x813_cf_pf_sf_af; 0x66, 0x81, 0xfb, 0x13, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp bx, 0x813
    ax_test![cmp_bx_0x813_cf_sf; 0x66, 0x81, 0xfb, 0x13, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x7);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp bx, 0x813
    ax_test![cmp_bx_0x813_cf_sf_af; 0x66, 0x81, 0xfb, 0x13, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x10);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp bx, 0x813
    ax_test![cmp_bx_0x813_pf_af; 0x66, 0x81, 0xfb, 0x13, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x1000);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp bx, 0x813
    ax_test![cmp_bx_0x813_pf_of_af; 0x66, 0x81, 0xfb, 0x13, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp ax, 0xff
    ax_test![cmp_ax_0xff_pf_zf; 0x66, 0x3d, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xff);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp eax, 0xff3
    ax_test![cmp_eax_0xff3_af; 0x3d, 0xf3, 0xf, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp eax, 0xff3
    ax_test![cmp_eax_0xff3_cf_pf_sf; 0x3d, 0xf3, 0xf, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp eax, 0xff3
    ax_test![cmp_eax_0xff3_cf_pf_sf_af; 0x3d, 0xf3, 0xf, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x10);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp eax, 0xff3
    ax_test![cmp_eax_0xff3_cf_sf; 0x3d, 0xf3, 0xf, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp eax, 0xff3
    ax_test![cmp_eax_0xff3_cf_sf_af; 0x3d, 0xf3, 0xf, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp eax, 0xff3
    ax_test![cmp_eax_0xff3_of_af; 0x3d, 0xf3, 0xf, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp eax, 0xff3
    ax_test![cmp_eax_0xff3_pf; 0x3d, 0xf3, 0xf, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7fff);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7fffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_af; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7d8392843083a172u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7d8392843083a172u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_cf_pf_sf; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_cf_pf_sf_af; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_cf_sf; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_cf_sf_af; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_pf; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x4db5a7310bfc3eaeu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4db5a7310bfc3eaeu64);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_pf_af; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x80000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x80000000u64);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_pf_of_af; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_OF; FLAG_CF | FLAG_ZF | FLAG_SF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_pf_sf; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xe3b1ba62b7f4d4b5u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xe3b1ba62b7f4d4b5u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_pf_sf_af; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xcb3a70e1af48a3b4u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xcb3a70e1af48a3b4u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_sf; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xb96276b85bc36cceu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xb96276b85bc36cceu64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rax, 0xff3315
    ax_test![cmp_rax_0xff3315_sf_af; 0x48, 0x3d, 0x15, 0x33, 0xff, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xd9582566f6a333d1u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xd9582566f6a333d1u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xda06ff0dd57dfcfu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xda06ff0dd57dfcfu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_af; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x80000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x80000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_cf_pf_sf; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_cf_pf_sf_af; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_cf_sf; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xf);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_cf_sf_af; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_of_af; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_pf; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_pf_af; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x3db9aaa5dfdbdbf4u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x3db9aaa5dfdbdbf4u64);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_pf_sf; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xdbbdec20391cd24fu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xdbbdec20391cd24fu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_pf_sf_af; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xb80aa7d8421bc9aeu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xb80aa7d8421bc9aeu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_pf_zf; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x7fffffffu64);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_sf; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xc528df29f33cc27fu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xc528df29f33cc27fu64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp rbx, 0x7fffffff
    ax_test![cmp_rbx_0x7fffffff_sf_af; 0x48, 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xcc8e3d353de4a7d4u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xcc8e3d353de4a7d4u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp ebx, 0x7fffffff
    ax_test![cmp_ebx_0x7fffffff_cf_pf_sf; 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp ebx, 0x7fffffff
    ax_test![cmp_ebx_0x7fffffff_cf_pf_sf_af; 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x8);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp ebx, 0x7fffffff
    ax_test![cmp_ebx_0x7fffffff_cf_sf; 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xf);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp ebx, 0x7fffffff
    ax_test![cmp_ebx_0x7fffffff_cf_sf_af; 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp ebx, 0x7fffffff
    ax_test![cmp_ebx_0x7fffffff_of_af; 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // cmp ebx, 0x7fffffff
    ax_test![cmp_ebx_0x7fffffff_pf_zf; 0x81, 0xfb, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // cmp cx, 0xffff
    ax_test![cmp_cx_0xffff_cf; 0x66, 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0xf);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp cx, 0xffff
    ax_test![cmp_cx_0xffff_cf_af; 0x66, 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x0);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp cx, 0xffff
    ax_test![cmp_cx_0xffff_cf_pf; 0x66, 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp cx, 0xffff
    ax_test![cmp_cx_0xffff_cf_pf_af; 0x66, 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp cx, 0xffff
    ax_test![cmp_cx_0xffff_cf_pf_sf_of; 0x66, 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x7fff);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp cx, 0xffff
    ax_test![cmp_cx_0xffff_cf_sf_af; 0x66, 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x8000);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // cmp ecx, 0xffffffff
    ax_test![cmp_ecx_0xffffffff_cf; 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0xf);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp ecx, 0xffffffff
    ax_test![cmp_ecx_0xffffffff_cf_af; 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x0);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp ecx, 0xffffffff
    ax_test![cmp_ecx_0xffffffff_cf_pf; 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp ecx, 0xffffffff
    ax_test![cmp_ecx_0xffffffff_cf_pf_af; 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x8);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp ecx, 0xffffffff
    ax_test![cmp_ecx_0xffffffff_cf_pf_sf_of; 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x7fffffffu32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp ecx, 0xffffffff
    ax_test![cmp_ecx_0xffffffff_cf_sf_af; 0x83, 0xf9, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x80000000u32);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];
    // cmp rcx, 0xfffffffffffffff0
    ax_test![cmp_rcx_0xfffffffffffffff0_cf; 0x48, 0x83, 0xf9, 0xf0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x0);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rcx, 0xfffffffffffffff0
    ax_test![cmp_rcx_0xfffffffffffffff0_cf_pf; 0x48, 0x83, 0xf9, 0xf0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cmp rcx, 0xfffffffffffffff0
    ax_test![cmp_rcx_0xfffffffffffffff0_cf_pf_sf; 0x48, 0x83, 0xf9, 0xf0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0xdf721273cc0aa214u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0xdf721273cc0aa214u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // cmp rcx, 0xfffffffffffffff0
    ax_test![cmp_rcx_0xfffffffffffffff0_cf_pf_sf_of; 0x48, 0x83, 0xf9, 0xf0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x7fffffffffffffffu64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // cmp rcx, 0xfffffffffffffff0
    ax_test![cmp_rcx_0xfffffffffffffff0_cf_sf; 0x48, 0x83, 0xf9, 0xf0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x8000000000000000u64);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];
}
