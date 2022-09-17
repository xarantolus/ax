use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Jmp;
use iced_x86::OpKind;
use iced_x86::Register::*;

use super::axecutor::Axecutor;
use super::errors::AxError;

impl Axecutor {
    pub fn mnemonic_jmp(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Jmp);

        match i.code() {
            Jmp_rel16 => self.instr_jmp_rel16(i),
            Jmp_rel32_32 => self.instr_jmp_rel32_32(i),
            Jmp_rel32_64 => self.instr_jmp_rel32_64(i),
            Jmp_ptr1616 => self.instr_jmp_ptr1616(i),
            Jmp_ptr1632 => self.instr_jmp_ptr1632(i),
            Jmp_rel8_16 => self.instr_jmp_rel8_16(i),
            Jmp_rel8_32 => self.instr_jmp_rel8_32(i),
            Jmp_rel8_64 => self.instr_jmp_rel8_64(i),
            Jmp_rm16 => self.instr_jmp_rm16(i),
            Jmp_rm32 => self.instr_jmp_rm32(i),
            Jmp_rm64 => self.instr_jmp_rm64(i),
            Jmp_m1616 => self.instr_jmp_m1616(i),
            Jmp_m1632 => self.instr_jmp_m1632(i),
            Jmp_m1664 => self.instr_jmp_m1664(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Jmp", i.code()),
        }
    }

    /// JMP rel16
    ///
    /// o16 E9 cw
    fn instr_jmp_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_rel16);

        todo!("instr_jmp_rel16 for Jmp")
    }

    /// JMP rel32
    ///
    /// o32 E9 cd
    fn instr_jmp_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_rel32_32);

        todo!("instr_jmp_rel32_32 for Jmp")
    }

    /// JMP rel32
    ///
    /// o64 E9 cd
    fn instr_jmp_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_rel32_64);

        todo!("instr_jmp_rel32_64 for Jmp")
    }

    /// JMP ptr16:16
    ///
    /// o16 EA cd
    fn instr_jmp_ptr1616(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_ptr1616);

        todo!("instr_jmp_ptr1616 for Jmp")
    }

    /// JMP ptr16:32
    ///
    /// o32 EA cp
    fn instr_jmp_ptr1632(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_ptr1632);

        todo!("instr_jmp_ptr1632 for Jmp")
    }

    /// JMP rel8
    ///
    /// o16 EB cb
    fn instr_jmp_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_rel8_16);

        todo!("instr_jmp_rel8_16 for Jmp")
    }

    /// JMP rel8
    ///
    /// o32 EB cb
    fn instr_jmp_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_rel8_32);

        todo!("instr_jmp_rel8_32 for Jmp")
    }

    /// JMP rel8
    ///
    /// o64 EB cb
    fn instr_jmp_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_rel8_64);

        match i.op0_kind() {
            OpKind::NearBranch64 => {
                let offset = i.near_branch64() as i64 as u64;
                let new_ip = self.reg_read_64(RIP.into()).wrapping_add(offset);
                self.reg_write_64(RIP.into(), new_ip);

                Ok(())
            }
            _ => panic!("Invalid op0_kind {:?} for JMP rel8", i.op0_kind()),
        }
    }

    /// JMP r/m16
    ///
    /// o16 FF /4
    fn instr_jmp_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_rm16);

        todo!("instr_jmp_rm16 for Jmp")
    }

    /// JMP r/m32
    ///
    /// o32 FF /4
    fn instr_jmp_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_rm32);

        todo!("instr_jmp_rm32 for Jmp")
    }

    /// JMP r/m64
    ///
    /// o64 FF /4
    fn instr_jmp_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_rm64);

        print!("JMP r/m64: {:#?}", i);

        todo!("instr_jmp_rm64 for Jmp")
    }

    /// JMP m16:16
    ///
    /// o16 FF /5
    fn instr_jmp_m1616(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_m1616);

        todo!("instr_jmp_m1616 for Jmp")
    }

    /// JMP m16:32
    ///
    /// o32 FF /5
    fn instr_jmp_m1632(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_m1632);

        todo!("instr_jmp_m1632 for Jmp")
    }

    /// JMP m16:64
    ///
    /// o64 FF /5
    fn instr_jmp_m1664(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jmp_m1664);

        todo!("instr_jmp_m1664 for Jmp")
    }
}

macro_rules! jmp_test {
    [$name:ident; $($bytes:expr),*; $initial_rip:expr; $final_rip:expr] => {
        #[test]
        fn $name() {
			let bytes = &[$($bytes),*];
            let mut ax = Axecutor::new(bytes, $initial_rip).expect("Failed to create axecutor");

            assert_eq!(ax.instructions.len(), 1, "Expected 1 jump instruction, but got {}", ax.instructions.len());
            assert_reg_value!(q; ax; RIP; $initial_rip);

            ax.step().expect("Could not execute a single jump instruction step");

            assert_reg_value!(q; ax; RIP; $final_rip);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_reg_value, instructions::registers::RegisterWrapper};
    use iced_x86::Register::*;

    /*
    // jmp main -- a symbol defined some bytes before the instruction
    jmp_test![jmp_rel8_64; 0xeb, 0xee; 0x1000; 0x0ff2];

    // jmp 0xff
    jmp_test![instr_jmp_rel32_64; 0xe9, 0x0, 0x0, 0x0, 0x0; 0x1000; 0x1005];
    */
}
