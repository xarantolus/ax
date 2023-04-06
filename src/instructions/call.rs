use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Call;
use iced_x86::OpKind;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;
use crate::helpers::operand::Operand;
use crate::state::registers::SupportedRegister::*;

macro_rules! push_rip {
    ($self:ident) => {{
        let rip = $self.reg_read_64(RIP)?;
        let rsp = $self.reg_read_64(RSP)?;
        $self.mem_write_64(rsp, rip)?;
        $self.reg_write_64(RSP, rsp - 8)?;
    }};
}

macro_rules! log_call {
    ($self:ident, $target:expr) => {{
        $crate::helpers::debug::debug_log!(
            "CALL function {}@{:#x}",
            $self
                .resolve_symbol($target)
                .unwrap_or_else(|| "<unknown>".to_string()),
            $target
        );
    }};
}

impl Axecutor {
    pub(crate) fn mnemonic_call(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Call);

        match i.code() {
            Call_ptr1616 => self.instr_call_ptr1616(i),
            Call_ptr1632 => self.instr_call_ptr1632(i),
            Call_rel16 => self.instr_call_rel16(i),
            Call_rel32_32 => self.instr_call_rel32_32(i),
            Call_rel32_64 => self.instr_call_rel32_64(i),
            Call_rm16 => self.instr_call_rm16(i),
            Call_rm32 => self.instr_call_rm32(i),
            Call_rm64 => self.instr_call_rm64(i),
            Call_m1616 => self.instr_call_m1616(i),
            Call_m1632 => self.instr_call_m1632(i),
            Call_m1664 => self.instr_call_m1664(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Call", i.code()),
        }
    }

    /// CALL ptr16:16
    ///
    /// o16 9A cd
    fn instr_call_ptr1616(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_ptr1616);

        opcode_unimplemented!("instr_call_ptr1616 for Call")
    }

    /// CALL ptr16:32
    ///
    /// o32 9A cp
    fn instr_call_ptr1632(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_ptr1632);

        opcode_unimplemented!("instr_call_ptr1632 for Call")
    }

    /// CALL rel16
    ///
    /// o16 E8 cw
    fn instr_call_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_rel16);

        opcode_unimplemented!("instr_call_rel16 for Call")
    }

    /// CALL rel32
    ///
    /// o32 E8 cd
    fn instr_call_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_rel32_32);

        opcode_unimplemented!("instr_call_rel32_32 for Call")
    }

    /// CALL rel32
    ///
    /// o64 E8 cd
    fn instr_call_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_rel32_64);

        match i.op0_kind() {
            OpKind::NearBranch64 => {
                let offset = i.near_branch64() as i64 as u64;
                push_rip!(self);
                self.trace_call(i, offset)?;
                self.reg_write_64(RIP, offset)?;
                self.state.call_stack.push(offset);
                log_call!(self, offset);
                Ok(())
            }
            _ => fatal_error!("Invalid op0_kind for CALL rel32: {:?}", i.op0_kind()),
        }
    }

    /// CALL r/m16
    ///
    /// o16 FF /2
    fn instr_call_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_rm16);

        opcode_unimplemented!("instr_call_rm16 for Call")
    }

    /// CALL r/m32
    ///
    /// o32 FF /2
    fn instr_call_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_rm32);

        opcode_unimplemented!("instr_call_rm32 for Call")
    }

    /// CALL r/m64
    ///
    /// o64 FF /2
    fn instr_call_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_rm64);

        let target = match self.instruction_operand(i, 0)? {
            Operand::Memory(m) => {
                let addr = self.mem_addr(m);
                self.mem_read_64(addr)?
            }
            Operand::Register(r) => self.reg_read_64(r)?,
            _ => fatal_error!("Invalid operand for CALL r/m64: {:?}", i.op0_kind()),
        };

        push_rip!(self);
        self.trace_call(i, target)?;
        self.reg_write_64(RIP, target)?;
        self.state.call_stack.push(target);
        log_call!(self, target);

        Ok(())
    }

    /// CALL m16:16
    ///
    /// o16 FF /3
    fn instr_call_m1616(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_m1616);

        opcode_unimplemented!("instr_call_m1616 for Call")
    }

    /// CALL m16:32
    ///
    /// o32 FF /3
    fn instr_call_m1632(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_m1632);

        opcode_unimplemented!("instr_call_m1632 for Call")
    }

    /// CALL m16:64
    ///
    /// o64 FF /3
    fn instr_call_m1664(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Call_m1664);

        opcode_unimplemented!("instr_call_m1664 for Call")
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{
        assert_mem_value, assert_reg_value, jmp_test, test_async, write_reg_value,
    };
    use crate::{axecutor::Axecutor, state::flags::*};
    use iced_x86::Register::*;

    test_async![test_call_rel32_64; async {
        let rip = 0x10000;

        let code = &[
            // mov rax, 0;
            0x48, 0xc7, 0xc0, 0x0, 0x0, 0x0, 0x0, // mov rbx, 0;
            0x48, 0xc7, 0xc3, 0x0, 0x0, 0x0, 0x0, // call myfunc
            0xe8, 0x7, 0x0, 0x0, 0x0, // mov rax, 42
            0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, // myfunc:mov rbx, 7
            0x48, 0xc7, 0xc3, 0x7, 0x0, 0x0, 0x0,
        ];
        let mut ax = Axecutor::new(code, rip, rip).expect("Failed to create axecutor");

        // Setup stack
        ax.reg_write_64(crate::state::registers::SupportedRegister::RSP, 0x1000 - 8).expect("Failed to write to register");
        ax.mem_init_zero(0x1000 - 8, 8)
            .expect("Failed to init memory");

        if let Err(e) = ax.execute().await {
            crate::helpers::macros::fatal_error!("Failed to execute: {:?}", e);
        }

        assert!(
            ax.state.rflags & (FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF) == 0,
            "Most flags should be clear"
        );

        assert_reg_value!(q; ax; RAX; 0x0);
        assert_reg_value!(q; ax; RBX; 0x7);
        assert_reg_value!(q; ax; RSP; 0x1000 - 16);
        // Did we reach the end?
        assert_reg_value!(q; ax; RIP; rip + code.len() as u64);
        // 19 is offset of mov rax, 42, the instruction after call
        assert_mem_value!(q; ax; 0x1000 - 8; rip + 19);
    }];

    jmp_test![jmp_lcall_func_mov_rax_42_ret_lcall_mov_rax_50_call_func_nop;
        start: 0x401010; end: 0x40d37a;
        0xe9, 0x58, 0xc3, 0x0, 0x0, 0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0xc3; // JMP Lcall; func: mov rax, 42; ret
        50000; // 50000 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x32, 0x0, 0x0, 0x0, 0xe8, 0x9c, 0x3c, 0xff, 0xff, 0x90; // Lcall: mov rax, 50; call func; nop
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x8000);
            a.mem_init_zero(0x8000, 8).expect("Failed to init memory");
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![jmp_lcall_func_mov_rax_42_ret_lcall_lea_rax_rip_func_call_rax_nop;
        start: 0x401010; end: 0x401056;
        0xeb, 0x3a, 0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0xc3; // JMP Lcall; func: mov rax, 42; ret
        50; // 50 bytes of 0x90 (nop) as padding
        0x48, 0x8d, 0x5, 0xbf, 0xff, 0xff, 0xff, 0xff, 0xd0, 0x90; // Lcall: lea rax, [rip+func]; call rax; nop
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x8000);
            a.mem_init_zero(0x8000, 8).expect("Failed to init memory");
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
