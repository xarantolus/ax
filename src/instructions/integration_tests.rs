#[cfg(test)]
mod test {
    // You can add more tests here using the a.py script, e.g. run `python3 a.py "mov rax, 0x1234567890abcdef; xor eax, eax"` (select "ts" for setup) to generate a test case
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::SupportedRegister, write_reg_value,
    };
    use iced_x86::Register::*;

    // push rax; xor rax, rax; pop rbx
    ax_test![push_rax_pop_rbx; 0x50, 0x48, 0x31, 0xc0, 0x5b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1234567890ABCDEFu64);
            write_reg_value!(q; a; RBX; 0x031591385913u64);

            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000);
            a.mem_init_zero(0x1000-8, 8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0);
            assert_reg_value!(q; a; RBX; 0x1234567890ABCDEFu64);

            let rsp = a.reg_read_64(RSP.into());
            assert_eq!(rsp, 0x1000);

            assert_eq!(a.mem_read_64(rsp-8).unwrap(), 0x1234567890ABCDEFu64);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // mov rax, 5; mov rbx, 7; add rax, rbx
    ax_test![mov_rax_5_mov_rbx_7_add_rax_rbx; 0x48, 0xc7, 0xc0, 0x5, 0x0, 0x0, 0x0, 0x48, 0xc7, 0xc3, 0x7, 0x0, 0x0, 0x0, 0x48, 0x1, 0xd8;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x4a644dabfbede0cu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 12);
            assert_reg_value!(q; a; RBX; 7);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // mov rax, 5; mov rbx, 7; add rax, rbx; xor rbx, rbx
    ax_test![mov_rax_5_mov_rbx_7_add_rax_rbx_xor_rbx_rbx; 0x48, 0xc7, 0xc0, 0x5, 0x0, 0x0, 0x0, 0x48, 0xc7, 0xc3, 0x7, 0x0, 0x0, 0x0, 0x48, 0x1, 0xd8, 0x48, 0x31, 0xdb;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x27b8b455c8f53915u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0xc);
            assert_reg_value!(q; a; RBX; 0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // mov eax, 5; mov rbx, 7; add eax, ebx; xor rbx, rbx
    ax_test![mov_eax_5_mov_rbx_7_add_eax_ebx_xor_rbx_rbx; 0xb8, 0x5, 0x0, 0x0, 0x0, 0x48, 0xc7, 0xc3, 0x7, 0x0, 0x0, 0x0, 0x1, 0xd8, 0x48, 0x31, 0xdb;
        |a: &mut Axecutor| {
            write_reg_value!(d; a; EAX; 0xdf760bf6u32);
        };
        |a: Axecutor| {
            assert_reg_value!(d; a; EAX; 0xc);
            assert_reg_value!(q; a; RBX; 0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // Writing to 32-bit registers clears the upper part of the 64-bit register
    // mov rax, 0x1234567890abcdef; add eax, 1
    ax_test![mov_rax_0x1234567890abcdef_add_eax_1; 0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34, 0x12, 0x83, 0xc0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x52e6f6b307c35798u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x90abcdf0u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // Writing to non-32-bit register doesn't clear the upper part of the 64-bit register
    // mov rax, 0x1234567890abcdef; add ax, 1
    ax_test![mov_rax_0x1234567890abcdef_add_ax_1; 0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34, 0x12, 0x66, 0x83, 0xc0, 0x1;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x23a1a5c719994edfu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1234567890abcdf0u64);
        };
        (FLAG_PF | FLAG_SF; FLAG_CF | FLAG_ZF | FLAG_OF)
    ];

    // mov rax, 0x1234567890abcdef; xor ax, ax
    ax_test![mov_rax_0x1234567890abcdef_xor_ax_ax; 0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34, 0x12, 0x66, 0x31, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x32b4008fc8254adcu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x1234567890ab0000u64);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];

    // mov rax, 0x1234567890abcdef; xor eax, eax
    ax_test![mov_rax_0x1234567890abcdef_xor_eax_eax; 0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34, 0x12, 0x31, 0xc0;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0xe0b28ab25656ef15u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x0);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
