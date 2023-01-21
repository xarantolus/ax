use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Push;
use iced_x86::OpKind;
use iced_x86::Register;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::helpers::operand::Operand;
use crate::state::registers::SupportedRegister;

impl Axecutor {
    pub fn mnemonic_push(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Push);

        match i.code() {
            Push_r16 => self.instr_push_r16(i),
            Push_r32 => self.instr_push_r32(i),
            Push_r64 => self.instr_push_r64(i),
            Push_imm16 => self.instr_push_imm16(i),
            Push_rm16 => self.instr_push_rm16(i),
            Push_rm32 => self.instr_push_rm32(i),
            Push_rm64 => self.instr_push_rm64(i),
            Pushq_imm8 => self.instr_pushq_imm8(i),
            Pushq_imm32 => self.instr_pushq_imm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Push", i.code()),
        }
    }

    /// PUSH r16
    ///
    /// o16 50+rw
    fn instr_push_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Push_r16);

        let reg: SupportedRegister = i.op0_register().into();

        let value = self.reg_read_16(reg)?;
        let rsp = self.reg_read_64(Register::RSP.into())?;

        self.mem_write_16(rsp, value)?;
        self.reg_write_64(Register::RSP.into(), rsp - 2)?;

        Ok(())
    }

    /// PUSH r32
    ///
    /// o32 50+rd
    fn instr_push_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Push_r32);

        fatal_error!("There's no prefix for encoding this in 64-bit x86-64 (see AMD64 manual)")
    }

    /// PUSH r64
    ///
    /// o64 50+ro
    fn instr_push_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Push_r64);

        let reg: SupportedRegister = i.op0_register().into();

        let value = self.reg_read_64(reg)?;
        let rsp = self.reg_read_64(Register::RSP.into())?;

        self.mem_write_64(rsp, value)?;
        self.reg_write_64(Register::RSP.into(), rsp - 8)?;

        Ok(())
    }

    /// PUSH imm16
    ///
    /// o16 68 iw
    fn instr_push_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Push_imm16);

        let value = i.immediate16() as u64;
        let rsp = self.reg_read_64(Register::RSP.into())?;

        self.mem_write_16(rsp, value)?;
        self.reg_write_64(Register::RSP.into(), rsp - 2)?;

        Ok(())
    }

    /// PUSH r/m16
    ///
    /// o16 FF /6
    fn instr_push_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Push_rm16);

        let src = match self.instruction_operand(i, 0)? {
            Operand::Register(r) => self.reg_read_16(r)?,
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            _ => fatal_error!("Invalid operand {:?} for PUSH r/m16", i.op0_kind()),
        };

        let rsp = self.reg_read_64(Register::RSP.into())?;

        self.mem_write_16(rsp, src)?;
        self.reg_write_64(Register::RSP.into(), rsp - 2)?;

        Ok(())
    }

    /// PUSH r/m32
    ///
    /// o32 FF /6
    fn instr_push_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Push_rm32);

        opcode_unimplemented!("instr_push_rm32 for Push")
    }

    /// PUSH r/m64
    ///
    /// o64 FF /6
    fn instr_push_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Push_rm64);

        opcode_unimplemented!("instr_push_rm64 for Push")
    }

    /// PUSH imm8
    ///
    /// o64 6A ib
    fn instr_pushq_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pushq_imm8);

        // There's some sign-extension magic happening
        match i.op0_kind() {
            // TODO: not sure if 16 and 32-bit are required here, but AMD manual says so
            OpKind::Immediate8to16 => {
                let value = i.immediate8to16();
                let rsp = self.reg_read_64(Register::RSP.into())?;

                self.mem_write_16(rsp, value as u16 as u64)?;
                self.reg_write_64(Register::RSP.into(), rsp - 2)?;
            }
            OpKind::Immediate8to32 => {
                let value = i.immediate8to32();
                let rsp = self.reg_read_64(Register::RSP.into())?;

                self.mem_write_32(rsp, value as u32 as u64)?;
                self.reg_write_64(Register::RSP.into(), rsp - 4)?;
            }
            OpKind::Immediate8to64 => {
                let value = i.immediate8to64();
                let rsp = self.reg_read_64(Register::RSP.into())?;

                self.mem_write_64(rsp, value as u64)?;
                self.reg_write_64(Register::RSP.into(), rsp - 8)?;
            }
            _ => fatal_error!("Invalid operand {:?} for PUSH imm8", i.op0_kind()),
        }

        Ok(())
    }

    /// PUSH imm64
    ///
    /// o64 68 id
    /// This opcode is imm64 in 64-bit mode, but imm32 in 32-bit mode. This implementation is 64-bit
    fn instr_pushq_imm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Pushq_imm32);

        match i.op0_kind() {
            OpKind::Immediate32to64 => {
                // Sign-extend the 32-bit immediate to 64-bit
                let value = i.immediate32to64() as u64;
                let rsp = self.reg_read_64(Register::RSP.into())?;

                self.mem_write_64(rsp, value)?;
                self.reg_write_64(Register::RSP.into(), rsp - 8)?;
            }
            _ => fatal_error!("Invalid operand {:?} for PUSH imm64", i.op0_kind()),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_mem_value, assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // push ax
    ax_test![push_ax; 0x66, 0x50;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1234);

            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000).unwrap();
            a.mem_init_zero(0x1000, 2).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x1234);

            assert_eq!(a.reg_read_64(RSP.into()).unwrap(), 0x1000-2);
            assert_mem_value!(w; a; 0x1000; 0x1234);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // push rbx
    ax_test![push_rbx; 0x53;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBX; 0x1234567890ABCDEF);

            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000).unwrap();
            a.mem_init_zero(0x1000, 8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RBX; 0x1234567890ABCDEFu64);

            assert_eq!(a.reg_read_64(RSP.into()).unwrap(), 0x1000-8);
            assert_mem_value!(q; a; 0x1000; 0x1234567890ABCDEFu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // push 0x1234
    ax_test![push_0x1234; 0x68, 0x34, 0x12, 0x0, 0x0;
        |a: &mut Axecutor| {
            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000).unwrap();
            a.mem_init_zero(0x1000, 8).unwrap();
        };
        |a: Axecutor| {
            assert_eq!(a.reg_read_64(RSP.into()).unwrap(), 0x1000-8);
            assert_mem_value!(q; a; 0x1000; 0x1234);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // push 0xff
    ax_test![push_0xff; 0x6a, 0xff;
        |a: &mut Axecutor| {
            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000).unwrap();
            a.mem_init_zero(0x1000, 8).unwrap();
        };
        |a: Axecutor| {
            assert_eq!(a.reg_read_64(RSP.into()).unwrap(), 0x1000-8);
            assert_mem_value!(q; a; 0x1000; 0xffffffffffffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // push 0x7f
    ax_test![push_0x7f; 0x6a, 0x7f;
        |a: &mut Axecutor| {
            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000).unwrap();
            a.mem_init_zero(0x1000, 8).unwrap();
        };
        |a: Axecutor| {
            assert_eq!(a.reg_read_64(RSP.into()).unwrap(), 0x1000-8);
            assert_mem_value!(q; a; 0x1000; 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // push 0x0ffffff
    ax_test![push_0xfffffff; 0x68, 0xff, 0xff, 0xff, 0x0;
        |a: &mut Axecutor| {
            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000).unwrap();
            a.mem_init_zero(0x1000, 8).unwrap();
        };
        |a: Axecutor| {
            assert_eq!(a.reg_read_64(RSP.into()).unwrap(), 0x1000-8);
            assert_mem_value!(q; a; 0x1000; 0xffffff);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // push 0x7fffffff
    ax_test![push_0x7fffffff; 0x68, 0xff, 0xff, 0xff, 0x7f;
        |a: &mut Axecutor| {
            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000).unwrap();
            a.mem_init_zero(0x1000, 8).unwrap();
        };
        |a: Axecutor| {
            assert_eq!(a.reg_read_64(RSP.into()).unwrap(), 0x1000-8);
            assert_mem_value!(q; a; 0x1000; 0x7fffffff);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
