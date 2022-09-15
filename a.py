
import os
from pickletools import markobject
import random
import subprocess
import sys
import tempfile
import pyperclip


if __name__ == '__main__':
    # First argument is x86-64 assembly code
    assembly_code = "mov rax, rbx" if len(sys.argv) == 1 else " ".join(sys.argv[1:])

    # create temporary directory
    with tempfile.TemporaryDirectory() as tmpdir:
        # write assembly code to file
        assembly_path = os.path.join(tmpdir, "a.asm")
        with open(assembly_path, "w", encoding='utf8') as f:
            f.write(f""".intel_syntax noprefix
			main:
			.word 0x1020
			.word 0x3040
			.word 0x1020
			.word 0x3040
			.word 0x1020
			.word 0x3040
			.word 0x1020
			.word 0x3040
			{assembly_code}
			.word 0x1020
			.word 0x3040
			.word 0x1020
			.word 0x3040
			.word 0x1020
			.word 0x3040
			.word 0x1020
			.word 0x3040
""")

        # assemble to binary
        binary_path = os.path.join(tmpdir, "a.bin")
        subprocess.run(["as", "-o", binary_path, assembly_path])

        with open(binary_path, "rb") as f:
            binary = f.read()

        marker = b"\x20\x10\x40\x30\x20\x10\x40\x30\x20\x10\x40\x30\x20\x10\x40\x30"

        # find offset of main
        binary_start = binary.find(marker) + len(marker)
        binary_end = binary.rfind(marker)

        hex_arr = []
        for b in binary[binary_start:binary_end]:
            hex_arr.append(hex(b))

        test_id = "".join(filter(lambda c: c.isalnum()
                          or c == " ", assembly_code.replace(";", " ")))
        test_id = " ".join(test_id.split()).replace(" ", "_")

        # ask if setup should be included y/n
        setup = input("Test type? (t: ax_test normal; ts: ax_test with setup; o: operand_test; os: operand_test with setup): ")
        if setup == "t":
            code = f"""// {assembly_code}
    ax_test![{test_id}; {", ".join(hex_arr)};
        |a: &mut Axecutor| {{
            write_reg_value!(a; AL; 0x0f);
            todo!("write setup code for \\\"{assembly_code}\\\"");
        }};
        |a: Axecutor| {{
            assert_reg_value!(a; RBX; 0x10);
            todo!("write test cases for \\\"{assembly_code}\\\"");
        }}
    ];"""
        elif setup == "ts":
            code = f"""// {assembly_code}
    ax_test![{test_id}; {", ".join(hex_arr)}; |a: Axecutor| {{
        assert_reg_value!(a; RBX; 0x10);
        todo!("write test cases for \\\"{assembly_code}\\\"");
    }}];"""
        elif setup == "o":
            code = f"""// {assembly_code}
    operand_test![{test_id};
        {", ".join(hex_arr)};
        vec![
            // TODO: Adjust memory operands
            Operand(Memory {{
                base: None,
                index: None,
                scale: 1,
                displacement: 0,
            }}),
            Operand(Immediate {{ data: 1, size: 1 }}),
        ]
    ];"""
        elif setup == "os":
            code = f"""// {assembly_code}
    operand_test![{test_id};
        {", ".join(hex_arr)};
        vec![
            // TODO: Adjust memory operands
            Operand(Memory {{
                base: None,
                index: None,
                scale: 1,
                displacement: 0,
            }}),
            Operand(Immediate {{ data: 1, size: 1 }}),
        ];
        |a: &mut Axecutor| {{
			use iced_x86::Register::*;
			a.reg_write_64(RSP, 0x1000)
		}};
		vec![
            // TODO: Adjust memory addresses
        ]
    ];"""
        else:
            print("Invalid input")
            sys.exit(1)




        pyperclip.copy(code)
        print(f"Copied test case for \"{assembly_code}\" to clipboard!")
