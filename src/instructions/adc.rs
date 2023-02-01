use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Adc;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::calculate_r_rm;
use crate::helpers::macros::calculate_rm_imm;
use crate::helpers::macros::calculate_rm_r;
use crate::helpers::macros::fatal_error;
use crate::state::flags::*;

impl Axecutor {
    pub(crate) fn mnemonic_adc(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Adc);

        match i.code() {
            Adc_rm8_r8 => self.instr_adc_rm8_r8(i),
            Adc_rm16_r16 => self.instr_adc_rm16_r16(i),
            Adc_rm32_r32 => self.instr_adc_rm32_r32(i),
            Adc_rm64_r64 => self.instr_adc_rm64_r64(i),
            Adc_r8_rm8 => self.instr_adc_r8_rm8(i),
            Adc_r16_rm16 => self.instr_adc_r16_rm16(i),
            Adc_r32_rm32 => self.instr_adc_r32_rm32(i),
            Adc_r64_rm64 => self.instr_adc_r64_rm64(i),
            Adc_AL_imm8 => self.instr_adc_al_imm8(i),
            Adc_AX_imm16 => self.instr_adc_ax_imm16(i),
            Adc_EAX_imm32 => self.instr_adc_eax_imm32(i),
            Adc_RAX_imm32 => self.instr_adc_rax_imm32(i),
            Adc_rm8_imm8 => self.instr_adc_rm8_imm8(i),
            Adc_rm16_imm16 => self.instr_adc_rm16_imm16(i),
            Adc_rm32_imm32 => self.instr_adc_rm32_imm32(i),
            Adc_rm64_imm32 => self.instr_adc_rm64_imm32(i),
            Adc_rm8_imm8_82 => self.instr_adc_rm8_imm8_82(i),
            Adc_rm16_imm8 => self.instr_adc_rm16_imm8(i),
            Adc_rm32_imm8 => self.instr_adc_rm32_imm8(i),
            Adc_rm64_imm8 => self.instr_adc_rm64_imm8(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Adc", i.code()),
        }
    }

