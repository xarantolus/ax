use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Idiv;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::fatal_error;
use crate::instructions::operand::Operand;
use crate::instructions::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_idiv(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Idiv);

        match i.code() {
            Idiv_rm8 => self.instr_idiv_rm8(i),
            Idiv_rm16 => self.instr_idiv_rm16(i),
            Idiv_rm32 => self.instr_idiv_rm32(i),
            Idiv_rm64 => self.instr_idiv_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Idiv", i.code()),
        }
    }

    /// IDIV r/m8
    ///
    /// F6 /7
    fn instr_idiv_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Idiv_rm8);

        let ax = self.reg_read_16(AX)? as i16;

        let op = self.instruction_operand(i, 0)?;
        let src_val = match op {
            Operand::Register(r) => self.reg_read_8(r)?,
            Operand::Memory(m) => self.mem_read_8(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Idiv_rm8", op),
        } as i16;

        if src_val == 0 {
            return Err(AxError::from(format!(
                "Divide by zero in Idiv_rm8: operand {:?} is 0",
                op
            )));
        }

        let (quotient, remainder) = (ax / src_val, ax % src_val);

        self.reg_write_8(AL, quotient as u8 as u64)?;
        self.reg_write_8(AH, remainder as u8 as u64)?;

        Ok(())
    }

    /// IDIV r/m16
    ///
    /// o16 F7 /7
    fn instr_idiv_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Idiv_rm16);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_16(r)?,
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Idiv_rm16", op),
        } as i32;

        if src_val == 0 {
            return Err(AxError::from(format!(
                "Divide by zero in Idiv_rm16: operand {:?} is 0",
                op
            )));
        }

        let dst_val =
            (self.reg_read_16(AX)? as u32 | ((self.reg_read_16(DX)? as u32) << 16)) as i32;

        let (quotient, remainder) = (dst_val / src_val, dst_val % src_val);

        self.reg_write_16(AX, quotient as u16 as u64)?;
        self.reg_write_16(DX, remainder as u16 as u64)?;

        Ok(())
    }

    /// IDIV r/m32
    ///
    /// o32 F7 /7
    fn instr_idiv_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Idiv_rm32);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_32(r)?,
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Idiv_rm32", op),
        } as i64;

        if src_val == 0 {
            return Err(AxError::from(format!(
                "Divide by zero in Idiv_rm32: operand {:?} is 0",
                op
            )));
        }

        let dst_val =
            (self.reg_read_32(EAX)? as u64 | ((self.reg_read_32(EDX)? as u64) << 32)) as i64;

        let (quotient, remainder) = (dst_val / src_val, dst_val % src_val);

        self.reg_write_32(EAX, quotient as u32 as u64)?;
        self.reg_write_32(EDX, remainder as u32 as u64)?;

        Ok(())
    }

    /// IDIV r/m64
    ///
    /// o64 F7 /7
    fn instr_idiv_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Idiv_rm64);

        let op = self.instruction_operand(i, 0)?;

        let src_val = match op {
            Operand::Register(r) => self.reg_read_64(r)?,
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for Idiv_rm64", op),
        } as u128 as i128;

        if src_val == 0 {
            return Err(AxError::from(format!(
                "Divide by zero in Idiv_rm64: operand {:?} is 0",
                op
            )));
        }

        let dst_val =
            (self.reg_read_64(RAX)? as u128 | ((self.reg_read_64(RDX)? as u128) << 64)) as i128;

        let (quotient, remainder) = (dst_val / src_val, dst_val % src_val);

        self.reg_write_64(RAX, quotient as u64)?;
        self.reg_write_64(RDX, remainder as u64)?;

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
        ax.reg_write_64(SupportedRegister::RAX, 0).unwrap();
        let _ = ax.execute().await;
    }

    #[test]
    #[should_panic]
    fn signed_divide_by_zero_error_r8() {
        async_std::task::block_on(async {
            // idiv al
            assert_divide_by_zero_error(&[0xf6, 0xf8]).await;
        });
    }

    #[test]
    #[should_panic]
    fn signed_divide_by_zero_error_r16() {
        async_std::task::block_on(async {
            // idiv ax
            assert_divide_by_zero_error(&[0x66, 0xf7, 0xf8]).await;
        });
    }

    #[test]
    #[should_panic]
    fn divide_by_zero_error_r32() {
        async_std::task::block_on(async {
            // idiv eax
            assert_divide_by_zero_error(&[0xf7, 0xf8]).await;
        });
    }

    #[test]
    #[should_panic]
    fn divide_by_zero_error_r64() {
        async_std::task::block_on(async {
            // idiv rax
            assert_divide_by_zero_error(&[0x48, 0xf7, 0xf8]).await;
        });
    }

    // idiv bl
    ax_test![idiv_bl_ax_0; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x0);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_1; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x1);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_1024; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0xf);
            write_reg_value!(w; a; AX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0xf);
            assert_reg_value!(w; a; AX; 0x444);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_127; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x7f);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_128; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0x212);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_15; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0xf);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_16; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x10);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_16384; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x80);
            write_reg_value!(w; a; AX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x80);
            assert_reg_value!(w; a; AX; 0x80);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_17; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x11);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_2; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x2);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_2048; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x11);
            write_reg_value!(w; a; AX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x11);
            assert_reg_value!(w; a; AX; 0x878);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_255; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0x324);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_256; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0x424);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_31; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x1f);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_32; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x20);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_33; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x21);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_4; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x4);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_4096; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x21);
            write_reg_value!(w; a; AX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x21);
            assert_reg_value!(w; a; AX; 0x47c);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_512; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x7);
            write_reg_value!(w; a; AX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x7);
            assert_reg_value!(w; a; AX; 0x149);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_63; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x3f);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_64; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x40);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x40);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_65; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x41);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_7; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x7);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_8; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x1);
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x1);
            assert_reg_value!(w; a; AX; 0x8);
        }
    ];

    // idiv bl
    ax_test![idiv_bl_ax_8192; 0xf6, 0xfb; |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x41);
            write_reg_value!(w; a; AX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x41);
            assert_reg_value!(w; a; AX; 0x27e);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_0; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(w; a; DX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1);
            assert_reg_value!(w; a; DX; 0x0);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_1; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
            write_reg_value!(w; a; DX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x2493);
            assert_reg_value!(w; a; DX; 0x2);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_1024; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x801);
            assert_reg_value!(w; a; DX; 0x800);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_127; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7f80);
            assert_reg_value!(w; a; DX; 0x7f);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_128; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x101);
            assert_reg_value!(w; a; DX; 0x100);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_15; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1f);
            write_reg_value!(w; a; DX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7bdf);
            assert_reg_value!(w; a; DX; 0x1e);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_16; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x21);
            write_reg_value!(w; a; DX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7c20);
            assert_reg_value!(w; a; DX; 0x1);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_17; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x3f);
            write_reg_value!(w; a; DX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4515);
            assert_reg_value!(w; a; DX; 0x14);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_2; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7);
            write_reg_value!(w; a; DX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4925);
            assert_reg_value!(w; a; DX; 0x4);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_2048; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1001);
            assert_reg_value!(w; a; DX; 0x1000);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_255; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1ff);
            assert_reg_value!(w; a; DX; 0x1fe);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_256; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x100);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x201);
            assert_reg_value!(w; a; DX; 0x200);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_31; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x3f);
            write_reg_value!(w; a; DX; 0x1f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7df8);
            assert_reg_value!(w; a; DX; 0x37);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_32; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x41);
            write_reg_value!(w; a; DX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7e08);
            assert_reg_value!(w; a; DX; 0x39);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_33; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7f);
            write_reg_value!(w; a; DX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4286);
            assert_reg_value!(w; a; DX; 0x5);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_4; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xf);
            write_reg_value!(w; a; DX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4445);
            assert_reg_value!(w; a; DX; 0x4);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_4096; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x2001);
            assert_reg_value!(w; a; DX; 0x2000);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_512; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x401);
            assert_reg_value!(w; a; DX; 0x400);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_63; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7f);
            write_reg_value!(w; a; DX; 0x3f);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7efe);
            assert_reg_value!(w; a; DX; 0x7d);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_64; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x40);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4041);
            assert_reg_value!(w; a; DX; 0x40);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_65; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xff);
            write_reg_value!(w; a; DX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4142);
            assert_reg_value!(w; a; DX; 0x41);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_7; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0xf);
            write_reg_value!(w; a; DX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7778);
            assert_reg_value!(w; a; DX; 0x7);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_8; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x11);
            write_reg_value!(w; a; DX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x7879);
            assert_reg_value!(w; a; DX; 0x8);
        }
    ];

    // idiv ax
    ax_test![idiv_ax_dx_8192; 0x66, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x7fff);
            write_reg_value!(w; a; DX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x4001);
            assert_reg_value!(w; a; DX; 0x4000);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_0; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(d; a; EDX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_1; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
            write_reg_value!(d; a; EDX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x24924925);
            assert_reg_value!(d; a; EDX; 0x4);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_1024; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x400);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x8001001);
            assert_reg_value!(d; a; EDX; 0x1000);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_1048576; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x100000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x200001);
            assert_reg_value!(d; a; EDX; 0x200000);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_127; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xff);
            write_reg_value!(d; a; EDX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7f7f7f80);
            assert_reg_value!(d; a; EDX; 0x7f);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_128; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1000201);
            assert_reg_value!(d; a; EDX; 0x200);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_131072; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x20000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x40001);
            assert_reg_value!(d; a; EDX; 0x40000);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_134217728; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x8000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x10000001);
            assert_reg_value!(d; a; EDX; 0x10000000);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_15; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1f);
            write_reg_value!(d; a; EDX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7bdef7be);
            assert_reg_value!(d; a; EDX; 0x1d);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_16; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x21);
            write_reg_value!(d; a; EDX; 0x10);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7c1f07c2);
            assert_reg_value!(d; a; EDX; 0x1f);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_16384; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000);
            write_reg_value!(d; a; EDX; 0x4000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x40000001);
            assert_reg_value!(d; a; EDX; 0x0);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_16777216; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x1000000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x2000001);
            assert_reg_value!(d; a; EDX; 0x2000000);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_17; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x3f);
            write_reg_value!(d; a; EDX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x45145146);
            assert_reg_value!(d; a; EDX; 0x5);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_2; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7);
            write_reg_value!(d; a; EDX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x4924924a);
            assert_reg_value!(d; a; EDX; 0x1);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_2048; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(d; a; EDX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x10002001);
            assert_reg_value!(d; a; EDX; 0x2000);
        }
    ];

    // idiv eax
    ax_test![idiv_eax_edx_2097152; 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(d; a; EDX; 0x200000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x400001);
            assert_reg_value!(d; a; EDX; 0x400000);
        }
    ];
    // 11acf0481b690c6aca93a1eeae012a0c
    // idiv rax
    ax_test![idiv_rax_rdx_1273656987127188586; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xca93a1eeae012a0cu64);
            write_reg_value!(q; a; RDX; 0x11acf0481b690c6au64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x16564028cda3664au64);
            assert_reg_value!(q; a; RDX; 0x8a586e5289463a94u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_18014398509481984; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x40000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x80000000000001u64);
            assert_reg_value!(q; a; RDX; 0x80000000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_262144; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
            write_reg_value!(q; a; RDX; 0x40000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000100001u64);
            assert_reg_value!(q; a; RDX; 0x100000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_65536; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
            write_reg_value!(q; a; RDX; 0x10000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2000000040001u64);
            assert_reg_value!(q; a; RDX; 0x40000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_140737488355328; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x800000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000000000001u64);
            assert_reg_value!(q; a; RDX; 0x1000000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_255; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fff);
            write_reg_value!(q; a; RDX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1fe03fc07f80ff1u64);
            assert_reg_value!(q; a; RDX; 0xff0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_536870912; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
            write_reg_value!(q; a; RDX; 0x20000000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4000000080000002u64);
            assert_reg_value!(q; a; RDX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_1048576; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
            write_reg_value!(q; a; RDX; 0x100000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x20000000400001u64);
            assert_reg_value!(q; a; RDX; 0x400000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_134217728; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
            write_reg_value!(q; a; RDX; 0x8000000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000000020000001u64);
            assert_reg_value!(q; a; RDX; 0x20000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_65; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xff);
            write_reg_value!(q; a; RDX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4141414141414142u64);
            assert_reg_value!(q; a; RDX; 0x41);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_144115188075855872; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x200000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x400000000000001u64);
            assert_reg_value!(q; a; RDX; 0x400000000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_18096121601468305152; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x352592d19fcabb44u64);
            write_reg_value!(q; a; RDX; 0xfb2256b81bca8f00u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xe88fd65385bb9f32u64);
            assert_reg_value!(q; a; RDX; 0xd43918346136ebfcu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_2048; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fff);
            write_reg_value!(q; a; RDX; 0x800);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000200040008002u64);
            assert_reg_value!(q; a; RDX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_268435456; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
            write_reg_value!(q; a; RDX; 0x10000000);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2000000040000001u64);
            assert_reg_value!(q; a; RDX; 0x40000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_17179869184; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x400000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x800000001u64);
            assert_reg_value!(q; a; RDX; 0x800000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_33; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7f);
            write_reg_value!(q; a; RDX; 0x21);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x42850a142850a143u64);
            assert_reg_value!(q; a; RDX; 0x42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_288230376151711744; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x400000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x800000000000001u64);
            assert_reg_value!(q; a; RDX; 0x800000000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_1993492831912946640; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x3b63534e0f53397eu64);
            write_reg_value!(q; a; RDX; 0x1baa4f2b2c756bd0u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x774141a1b8e9d1e9u64);
            assert_reg_value!(q; a; RDX; 0x39fbdc6f46f607d0u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_4; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xf);
            write_reg_value!(q; a; RDX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4444444444444445u64);
            assert_reg_value!(q; a; RDX; 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_1125899906842624; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x4000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000001u64);
            assert_reg_value!(q; a; RDX; 0x8000000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_17592186044416; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x100000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x200000000001u64);
            assert_reg_value!(q; a; RDX; 0x200000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_17; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x3f);
            write_reg_value!(q; a; RDX; 0x11);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4514514514514515u64);
            assert_reg_value!(q; a; RDX; 0x14);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_68719476736; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x1000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2000000001u64);
            assert_reg_value!(q; a; RDX; 0x2000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_562949953421312; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x2000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4000000000001u64);
            assert_reg_value!(q; a; RDX; 0x4000000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_2199023255552; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x20000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x40000000001u64);
            assert_reg_value!(q; a; RDX; 0x40000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
    // idiv rax
    ax_test![idiv_rax_rdx_2147483647; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x100000000u64);
            write_reg_value!(q; a; RDX; 0x7fffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7fffffff00000001u64);
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_4398046511104; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x40000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x80000000001u64);
            assert_reg_value!(q; a; RDX; 0x80000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_2; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
            write_reg_value!(q; a; RDX; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4924924924924925u64);
            assert_reg_value!(q; a; RDX; 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // idiv rax
    ax_test![idiv_rax_rdx_281474976710656; 0x48, 0xf7, 0xf8; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RDX; 0x1000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2000000000001u64);
            assert_reg_value!(q; a; RDX; 0x2000000000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
