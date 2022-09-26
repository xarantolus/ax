use iced_x86::Instruction;
use iced_x86::Mnemonic::Cqo;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::registers::SupportedRegister::*;

impl Axecutor {
    pub fn mnemonic_cqo(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Cqo);

        match i.code() {
            iced_x86::Code::Cqo => self.instr_cqo(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Cqo", i.code()),
        }
    }

    /// CQO
    ///
    /// o64 99
    fn instr_cqo(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), iced_x86::Code::Cqo);

        let rax = self.reg_read_64(RAX);
        let rdx = if rax & 0x8000_0000_0000_0000 == 0 {
            0
        } else {
            0xFFFF_FFFF_FFFF_FFFF
        };

        self.reg_write_64(RDX, rdx);

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

    // cqo
    ax_test![cqo_rax_2305843009213693952_rdx_9007199254740992; 0x48, 0x99; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x2000000000000000u64);
            write_reg_value!(q; a; RDX; 0x20000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2000000000000000u64);
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cqo
    ax_test![cqo_rax_288230376151711744_rdx_68719476736; 0x48, 0x99; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x400000000000000u64);
            write_reg_value!(q; a; RDX; 0x1000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x400000000000000u64);
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cqo
    ax_test![cqo_rax_1099511627776_rdx_281474976710656; 0x48, 0x99; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10000000000u64);
            write_reg_value!(q; a; RDX; 0x1000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x10000000000u64);
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cqo
    ax_test![cqo_rax_32768_rdx_562949953421312; 0x48, 0x99; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000);
            write_reg_value!(q; a; RDX; 0x2000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000);
            assert_reg_value!(q; a; RDX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cqo
    ax_test![cqo_rax_11269479258687003689_rdx_6181271058541996644; 0x48, 0x99; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x9c653bad71abdc29u64);
            write_reg_value!(q; a; RDX; 0x55c8494728cc6a64u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x9c653bad71abdc29u64);
            assert_reg_value!(q; a; RDX; 0xffffffffffffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cqo
    ax_test![cqo_rax_10122414085578855704_rdx_512; 0x48, 0x99; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8c7a09fae0569518u64);
            write_reg_value!(q; a; RDX; 0x200);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8c7a09fae0569518u64);
            assert_reg_value!(q; a; RDX; 0xffffffffffffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cqo
    ax_test![cqo_rax_17228847572177372304_rdx_8; 0x48, 0x99; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xef192995a4fbc490u64);
            write_reg_value!(q; a; RDX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xef192995a4fbc490u64);
            assert_reg_value!(q; a; RDX; 0xffffffffffffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // cqo
    ax_test![cqo_rax_15029376158311628101_rdx_2650862518581141615; 0x48, 0x99; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xd09311fca7e3bd45u64);
            write_reg_value!(q; a; RDX; 0x24c9c16a706d206fu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xd09311fca7e3bd45u64);
            assert_reg_value!(q; a; RDX; 0xffffffffffffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
