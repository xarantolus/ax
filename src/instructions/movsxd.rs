use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Movsxd;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::fatal_error;
use crate::instructions::flags::*;

use crate::calculate_r_rm;

impl Axecutor {
    pub fn mnemonic_movsxd(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Movsxd);

        match i.code() {
            Movsxd_r64_rm32 => self.instr_movsxd_r64_rm32(i),
            _ => fatal_error!(
                "Invalid instruction code {:?} for mnemonic Movsxd",
                i.code()
            ),
        }
    }

    /// MOVSXD r64, r/m32
    ///
    /// o64 63 /r
    fn instr_movsxd_r64_rm32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Movsxd_r64_rm32);

        calculate_r_rm![u64; u32; self; i; |_:u64, s:u32|{
            // Sign-extend d to 64 bits
            s as i32 as i64 as u64
        }; (set: FLAGS_UNAFFECTED; clear: 0)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_reg_value, ax_test, write_reg_value};
    use iced_x86::Register::*;

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_24_38; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x21);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000000u64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xffffffff80000000u64);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x80000000u64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_37_73; 0x48, 0x63, 0x3;
    |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x100000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x41).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x41);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x41);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_72_63; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x400000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x100).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x100);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x100);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_62_55; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x2000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x1);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_17_24; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7f);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x1000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_63_76; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x40000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x40000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x40000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_53_91; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1f);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x4).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_20_75; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x4000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_28_58; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x80);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x1000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_41_90_76; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x57ad080e4e3ed24cu64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x4).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x4);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_90_38; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x40);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x8000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_56_32_9; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x800000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x800000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x800000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x800000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_34_51; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x4000000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4000000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x4000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_53_59; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10000000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7f).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7f);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x7f);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_20_13; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x80000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x21).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x21);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x21);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_95_13; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7fffffffu64);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x7fffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_16_16; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x2).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_57_58; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x20000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x80000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x80000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x80000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_33_49; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffu64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x21).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x21);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x21);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_1_67; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x200).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x200);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x200);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_63_73; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x8);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_51_79; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x400000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x100000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x100000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x100000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_38_17; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7fffffffu64);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x7fffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_14_46; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0xff).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xff);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0xff);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_94_69_88_4; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x200000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x200000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x200000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_42_13; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x20000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7fffffffu64);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x7fffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_100_81; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x400000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x1000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_86_87_48; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8000000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8000000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x8000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_67_68; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x200).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x200);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x200);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_93_14; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x40);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x41).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x41);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x41);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_28_59; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x41);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7fffffffu64);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x7fffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_79_43_69; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x2000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x40).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x40);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x40);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_3_84_20; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x7fffffffu64).unwrap()
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x7fffffffu64);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x7fffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_22_56; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x10000000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x10000000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x10000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_27_48; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x7fffffffffffffffu64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x100000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x100000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x100000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_29_35_63; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x100000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x20000000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x20000000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x20000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_15_49; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x100000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x100).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x100);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x100);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_6_23; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x10);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x20000000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x20000000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x20000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_72_74; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x21).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x21);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x21);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_85; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x0);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x400000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x400000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x400000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_58_46; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x41).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x41);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x41);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_83_97; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xff);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x1000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x1000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_49_91; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x100000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x8);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x8);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_86_56; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x21);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x2).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x2);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_14_36; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x8000000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x20000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x20000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x20000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_21_90; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x40);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_69_39_63; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x800000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x4000000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x4000000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x4000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_39_47; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x40000000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x40000000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x40000000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x40000000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_43_42; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x800000000000u64);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x20000).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x20000);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x20000);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // movsxd rax, dword ptr [rbx]
    ax_test![movsxd_rax_dword_ptr_rbx_80_60; 0x48, 0x63, 0x3;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x400000);
            write_reg_value!(q; a; RBX; 0x1000);
            a.mem_init_zero(0x1000, 4).unwrap();
            a.mem_write_32(0x1000, 0x0).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
            assert_eq!(a.mem_read_32(0x1000).unwrap(), 0x0);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