    /// ADC r/m8, r8
    ///
    /// 10 /r
    fn instr_adc_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_rm8_r8);

        let flags = self.state.rflags;
        calculate_rm_r![u8f; self; i; |d:u8, s:u8| {
            let result = (d as u16).wrapping_add(s as u16).wrapping_add(u16::from(flags & FLAG_CF != 0));

            (
                result as u8,
                if (result & 0x80 != (d as u16) & 0x80) && (result & 0x80 != (s as u16) & 0x80) { FLAG_OF } else { 0 } |
                if result & 0x100 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_CF)]
    }

    /// ADC r/m16, r16
    ///
    /// o16 11 /r
    fn instr_adc_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_rm16_r16);

        let flags = self.state.rflags;
        calculate_rm_r![u16f; self; i; |d:u16, s:u16| {
            let result = (d as u32).wrapping_add(s as u32).wrapping_add(u32::from(flags & FLAG_CF != 0));

            (
                result as u16,
                if (result & 0x8000 != (d as u32) & 0x8000) && (result & 0x8000 != (s as u32) & 0x8000) { FLAG_OF } else { 0 } |
                if result & 0x10000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_CF)]
    }

    /// ADC r/m32, r32
    ///
    /// o32 11 /r
    fn instr_adc_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_rm32_r32);

        let flags = self.state.rflags;
        calculate_rm_r![u32f; self; i; |d:u32, s:u32| {
            let result = (d as u64).wrapping_add(s as u64).wrapping_add(u64::from(flags & FLAG_CF != 0));

            (
                result as u32,
                if (result & 0x80000000 != (d as u64) & 0x80000000) && (result & 0x80000000 != (s as u64) & 0x80000000) { FLAG_OF } else { 0 } |
                if result & 0x100000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_CF)]
    }

    /// ADC r/m64, r64
    ///
    /// o64 11 /r
    fn instr_adc_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_rm64_r64);

        let flags = self.state.rflags;
        calculate_rm_r![u64f; self; i; |d:u64, s:u64| {
            let result = (d as u128).wrapping_add(s as u128).wrapping_add(u128::from(flags & FLAG_CF != 0));

            (
                result as u64,
                if (result & 0x8000000000000000 != (d as u128) & 0x8000000000000000) && (result & 0x8000000000000000 != (s as u128) & 0x8000000000000000) { FLAG_OF } else { 0 } |
                if result & 0x10000000000000000u128 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r8, r/m8
    ///
    /// 12 /r
    fn instr_adc_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_r8_rm8);

        let flags = self.state.rflags;
        calculate_r_rm![u8f; self; i; |d:u8, s:u8| {
            let result = (d as u16).wrapping_add(s as u16).wrapping_add(u16::from(flags & FLAG_CF != 0));

            (
                result as u8,
                if (result & 0x80 != (d as u16) & 0x80) && (result & 0x80 != (s as u16) & 0x80) { FLAG_OF } else { 0 } |
                if result & 0x100 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r16, r/m16
    ///
    /// o16 13 /r
    fn instr_adc_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_r16_rm16);

        let flags = self.state.rflags;
        calculate_r_rm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as u32).wrapping_add(s as u32).wrapping_add(u32::from(flags & FLAG_CF != 0));

            (
                result as u16,
                if (result & 0x8000 != (d as u32) & 0x8000) && (result & 0x8000 != (s as u32) & 0x8000) { FLAG_OF } else { 0 } |
                if result & 0x10000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r32, r/m32
    ///
    /// o32 13 /r
    fn instr_adc_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_r32_rm32);

        let flags = self.state.rflags;
        calculate_r_rm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as u64).wrapping_add(s as u64).wrapping_add(u64::from(flags & FLAG_CF != 0));

            (
                result as u32,
                if (result & 0x80000000 != (d as u64) & 0x80000000) && (result & 0x80000000 != (s as u64) & 0x80000000) { FLAG_OF } else { 0 } |
                if result & 0x100000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r64, r/m64
    ///
    /// o64 13 /r
    fn instr_adc_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_r64_rm64);

        let flags = self.state.rflags;
        calculate_r_rm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as u128).wrapping_add(s as u128).wrapping_add(u128::from(flags & FLAG_CF != 0));

            (
                result as u64,
                if (result & 0x8000000000000000 != (d as u128) & 0x8000000000000000) && (result & 0x8000000000000000 != (s as u128) & 0x8000000000000000) { FLAG_OF } else { 0 } |
                if result & 0x10000000000000000u128 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC AL, imm8
    ///
    /// 14 ib
    fn instr_adc_al_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_AL_imm8);

        self.instr_adc_rm8_imm8(i)
    }

    /// ADC AX, imm16
    ///
    /// o16 15 iw
    fn instr_adc_ax_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_AX_imm16);

        self.instr_adc_rm16_imm16(i)
    }

    /// ADC EAX, imm32
    ///
    /// o32 15 id
    fn instr_adc_eax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_EAX_imm32);

        self.instr_adc_rm32_imm32(i)
    }

    /// ADC RAX, imm32
    ///
    /// o64 15 id
    fn instr_adc_rax_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_RAX_imm32);

        self.instr_adc_rm64_imm32(i)
    }

    /// ADC r/m8, imm8
    ///
    /// 80 /2 ib
    fn instr_adc_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        let flags = self.state.rflags;
        calculate_rm_imm![u8f; self; i; |d:u8, s:u8| {
            let result = (d as u16).wrapping_add(s as u16).wrapping_add(u16::from(flags & FLAG_CF != 0));

            (
                result as u8,
                if (result & 0x80 != (d as u16) & 0x80) && (result & 0x80 != (s as u16) & 0x80) { FLAG_OF } else { 0 } |
                if result & 0x100 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r/m16, imm16
    ///
    /// o16 81 /2 iw
    fn instr_adc_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        let flags = self.state.rflags;
        calculate_rm_imm![u16f; self; i; |d:u16, s:u16| {
            let result = (d as u32).wrapping_add(s as u32).wrapping_add(u32::from(flags & FLAG_CF != 0));

            (
                result as u16,
                if (result & 0x8000 != (d as u32) & 0x8000) && (result & 0x8000 != (s as u32) & 0x8000) { FLAG_OF } else { 0 } |
                if result & 0x10000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r/m32, imm32
    ///
    /// o32 81 /2 id
    fn instr_adc_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        let flags = self.state.rflags;
        calculate_rm_imm![u32f; self; i; |d:u32, s:u32| {
            let result = (d as u64).wrapping_add(s as u64).wrapping_add(u64::from(flags & FLAG_CF != 0));

            (
                result as u32,
                if (result & 0x80000000 != (d as u64) & 0x80000000) && (result & 0x80000000 != (s as u64) & 0x80000000) { FLAG_OF } else { 0 } |
                if result & 0x100000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r/m64, imm32
    ///
    /// o64 81 /2 id
    fn instr_adc_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        let flags = self.state.rflags;
        calculate_rm_imm![u64f; self; i; |d:u64, s:u64| {
            let result = (d as u128).wrapping_add(s as u128).wrapping_add(u128::from(flags & FLAG_CF != 0));

            (
                result as u64,
                if (result & 0x8000000000000000 != (d as u128) & 0x8000000000000000) && (result & 0x8000000000000000 != (s as u128) & 0x8000000000000000) { FLAG_OF } else { 0 } |
                if result & 0x10000000000000000u128 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r/m8, imm8
    ///
    /// 82 /2 ib
    fn instr_adc_rm8_imm8_82(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_rm8_imm8_82);

        self.instr_adc_rm8_imm8(i)
    }

    /// ADC r/m16, imm8
    ///
    /// o16 83 /2 ib
    fn instr_adc_rm16_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_rm16_imm8);

        let flags = self.state.rflags;
        calculate_rm_imm![u16f; u8; self; i; |d:u16, s:u8| {
            let result = (d as u32).wrapping_add(s as u32).wrapping_add(u32::from(flags & FLAG_CF != 0));

            (
                result as u16,
                if (result & 0x8000 != (d as u32) & 0x8000) && (result & 0x8000 != (s as u32) & 0x8000) { FLAG_OF } else { 0 } |
                if result & 0x10000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r/m32, imm8
    ///
    /// o32 83 /2 ib
    fn instr_adc_rm32_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_rm32_imm8);

        let flags = self.state.rflags;
        calculate_rm_imm![u32f; u8; self; i; |d:u32, s:u8| {
            let result = (d as u64).wrapping_add(s as u64).wrapping_add(u64::from(flags & FLAG_CF != 0));

            (
                result as u32,
                if (result & 0x80000000 != (d as u64) & 0x80000000) && (result & 0x80000000 != (s as u64) & 0x80000000) { FLAG_OF } else { 0 } |
                if result & 0x100000000 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }

    /// ADC r/m64, imm8
    ///
    /// o64 83 /2 ib
    fn instr_adc_rm64_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Adc_rm64_imm8);

        let flags = self.state.rflags;
        calculate_rm_imm![u64f; u8; self; i; |d:u64, s:u8| {
            let result = (d as u128).wrapping_add(s as u128).wrapping_add(u128::from(flags & FLAG_CF != 0));

            (
                result as u64,
                if (result & 0x8000000000000000 != (d as u128) & 0x8000000000000000) && (result & 0x8000000000000000 != (s as u128) & 0x8000000000000000) { FLAG_OF } else { 0 } |
                if result & 0x10000000000000000u128 != 0 { FLAG_CF } else { 0 }
            )
        }; (set: FLAG_SF | FLAG_ZF | FLAG_PF; clear: FLAG_OF | FLAG_CF)]
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{
        assert_mem_value, assert_reg_value, ax_test, write_flags, write_reg_value,
    };
    use iced_x86::Register::*;

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
            assert_reg_value!(b; a; BL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_9; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x7);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_of; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x7f);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_of_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
            write_reg_value!(b; a; BL; 0x80);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_pf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7).unwrap();
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x6);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_pf_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xf);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_pf_sf_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_pf_zf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_pf_zf_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_pf_zf_of; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_sf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xfe);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_cf_sf_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_pf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xf);
            assert_reg_value!(b; a; BL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_pf_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0x8);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x9);
            assert_reg_value!(b; a; BL; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_pf_sf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_pf_sf_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0x80);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x81);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_pf_sf_of; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
            write_reg_value!(b; a; BL; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x87);
            assert_reg_value!(b; a; BL; 0x7f);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_pf_sf_of_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
            write_reg_value!(b; a; BL; 0x7f);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x81);
            assert_reg_value!(b; a; BL; 0x7f);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_pf_zf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_sf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_sf_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
            write_reg_value!(b; a; BL; 0x80);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x89);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_sf_of; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
            write_reg_value!(b; a; BL; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
            assert_reg_value!(b; a; BL; 0x7f);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc byte ptr [rcx], bl
    ax_test![adc_byte_ptr_rcx_bl_sf_of_cf; 0x10, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_reg_value!(b; a; BL; 0x7f);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
            assert_reg_value!(b; a; BL; 0x7f);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_reg_value!(w; a; BX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
            assert_reg_value!(w; a; BX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_cf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_reg_value!(w; a; BX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
            assert_reg_value!(w; a; BX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_cf_of_cf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
            write_reg_value!(w; a; BX; 0x8000);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_cf_pf_zf_cf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
            write_reg_value!(w; a; BX; 0x8000);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_cf_pf_zf_of; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_pf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_reg_value!(w; a; BX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0xf);
            assert_reg_value!(w; a; BX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_pf_cf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_reg_value!(w; a; BX; 0x8);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x9);
            assert_reg_value!(w; a; BX; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_pf_sf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_pf_sf_cf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
            write_reg_value!(w; a; BX; 0x8000);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8009);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_pf_sf_of; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
            write_reg_value!(w; a; BX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
            assert_reg_value!(w; a; BX; 0x7fff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_pf_sf_of_cf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_reg_value!(w; a; BX; 0x7fff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
            assert_reg_value!(w; a; BX; 0x7fff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_pf_zf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
            assert_reg_value!(w; a; BX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_sf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8001);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_sf_cf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_reg_value!(w; a; BX; 0x8000);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8001);
            assert_reg_value!(w; a; BX; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_sf_of; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
            write_reg_value!(w; a; BX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8007);
            assert_reg_value!(w; a; BX; 0x7fff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc word ptr [rcx], bx
    ax_test![adc_word_ptr_rcx_bx_sf_of_cf; 0x66, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
            write_reg_value!(w; a; BX; 0x7fff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8001);
            assert_reg_value!(w; a; BX; 0x7fff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_reg_value!(d; a; EBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
            assert_reg_value!(d; a; EBX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_cf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_reg_value!(d; a; EBX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_cf_of_cf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap();
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_cf_pf_zf_cf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap();
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_cf_pf_zf_of; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap();
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_pf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_reg_value!(d; a; EBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0xf);
            assert_reg_value!(d; a; EBX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_pf_cf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_reg_value!(d; a; EBX; 0x8);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x9);
            assert_reg_value!(d; a; EBX; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_pf_sf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_pf_sf_cf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000009u32);
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_pf_sf_of; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
            assert_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_pf_sf_of_cf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u32);
            assert_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_pf_zf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
            assert_reg_value!(d; a; EBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_sf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000001u32);
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_sf_cf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000001u32);
            assert_reg_value!(d; a; EBX; 0x80000000u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_sf_of; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000007u32);
            assert_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc dword ptr [rcx], ebx
    ax_test![adc_dword_ptr_rcx_ebx_sf_of_cf; 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000001u32);
            assert_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_reg_value!(q; a; RBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
            assert_reg_value!(q; a; RBX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_cf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_reg_value!(q; a; RBX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_pf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_reg_value!(q; a; RBX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0xf);
            assert_reg_value!(q; a; RBX; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_pf_cf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_reg_value!(q; a; RBX; 0x8);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x9);
            assert_reg_value!(q; a; RBX; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_pf_sf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8000000000000000u64);
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_pf_sf_cf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8000000000000009u64);
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_pf_sf_of; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8000000000000000u64);
            assert_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_pf_sf_of_cf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8000000000000000u64);
            assert_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_pf_zf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
            assert_reg_value!(q; a; RBX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_sf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8000000000000001u64);
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_sf_cf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8000000000000001u64);
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_sf_of; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8000000000000007u64);
            assert_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc qword ptr [rcx], rbx
    ax_test![adc_qword_ptr_rcx_rbx_sf_of_cf; 0x48, 0x11, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8000000000000001u64);
            assert_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_17; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_of; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7f);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_of_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_pf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x6);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_pf_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_pf_sf_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xff);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xff);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_pf_zf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_pf_zf_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_pf_zf_of; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_sf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xff);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xfe);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_cf_sf_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_pf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_pf_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x8).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x9);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_pf_sf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xff);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_pf_sf_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x81);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_pf_sf_of; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x87);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x7f);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_pf_sf_of_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7f).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x81);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x7f);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_pf_zf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_sf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_sf_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x80).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x89);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_sf_of; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x7f);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc bl, byte ptr [rcx]
    ax_test![adc_bl_byte_ptr_rcx_sf_of_cf; 0x12, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 1).unwrap();
            a.mem_write_8(0x1000, 0x7f).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(b; a; 0x1000; 0x7f);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_cf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_cf_of_cf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_cf_pf_zf_cf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7fff);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_cf_pf_zf_of; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_pf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xf);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_pf_cf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x9);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_pf_sf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_pf_sf_cf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8009);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_pf_sf_of; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x7fff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_pf_sf_of_cf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8000);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x7fff);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_pf_zf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_sf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8001);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_sf_cf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x8000).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8001);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x8000);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_sf_of; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8007);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x7fff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc bx, word ptr [rcx]
    ax_test![adc_bx_word_ptr_rcx_sf_of_cf; 0x66, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x7fff).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x8001);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(w; a; 0x1000; 0x7fff);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_cf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_cf_of_cf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u64);
        };
        (FLAG_CF | FLAG_OF; FLAG_PF | FLAG_ZF | FLAG_SF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_cf_pf_zf_cf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7fffffffu64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_cf_pf_zf_of; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF; FLAG_SF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_pf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0xf);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_pf_cf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x9);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_pf_sf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_pf_sf_cf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x8);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000009u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_pf_sf_of; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x7fffffffu64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_pf_sf_of_cf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000000u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x7fffffffu64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_pf_zf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_sf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000001u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_sf_cf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000001u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x80000000u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_sf_of; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x8);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000007u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x7fffffffu64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc ebx, dword ptr [rcx]
    ax_test![adc_ebx_dword_ptr_rcx_sf_of_cf; 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x80000001u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(d; a; 0x1000; 0x7fffffffu64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_cf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_pf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0xf).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xf);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_pf_cf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x9);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_pf_sf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_pf_sf_cf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000009u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_pf_sf_of; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_pf_sf_of_cf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000000u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_pf_zf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x0);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_sf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000001u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_sf_cf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x0).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000001u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x0);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_sf_of; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000007u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x8);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc rbx, qword ptr [rcx]
    ax_test![adc_rbx_qword_ptr_rcx_sf_of_cf; 0x48, 0x13, 0x19;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RCX; 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1).unwrap();
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000000000001u64);
            assert_reg_value!(q; a; RCX; 0x1000);
            assert_mem_value!(q; a; 0x1000; 0x1);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_cf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_cf_pf_zf_cf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_pf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_pf_cf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_pf_sf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_pf_sf_cf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x81);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_pf_zf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_sf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_sf_cf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x82);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x83);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc al, 0x0
    ax_test![adc_al_0x0_sf_of_cf; 0x14, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7f);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];
    // adc al, 0x5
    ax_test![adc_al_0x5; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xd);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_cf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_cf_39; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x4);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_cf_cf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xfb);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_cf_pf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xfe);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x3);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_cf_pf_cf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x5);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_cf_pf_zf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xfb);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_cf_pf_zf_cf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xfa);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_pf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x5);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_pf_cf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_pf_sf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x82);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x87);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_pf_sf_cf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x81);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x87);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_pf_sf_of; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x84);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_pf_sf_of_cf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7b);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x81);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_sf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x85);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_sf_cf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x86);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_sf_of; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7b);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc al, 0x5
    ax_test![adc_al_0x5_sf_of_cf; 0x14, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x7f);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x85);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc ax, 0x1
    ax_test![adc_ax_0x1; 0x66, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x1
    ax_test![adc_ax_0x1_cf; 0x66, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x1
    ax_test![adc_ax_0x1_pf; 0x66, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x1
    ax_test![adc_ax_0x1_pf_cf; 0x66, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x1
    ax_test![adc_ax_0x1_pf_sf_of; 0x66, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc ax, 0x1
    ax_test![adc_ax_0x1_sf; 0x66, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8001);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc ax, 0x1
    ax_test![adc_ax_0x1_sf_cf; 0x66, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8002);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc ax, 0x1
    ax_test![adc_ax_0x1_sf_of_cf; 0x66, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8001);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc ax, 0x5
    ax_test![adc_ax_0x5; 0x66, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xd);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x5
    ax_test![adc_ax_0x5_cf; 0x66, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x5
    ax_test![adc_ax_0x5_pf; 0x66, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x5);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x5
    ax_test![adc_ax_0x5_pf_cf; 0x66, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x5
    ax_test![adc_ax_0x5_pf_sf; 0x66, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8005);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc ax, 0x5
    ax_test![adc_ax_0x5_pf_sf_cf; 0x66, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8006);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc ax, 0x5
    ax_test![adc_ax_0x5_pf_sf_of_cf; 0x66, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8005);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc ax, 0x5
    ax_test![adc_ax_0x5_sf_of; 0x66, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8004);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc ax, 0x1398
    ax_test![adc_ax_0x1398; 0x66, 0x15, 0x98, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1398);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x1398
    ax_test![adc_ax_0x1398_cf; 0x66, 0x15, 0x98, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x13a1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x1398
    ax_test![adc_ax_0x1398_pf; 0x66, 0x15, 0x98, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1399);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x1398
    ax_test![adc_ax_0x1398_pf_cf; 0x66, 0x15, 0x98, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1399);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ax, 0x1398
    ax_test![adc_ax_0x1398_pf_sf_cf; 0x66, 0x15, 0x98, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x9399);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc ax, 0x1398
    ax_test![adc_ax_0x1398_sf; 0x66, 0x15, 0x98, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x9398);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc ax, 0x1398
    ax_test![adc_ax_0x1398_sf_of; 0x66, 0x15, 0x98, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x9397);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc ax, 0x1398
    ax_test![adc_ax_0x1398_sf_of_cf; 0x66, 0x15, 0x98, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x9398);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc eax, 0x5
    ax_test![adc_eax_0x5; 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xd);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x5
    ax_test![adc_eax_0x5_cf; 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x5
    ax_test![adc_eax_0x5_pf; 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x5);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x5
    ax_test![adc_eax_0x5_pf_cf; 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x5
    ax_test![adc_eax_0x5_pf_sf; 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000005u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc eax, 0x5
    ax_test![adc_eax_0x5_pf_sf_cf; 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000006u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc eax, 0x5
    ax_test![adc_eax_0x5_pf_sf_of_cf; 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000005u32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc eax, 0x5
    ax_test![adc_eax_0x5_sf_of; 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000004u32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc eax, 0x1
    ax_test![adc_eax_0x1; 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x1
    ax_test![adc_eax_0x1_cf; 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x1
    ax_test![adc_eax_0x1_pf; 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x1
    ax_test![adc_eax_0x1_pf_cf; 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x1
    ax_test![adc_eax_0x1_pf_sf_of; 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc eax, 0x1
    ax_test![adc_eax_0x1_sf; 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000001u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc eax, 0x1
    ax_test![adc_eax_0x1_sf_cf; 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000002u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc eax, 0x1
    ax_test![adc_eax_0x1_sf_of_cf; 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000001u32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];
    // adc eax, 0x1385713
    ax_test![adc_eax_0x1385713; 0x15, 0x13, 0x57, 0x38, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1385713);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x1385713
    ax_test![adc_eax_0x1385713_cf; 0x15, 0x13, 0x57, 0x38, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1385715);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x1385713
    ax_test![adc_eax_0x1385713_pf; 0x15, 0x13, 0x57, 0x38, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1385714);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x1385713
    ax_test![adc_eax_0x1385713_pf_cf; 0x15, 0x13, 0x57, 0x38, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1385714);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc eax, 0x1385713
    ax_test![adc_eax_0x1385713_pf_sf_cf; 0x15, 0x13, 0x57, 0x38, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x81385714u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc eax, 0x1385713
    ax_test![adc_eax_0x1385713_pf_sf_of; 0x15, 0x13, 0x57, 0x38, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x81385712u32);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc eax, 0x1385713
    ax_test![adc_eax_0x1385713_sf; 0x15, 0x13, 0x57, 0x38, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x81385713u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc eax, 0x1385713
    ax_test![adc_eax_0x1385713_sf_of_cf; 0x15, 0x13, 0x57, 0x38, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x81385713u32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1_cf; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1_pf; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1_pf_cf; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1_pf_sf; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xba4a39947d43af95u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xba4a39947d43af96u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1_pf_sf_cf; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x920346527cb936e6u64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x920346527cb936e8u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1_pf_sf_of; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1_sf; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000001u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1_sf_cf; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000002u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x1
    ax_test![adc_rax_0x1_sf_of_cf; 0x48, 0x83, 0xd0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000001u64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xd);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5_cf; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5_pf; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x5);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5_pf_cf; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5_pf_sf; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000005u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5_pf_sf_cf; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000006u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5_pf_sf_of_cf; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000005u64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5_sf; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xe702189c83acc7deu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xe702189c83acc7e3u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5_sf_cf; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xa0c0c73a3a9dab29u64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xa0c0c73a3a9dab2fu64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x5
    ax_test![adc_rax_0x5_sf_of; 0x48, 0x83, 0xd0, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000004u64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x71357915);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915_cf; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x71357916);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915_pf; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7135791d);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915_pf_cf; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x71357917);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915_pf_sf; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xc1c6c999ba0782c9u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xc1c6c99a2b3cfbdeu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915_pf_sf_cf; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x9cb47082df9282ddu64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x9cb4708350c7fbf3u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915_pf_sf_of; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000071357914u64);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915_sf; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000071357915u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915_sf_cf; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000071357916u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rax, 0x71357915
    ax_test![adc_rax_0x71357915_sf_of_cf; 0x48, 0x15, 0x15, 0x79, 0x35, 0x71;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000071357915u64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_cf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_cf_pf_zf_cf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_CF | FLAG_PF | FLAG_ZF; FLAG_SF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_pf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_pf_cf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x8);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x9);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_pf_sf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xff);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_pf_sf_cf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x81);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_pf_zf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_sf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_sf_cf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x82);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x83);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc bl, 0x0
    ax_test![adc_bl_0x0_sf_of_cf; 0x80, 0xd3, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7f);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc bx, 0x35be
    ax_test![adc_bx_0x35be; 0x66, 0x81, 0xd3, 0xbe, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x35bf);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bx, 0x35be
    ax_test![adc_bx_0x35be_cf; 0x66, 0x81, 0xd3, 0xbe, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x35bf);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bx, 0x35be
    ax_test![adc_bx_0x35be_pf; 0x66, 0x81, 0xd3, 0xbe, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x35be);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bx, 0x35be
    ax_test![adc_bx_0x35be_pf_cf; 0x66, 0x81, 0xd3, 0xbe, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x35c0);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc bx, 0x35be
    ax_test![adc_bx_0x35be_pf_sf; 0x66, 0x81, 0xd3, 0xbe, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xb5be);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc bx, 0x35be
    ax_test![adc_bx_0x35be_pf_sf_of; 0x66, 0x81, 0xd3, 0xbe, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xb5bd);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc bx, 0x35be
    ax_test![adc_bx_0x35be_pf_sf_of_cf; 0x66, 0x81, 0xd3, 0xbe, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x7fff);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xb5be);
        };
        (FLAG_PF | FLAG_SF | FLAG_OF; FLAG_CF | FLAG_ZF)
    ];

    // adc bx, 0x35be
    ax_test![adc_bx_0x35be_sf_cf; 0x66, 0x81, 0xd3, 0xbe, 0x35;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x8000);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0xb5bf);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc ebx, 0x13513758
    ax_test![adc_ebx_0x13513758; 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x13513758);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ebx, 0x13513758
    ax_test![adc_ebx_0x13513758_cf; 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x8);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x13513761);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ebx, 0x13513758
    ax_test![adc_ebx_0x13513758_pf; 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x13513759);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ebx, 0x13513758
    ax_test![adc_ebx_0x13513758_pf_cf; 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x13513759);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc ebx, 0x13513758
    ax_test![adc_ebx_0x13513758_pf_sf_cf; 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x93513759u32);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc ebx, 0x13513758
    ax_test![adc_ebx_0x13513758_sf; 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x93513758u32);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc ebx, 0x13513758
    ax_test![adc_ebx_0x13513758_sf_of; 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x93513757u32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc ebx, 0x13513758
    ax_test![adc_ebx_0x13513758_sf_of_cf; 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x7fffffffu32);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x93513758u32);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x13513758);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758_cf; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x13513761);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758_pf; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x13513759);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758_pf_cf; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x0);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x13513759);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758_pf_sf; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xa86e0e5f9f96e1e7u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xa86e0e5fb2e8193fu64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758_pf_sf_cf; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000013513759u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758_sf; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000013513758u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758_sf_cf; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0xcecab0472743deafu64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0xcecab0473a951608u64);
        };
        (FLAG_SF; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758_sf_of; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000013513757u64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];

    // adc rbx, 0x13513758
    ax_test![adc_rbx_0x13513758_sf_of_cf; 0x48, 0x81, 0xd3, 0x58, 0x37, 0x51, 0x13;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x7fffffffffffffffu64);
            write_flags!(a; FLAG_CF);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x8000000013513758u64);
        };
        (FLAG_SF | FLAG_OF; FLAG_CF | FLAG_PF | FLAG_ZF)
    ];
}
