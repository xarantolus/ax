use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Mov;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::calculate_rm_imm;
use crate::state::flags::FLAGS_UNAFFECTED;

use crate::helpers::macros::calculate_r_rm;
use crate::helpers::macros::calculate_rm_r;
use crate::helpers::macros::fatal_error;
use crate::helpers::macros::opcode_unimplemented;

impl Axecutor {
    pub fn mnemonic_mov(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Mov);

        match i.code() {
            Mov_rm8_r8 => self.instr_mov_rm8_r8(i),
            Mov_rm16_r16 => self.instr_mov_rm16_r16(i),
            Mov_rm32_r32 => self.instr_mov_rm32_r32(i),
            Mov_rm64_r64 => self.instr_mov_rm64_r64(i),
            Mov_r8_rm8 => self.instr_mov_r8_rm8(i),
            Mov_r16_rm16 => self.instr_mov_r16_rm16(i),
            Mov_r32_rm32 => self.instr_mov_r32_rm32(i),
            Mov_r64_rm64 => self.instr_mov_r64_rm64(i),
            Mov_rm16_Sreg => self.instr_mov_rm16_sreg(i),
            Mov_r32m16_Sreg => self.instr_mov_r32m16_sreg(i),
            Mov_r64m16_Sreg => self.instr_mov_r64m16_sreg(i),
            Mov_Sreg_rm16 => self.instr_mov_sreg_rm16(i),
            Mov_Sreg_r32m16 => self.instr_mov_sreg_r32m16(i),
            Mov_Sreg_r64m16 => self.instr_mov_sreg_r64m16(i),
            Mov_AL_moffs8 => self.instr_mov_al_moffs8(i),
            Mov_AX_moffs16 => self.instr_mov_ax_moffs16(i),
            Mov_EAX_moffs32 => self.instr_mov_eax_moffs32(i),
            Mov_RAX_moffs64 => self.instr_mov_rax_moffs64(i),
            Mov_moffs8_AL => self.instr_mov_moffs8_al(i),
            Mov_moffs16_AX => self.instr_mov_moffs16_ax(i),
            Mov_moffs32_EAX => self.instr_mov_moffs32_eax(i),
            Mov_moffs64_RAX => self.instr_mov_moffs64_rax(i),
            Mov_r8_imm8 => self.instr_mov_r8_imm8(i),
            Mov_r16_imm16 => self.instr_mov_r16_imm16(i),
            Mov_r32_imm32 => self.instr_mov_r32_imm32(i),
            Mov_r64_imm64 => self.instr_mov_r64_imm64(i),
            Mov_rm8_imm8 => self.instr_mov_rm8_imm8(i),
            Mov_rm16_imm16 => self.instr_mov_rm16_imm16(i),
            Mov_rm32_imm32 => self.instr_mov_rm32_imm32(i),
            Mov_rm64_imm32 => self.instr_mov_rm64_imm32(i),
            Mov_r32_cr => self.instr_mov_r32_cr(i),
            Mov_r64_cr => self.instr_mov_r64_cr(i),
            Mov_r32_dr => self.instr_mov_r32_dr(i),
            Mov_r64_dr => self.instr_mov_r64_dr(i),
            Mov_cr_r32 => self.instr_mov_cr_r32(i),
            Mov_cr_r64 => self.instr_mov_cr_r64(i),
            Mov_dr_r32 => self.instr_mov_dr_r32(i),
            Mov_dr_r64 => self.instr_mov_dr_r64(i),
            Mov_r32_tr => self.instr_mov_r32_tr(i),
            Mov_tr_r32 => self.instr_mov_tr_r32(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Mov", i.code()),
        }
    }

