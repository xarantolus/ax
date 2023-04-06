use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Movups;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;

use crate::helpers::operand::Operand;

use crate::state::registers::SupportedRegister;

impl Axecutor {
    pub(crate) fn mnemonic_movups(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Movups);

        match i.code() {
            Movups_xmm_xmmm128 => self.instr_movups_xmm_xmmm128(i),
            Movups_xmmm128_xmm => self.instr_movups_xmmm128_xmm(i),
            _ => fatal_error!(
                "Invalid instruction code {:?} for mnemonic Movups",
                i.code()
            ),
        }
    }

    /// MOVUPS xmm1, xmm2/m128
    ///
    /// NP 0F 10 /r
    fn instr_movups_xmm_xmmm128(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movups_xmm_xmmm128);

        let (dest, src) = self.instruction_operands_2(i)?;

        let dest_reg: SupportedRegister = dest.try_into()?;

        let src_value = match src {
            Operand::Memory(m) => self.internal_mem_read_128(self.mem_addr(m))?,
            Operand::Register(r) => self.internal_reg_read_128(r)?,
            _ => fatal_error!("Invalid operand for Movups_xmm_xmmm128"),
        };

        self.internal_reg_write_128(dest_reg, src_value)
    }

    /// MOVUPS xmm2/m128, xmm1
    ///
    /// NP 0F 11 /r
    fn instr_movups_xmmm128_xmm(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movups_xmmm128_xmm);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_reg: SupportedRegister = src.try_into()?;

        let src_value = self.internal_reg_read_128(src_reg)?;

        match dest {
            Operand::Memory(m) => self.internal_mem_write_128(self.mem_addr(m), src_value),
            Operand::Register(r) => self.internal_reg_write_128(r, src_value),
            _ => fatal_error!("Invalid operand for Movups_xmmm128_xmm"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{
        assert_mem_value, assert_reg_value, ax_test, init_mem_value, write_reg_value,
    };
    use iced_x86::Register::*;

    // movups xmm0, xmm1
    ax_test![movups_xmm0_xmm1; 0xf, 0x10, 0xc1; |a: &mut Axecutor| {
        write_reg_value!(x; a; XMM0; 0x1234_5678_9abc_def0_u128);
        write_reg_value!(x; a; XMM1; 0x1234_5678_9abc_def0_u128);
    }; |a: Axecutor| {
        assert_reg_value!(x; a; XMM0; 0x1234_5678_9abc_def0_u128);
        assert_reg_value!(x; a; XMM1; 0x1234_5678_9abc_def0_u128);
    }];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_5_68_13; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xa948f688980e1866583e19cd98c54056u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x100000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x100000000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x100000000000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_64_14_60; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x8u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x8000000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_50_60; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xdb580209d6e0734da3111981aa9b1211u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x11u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x11u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x11u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_14_93_1; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x71173ef2d7674a9f9b68047ace7871eau128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x2000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x2000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x2000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_73_86; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x800000000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x20u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x20u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_21_13; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x8000000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x20u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x20u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_43_95; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x1000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x1000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x1000000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1000000000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_19_81; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xfdfbd1c4e7f33b1ea985b7b30f91ace5u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0xd39f227fe4a522da4986abd2aa7399b7u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0xd39f227fe4a522da4986abd2aa7399b7u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0xd39f227fe4a522da4986abd2aa7399b7u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_69_12_88; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xc5659038027afecfc1fc627f58fac58au128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x1000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x1000000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1000000000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_97_56; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xffffffffffffffffffffffffffffffffu128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x80u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x80u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x80u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_10_13_16; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xba2f9f18b746bc59d5e99ab0217ea0f1u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x20634f0b66933462u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x20634f0b66933462u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20634f0b66933462u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_76_52; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x800u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x8000000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_12_14; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x80000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0xe8c5e6380e7a98447274fe762b8ecd9au128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0xe8c5e6380e7a98447274fe762b8ecd9au128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0xe8c5e6380e7a98447274fe762b8ecd9au128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_42_60; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x3fu128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x40u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x40u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x40u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_87_75_6; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x4000000000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x8u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_30_38; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xdb1b23e3704f410c5ce6e831563a688du128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8000000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x8000000000000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000000000000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_89_10; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x40000000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0xba2f9f18b746bc59d5e99ab0217ea0f1u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0xba2f9f18b746bc59d5e99ab0217ea0f1u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0xba2f9f18b746bc59d5e99ab0217ea0f1u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_29_11_81; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xfdfbd1c4e7f33b1ea985b7b30f91ace5u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x8000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_53_11; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x10u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x4000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x4000000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x4000000000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_1_97; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x0u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x1b2e15de166cd33639fcd5039029a960u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x1b2e15de166cd33639fcd5039029a960u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x1b2e15de166cd33639fcd5039029a960u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_1_68; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xfdfbd1c4e7f33b1ea985b7b30f91ace5u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x0u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x0u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x0u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_97_16; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x1000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x10000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x10000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x10000000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_43_4_69; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x10000000u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x2da3c5448f893999u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x2da3c5448f893999u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x2da3c5448f893999u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_15_33; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0x11u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x200000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x200000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x200000u128);
        }
    ];

    // movups xmm0, xmmword ptr [rbx]
    ax_test![movups_xmm0_xmmword_ptr_rbx_5_76_74; 0xf, 0x10, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(x; a; XMM0; 0xa18219f7dd08ab4cc5e27930b38c95d7u128);
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x20000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(x; a; XMM0; 0x20000000u128);
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20000000u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_73_74; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x7fffffffffffffffffffffffffffffffu128);
            write_reg_value!(x; a; XMM15; 0x15c409881805efdfdc30033b3cd70c58u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x15c409881805efdfdc30033b3cd70c58u128);
            assert_reg_value!(x; a; XMM15; 0x15c409881805efdfdc30033b3cd70c58u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_84_50; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x10000000000000u128);
            write_reg_value!(x; a; XMM15; 0x20000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20000000000u128);
            assert_reg_value!(x; a; XMM15; 0x20000000000u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_9_82_24; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x1000000000000000u128);
            write_reg_value!(x; a; XMM15; 0x10000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x10000000000000u128);
            assert_reg_value!(x; a; XMM15; 0x10000000000000u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_39_50_1; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x2000000000u128);
            write_reg_value!(x; a; XMM15; 0x2000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x2000000000u128);
            assert_reg_value!(x; a; XMM15; 0x2000000000u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_95_71; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x2000000000u128);
            write_reg_value!(x; a; XMM15; 0xffu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0xffu128);
            assert_reg_value!(x; a; XMM15; 0xffu128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_74_99; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x10000000000000000u128);
            write_reg_value!(x; a; XMM15; 0x6c60be8a0c68b00e340c477b357ba896u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x6c60be8a0c68b00e340c477b357ba896u128);
            assert_reg_value!(x; a; XMM15; 0x6c60be8a0c68b00e340c477b357ba896u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_56_24_53; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x40000000000000u128);
            write_reg_value!(x; a; XMM15; 0x4000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x4000000000u128);
            assert_reg_value!(x; a; XMM15; 0x4000000000u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_58_40; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x10000u128);
            write_reg_value!(x; a; XMM15; 0x20000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20000000000u128);
            assert_reg_value!(x; a; XMM15; 0x20000000000u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_1_76_93; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x2000000000u128);
            write_reg_value!(x; a; XMM15; 0x3fu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x3fu128);
            assert_reg_value!(x; a; XMM15; 0x3fu128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_4_10; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x200000000000u128);
            write_reg_value!(x; a; XMM15; 0x7u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x7u128);
            assert_reg_value!(x; a; XMM15; 0x7u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_97_57; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x10000u128);
            write_reg_value!(x; a; XMM15; 0xc85f56a33fde4ffdb8ca097cd1a781f2u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0xc85f56a33fde4ffdb8ca097cd1a781f2u128);
            assert_reg_value!(x; a; XMM15; 0xc85f56a33fde4ffdb8ca097cd1a781f2u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_48_7; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x1fu128);
            write_reg_value!(x; a; XMM15; 0x200000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x200000000000000u128);
            assert_reg_value!(x; a; XMM15; 0x200000000000000u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_15_60_42; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x200000u128);
            write_reg_value!(x; a; XMM15; 0x20000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x20000000000000u128);
            assert_reg_value!(x; a; XMM15; 0x20000000000000u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_65_100_69; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x78abfebcda7e665c008cb267c8a3c2bcu128);
            write_reg_value!(x; a; XMM15; 0xe7bd13a959f2a8d61edd963a36101aacu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0xe7bd13a959f2a8d61edd963a36101aacu128);
            assert_reg_value!(x; a; XMM15; 0xe7bd13a959f2a8d61edd963a36101aacu128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_48_72; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x11u128);
            write_reg_value!(x; a; XMM15; 0x8u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8u128);
            assert_reg_value!(x; a; XMM15; 0x8u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_86_84_78; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x1000u128);
            write_reg_value!(x; a; XMM15; 0x41u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x41u128);
            assert_reg_value!(x; a; XMM15; 0x41u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_27_62_19; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x200000000000u128);
            write_reg_value!(x; a; XMM15; 0x3c147257eea77ac2046436e2771e874u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x3c147257eea77ac2046436e2771e874u128);
            assert_reg_value!(x; a; XMM15; 0x3c147257eea77ac2046436e2771e874u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_41_49; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x10000u128);
            write_reg_value!(x; a; XMM15; 0x0u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x0u128);
            assert_reg_value!(x; a; XMM15; 0x0u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_86_83; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x5fdb1eecef01ef729e09bd8e4cb91026u128);
            write_reg_value!(x; a; XMM15; 0x532a4a4ad440d19f8c60154428af2661u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x532a4a4ad440d19f8c60154428af2661u128);
            assert_reg_value!(x; a; XMM15; 0x532a4a4ad440d19f8c60154428af2661u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_65_43; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x200u128);
            write_reg_value!(x; a; XMM15; 0x6e65e97419ccfc3ab5aefec84f010ddcu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x6e65e97419ccfc3ab5aefec84f010ddcu128);
            assert_reg_value!(x; a; XMM15; 0x6e65e97419ccfc3ab5aefec84f010ddcu128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_72_57; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x8000000000u128);
            write_reg_value!(x; a; XMM15; 0x40u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x40u128);
            assert_reg_value!(x; a; XMM15; 0x40u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_99_10; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x15c409881805efdfdc30033b3cd70c58u128);
            write_reg_value!(x; a; XMM15; 0x7fffffffffffffffffffffffffffffffu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x7fffffffffffffffffffffffffffffffu128);
            assert_reg_value!(x; a; XMM15; 0x7fffffffffffffffffffffffffffffffu128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_36_9; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x4000000000000000u128);
            write_reg_value!(x; a; XMM15; 0x8000000000000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x8000000000000000u128);
            assert_reg_value!(x; a; XMM15; 0x8000000000000000u128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_71_12; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0x2000u128);
            write_reg_value!(x; a; XMM15; 0x265f6dda04c9f566ce05658cc22769abu128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x265f6dda04c9f566ce05658cc22769abu128);
            assert_reg_value!(x; a; XMM15; 0x265f6dda04c9f566ce05658cc22769abu128);
        }
    ];

    // movups xmmword ptr [rbx], xmm15
    ax_test![movups_xmmword_ptr_rbx_xmm15_87_20_58; 0x44, 0xf, 0x11, 0x3b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1000);
            init_mem_value!(x; a; 0x1000; 0xdee17ddaa9b32e0bcca1f3e5e203951eu128);
            write_reg_value!(x; a; XMM15; 0x40000000u128);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1000);
            assert_mem_value!(x; a; 0x1000; 0x40000000u128);
            assert_reg_value!(x; a; XMM15; 0x40000000u128);
        }
    ];
}
