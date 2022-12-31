use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Div;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::fatal_error;
use crate::instructions::operand::Operand;
use crate::instructions::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_div(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Div);

        match i.code() {
            Div_rm8 => self.instr_div_rm8(i),
            Div_rm16 => self.instr_div_rm16(i),
            Div_rm32 => self.instr_div_rm32(i),
            Div_rm64 => self.instr_div_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Div", i.code()),
        }
    }

    /// DIV r/m8
    ///
    /// F6 /6
    fn instr_div_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Div_rm8);

        let ax = self.reg_read_16(AX) as u16;

        let op = self.instruction_operand(i, 0)?;
        let src_val = match op {
            Operand::Register(r) => self.reg_read_8(r),
            Operand::Memory(m) => self.mem_read_8(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Div_rm8", op),
        } as u16;

        if src_val == 0 {
            return Err(AxError::from(format!(
                "Divide by zero in Div_rm8: operand {:?} is 0",
                op
            )));
        }

        let (quotient, remainder) = (ax / src_val, ax % src_val);

        self.reg_write_8(AL, quotient as u8 as u64);
        self.reg_write_8(AH, remainder as u8 as u64);

        Ok(())
    }

    /// DIV r/m16
    ///
    /// o16 F7 /6
    fn instr_div_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Div_rm16);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_16(r),
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Div_rm16", op),
        } as u32;

        if src_val == 0 {
            return Err(AxError::from(format!(
                "Divide by zero in Div_rm16: operand {:?} is 0",
                op
            )));
        }

        let dst_val = self.reg_read_16(AX) as u32 | ((self.reg_read_16(DX) as u32) << 16);

        let (quotient, remainder) = (dst_val / src_val, dst_val % src_val);

        self.reg_write_16(AX, quotient as u16 as u64);
        self.reg_write_16(DX, remainder as u16 as u64);

        Ok(())
    }

    /// DIV r/m32
    ///
    /// o32 F7 /6
    fn instr_div_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Div_rm32);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_32(r),
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Div_rm32", op),
        } as u64;

        if src_val == 0 {
            return Err(AxError::from(format!(
                "Divide by zero in Div_rm32: operand {:?} is 0",
                op
            )));
        }

        let dst_val = self.reg_read_32(EAX) as u64 | ((self.reg_read_32(EDX) as u64) << 32);

        let (quotient, remainder) = (dst_val / src_val, dst_val % src_val);

        self.reg_write_32(EAX, quotient as u32 as u64);
        self.reg_write_32(EDX, remainder as u32 as u64);

        Ok(())
    }

    /// DIV r/m64
    ///
    /// o64 F7 /6
    fn instr_div_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Div_rm64);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_64(r),
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Div_rm64", op),
        } as u128;

        if src_val == 0 {
            return Err(AxError::from(format!(
                "Divide by zero in Div_rm64: operand {:?} is 0",
                op
            )));
        }

        let dst_val = (self.reg_read_64(RAX) as u128) | ((self.reg_read_64(RDX) as u128) << 64);

        let (quotient, remainder) = (dst_val / src_val, dst_val % src_val);

        self.reg_write_64(RAX, quotient as u64);
        self.reg_write_64(RDX, remainder as u64);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::SupportedRegister, write_reg_value,
    };
    use iced_x86::Register::*;

    async fn assert_divide_by_zero_error(code: &[u8]) {
        let mut ax = Axecutor::new(code, 0x1000, 0x1000).expect("Failed to create Axecutor");
        ax.reg_write_64(SupportedRegister::RAX, 0);
        let _ = ax.execute().await;
    }

    #[test]
    #[should_panic]
    fn divide_by_zero_error_r8() {
        async_std::task::block_on(async {
            // div al
            assert_divide_by_zero_error(&[0xf6, 0xf0]).await;
        });
    }

    #[test]
    #[should_panic]
    fn divide_by_zero_error_r16() {
        async_std::task::block_on(async {
            // div ax
            assert_divide_by_zero_error(&[0x66, 0xf7, 0xf0]).await;
        });
    }

    #[test]
    #[should_panic]
    fn divide_by_zero_error_r32() {
        async_std::task::block_on(async {
            // div eax
            assert_divide_by_zero_error(&[0xf7, 0xf0]).await;
        });
    }

    #[test]
    #[should_panic]
    fn divide_by_zero_error_r64() {
        async_std::task::block_on(async {
            // div rax
            assert_divide_by_zero_error(&[0x48, 0xf7, 0xf0]).await;
        });
    }

    // div bl
    ax_test![div_bl_ax_32; 0xf6, 0xf3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x40);
            write_reg_value!(w; a; AX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x40);
            assert_reg_value!(w; a; AX; 0x2000);
        }
    ];

    // div bl
    ax_test![div_bl_ax_8192; 0xf6, 0xf3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x3f);
            write_reg_value!(w; a; AX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x3f);
            assert_reg_value!(w; a; AX; 0x282);
        }
    ];

    // div bl
    ax_test![div_bl_ax_256; 0xf6, 0xf3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
            write_reg_value!(w; a; AX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf);
            assert_reg_value!(w; a; AX; 0x111);
        }
    ];

    // div bl
    ax_test![div_bl_ax_16384; 0xf6, 0xf3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(w; a; AX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(w; a; AX; 0x80);
        }
    ];

    // div bl
    ax_test![div_bl_ax_7; 0xf6, 0xf3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x10);
            write_reg_value!(w; a; AX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x10);
            assert_reg_value!(w; a; AX; 0x700);
        }
    ];

    // div bl
    ax_test![div_bl_ax_0; 0xf6, 0xf3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x10);
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x10);
            assert_reg_value!(w; a; AX; 0x0);
        }
    ];

    // div bl
    ax_test![div_bl_ax_127; 0xf6, 0xf3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x10);
            write_reg_value!(w; a; AX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x10);
            assert_reg_value!(w; a; AX; 0xf07);
        }
    ];

    // div bl
    ax_test![div_bl_ax_4; 0xf6, 0xf3; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x11);
            write_reg_value!(w; a; AX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x11);
            assert_reg_value!(w; a; AX; 0x400);
        }
    ];

    // div ax
    ax_test![div_ax_dx_0; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(w; a; DX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
            assert_reg_value!(w; a; DX; 0x0);
        }
    ];

    // div ax
    ax_test![div_ax_dx_1; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
            write_reg_value!(w; a; DX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x2493);
            assert_reg_value!(w; a; DX; 0x2);
        }
    ];

    // div ax
    ax_test![div_ax_dx_1024; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x801);
            assert_reg_value!(w; a; DX; 0x800);
        }
    ];

    // div ax
    ax_test![div_ax_dx_127; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x80);
            write_reg_value!(w; a; DX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfe01);
            assert_reg_value!(w; a; DX; 0x0);
        }
    ];

    // div ax
    ax_test![div_ax_dx_128; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8081);
            assert_reg_value!(w; a; DX; 0x80);
        }
    ];

    // div ax
    ax_test![div_ax_dx_15; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x10);
            write_reg_value!(w; a; DX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf001);
            assert_reg_value!(w; a; DX; 0x0);
        }
    ];

    // div ax
    ax_test![div_ax_dx_16; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x11);
            write_reg_value!(w; a; DX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf0f1);
            assert_reg_value!(w; a; DX; 0x10);
        }
    ];

    // div ax
    ax_test![div_ax_dx_16384; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8002);
            assert_reg_value!(w; a; DX; 0x1);
        }
    ];

    // div ax
    ax_test![div_ax_dx_17; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1f);
            write_reg_value!(w; a; DX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8c64);
            assert_reg_value!(w; a; DX; 0x3);
        }
    ];

    // div ax
    ax_test![div_ax_dx_2; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
            write_reg_value!(w; a; DX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4925);
            assert_reg_value!(w; a; DX; 0x4);
        }
    ];

    // div ax
    ax_test![div_ax_dx_2048; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1001);
            assert_reg_value!(w; a; DX; 0x1000);
        }
    ];

    // div ax
    ax_test![div_ax_dx_255; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x100);
            write_reg_value!(w; a; DX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xff01);
            assert_reg_value!(w; a; DX; 0x0);
        }
    ];

    // div ax
    ax_test![div_ax_dx_256; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x201);
            assert_reg_value!(w; a; DX; 0x200);
        }
    ];

    // div ax
    ax_test![div_ax_dx_31; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x20);
            write_reg_value!(w; a; DX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf801);
            assert_reg_value!(w; a; DX; 0x0);
        }
    ];

    // div ax
    ax_test![div_ax_dx_32; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x21);
            write_reg_value!(w; a; DX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xf83f);
            assert_reg_value!(w; a; DX; 0x2);
        }
    ];

    // div ax
    ax_test![div_ax_dx_32767; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
            write_reg_value!(w; a; DX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xffff);
            assert_reg_value!(w; a; DX; 0x0);
        }
    ];

    // div ax
    ax_test![div_ax_dx_33; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x3f);
            write_reg_value!(w; a; DX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8619);
            assert_reg_value!(w; a; DX; 0x18);
        }
    ];

    // div ax
    ax_test![div_ax_dx_4; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
            write_reg_value!(w; a; DX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x924a);
            assert_reg_value!(w; a; DX; 0x1);
        }
    ];

    // div ax
    ax_test![div_ax_dx_4096; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x2001);
            assert_reg_value!(w; a; DX; 0x2000);
        }
    ];

    // div ax
    ax_test![div_ax_dx_512; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x401);
            assert_reg_value!(w; a; DX; 0x400);
        }
    ];

    // div ax
    ax_test![div_ax_dx_63; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x40);
            write_reg_value!(w; a; DX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfc01);
            assert_reg_value!(w; a; DX; 0x0);
        }
    ];

    // div ax
    ax_test![div_ax_dx_64; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x41);
            write_reg_value!(w; a; DX; 0x40);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfc10);
            assert_reg_value!(w; a; DX; 0x31);
        }
    ];

    // div ax
    ax_test![div_ax_dx_65; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7f);
            write_reg_value!(w; a; DX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8307);
            assert_reg_value!(w; a; DX; 0x6);
        }
    ];

    // div ax
    ax_test![div_ax_dx_7; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
            write_reg_value!(w; a; DX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xe001);
            assert_reg_value!(w; a; DX; 0x0);
        }
    ];

    // div ax
    ax_test![div_ax_dx_8; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xf);
            write_reg_value!(w; a; DX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8889);
            assert_reg_value!(w; a; DX; 0x8);
        }
    ];

    // div ax
    ax_test![div_ax_dx_8192; 0x66, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4001);
            assert_reg_value!(w; a; DX; 0x4000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_0; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(d; a; EDX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_1; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
            write_reg_value!(d; a; EDX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x24924925);
            assert_reg_value!(d; a; EDX; 0x4);
        }
    ];

    // div eax
    ax_test![div_eax_edx_1024; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8001001);
            assert_reg_value!(d; a; EDX; 0x1000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_1048576; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x100000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x200001);
            assert_reg_value!(d; a; EDX; 0x200000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_1073741824; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x40000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000002u32);
            assert_reg_value!(d; a; EDX; 0x1);
        }
    ];

    // div eax
    ax_test![div_eax_edx_127; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80);
            write_reg_value!(d; a; EDX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xfe000001u32);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_128; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xff);
            write_reg_value!(d; a; EDX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80808081u32);
            assert_reg_value!(d; a; EDX; 0x80);
        }
    ];

    // div eax
    ax_test![div_eax_edx_131072; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x20000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x40001);
            assert_reg_value!(d; a; EDX; 0x40000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_134217728; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x8000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x10000001);
            assert_reg_value!(d; a; EDX; 0x10000000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_15; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10);
            write_reg_value!(d; a; EDX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf0000001u32);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_16; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x11);
            write_reg_value!(d; a; EDX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf0f0f0f1u32);
            assert_reg_value!(d; a; EDX; 0x10);
        }
    ];

    // div eax
    ax_test![div_eax_edx_16384; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80010003u32);
            assert_reg_value!(d; a; EDX; 0x2);
        }
    ];

    // div eax
    ax_test![div_eax_edx_16777216; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x1000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x2000001);
            assert_reg_value!(d; a; EDX; 0x2000000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_17; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1f);
            write_reg_value!(d; a; EDX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8c6318c7u32);
            assert_reg_value!(d; a; EDX; 0x6);
        }
    ];

    // div eax
    ax_test![div_eax_edx_2; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
            write_reg_value!(d; a; EDX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x4924924a);
            assert_reg_value!(d; a; EDX; 0x1);
        }
    ];

    // div eax
    ax_test![div_eax_edx_2048; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x10002001);
            assert_reg_value!(d; a; EDX; 0x2000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_2097152; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x200000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x400001);
            assert_reg_value!(d; a; EDX; 0x400000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_2147483647; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(d; a; EDX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffffffu32);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_255; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x100);
            write_reg_value!(d; a; EDX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xff000001u32);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_256; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x2000401);
            assert_reg_value!(d; a; EDX; 0x400);
        }
    ];

    // div eax
    ax_test![div_eax_edx_262144; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x40000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80001);
            assert_reg_value!(d; a; EDX; 0x80000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_268435456; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x10000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x20000001);
            assert_reg_value!(d; a; EDX; 0x20000000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_31; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x20);
            write_reg_value!(d; a; EDX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf8000001u32);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_32; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x21);
            write_reg_value!(d; a; EDX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf83e0f84u32);
            assert_reg_value!(d; a; EDX; 0x1d);
        }
    ];

    // div eax
    ax_test![div_eax_edx_32767; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8000);
            write_reg_value!(d; a; EDX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xfffe0001u32);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_32768; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000001u32);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_33; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x3f);
            write_reg_value!(d; a; EDX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x86186187u32);
            assert_reg_value!(d; a; EDX; 0x6);
        }
    ];

    // div eax
    ax_test![div_eax_edx_33554432; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x2000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x4000001);
            assert_reg_value!(d; a; EDX; 0x4000000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_4; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
            write_reg_value!(d; a; EDX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x92492493u32);
            assert_reg_value!(d; a; EDX; 0x2);
        }
    ];

    // div eax
    ax_test![div_eax_edx_4096; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x20004001);
            assert_reg_value!(d; a; EDX; 0x4000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_4194304; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x400000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x800001);
            assert_reg_value!(d; a; EDX; 0x800000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_512; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x4000801);
            assert_reg_value!(d; a; EDX; 0x800);
        }
    ];

    // div eax
    ax_test![div_eax_edx_524288; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x80000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x100001);
            assert_reg_value!(d; a; EDX; 0x100000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_536870912; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x20000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x40000001);
            assert_reg_value!(d; a; EDX; 0x40000000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_63; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x40);
            write_reg_value!(d; a; EDX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xfc000001u32);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_64; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x41);
            write_reg_value!(d; a; EDX; 0x40);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xfc0fc0fdu32);
            assert_reg_value!(d; a; EDX; 0x4);
        }
    ];

    // div eax
    ax_test![div_eax_edx_65; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7f);
            write_reg_value!(d; a; EDX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x83060c19u32);
            assert_reg_value!(d; a; EDX; 0x18);
        }
    ];

    // div eax
    ax_test![div_eax_edx_65536; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x10000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x20001);
            assert_reg_value!(d; a; EDX; 0x20000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_67108864; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x4000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8000001);
            assert_reg_value!(d; a; EDX; 0x8000000);
        }
    ];

    // div eax
    ax_test![div_eax_edx_7; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
            write_reg_value!(d; a; EDX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xe0000001u32);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // div eax
    ax_test![div_eax_edx_8; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xf);
            write_reg_value!(d; a; EDX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x88888889u32);
            assert_reg_value!(d; a; EDX; 0x8);
        }
    ];

    // div eax
    ax_test![div_eax_edx_8192; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x40008002);
            assert_reg_value!(d; a; EDX; 0x1);
        }
    ];

    // div eax
    ax_test![div_eax_edx_8388608; 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x800000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1000001);
            assert_reg_value!(d; a; EDX; 0x1000000);
        }
    ];

    // div rax
    ax_test![div_rax_rdx_2048; 0x48, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fff);
            write_reg_value!(q; a; RDX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000200040008002u64);
            assert_reg_value!(q; a; RDX; 0x1);
        }
    ];

    // div rax
    ax_test![div_rax_rdx_5440170163614280269; 0x48, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xe39a81317bcd3aa7u64);
            write_reg_value!(q; a; RDX; 0x4b7f5df0c7e86a4du64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x54eab45306e2c3a6u64);
            assert_reg_value!(q; a; RDX; 0x8cfe7f1b2f9dfd5du64);
        }
    ];

    // div rax
    ax_test![div_rax_rdx_72057594037927936; 0x48, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x100000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x200000000000001u64);
            assert_reg_value!(q; a; RDX; 0x200000000000000u64);
        }
    ];

    // div rax
    ax_test![div_rax_rdx_1152921504606846976; 0x48, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x1000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2000000000000001u64);
            assert_reg_value!(q; a; RDX; 0x2000000000000000u64);
        }
    ];

    // div rax
    ax_test![div_rax_rdx_137438953472; 0x48, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x2000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4000000001u64);
            assert_reg_value!(q; a; RDX; 0x4000000000u64);
        }
    ];

    // div rax
    ax_test![div_rax_rdx_16384; 0x48, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fff);
            write_reg_value!(q; a; RDX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8001000200040009u64);
            assert_reg_value!(q; a; RDX; 0x8);
        }
    ];

    // div rax
    ax_test![div_rax_rdx_8543780753131969330; 0x48, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xf16e9c5071a03476u64);
            write_reg_value!(q; a; RDX; 0x76919b53fd401b32u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7db922011f384866u64);
            assert_reg_value!(q; a; RDX; 0x742c9272593a1d72u64);
        }
    ];

    // div rax
    ax_test![div_rax_rdx_562949953421312; 0x48, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x2000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4000000000001u64);
            assert_reg_value!(q; a; RDX; 0x4000000000000u64);
        }
    ];

    // div rax
    ax_test![div_rax_rdx_8388608; 0x48, 0xf7, 0xf0; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
            write_reg_value!(q; a; RDX; 0x800000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x100000002000001u64);
            assert_reg_value!(q; a; RDX; 0x2000000);
        }
    ];
}
