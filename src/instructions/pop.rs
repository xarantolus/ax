use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Pop;

use iced_x86::Register;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::registers::SupportedRegister;

impl Axecutor {
    pub fn mnemonic_pop(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Pop);

        match i.code() {
            Pop_r16 => self.instr_pop_r16(i),
            Pop_r32 => self.instr_pop_r32(i),
            Pop_r64 => self.instr_pop_r64(i),
            Pop_rm16 => self.instr_pop_rm16(i),
            Pop_rm32 => self.instr_pop_rm32(i),
            Pop_rm64 => self.instr_pop_rm64(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Pop", i.code()),
        }
    }

    /// POP r16
    ///
    /// o16 58+rw
    fn instr_pop_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_r16);

        let reg: SupportedRegister = i.op0_register().into();
        let rsp = self.reg_read_64(Register::RSP.into());

        let value = self.mem_read_16(rsp)?;
        self.reg_write_16(reg, value);

        self.reg_write_64(Register::RSP.into(), rsp + 2);

        Ok(())
    }

    /// POP r32
    ///
    /// o32 58+rd
    fn instr_pop_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_r32);

        panic!("There's no prefix for encoding this in 64-bit x86-64 (see Intel manual)");
    }

    /// POP r64
    ///
    /// o64 58+ro
    fn instr_pop_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_r64);

        let reg: SupportedRegister = i.op0_register().into();
        let rsp = self.reg_read_64(Register::RSP.into());

        let value = self.mem_read_64(rsp)?;
        self.reg_write_64(reg, value);

        self.reg_write_64(Register::RSP.into(), rsp + 8);

        Ok(())
    }

    /// POP r/m16
    ///
    /// o16 8F /0
    fn instr_pop_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_rm16);

        todo!("instr_pop_rm16 for Pop")
    }

    /// POP r/m32
    ///
    /// o32 8F /0
    fn instr_pop_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_rm32);

        todo!("instr_pop_rm32 for Pop")
    }

    /// POP r/m64
    ///
    /// o64 8F /0
    fn instr_pop_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pop_rm64);

        todo!("instr_pop_rm64 for Pop")
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_reg_value, ax_test, instructions::registers::SupportedRegister};
    use iced_x86::Register::*;

    // pop ax
    ax_test![pop_ax; 0x66, 0x58;
        |a: &mut Axecutor| {
            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000);
            a.mem_init_zero(0x1000, 2).unwrap();
            a.mem_write_16(0x1000, 0x1234).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1234);

            assert_eq!(a.reg_read_64(RSP.into()), 0x1002);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // pop rax
    ax_test![pop_rax; 0x58;
        |a: &mut Axecutor| {
            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000);
            a.mem_init_zero(0x1000, 8).unwrap();
            a.mem_write_64(0x1000, 0x1234567890ABCDEF).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1234567890ABCDEFu64);

            assert_eq!(a.reg_read_64(RSP.into()), 0x1008);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
