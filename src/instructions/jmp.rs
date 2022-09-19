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

        match i.op0_kind() {
            OpKind::NearBranch64 => {
                let offset = i.near_branch64() as i64 as u64;
                self.reg_write_64(RIP.into(), offset);
                Ok(())
            }
            _ => panic!("Invalid op0_kind for JMP rel32: {:?}", i.op0_kind()),
        }
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

        // imm8 sign-extended
        let offset = i.immediate8() as i8 as u64;
        let rip = self.reg_read_64(RIP.into()) as i64 as u64;
        self.reg_write_64(RIP.into(), rip + offset);
        Ok(())
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
                self.reg_write_64(RIP.into(), offset);
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

macro_rules! code_with_nops {
    ($($bytes:expr),*; $count:expr; $($bytes2:expr),*) => {
        {
            // Concatenate bytes, then add count times 0x90 (nop), then the rest of bytes 2
            let mut bytes = vec![$($bytes),*];
            bytes.extend(vec![0x90; $count]);
            bytes.extend(vec![$($bytes2),*]);
            bytes
        }
    };
}

macro_rules! jmp_test {
    [$name:ident; start: $initial_rip:expr; end: $final_rip:expr; $($bytes_start:expr),*; $count:expr; $($bytes_end:expr),*; ($flags_to_set:expr; $flags_not_to_set:expr)] => {
        jmp_test![$name; start: $initial_rip; end: $final_rip;
        $($bytes_start),*; $count; $($bytes_end),*;
        |_ax| {}; |_ax| {};
        ($flags_to_set; $flags_not_to_set)];
    };
    [$name:ident; start: $initial_rip:expr; end: $final_rip:expr; $($bytes_start:expr),*; $count:expr; $($bytes_end:expr),*; $setup:expr; $asserts:expr; ($flags_to_set:expr; $flags_not_to_set:expr)] => {
        #[test]
        fn $name() {
			let bytes = code_with_nops!($($bytes_start),*; $count; $($bytes_end),*);

            let mut ax = Axecutor::new(&bytes, $initial_rip).expect("Failed to create axecutor");
            $setup(&mut ax);

            assert_reg_value!(q; ax; RIP; $initial_rip);

            ax.execute().expect("Could not execute code snippet");

            assert_reg_value!(q; ax; RIP; $final_rip);

            let flags = ax.state.rflags;

            $asserts(ax);

            // Check flags
            use crate::instructions::flags::*;
            for flag in FLAG_LIST {
                // If the flag should be set, it must be != 0
                if $flags_to_set & flag != 0 {
                    assert!(flags & flag != 0, "FLAG_{} should be set, but wasn't", FLAG_TO_NAMES.get(&flag).expect("Flag not found"));
                }

                if $flags_not_to_set & flag != 0 {
                    assert!(flags & flag == 0, "FLAG_{} should not be set, but was", FLAG_TO_NAMES.get(&flag).expect("Flag not found"));
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::RegisterWrapper, write_reg_value,
    };
    use iced_x86::Register::*;

    // mov rax, 5; JMP .Llabel; xor rax, rax; .Llabel:
    ax_test![mov_rax_5_jmp_label_xor_rax_rax_label;
        0x48, 0xc7, 0xc0, 0x5, 0x0, 0x0, 0x0, 0xeb, 0x3, 0x48, 0x31, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xee1e96ff08a61c35u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x5);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    jmp_test![mov_rax_5_jmp_label_sub_rax_5_label_add_rax_1_pf;
        start: 0x401010; end: 0x401029;
        0x48, 0xc7, 0xc0, 0x5, 0x0, 0x0, 0x0, 0xeb, 0xc; // mov rax, 5; jmp .Label
        8; // 8 bytes of 0x90 (nop) as padding
        0x48, 0x83, 0xe8, 0x5, 0x48, 0x83, 0xc0, 0x1; // sub rax, 5; .Label: add rax, 1
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xee1e96ff08a61c35u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x6);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    jmp_test![mov_rax_5_jmp_label_label_sub_rax_1;
        start: 0x401010; end: 0x40104f;
        0x48, 0xc7, 0xc0, 0x5, 0x0, 0x0, 0x0, 0xeb, 0x32; // mov rax, 5; JMP .label
        50; // 50 bytes of 0x90 (nop) as padding
        0x48, 0x83, 0xe8, 0x1; // .label: sub rax, 1
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];
}
