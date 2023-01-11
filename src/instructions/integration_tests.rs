#[cfg(test)]
mod test {
    // You can add more tests here using the a.py script, e.g. run `python3 a.py "mov rax, 0x1234567890abcdef; xor eax, eax"` (select "ts" for setup) to generate a test case
    use super::super::axecutor::Axecutor;
    use crate::{assert_reg_value, ax_test, debug_log, write_reg_value};
    use iced_x86::Register::*;

    // push rax; xor rax, rax; pop rbx
    ax_test![push_rax_pop_rbx; 0x50, 0x48, 0x31, 0xc0, 0x5b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1234567890ABCDEFu64);
            write_reg_value!(q; a; RBX; 0x031591385913u64);

            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000).unwrap();
            a.mem_init_zero(0x1000, 8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0);
            assert_reg_value!(q; a; RBX; 0x1234567890ABCDEFu64);

            let rsp = a.reg_read_64(RSP.into()).unwrap();
            assert_eq!(rsp, 0x1000);

            assert_eq!(a.mem_read_64(rsp).unwrap(), 0x1234567890ABCDEFu64);
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

    // mov rax, 4; cmp rax, 3; je .end; nop; mov rax, 42; .end: nop
    ax_test![mov_rax_4_cmp_rax_3_je_end_nop_mov_rax_42_end_nop;
        0x48, 0xc7, 0xc0, 0x4, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x3, 0x74, 0x8, 0x90, 0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x4686d92fabdcc717u64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x2a);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // JMP .Lstart; fn_add: add rax, rbx; ret; .Lstart: mov rax, 1; mov rbx, 2; call fn_add
    ax_test![jmp_lstart_fnadd_add_rax_rbx_ret_lstart_mov_rax_1_mov_rbx_2_call_fnadd; 0xeb, 0x4, 0x48, 0x1, 0xd8, 0xc3, 0x48, 0xc7, 0xc0, 0x1, 0x0, 0x0, 0x0, 0x48, 0xc7, 0xc3, 0x2, 0x0, 0x0, 0x0, 0xe8, 0xe9, 0xff, 0xff, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x50f01be8d7485109u64);

            // Setup stack for address
            a.reg_write_64(RSP.into(), 0x1000).unwrap();
            a.mem_init_zero(0x1000, 8).expect("Failed to initialize memory");
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0x3);
        };
        (FLAG_PF; FLAG_CF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];

    // Note that "\n" is used as string end marker by the program, simulating reading one line from stdin
    const STRING_REVERSE_INPUT: &[u8] = b"This is a very interesting string!\n";
    const STRING_REVERSE_INPUT_LEN: u64 = STRING_REVERSE_INPUT.len() as u64;
    const STRING_REVERSE_INPUT_START_ADDR: u64 = 0x1000;

    // See testdata/string_reverse.S for source
    ax_test![string_reverse;
    0x48, 0x31, 0xdb, 0x48, 0x31, 0xc9, 0x48, 0xf7, 0xc4, 0x0f, 0x00, 0x00, 0x00, 0x74, 0x0f, 0x48, 0x83, 0xec, 0x08, 0xe8, 0x83, 0x00, 0x00, 0x00, 0x48, 0x83,
    0xc4, 0x08, 0xeb, 0x05, 0xe8, 0x78, 0x00, 0x00, 0x00, 0x41, 0x8a, 0x1a, 0x80, 0xfb, 0x0a, 0x74, 0x07, 0x53, 0x48, 0x83, 0xc1, 0x01, 0xeb, 0xd4, 0x48, 0x83, 0xf9, 0x00, 0x7f, 0x04,
    0x7c, 0x00, 0xeb, 0x27, 0x5b, 0x41, 0x88, 0x1a, 0x48, 0xf7, 0xc4, 0x0f, 0x00, 0x00, 0x00, 0x74, 0x0f, 0x48, 0x83, 0xec, 0x08, 0xe8, 0x36, 0x00, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x08,
    0xeb, 0x05, 0xe8, 0x2b, 0x00, 0x00, 0x00, 0x48, 0x83, 0xe9, 0x01, 0xeb, 0xcf, 0x41, 0xc6, 0x02, 0x0a, 0x48, 0xf7, 0xc4, 0x0f, 0x00, 0x00, 0x00, 0x74, 0x0f, 0x48, 0x83, 0xec, 0x08,
    0xe8, 0x0f, 0x00, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x08, 0xeb, 0x05, 0xe8, 0x04, 0x00, 0x00, 0x00, 0x48, 0x31, 0xc0, 0xc3, 0x50, 0x53, 0x49, 0x8d, 0x1e, 0x4c, 0x01, 0xfb, 0x41, 0x8a,
    0x02, 0x88, 0x03, 0x49, 0xff, 0xc7, 0x5b, 0x58, 0xc3, 0x53, 0x49, 0x8d, 0x1b, 0x4c, 0x01, 0xe3, 0x8a, 0x1b, 0x41, 0x88, 0x1a, 0x49, 0xff, 0xc4, 0x5b, 0xc3;
    |a: &mut Axecutor| {
            assert!(STRING_REVERSE_INPUT.ends_with(b"\n"));
            assert!(!STRING_REVERSE_INPUT.is_empty());

            // Set up input data
            a.mem_init_area(STRING_REVERSE_INPUT_START_ADDR, Vec::from(STRING_REVERSE_INPUT)).expect("Failed to initialize input area memory");
            write_reg_value!(q; a; R11; STRING_REVERSE_INPUT_START_ADDR); // input addr
            write_reg_value!(q; a; R12; 0); // input progress, how many bytes have been read
            write_reg_value!(q; a; R13; STRING_REVERSE_INPUT_LEN); // input length

            // Set up output area
            a.mem_init_zero(0x2000, STRING_REVERSE_INPUT_LEN).expect("Failed to initialize output area memory");
            write_reg_value!(q; a; R14; 0x2000); // output addr
            write_reg_value!(q; a; R15; 0); // output progress, how many bytes have been written

            // Set up the single char buffer
            a.mem_init_zero(0x3000, 1).expect("Failed to initialize char buffer memory");
            write_reg_value!(q; a; R10; 0x3000); // char buffer addr

            // This should be more depending on the test string
            a.init_stack(512).expect("Failed to initialize stack");
        };
        |a: Axecutor| {
            // After running, the output should be reversed
            let output = a.mem_read_bytes(0x2000, STRING_REVERSE_INPUT_LEN).expect("Failed to read output area");

            #[allow(unused_variables)]
            {
                let output_str = String::from_utf8(output).expect("Failed to convert output to string");
                let input_str = String::from_utf8(Vec::from(STRING_REVERSE_INPUT)).expect("Failed to convert input to string");
                debug_log!("Reversed string {:?} to {:?}", input_str, output_str);
                assert_eq!(output_str, STRING_REVERSE_INPUT.iter().rev().skip(1).map(|&b| b as char).collect::<String>() + "\n");
            }
        }
    ];
}
