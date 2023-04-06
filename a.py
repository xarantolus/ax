from __future__ import annotations

import os
import random
import subprocess
import sys
import tempfile
from typing import Union, Final

import pyperclip

FLAG_CF: Final[int] = 0x0001
FLAG_PF: Final[int] = 0x0004
FLAG_ZF: Final[int] = 0x0040
FLAG_SF: Final[int] = 0x0080
FLAG_OF: Final[int] = 0x0800
FLAGS = [
    (FLAG_CF, "CF"),
    (FLAG_PF, "PF"),
    (FLAG_ZF, "ZF"),
    (FLAG_SF, "SF"),
    (FLAG_OF, "OF"),
]

qword_registers = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp",
                   "rsp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"]
dword_registers = ["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp",
                   "esp", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d"]
word_registers = ["ax", "bx", "cx", "dx", "si", "di", "bp", "sp",
                  "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w"]
byte_registers = ["al", "ah", "bl", "bh", "cl", "ch", "dl", "dh", "sil", "dil",
                  "bpl", "spl", "r8b", "r9b", "r10b", "r11b", "r12b", "r13b", "r14b", "r15b"]

registers: Final[list[str]] = qword_registers + dword_registers + word_registers + byte_registers


def find_register(assembly_code: str) -> str:
    found_registers = [register for register in registers
                       if " " + register in assembly_code and "[" + register not in assembly_code]
    assert len(found_registers) > 0, "No register found in assembly code: " + assembly_code

    # now return the one that is mentioned first in assembly_code
    return_register = found_registers[0]
    index = assembly_code.index(return_register)
    for register in found_registers[1:]:
        if assembly_code.index(register) < index:
            return_register = register
            index = assembly_code.index(register)
    return return_register


def register_size_letter(register: str) -> str:
    if register in qword_registers:
        return "q"
    elif register in dword_registers:
        return "d"
    elif register in word_registers:
        return "w"
    elif register in byte_registers:
        return "b"
    else:
        raise Exception("Unknown register: " + register)


def register_size_bytes(register: str) -> int:
    if register in qword_registers:
        return 8
    elif register in dword_registers:
        return 4
    elif register in word_registers:
        return 2
    elif register in byte_registers:
        return 1
    else:
        raise Exception("Unknown register: " + register)


def random_hex_value(register: str) -> str:
    if register in qword_registers:
        x = 64
    elif register in dword_registers:
        x = 32
    elif register in word_registers:
        x = 16
    elif register in byte_registers:
        x = 8
    else:
        raise Exception("Unknown register: " + register)
    return hex(random.randint(0, 2 ** x - 1))


def learn_flags(assembly_code: str, hex_val: str) -> tuple[list[str], list[str], int]:
    with tempfile.TemporaryDirectory() as tmpdir:
        register = find_register(assembly_code)

        # write assembly code to file
        assembly_path = os.path.join(tmpdir, "a.S")

        with open(assembly_path, "w", encoding='utf8') as f:
            f.write(
                f""".intel_syntax noprefix
                .data
                rflags_dest: .space 8
                reg_val: .space 8
                .text
                .global _start
                _start:
                mov rax, 0x00000000
                push rax
                POPFQ

                mov {register}, {hex_val}
                {assembly_code}
                PUSHFQ

                mov [rip+reg_val], {register}
                pop rax
                mov [rip+rflags_dest], rax

                mov rax, 1
                mov rdi, 1
                lea rsi, [rip+rflags_dest]
                mov rdx, 8
                syscall


                mov rax, 1
                mov rdi, 1
                lea rsi, [rip+reg_val]
                mov rdx, 8
                syscall

                mov rax, 60
                mov rdi, 0

                syscall
                """)

        # turn into executable with gcc, symbol _start
        executable_path = os.path.join(tmpdir, "a")
        subprocess.run(["gcc", "-m64", "-nostdlib", "-static", "-o", executable_path, assembly_path])

        # run executable and capture 16 bytes of output
        output = subprocess.run([executable_path], stdout=subprocess.PIPE).stdout

        assert len(output) == 16, "Output is not 16 bytes long"

        rflags = int.from_bytes(output[:8], byteorder="little", signed=False)

        # find out which flags were set
        set_flags, flags_not_set = [], []
        for flag, flag_name in FLAGS:
            if rflags & flag:
                set_flags.append("FLAG_" + flag_name)
            else:
                flags_not_set.append("FLAG_" + flag_name)

        reg_val = int.from_bytes(output[8:8 + register_size_bytes(register)], byteorder="little", signed=False)

        return set_flags, flags_not_set, reg_val


def assembled_bytes(assembly_code: str):
    # create temporary directory
    with tempfile.TemporaryDirectory() as tmpdir:
        # write assembly code to file
        assembly_path = os.path.join(tmpdir, "a.S")
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

        subprocess.run(["gcc", "-c", "-m64", "-nostdlib", "-static", "-o", binary_path, assembly_path])

        with open(binary_path, "rb") as f:
            binary = f.read()

        marker = b"\x20\x10\x40\x30\x20\x10\x40\x30\x20\x10\x40\x30\x20\x10\x40\x30"

        # find offset of main
        binary_start = binary.find(marker) + len(marker)
        binary_end = binary.rfind(marker)

        hex_arr = [hex(b) if b > 0 else '0' for b in binary[binary_start:binary_end]]

        return hex_arr


def stringify_flags(flags: list[str]) -> str:
    return "0" if len(flags) == 0 else " | ".join(flags)


def hexify(number: Union[str, int], register_size: int | None = None) -> str:
    if isinstance(number, str):
        number = int(number, base=0)

    if number >= 2147483647:
        if register_size == 4:
            return hex(number) + "u32"
        return hex(number) + "u64"
    return hex(number)


def generate_test(assembly_code: str, hex_arr: list):
    test_id = "".join(filter(lambda c: c.isalnum() or c == " ", assembly_code.replace(";", " ")))
    test_id = " ".join(test_id.split()).replace(" ", "_")

    # ask if setup should be included y/n
    setup = input("Test type? ("
                  "t: ax_test normal; "
                  "ts: ax_test with setup; "
                  "o: operand_test; "
                  "os: operand_test with setup; "
                  "u: JS Uint8Array, "
                  "b: Binary): ")

    code = ""
    if setup == "t" or setup == "ts":
        register = find_register(assembly_code)
        hex_val = random_hex_value(register)
        flags_set, flags_not_set, register_output = learn_flags(assembly_code, hex_val)

        if setup == "t":
            code = f"""// {assembly_code}
    ax_test![{test_id}; {", ".join(hex_arr)}; |a: Axecutor| {{
            assert_reg_value!({register_size_letter(register)}; a; {register.upper()}; {hexify(register_output, register_size=register_size_bytes(register))});
        }}; ({stringify_flags(flags_set)}; {stringify_flags(flags_not_set)})];"""
        elif setup == "ts":
            code = f"""// {assembly_code}
    ax_test![{test_id}; {", ".join(hex_arr)};
        |a: &mut Axecutor| {{
            write_reg_value!({register_size_letter(register)}; a; {register.upper()}; {hexify(hex_val, register_size=register_size_bytes(register))});
        }};
        |a: Axecutor| {{
            assert_reg_value!({register_size_letter(register)}; a; {register.upper()}; {hexify(register_output, register_size=register_size_bytes(register))});
        }};
        ({stringify_flags(flags_set)}; {stringify_flags(flags_not_set)})
    ];"""
    elif setup == "o":
        code = f"""// {assembly_code}
operand_test![{test_id};
    {", ".join(hex_arr)};
    vec![
        // TODO: Adjust memory operands
        Operand(Memory {{
            base: Some(SupportedRegister::RSP),
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
            base: Some(SupportedRegister::RSP),
            index: None,
            scale: 1,
            displacement: 0,
        }}),
        Operand(Immediate {{ data: 1, size: 1 }}),
    ];
    |a: &mut Axecutor| {{
        use iced_x86::Register::*;
        a.reg_write_64(SupportedRegister::RSP, 0x1000)
    }};
    vec![
        // TODO: Adjust memory addresses
    ]
];"""
    elif setup == "u":
        # For javascript
        code = f"""// {assembly_code}\nlet code = new Uint8Array([{", ".join(hex_arr)}]);
        """
    elif setup == "b":
        # just the binary hex representation
        code = f"""// {assembly_code}\n[{", ".join(hex_arr)}]"""
    else:
        print("Invalid input")
        raise Exception

    try:
        pyperclip.copy(code)
        print(f"Copied code for \"{assembly_code}\" to clipboard!")
    except:
        print(code)


def main():
    # First argument is x86-64 assembly code
    assembly_code = "mov rax, rbx" if len(sys.argv) == 1 else " ".join(sys.argv[1:])

    generate_test(assembly_code, assembled_bytes(assembly_code))


if __name__ == '__main__':
    main()
