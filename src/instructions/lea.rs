use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Lea;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::fatal_error;
use crate::instructions::operand::Operand;

impl Axecutor {
    pub fn mnemonic_lea(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Lea);

        match i.code() {
            Lea_r16_m => self.instr_lea_r16_m(i),
            Lea_r32_m => self.instr_lea_r32_m(i),
            Lea_r64_m => self.instr_lea_r64_m(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Lea", i.code()),
        }
    }

    /// LEA r16, m
    ///
    /// o16 8D /r
    fn instr_lea_r16_m(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Lea_r16_m);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_addr = match src {
            Operand::Memory(m) => self.mem_addr(m),
            _ => fatal_error!("Invalid source operand {:?} for LEA r16, m", src),
        };

        let dest = dest.into();

        self.reg_write_16(dest, src_addr as u16 as u64)?;
        Ok(())
    }

    /// LEA r32, m
    ///
    /// o32 8D /r
    fn instr_lea_r32_m(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Lea_r32_m);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_addr = match src {
            Operand::Memory(m) => self.mem_addr(m),
            _ => fatal_error!("Invalid source operand {:?} for LEA r32, m", src),
        };

        let dest = dest.into();

        self.reg_write_32(dest, src_addr as u32 as u64)?;
        Ok(())
    }

    /// LEA r64, m
    ///
    /// o64 8D /r
    fn instr_lea_r64_m(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Lea_r64_m);

        let (dest, src) = self.instruction_operands_2(i)?;

        let src_addr = match src {
            Operand::Memory(m) => self.mem_addr(m),
            _ => fatal_error!("Invalid source operand {:?} for LEA r64, m", src),
        };

        let dest = dest.into();

        self.reg_write_64(dest, src_addr as u64)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // lea ax, word ptr [rax]
    ax_test![lea_ax_word_ptr_rax; 0x66, 0x8d, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // lea ax, word ptr [rax+8]
    ax_test![lea_ax_word_ptr_rax_8; 0x66, 0x8d, 0x40, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1008);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // lea ax, word ptr [rax+4*rcx]
    ax_test![lea_ax_word_ptr_rax_4_rcx; 0x66, 0x8d, 0x4, 0x88;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
            write_reg_value!(q; a; RAX; 0x1000);
            write_reg_value!(q; a; RCX; 0x4);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1000 + 4 * 0x4);
            assert_reg_value!(q; a; RCX; 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // lea eax, [rax]
    ax_test![lea_eax_rax; 0x8d, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x41aa2697);

            write_reg_value!(q; a; RAX; 0x1000);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // lea r11, [rax+4*rcx]
    ax_test![lea_r11_rax_4_rcx; 0x4c, 0x8d, 0x1c, 0x88;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0xccdaef98590216f9u64);
            write_reg_value!(q; a; RAX; 0x2000);
            write_reg_value!(q; a; RCX; 0x3);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x2000 + 4 * 0x3);
            assert_reg_value!(q; a; RAX; 0x2000);
            assert_reg_value!(q; a; RCX; 0x3);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
