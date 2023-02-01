use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Movzx;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;

use crate::helpers::macros::calculate_r_rm;
use crate::helpers::macros::calculate_rm_r;
use crate::helpers::macros::fatal_error;
use crate::state::flags::*;

impl Axecutor {
    pub(crate) fn mnemonic_movzx(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Movzx);

        match i.code() {
            Movzx_r16_rm8 => self.instr_movzx_r16_rm8(i),
            Movzx_r32_rm8 => self.instr_movzx_r32_rm8(i),
            Movzx_r64_rm8 => self.instr_movzx_r64_rm8(i),
            Movzx_r32_rm16 => self.instr_movzx_r32_rm16(i),
            Movzx_r64_rm16 => self.instr_movzx_r64_rm16(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Movzx", i.code()),
        }
    }

    /// MOVZX r16, r/m8
    ///
    /// o16 0F B6 /r
    fn instr_movzx_r16_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movzx_r16_rm8);

        calculate_rm_r![u16f; u8; self; i; |_, s| {
            (s as u16, 0)
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOVZX r32, r/m8
    ///
    /// o32 0F B6 /r
    fn instr_movzx_r32_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movzx_r32_rm8);

        calculate_rm_r![u32f; u8; self; i; |_, s| {
            (s as u32, 0)
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOVZX r64, r/m8
    ///
    /// o64 0F B6 /r
    fn instr_movzx_r64_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movzx_r64_rm8);

        calculate_rm_r![u64f; u8; self; i; |_, s| {
            (s as u64, 0)
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOVZX r32, r/m16
    ///
    /// o32 0F B7 /r
    fn instr_movzx_r32_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movzx_r32_rm16);

        calculate_r_rm![u32; u16; self; i; |_, s| {
            s as u32
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }

    /// MOVZX r64, r/m16
    ///
    /// o64 0F B7 /r
    fn instr_movzx_r64_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movzx_r64_rm16);

        calculate_r_rm![u64; u16; self; i; |_, s| {
            s as u64
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // movzx ax, bl
    ax_test![movzx_ax_bl; 0x66, 0xf, 0xb6, 0xc3; |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
            write_reg_value!(b; a; BL; 0x2);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x2);
            assert_reg_value!(b; a; BL; 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movzx eax, bl
    ax_test![movzx_eax_bl_51_30; 0xf, 0xb6, 0xc3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x400);
            write_reg_value!(b; a; BL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80);
            assert_reg_value!(b; a; BL; 0x80);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movzx rax, bl
    ax_test![movzx_rax_bl_21_27; 0x48, 0xf, 0xb6, 0xc3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x20000000000u64);
            write_reg_value!(b; a; BL; 0x41);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x41);
            assert_reg_value!(b; a; BL; 0x41);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movzx eax, bx
    ax_test![movzx_eax_bx; 0xf, 0xb7, 0xc3; |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
            write_reg_value!(w; a; BX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x1);
            assert_reg_value!(w; a; BX; 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movzx rax, bx
    ax_test![movzx_rax_bx; 0x48, 0xf, 0xb7, 0xc3; |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(w; a; BX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_reg_value!(w; a; BX; 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
