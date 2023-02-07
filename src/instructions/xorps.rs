use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Xorps;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;

use crate::helpers::operand::Operand;

use crate::state::registers::SupportedRegister;

impl Axecutor {
    pub(crate) fn mnemonic_xorps(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Xorps);

        match i.code() {
            Xorps_xmm_xmmm128 => self.instr_xorps_xmm_xmmm128(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Xorps", i.code()),
        }
    }

    /// XORPS xmm1, xmm2/m128
    ///
    /// NP 0F 57 /r
    fn instr_xorps_xmm_xmmm128(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Xorps_xmm_xmmm128);

        let (dest, src) = self.instruction_operands_2(i)?;

        let dest_reg: SupportedRegister = dest.into();

        let src_value = match src {
            Operand::Memory(m) => self.internal_mem_read_128(self.mem_addr(m))?,
            Operand::Register(r) => self.internal_reg_read_128(r)?,
            _ => fatal_error!("Invalid operand for Movups_xmm_xmmm128"),
        };

        let dest_value = self.internal_reg_read_128(dest_reg)?;

        self.internal_reg_write_128(dest_reg, dest_value ^ src_value)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{
        assert_mem_value, assert_reg_value, ax_test, init_mem_value, write_reg_value,
    };
    use iced_x86::Register::*;

    // xorps xmm0, xmm1
    ax_test![xorps_xmm0_xmm1_42_30; 0xf, 0x57, 0xc1;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x1000000);
            write_reg_value!(x; a; XMM1; 0x100000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x100001000000u64);
            assert_reg_value!(x; a; XMM1; 0x100000000000u64);
        }
    ];

    // xorps xmm0, xmm1
    ax_test![xorps_xmm0_xmm1_8_45; 0xf, 0x57, 0xc1;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x8);
            write_reg_value!(x; a; XMM1; 0x20000000);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x20000008);
            assert_reg_value!(x; a; XMM1; 0x20000000);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_11_53; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x1fu128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x400000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x40000000001fu128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x400000000000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_48_26; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x964b401d4fe1f0707737810d2358ffb2u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x964b401d4fe1f070773f810d2358ffb2u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000000000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_95_53_46; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xd8960da43ba930c71bdeeafb14dba4bdu128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x1f9d734e551fcda8eb6d1f6a96a4c214u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0xc70b7eea6eb6fd6ff0b3f591827f66a9u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1f9d734e551fcda8eb6d1f6a96a4c214u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_88_79_96; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x20000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x3fu128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x2000000003fu128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x3fu128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_87_21; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x800000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x20000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x820000000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20000000000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_59_85_100; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x10000000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x100000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x10000000100000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x100000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_61_87_92; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xd3e1ea74adb39d3cbde8cbf993f949c4u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0xd3e1ea74adb39d3cbde8cb7993f949c4u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_70_3_39; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x20000000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x20000000000008u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_92_24; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x10000000000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x40u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x10000000000000040u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x40u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_17_58; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x400000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x80000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x400000080000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x80000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_94_50_21; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x20000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x4000000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x4000020000000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x4000000000000000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_31_92; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x4000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x100000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x4100000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x100000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_96_9; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x4u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x800000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x800004u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x800000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_4_20; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x400000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x964b401d4fe1f0707737810d2358ffb2u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x964b401d4fe1f070773781092358ffb2u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x964b401d4fe1f0707737810d2358ffb2u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_96_87; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x10u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x18u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_41_99; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x3bcf5022f4b71e4f9984d9d22b183ed3u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x1f9d734e551fcda8eb6d1f6a96a4c214u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x2452236ca1a8d3e772e9c6b8bdbcfcc7u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1f9d734e551fcda8eb6d1f6a96a4c214u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_3_84_17; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x80000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x20000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x20000080000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20000000000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_57_7; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x800000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x20000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x20000000800000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20000000000000u128);
        }
    ];

    // xorps xmm0, xmmword ptr [rbx]
    ax_test![xorps_xmm0_xmmword_ptr_rbx_44_1; 0xf, 0x57, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x8u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x100000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x100008u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x100000u128);
        }
    ];
}
