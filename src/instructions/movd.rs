use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Movd;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::helpers::operand::Operand;

use crate::state::registers::SupportedRegister;

impl Axecutor {
    pub(crate) fn mnemonic_movd(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Movd);

        match i.code() {
            Movd_mm_rm32 => self.instr_movd_mm_rm32(i),
            Movd_xmm_rm32 => self.instr_movd_xmm_rm32(i),
            Movd_rm32_mm => self.instr_movd_rm32_mm(i),
            Movd_rm32_xmm => self.instr_movd_rm32_xmm(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Movd", i.code()),
        }
    }

    /// MOVD mm, r/m32
    ///
    /// NP 0F 6E /r
    fn instr_movd_mm_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movd_mm_rm32);

        opcode_unimplemented!("instr_movd_mm_rm32 for Movd")
    }

    /// MOVD xmm, r/m32
    ///
    /// 66 0F 6E /r
    fn instr_movd_xmm_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movd_xmm_rm32);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_value = match src {
            Operand::Register(src_reg) => self.reg_read_32(src_reg)?,
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand for MOVD xmm, r/m32 instruction"),
        } as u128;

        match dest {
            Operand::Register(dest_reg) => self.internal_reg_write_128(dest_reg, src_value),
            _ => fatal_error!("Invalid operand for MOVD xmm, r/m32 instruction"),
        }
    }

    /// MOVD r/m32, mm
    ///
    /// NP 0F 7E /r
    fn instr_movd_rm32_mm(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movd_rm32_mm);

        opcode_unimplemented!("instr_movd_rm32_mm for Movd")
    }

    /// MOVD r/m32, xmm
    ///
    /// 66 0F 7E /r
    fn instr_movd_rm32_xmm(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movd_rm32_xmm);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_value = match src {
            Operand::Register(r) => {
                let src_reg: SupportedRegister = r;
                (self.internal_reg_read_128(src_reg)? & 0xffffffff) as u64
            }
            _ => fatal_error!("Invalid operand for MOVD r/m32, xmm instruction"),
        };

        match dest {
            Operand::Register(r) => self.reg_write_32(r, src_value),
            Operand::Memory(m) => self.mem_write_32(self.mem_addr(m), src_value),
            _ => fatal_error!("Invalid operand for MOVD r/m32, xmm instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_mem_value, assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_9_45_46; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000);
            write_reg_value!(x; a; XMM0; 0xea776810401ee1a3aca34627f35baaf4u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf35baaf4u32);
            assert_reg_value!(x; a; XMM0; 0xea776810401ee1a3aca34627f35baaf4u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_18_24; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(x; a; XMM0; 0x20u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x20);
            assert_reg_value!(x; a; XMM0; 0x20u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_28_66; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8000);
            write_reg_value!(x; a; XMM0; 0x7ab29d4818a1f1cf5de7d9b57158eff5u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7158eff5);
            assert_reg_value!(x; a; XMM0; 0x7ab29d4818a1f1cf5de7d9b57158eff5u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_26_95; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x4000);
            write_reg_value!(x; a; XMM0; 0x10000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x10000000);
            assert_reg_value!(x; a; XMM0; 0x10000000u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_44_82; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8000);
            write_reg_value!(x; a; XMM0; 0x2u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x2);
            assert_reg_value!(x; a; XMM0; 0x2u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_91_38; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(x; a; XMM0; 0xe9e96c118776000c5fe1755e03d199bu128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xe03d199bu32);
            assert_reg_value!(x; a; XMM0; 0xe9e96c118776000c5fe1755e03d199bu128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_3_20; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8000);
            write_reg_value!(x; a; XMM0; 0x200000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(x; a; XMM0; 0x200000000000000u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_18; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(x; a; XMM0; 0x100000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x100000);
            assert_reg_value!(x; a; XMM0; 0x100000u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_1_17; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x2000);
            write_reg_value!(x; a; XMM0; 0x7fffffffffffffffffffffffffffffffu128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffffffu32);
            assert_reg_value!(x; a; XMM0; 0x7fffffffffffffffffffffffffffffffu128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_92_41_52; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x20000);
            write_reg_value!(x; a; XMM0; 0xffu128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xff);
            assert_reg_value!(x; a; XMM0; 0xffu128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_12_22; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x21);
            write_reg_value!(x; a; XMM0; 0x8000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(x; a; XMM0; 0x8000000000u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_51_89_26; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x10000000);
            write_reg_value!(x; a; XMM0; 0x538845c15b5fc38ed1195bd57eaf8fcu128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x57eaf8fc);
            assert_reg_value!(x; a; XMM0; 0x538845c15b5fc38ed1195bd57eaf8fcu128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_38_18; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fff);
            write_reg_value!(x; a; XMM0; 0x538845c15b5fc38ed1195bd57eaf8fcu128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x57eaf8fc);
            assert_reg_value!(x; a; XMM0; 0x538845c15b5fc38ed1195bd57eaf8fcu128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_46_31; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x20);
            write_reg_value!(x; a; XMM0; 0x7fffffffu128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7fffffffu32);
            assert_reg_value!(x; a; XMM0; 0x7fffffffu128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_100_4_49; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x100000);
            write_reg_value!(x; a; XMM0; 0x10000000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(x; a; XMM0; 0x10000000000000000u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_66_24; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x41);
            write_reg_value!(x; a; XMM0; 0x8429d3513504a6772a8bbffc7606d8a1u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7606d8a1);
            assert_reg_value!(x; a; XMM0; 0x8429d3513504a6772a8bbffc7606d8a1u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_31_4_40; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1000);
            write_reg_value!(x; a; XMM0; 0x1fu128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1f);
            assert_reg_value!(x; a; XMM0; 0x1fu128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_94_27_55; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x4000);
            write_reg_value!(x; a; XMM0; 0x8000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(x; a; XMM0; 0x8000000000u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_35_56; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x20000);
            write_reg_value!(x; a; XMM0; 0x0u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(x; a; XMM0; 0x0u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_47_43_36; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x40000000);
            write_reg_value!(x; a; XMM0; 0x1u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_reg_value!(x; a; XMM0; 0x1u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_4_40; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000);
            write_reg_value!(x; a; XMM0; 0x100u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x100);
            assert_reg_value!(x; a; XMM0; 0x100u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_25_37; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x21);
            write_reg_value!(x; a; XMM0; 0x7ab29d4818a1f1cf5de7d9b57158eff5u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x7158eff5);
            assert_reg_value!(x; a; XMM0; 0x7ab29d4818a1f1cf5de7d9b57158eff5u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_5_97_100; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
            write_reg_value!(x; a; XMM0; 0x1000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
            assert_reg_value!(x; a; XMM0; 0x1000000000u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_30_46_0; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
            write_reg_value!(x; a; XMM0; 0xea776810401ee1a3aca34627f35baaf4u128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xf35baaf4u32);
            assert_reg_value!(x; a; XMM0; 0xea776810401ee1a3aca34627f35baaf4u128);
        }
    ];

    // movd eax, xmm0
    ax_test![movd_eax_xmm0_87_49; 0x66, 0xf, 0x7e, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x100);
            write_reg_value!(x; a; XMM0; 0x764d3a8e9118dbfcd523e8e1ab37b7aeu128);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xab37b7aeu32);
            assert_reg_value!(x; a; XMM0; 0x764d3a8e9118dbfcd523e8e1ab37b7aeu128);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_50_99; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x4000u128);
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x7fffffffu128);
            assert_reg_value!(d; a; EAX; 0x7fffffffu32);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_52_81; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x8000000000u128);
            write_reg_value!(d; a; EAX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x20u128);
            assert_reg_value!(d; a; EAX; 0x20);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_79_36; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x40000000000000u128);
            write_reg_value!(d; a; EAX; 0x1000000);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x1000000u128);
            assert_reg_value!(d; a; EAX; 0x1000000);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_16_1; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x400u128);
            write_reg_value!(d; a; EAX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x200u128);
            assert_reg_value!(d; a; EAX; 0x200);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_46_8; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x41u128);
            write_reg_value!(d; a; EAX; 0x1000000);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x1000000u128);
            assert_reg_value!(d; a; EAX; 0x1000000);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_42_23; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x8000000000000000u128);
            write_reg_value!(d; a; EAX; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0xffu128);
            assert_reg_value!(d; a; EAX; 0xff);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_79_86_68; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x2000000u128);
            write_reg_value!(d; a; EAX; 0x40000);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x40000u128);
            assert_reg_value!(d; a; EAX; 0x40000);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_92_71; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x989798c52b9f7feb3082eb9a8a47eff0u128);
            write_reg_value!(d; a; EAX; 0x7fffffffu32);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x7fffffffu128);
            assert_reg_value!(d; a; EAX; 0x7fffffffu32);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_6_53; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x400000000u128);
            write_reg_value!(d; a; EAX; 0x20);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x20u128);
            assert_reg_value!(d; a; EAX; 0x20);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_43_31; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0xfu128);
            write_reg_value!(d; a; EAX; 0x2000);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x2000u128);
            assert_reg_value!(d; a; EAX; 0x2000);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_2_50; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x800000000u128);
            write_reg_value!(d; a; EAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x1u128);
            assert_reg_value!(d; a; EAX; 0x1);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_55_28; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x2000000000u128);
            write_reg_value!(d; a; EAX; 0x7);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x7u128);
            assert_reg_value!(d; a; EAX; 0x7);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_28_92; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x100000u128);
            write_reg_value!(d; a; EAX; 0x400000);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x400000u128);
            assert_reg_value!(d; a; EAX; 0x400000);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_93_2; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x400000u128);
            write_reg_value!(d; a; EAX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x7fu128);
            assert_reg_value!(d; a; EAX; 0x7f);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_70_59_8; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x4000000000u128);
            write_reg_value!(d; a; EAX; 0x200000);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x200000u128);
            assert_reg_value!(d; a; EAX; 0x200000);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_75; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x1u128);
            write_reg_value!(d; a; EAX; 0xf);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0xfu128);
            assert_reg_value!(d; a; EAX; 0xf);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_9_54; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x400000000000u128);
            write_reg_value!(d; a; EAX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x80u128);
            assert_reg_value!(d; a; EAX; 0x80);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_17_95; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x1u128);
            write_reg_value!(d; a; EAX; 0x100000);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x100000u128);
            assert_reg_value!(d; a; EAX; 0x100000);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_30; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x1u128);
            write_reg_value!(d; a; EAX; 0x7f);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x7fu128);
            assert_reg_value!(d; a; EAX; 0x7f);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_52_19; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x2u128);
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x80000000u128);
            assert_reg_value!(d; a; EAX; 0x80000000u32);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_84_56; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x20000000000u128);
            write_reg_value!(d; a; EAX; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x80u128);
            assert_reg_value!(d; a; EAX; 0x80);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_41_95; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x80000000000000u128);
            write_reg_value!(d; a; EAX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x200u128);
            assert_reg_value!(d; a; EAX; 0x200);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_22_23_30; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0xc98e180bbd948a5ccb7ab637a8a726bau128);
            write_reg_value!(d; a; EAX; 0x7fff);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x7fffu128);
            assert_reg_value!(d; a; EAX; 0x7fff);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_79_10; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x4000000000u128);
            write_reg_value!(d; a; EAX; 0x4000000);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x4000000u128);
            assert_reg_value!(d; a; EAX; 0x4000000);
        }
    ];

    // movd xmm3, eax
    ax_test![movd_xmm3_eax_71_51; 0x66, 0xf, 0x6e, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x40000u128);
            write_reg_value!(d; a; EAX; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x41u128);
            assert_reg_value!(d; a; EAX; 0x41);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_8_29_53; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x100000000000u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x100u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x100u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x100u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_56_6; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x80u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x200u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x200u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x200u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_75_20; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x6b0330c4e86b13132b4903d509e69259u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x20u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x20u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_23_22_9; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x9ef1f1b8f33e3439a586d38d09e1b422u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x8000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x8000000u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_17_14_37; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x800de77367246f1ff54a0308ec898959u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x2u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x2u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x2u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_57_40; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x8000u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x2000000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x0u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x2000000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_10_77; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0xffu128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x8000000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x0u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_85_83_93; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x2000000000u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x4000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x4000000u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x4000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_22_9; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x4000000000000000u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x1000000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x0u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1000000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_9_17_57; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x100000000000000u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x400u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x400u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x400u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_95_89; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0xfu128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x77d09f1132ec37fe001e66d74bda0790u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x4bda0790u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x77d09f1132ec37fe001e66d74bda0790u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_76_45; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x200u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x1000000000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x0u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1000000000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_51_82; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x40u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x10u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x10u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x10u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_43_26; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0xeaa6ad9dacacabc8d25ae7dc8e9aa78fu128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x400000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x400000u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x400000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_1_89_69; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x2da54c6a269694a0f37ef3ed34233881u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x20u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x20u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_45_63; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x4000000000u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0xa3f5dea14b9b8353459e8ea7b48cd0c7u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0xb48cd0c7u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0xa3f5dea14b9b8353459e8ea7b48cd0c7u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_85_45; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x100000000u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x1000000000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x0u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1000000000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_66_79; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0xffu128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x40000000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x0u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x40000000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_8_90_19; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x7277264ad2f41a5383ee925738989d88u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x8000000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x0u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_21_47; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x400u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x10000000000000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x0u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x10000000000000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_39_63; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x11u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x1000000000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x0u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1000000000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_89_39_85; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x83ea560ede4d200e445cef4497a3226u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x10000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x10000u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x10000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_81_15; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0xffffffffffffffffffffffffffffffffu128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x40000000u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x40000000u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x40000000u128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_36_8_3; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x100000u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x1fu128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0x1fu128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1fu128);
        }
    ];

    // movd xmm3, [rax]
    ax_test![movd_xmm3_rax_48_58; 0x66, 0xf, 0x6e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM3; 0x40000000000u128);
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x1e43ba89cd605b0eff159ce7cc584403u128).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM3; 0xcc584403u128);
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1e43ba89cd605b0eff159ce7cc584403u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_22_15_36; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x5993f63db6898d7c4b1db72e41c680b2u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x1000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x5993f63db6898d7c4b1db72e00000000u128);
            assert_reg_value!(x; a; XMM3; 0x1000000000000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_93_56_71; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x20000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x8000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x0u128);
            assert_reg_value!(x; a; XMM3; 0x8000000000000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_33_100; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x40000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0xefd66399a19309c1959b90801c1a13fau128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1c1a13fau128);
            assert_reg_value!(x; a; XMM3; 0xefd66399a19309c1959b90801c1a13fau128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_36_21_29; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x100000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x200u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x200u128);
            assert_reg_value!(x; a; XMM3; 0x200u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_99_48_12; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x20000000000000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x400000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20000000400000u128);
            assert_reg_value!(x; a; XMM3; 0x400000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_96_76; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x41u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x10u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x10u128);
            assert_reg_value!(x; a; XMM3; 0x10u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_14_6_94; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x8000000000000000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x40000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000000040000u128);
            assert_reg_value!(x; a; XMM3; 0x40000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_40_75; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x200000000000000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x7fffffffffffffffu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x2000000ffffffffu128);
            assert_reg_value!(x; a; XMM3; 0x7fffffffffffffffu128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_26_23; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x40u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x8u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8u128);
            assert_reg_value!(x; a; XMM3; 0x8u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_35_87; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x1u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x800000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x0u128);
            assert_reg_value!(x; a; XMM3; 0x800000000000000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_9_28; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x800000000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x80000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x800000000u128);
            assert_reg_value!(x; a; XMM3; 0x80000000000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_48_46; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x71d0c687d4df21de907b0daba8a7716cu128).unwrap();
            write_reg_value!(x; a; XMM3; 0xbefc0c2b534c249dc3009837a9880f6fu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x71d0c687d4df21de907b0daba9880f6fu128);
            assert_reg_value!(x; a; XMM3; 0xbefc0c2b534c249dc3009837a9880f6fu128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_96_1_26; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0xb81ed1c1b7b338beb6f203eee340bd8bu128).unwrap();
            write_reg_value!(x; a; XMM3; 0x8u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0xb81ed1c1b7b338beb6f203ee00000008u128);
            assert_reg_value!(x; a; XMM3; 0x8u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_39_6_37; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x20000000000000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x7fffffffffffffffffffffffffffffffu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x200000ffffffffu128);
            assert_reg_value!(x; a; XMM3; 0x7fffffffffffffffffffffffffffffffu128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_79_15; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x479befd4f300589a40b59b3a5b5284cau128).unwrap();
            write_reg_value!(x; a; XMM3; 0x10u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x479befd4f300589a40b59b3a00000010u128);
            assert_reg_value!(x; a; XMM3; 0x10u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_7_7_10; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x7b101aec9255437e86cad3db227ae5d0u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x10000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x7b101aec9255437e86cad3db10000000u128);
            assert_reg_value!(x; a; XMM3; 0x10000000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_51_4; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x7fffffffffffffffffffffffffffffffu128).unwrap();
            write_reg_value!(x; a; XMM3; 0x800000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x7fffffffffffffffffffffff00000000u128);
            assert_reg_value!(x; a; XMM3; 0x800000000000000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_63_39; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x200000000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x4000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x200000000u128);
            assert_reg_value!(x; a; XMM3; 0x4000000000000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_90_41; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x40000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x2000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x2000u128);
            assert_reg_value!(x; a; XMM3; 0x2000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_37_64; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x4df9d8bd5cc2d8f57451d5fbeefd296fu128).unwrap();
            write_reg_value!(x; a; XMM3; 0x7fffffffffffffffffffffffffffffffu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x4df9d8bd5cc2d8f57451d5fbffffffffu128);
            assert_reg_value!(x; a; XMM3; 0x7fffffffffffffffffffffffffffffffu128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_71_22_33; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x400000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x1000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x0u128);
            assert_reg_value!(x; a; XMM3; 0x1000000000000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_36_54; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x200u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x3b1306c31a191c305a79938911fee6bau128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x11fee6bau128);
            assert_reg_value!(x; a; XMM3; 0x3b1306c31a191c305a79938911fee6bau128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_64_53_75; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x800000000000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0xefd66399a19309c1959b90801c1a13fau128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x80001c1a13fau128);
            assert_reg_value!(x; a; XMM3; 0xefd66399a19309c1959b90801c1a13fau128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_98_95_4; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x100000000000u128).unwrap();
            write_reg_value!(x; a; XMM3; 0x2000000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x100000000000u128);
            assert_reg_value!(x; a; XMM3; 0x2000000000000000u128);
        }
    ];

    // movd [rax], xmm3
    ax_test![movd_rax_xmm3_82_85; 0x66, 0xf, 0x7e, 0x18;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            a.mem_init_zero(0x1000, 16).unwrap();
            a.mem_write_128(0x1000, 0x7fu128).unwrap();
            write_reg_value!(x; a; XMM3; 0x10000000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x0u128);
            assert_reg_value!(x; a; XMM3; 0x10000000000000000u128);
        }
    ];
}
