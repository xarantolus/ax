use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Neg;

use crate::axecutor::Axecutor;
use crate::helpers::errors::AxError;
use crate::helpers::macros::calculate_rm;

use crate::helpers::macros::fatal_error;

use crate::state::flags::*;

impl Axecutor {
    pub(crate) fn mnemonic_neg(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Neg);

        match i.code() {
            Neg_rm8 => self.instr_neg_rm8(i),
            Neg_rm16 => self.instr_neg_rm16(i),
            Neg_rm32 => self.instr_neg_rm32(i),
            Neg_rm64 => self.instr_neg_rm64(i),
            _ => fatal_error!("Invalid instruction code {:?} for mnemonic Neg", i.code()),
        }
    }

    /// NEG r/m8
    ///
    /// F6 /3
    fn instr_neg_rm8(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Neg_rm8);

        calculate_rm![u8f; self; i; |v| {
            let (r, _) = (!v).overflowing_add(1);
            (r, if v == 0 {0} else {FLAG_CF} | if r == 0x80 {FLAG_OF} else {0})
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// NEG r/m16
    ///
    /// o16 F7 /3
    fn instr_neg_rm16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Neg_rm16);

        calculate_rm![u16f; self; i; |v| {
            let (r, _) = (!v).overflowing_add(1);
            (r, if v == 0 {0} else {FLAG_CF} | if r == 0x8000 {FLAG_OF} else {0})
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// NEG r/m32
    ///
    /// o32 F7 /3
    fn instr_neg_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Neg_rm32);

        calculate_rm![u32f; self; i; |v| {
            let (r, _) = (!v).overflowing_add(1);
            (r, if v == 0 {0} else {FLAG_CF} | if r == 0x80000000 {FLAG_OF} else {0})
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }

    /// NEG r/m64
    ///
    /// o64 F7 /3
    fn instr_neg_rm64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Neg_rm64);

        calculate_rm![u64f; self; i; |v| {
            let (r, _) = (!v).overflowing_add(1);
            (r, if v == 0 {0} else {FLAG_CF} | if r == 0x8000000000000000 {FLAG_OF} else {0})
        }; (set: FLAG_PF | FLAG_ZF | FLAG_SF; clear: FLAG_CF | FLAG_OF)]
    }
}

#[cfg(test)]
mod tests {
    use crate::axecutor::Axecutor;
    use crate::helpers::tests::{assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // neg al
    ax_test![neg_al_cf; 0xf6, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0xff);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x1);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // neg al
    ax_test![neg_al_cf_pf; 0xf6, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x82);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x7e);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // neg al
    ax_test![neg_al_cf_pf_sf; 0xf6, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xff);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // neg al
    ax_test![neg_al_cf_sf; 0xf6, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0xf8);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // neg al
    ax_test![neg_al_cf_sf_of; 0xf6, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x80);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x80);
        };
        (FLAG_CF | FLAG_SF | FLAG_OF; FLAG_PF | FLAG_ZF)
    ];

    // neg al
    ax_test![neg_al_pf_zf; 0xf6, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(b; a; AL; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(b; a; AL; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // neg ax
    ax_test![neg_ax_cf_pf_sf; 0x66, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xffff);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // neg ax
    ax_test![neg_ax_cf_pf_sf_of; 0x66, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8000);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x8000);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // neg ax
    ax_test![neg_ax_cf_sf; 0x66, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0xfff8);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // neg ax
    ax_test![neg_ax_pf_zf; 0x66, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(w; a; AX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(w; a; AX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // neg eax
    ax_test![neg_eax_cf_pf_sf; 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xffffffffu32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // neg eax
    ax_test![neg_eax_cf_pf_sf_of; 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x80000000u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x80000000u32);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // neg eax
    ax_test![neg_eax_cf_sf; 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xfffffff8u32);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // neg eax
    ax_test![neg_eax_pf_zf; 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // neg rax
    ax_test![neg_rax_cf; 0x48, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8e1c32d02215d208u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x71e3cd2fddea2df8u64);
        };
        (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // neg rax
    ax_test![neg_rax_cf_pf; 0x48, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xd8dae173d0ff9270u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x27251e8c2f006d90u64);
        };
        (FLAG_CF | FLAG_PF; FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // neg rax
    ax_test![neg_rax_cf_pf_sf; 0x48, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xffffffffffffffffu64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF; FLAG_ZF | FLAG_OF)
    ];

    // neg rax
    ax_test![neg_rax_cf_pf_sf_of; 0x48, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000000000000u64);
        };
        (FLAG_CF | FLAG_PF | FLAG_SF | FLAG_OF; FLAG_ZF)
    ];

    // neg rax
    ax_test![neg_rax_cf_sf; 0x48, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xfffffffffffffff8u64);
        };
        (FLAG_CF | FLAG_SF; FLAG_PF | FLAG_ZF | FLAG_OF)
    ];

    // neg rax
    ax_test![neg_rax_pf_zf; 0x48, 0xf7, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
