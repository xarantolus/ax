use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Ret;

use super::axecutor::Axecutor;
use super::errors::AxError;

use crate::instructions::registers::SupportedRegister::*;
use crate::{fatal_error, opcode_unimplemented};

macro_rules! pop_rip {
    ($self:ident) => {{
        let rsp = $self.reg_read_64(RSP) + 8;
        if rsp == $self.stack_top {
            return Err(AxError::from("Cannot pop from empty stack").end_execution());
        }
        let rip = $self.mem_read_64(rsp)?;
        $self.reg_write_64(RIP, rip);
        $self.reg_write_64(RSP, rsp);
    }};
}

impl Axecutor {
    pub fn mnemonic_ret(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Ret);

        match i.code() {
            Retnw_imm16 => self.instr_retnw_imm16(i),
            Retnd_imm16 => self.instr_retnd_imm16(i),
            Retnq_imm16 => self.instr_retnq_imm16(i),
            Retnw => self.instr_retnw(i),
            Retnd => self.instr_retnd(i),
            Retnq => self.instr_retnq(i),
            Retfw_imm16 => self.instr_retfw_imm16(i),
            Retfd_imm16 => self.instr_retfd_imm16(i),
            Retfq_imm16 => self.instr_retfq_imm16(i),
            Retfw => self.instr_retfw(i),
            Retfd => self.instr_retfd(i),
            Retfq => self.instr_retfq(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Ret", i.code()),
        }
    }

    /// RET imm16
    ///
    /// o16 C2 iw
    fn instr_retnw_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retnw_imm16);

        opcode_unimplemented!("instr_retnw_imm16 for Ret")
    }

    /// RET imm16
    ///
    /// o32 C2 iw
    fn instr_retnd_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retnd_imm16);

        opcode_unimplemented!("instr_retnd_imm16 for Ret")
    }

    /// RET imm16
    ///
    /// o64 C2 iw
    fn instr_retnq_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retnq_imm16);

        opcode_unimplemented!("instr_retnq_imm16 for Ret")
    }

    /// RET
    ///
    /// o16 C3
    fn instr_retnw(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retnw);

        opcode_unimplemented!("instr_retnw for Ret")
    }

    /// RET
    ///
    /// o32 C3
    fn instr_retnd(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retnd);

        opcode_unimplemented!("instr_retnd for Ret")
    }

    /// RET
    ///
    /// o64 C3
    fn instr_retnq(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retnq);

        pop_rip!(self);

        Ok(())
    }

    /// RETF imm16
    ///
    /// o16 CA iw
    fn instr_retfw_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retfw_imm16);

        opcode_unimplemented!("instr_retfw_imm16 for Ret")
    }

    /// RETF imm16
    ///
    /// o32 CA iw
    fn instr_retfd_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retfd_imm16);

        opcode_unimplemented!("instr_retfd_imm16 for Ret")
    }

    /// RETF imm16
    ///
    /// o64 CA iw
    fn instr_retfq_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retfq_imm16);

        opcode_unimplemented!("instr_retfq_imm16 for Ret")
    }

    /// RETF
    ///
    /// o16 CB
    fn instr_retfw(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retfw);

        opcode_unimplemented!("instr_retfw for Ret")
    }

    /// RETF
    ///
    /// o32 CB
    fn instr_retfd(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retfd);

        opcode_unimplemented!("instr_retfd for Ret")
    }

    /// RETF
    ///
    /// o64 CB
    fn instr_retfq(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Retfq);

        opcode_unimplemented!("instr_retfq for Ret")
    }
}

#[cfg(test)]
mod tests {

    use crate::jmp_test;

    // The same testcase is available for the call instruction
    jmp_test![jmp_lcall_func_mov_rax_42_ret_lcall_mov_rax_50_call_func_nop_ret;
        start: 0x401010; end: 0x40d37a;
        0xe9, 0x58, 0xc3, 0x0, 0x0, 0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0xc3; // JMP Lcall; func: mov rax, 42; ret
        50000; // 50000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x32, 0x0, 0x0, 0x0, 0xe8, 0x9c, 0x3c, 0xff, 0xff, 0x90; // Lcall: mov rax, 50; call func; nop
        |a: &mut Axecutor| {
            a.reg_write_64(RSP.into(), 0x8000);
            a.mem_init_zero(0x8000, 8).expect("Failed to init memory");
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