    /// MOV r/m8, r8
    ///
    /// 88 /r
    fn instr_mov_rm8_r8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_rm8_r8);

        calculate_rm_r![u8; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r/m16, r16
    ///
    /// o16 89 /r
    fn instr_mov_rm16_r16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_rm16_r16);

        calculate_rm_r![u16; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r/m32, r32
    ///
    /// o32 89 /r
    fn instr_mov_rm32_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_rm32_r32);

        calculate_rm_r![u32; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r/m64, r64
    ///
    /// o64 89 /r
    fn instr_mov_rm64_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_rm64_r64);

        calculate_rm_r![u64; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r8, r/m8
    ///
    /// 8A /r
    fn instr_mov_r8_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r8_rm8);

        calculate_r_rm![u8; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r16, r/m16
    ///
    /// o16 8B /r
    fn instr_mov_r16_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r16_rm16);

        calculate_r_rm![u16; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r32, r/m32
    ///
    /// o32 8B /r
    fn instr_mov_r32_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r32_rm32);

        calculate_r_rm![u32; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r64, r/m64
    ///
    /// o64 8B /r
    fn instr_mov_r64_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r64_rm64);

        calculate_r_rm![u64; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r/m16, Sreg
    ///
    /// o16 8C /r
    fn instr_mov_rm16_sreg(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_rm16_Sreg);

        opcode_unimplemented!("instr_mov_rm16_sreg for Mov")
    }

    /// MOV r32/m16, Sreg
    ///
    /// o32 8C /r
    fn instr_mov_r32m16_sreg(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r32m16_Sreg);

        opcode_unimplemented!("instr_mov_r32m16_sreg for Mov")
    }

    /// MOV r64/m16, Sreg
    ///
    /// o64 8C /r
    fn instr_mov_r64m16_sreg(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r64m16_Sreg);

        opcode_unimplemented!("instr_mov_r64m16_sreg for Mov")
    }

    /// MOV Sreg, r/m16
    ///
    /// o16 8E /r
    fn instr_mov_sreg_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_Sreg_rm16);

        opcode_unimplemented!("instr_mov_sreg_rm16 for Mov")
    }

    /// MOV Sreg, r32/m16
    ///
    /// o32 8E /r
    fn instr_mov_sreg_r32m16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_Sreg_r32m16);

        opcode_unimplemented!("instr_mov_sreg_r32m16 for Mov")
    }

    /// MOV Sreg, r64/m16
    ///
    /// o64 8E /r
    fn instr_mov_sreg_r64m16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_Sreg_r64m16);

        opcode_unimplemented!("instr_mov_sreg_r64m16 for Mov")
    }

    /// MOV AL, moffs8
    ///
    /// A0 mo
    fn instr_mov_al_moffs8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_AL_moffs8);

        calculate_rm_r![u8; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV AX, moffs16
    ///
    /// o16 A1 mo
    fn instr_mov_ax_moffs16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_AX_moffs16);

        calculate_rm_r![u16; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV EAX, moffs32
    ///
    /// o32 A1 mo
    fn instr_mov_eax_moffs32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_EAX_moffs32);

        calculate_rm_r![u32; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV RAX, moffs64
    ///
    /// o64 A1 mo
    fn instr_mov_rax_moffs64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_RAX_moffs64);

        calculate_rm_r![u64; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV moffs8, AL
    ///
    /// A2 mo
    fn instr_mov_moffs8_al(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_moffs8_AL);

        calculate_r_rm![u8; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV moffs16, AX
    ///
    /// o16 A3 mo
    fn instr_mov_moffs16_ax(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_moffs16_AX);

        calculate_r_rm![u16; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV moffs32, EAX
    ///
    /// o32 A3 mo
    fn instr_mov_moffs32_eax(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_moffs32_EAX);

        calculate_r_rm![u32; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV moffs64, RAX
    ///
    /// o64 A3 mo
    fn instr_mov_moffs64_rax(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_moffs64_RAX);

        calculate_r_rm![u64; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r8, imm8
    ///
    /// B0+rb ib
    fn instr_mov_r8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r8_imm8);

        calculate_rm_imm![u8; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r16, imm16
    ///
    /// o16 B8+rw iw
    fn instr_mov_r16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r16_imm16);

        calculate_rm_imm![u16; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r32, imm32
    ///
    /// o32 B8+rd id
    fn instr_mov_r32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r32_imm32);

        calculate_rm_imm![u32; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r64, imm64
    ///
    /// o64 B8+ro io
    fn instr_mov_r64_imm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r64_imm64);

        calculate_rm_imm![u64; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r/m8, imm8
    ///
    /// C6 /0 ib
    fn instr_mov_rm8_imm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_rm8_imm8);

        calculate_rm_imm![u8; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r/m16, imm16
    ///
    /// o16 C7 /0 iw
    fn instr_mov_rm16_imm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_rm16_imm16);

        calculate_rm_imm![u16; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r/m32, imm32
    ///
    /// o32 C7 /0 id
    fn instr_mov_rm32_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_rm32_imm32);

        calculate_rm_imm![u32; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r/m64, imm32
    ///
    /// o64 C7 /0 id
    fn instr_mov_rm64_imm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_rm64_imm32);

        // this sign-extends automatically
        calculate_rm_imm![u64; self; i; |_, s| {
            s
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOV r32, cr
    ///
    /// 0F 20 /r
    fn instr_mov_r32_cr(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r32_cr);

        opcode_unimplemented!("instr_mov_r32_cr for Mov")
    }

    /// MOV r64, cr
    ///
    /// 0F 20 /r
    fn instr_mov_r64_cr(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r64_cr);

        opcode_unimplemented!("instr_mov_r64_cr for Mov")
    }

    /// MOV r32, dr
    ///
    /// 0F 21 /r
    fn instr_mov_r32_dr(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r32_dr);

        opcode_unimplemented!("instr_mov_r32_dr for Mov")
    }

    /// MOV r64, dr
    ///
    /// 0F 21 /r
    fn instr_mov_r64_dr(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r64_dr);

        opcode_unimplemented!("instr_mov_r64_dr for Mov")
    }

    /// MOV cr, r32
    ///
    /// 0F 22 /r
    fn instr_mov_cr_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_cr_r32);

        opcode_unimplemented!("instr_mov_cr_r32 for Mov")
    }

    /// MOV cr, r64
    ///
    /// 0F 22 /r
    fn instr_mov_cr_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_cr_r64);

        opcode_unimplemented!("instr_mov_cr_r64 for Mov")
    }

    /// MOV dr, r32
    ///
    /// 0F 23 /r
    fn instr_mov_dr_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_dr_r32);

        opcode_unimplemented!("instr_mov_dr_r32 for Mov")
    }

    /// MOV dr, r64
    ///
    /// 0F 23 /r
    fn instr_mov_dr_r64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_dr_r64);

        opcode_unimplemented!("instr_mov_dr_r64 for Mov")
    }

    /// MOV r32, tr
    ///
    /// 0F 24 /r
    fn instr_mov_r32_tr(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_r32_tr);

        opcode_unimplemented!("instr_mov_r32_tr for Mov")
    }

    /// MOV tr, r32
    ///
    /// 0F 26 /r
    fn instr_mov_tr_r32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Mov_tr_r32);

        opcode_unimplemented!("instr_mov_tr_r32 for Mov")
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_mem_value, assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // mov byte ptr [rsp+8], bl
    ax_test![mov_byte_ptr_rsp8_bl; 0x88, 0x5c, 0x24, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x12);
            // Create a small stack
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_8(0x1008, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x12);
            assert_mem_value!(b; a; 0x1008; 0x12);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov word ptr [rsp+8], r11w
    ax_test![mov_word_ptr_rsp8_r11w; 0x66, 0x44, 0x89, 0x5c, 0x24, 0x8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; R11W; 0x1234);
            // Create a small stack
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_16(0x1008, 0xffff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; R11W; 0x1234);
            assert_mem_value!(w; a; 0x1008; 0x1234);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov dword ptr [rsp], edx
    ax_test![mov_dword_ptr_rsp_edx; 0x89, 0x14, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EDX; 0x12345678);
            // Create a small stack
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_32(0x1000, 0xffffffff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EDX; 0x12345678);
            assert_mem_value!(d; a; 0x1000; 0x12345678);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov qword ptr [rsp], rcx
    ax_test![mov_qword_ptr_rsp_rcx; 0x48, 0x89, 0xc, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RCX; 0x123456789abcdef0u64);
            // Create a small stack
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_64(0x1000, 0xffffffffffffffff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x123456789abcdef0u64);
            assert_mem_value!(q; a; 0x1000; 0x123456789abcdef0u64);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov r12b, r11b
    ax_test![mov_r12b_r11b; 0x45, 0x88, 0xdc;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; R11L; 0x12);
            write_reg_value!(b; a; R12L; 0x34);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; R11L; 0x12);
            assert_reg_value!(b; a; R12L; 0x12);
        };
        (0; FLAGS_UNAFFECTED)
    ];
    // mov al, bl
    ax_test![mov_al_bl; 0x88, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x34);
            write_reg_value!(b; a; BL; 0x12);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x12);
            assert_reg_value!(b; a; BL; 0x12);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov bl, [rsp]
    ax_test![mov_bl_rsp; 0x8a, 0x1c, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x34);
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_8(0x1000, 0x12).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x12);
            assert_mem_value!(b; a; 0x1000; 0x12);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov cx, [rsp]
    ax_test![mov_cx_rsp; 0x66, 0x8b, 0xc, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; CX; 0x1234);
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_16(0x1000, 0x5678).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; CX; 0x5678);
            assert_mem_value!(w; a; 0x1000; 0x5678);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov ecx, [rsp]
    ax_test![mov_ecx_rsp; 0x8b, 0xc, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; ECX; 0x12345678);
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_32(0x1000, 0x9abcdef0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; ECX; 0x9abcdef0u32);
            assert_mem_value!(d; a; 0x1000; 0x9abcdef0u32);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov r11, [rsp]
    ax_test![mov_r11_rsp; 0x4c, 0x8b, 0x1c, 0x24;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x120u64);
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
            a.mem_write_64(0x1000, 0x123456789abcdef0u64).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x123456789abcdef0u64);
            assert_mem_value!(q; a; 0x1000; 0x123456789abcdef0u64);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov bl, 5
    ax_test![mov_bl_5; 0xb3, 0x5;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x34);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x5);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov bx, 50
    ax_test![mov_bx_50; 0x66, 0xbb, 0x32, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; BX; 0x1234);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; BX; 0x32);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov ebx, 5000
    ax_test![mov_ebx_5000; 0xbb, 0x88, 0x13, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x12345678);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x1388);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov r11, 0x15ffffffff
    ax_test![mov_r11_0x15ffffffff; 0x49, 0xbb, 0xff, 0xff, 0xff, 0xff, 0x15, 0x0, 0x0, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x120u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; R11; 0x15ffffffffu64);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov bl, 0x15
    ax_test![mov_bl_0x15; 0xb3, 0x15;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; BL; 0x34);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; BL; 0x15);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov byte ptr [rsp], 0x15
    ax_test![mov_byte_ptr_rsp_0x15; 0xc6, 0x4, 0x24, 0x15;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
        };
        |a: Axecutor| {
            assert_mem_value!(b; a; 0x1000; 0x15);
        };
        (0; FLAGS_UNAFFECTED)
    ];
    // mov word ptr [rsp], 0x15
    ax_test![mov_word_ptr_rsp_0x15; 0x66, 0xc7, 0x4, 0x24, 0x15, 0x0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
        };
        |a: Axecutor| {
            assert_mem_value!(w; a; 0x1000; 0x15);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov ebx, 0x31535135
    ax_test![mov_ebx_0x31535135; 0xbb, 0x35, 0x51, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EBX; 0x12345678);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EBX; 0x31535135);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov dword ptr [rsp], 0x31535135
    ax_test![mov_dword_ptr_rsp_0x31535135; 0xc7, 0x4, 0x24, 0x35, 0x51, 0x53, 0x31;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
        };
        |a: Axecutor| {
            assert_mem_value!(d; a; 0x1000; 0x31535135);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov qword ptr [rsp], 0xfffffffffffff135
    ax_test![mov_qword_ptr_rsp_0xfffffffffffff135_sign_extension; 0x48, 0xc7, 0x4, 0x24, 0x35, 0xf1, 0xff, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x1000);
            a.mem_init_zero(0x1000, 256).unwrap();
        };
        |a: Axecutor| {
            assert_mem_value!(q; a; 0x1000; 0xfffffffffffff135u64);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov al, BYTE PTR ds:0x10
    ax_test![mov_al_ds_moffs; 0x8a, 0x4, 0x25, 0x10, 0, 0, 0;
        |a: &mut Axecutor| {
            a.mem_init_zero(0x10, 1).unwrap();
            a.mem_write_8(0x10, 0x15).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x15);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov al, BYTE PTR fs:0x10
    ax_test![mov_al_fs_moffs; 0x64, 0x8a, 0x4, 0x25, 0x10, 0, 0, 0;
        |a: &mut Axecutor| {
            a.write_fs(0x1000);
            a.mem_init_zero(0x1010, 1).unwrap();
            a.mem_write_8(0x1010, 0x15).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x15);
        };
        (0; FLAGS_UNAFFECTED)
    ];

    // mov al, BYTE PTR gs:0x10
    ax_test![mov_al_gs_moffs; 0x65, 0x8a, 0x4, 0x25, 0x10, 0, 0, 0;
        |a: &mut Axecutor| {
            a.write_gs(0x1000);
            a.mem_init_zero(0x1010, 1).unwrap();
            a.mem_write_8(0x1010, 0x15).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x15);
        };
        (0; FLAGS_UNAFFECTED)
    ];
}
