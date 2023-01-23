from __future__ import annotations

import abc
import os
import random
import re
import shutil
import subprocess
import sys
import tempfile
import traceback
import unittest
from curses.ascii import isspace
from multiprocessing.dummy import Pool
from typing import Literal, Final

import pyperclip
from tqdm import tqdm

# TODO: why rip here but not in a.py?
qword_registers = ["rip", "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp",
                   "rsp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"]
dword_registers = ["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp",
                   "esp", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d"]
word_registers = ["ax", "bx", "cx", "dx", "si", "di", "bp", "sp",
                  "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w"]
byte_registers = ["al", "ah", "bl", "bh", "cl", "ch", "dl", "dh", "sil", "dil",
                  "bpl", "spl", "r8b", "r9b", "r10b", "r11b", "r12b", "r13b", "r14b", "r15b"]

registers: Final[list[str]] = (byte_registers + word_registers + dword_registers + qword_registers)
# Sorting makes sure we find e.g. "rax" first instead of "ax"
registers.sort(key=len, reverse=True)

FLAG_CF: Final[int] = 0x0001
FLAG_PF: Final[int] = 0x0004
FLAG_ZF: Final[int] = 0x0040
FLAG_SF: Final[int] = 0x0080
FLAG_OF: Final[int] = 0x0800
OUTPUT_FLAGS_TO_ANALYZE = [
    (FLAG_CF, "CF"),
    (FLAG_PF, "PF"),
    (FLAG_ZF, "ZF"),
    (FLAG_SF, "SF"),
    (FLAG_OF, "OF"),
]
FLAGS = OUTPUT_FLAGS_TO_ANALYZE


def check_temp_dir():
    # test if /dev/shm is available by writing a file
    temp_dir_filesystem = "/dev/shm"
    delete_at_exit = False
    try:
        with open(os.path.join(temp_dir_filesystem, "test"), "w") as f:
            f.write("test")
        os.remove(os.path.join(temp_dir_filesystem, "test"))
    except:
        temp_dir_filesystem = tempfile.gettempdir()
        delete_at_exit = True
        print("WARNING: /dev/shm not available, using non-RAM " + temp_dir_filesystem)
    return temp_dir_filesystem, delete_at_exit


temp_dir_filesystem, delete_at_exit = check_temp_dir()


class Operand(abc.ABC):
    name: str
    base_register: RegisterOperand

    @abc.abstractmethod
    def size(self) -> Literal[1, 2, 4, 8]:
        ...

    def size_letter(self):
        size = self.size()
        if size == 1:
            return "b"
        elif size == 2:
            return "w"
        elif size == 4:
            return "d"
        elif size == 8:
            return "q"
        else:
            # illegal argument
            raise ValueError("size must be 1, 2, 4 or 8")


class RegisterOperand(Operand):

    def __init__(self, name):
        self.name = name.lower()
        self.size()  # TODO: why call this here?

    def size(self):
        if self.name in qword_registers:
            return 8
        elif self.name in dword_registers:
            return 4
        elif self.name in word_registers:
            return 2
        elif self.name in byte_registers:
            return 1
        else:
            raise ValueError("Unknown register: " + str(self.name))

    def __eq__(self, other):
        return self.name == other.name

    def __str__(self):
        return self.name


class ImmediateOperand(Operand):
    def __init__(self, number: str | int):
        if isinstance(number, str):
            number = int(number, base=0)
        self.number = number

    def hexify(self, target: Operand):
        if pow(2, target.size() * 8) <= self.number:
            raise ValueError(f"Number {hex(self.number)} too big for target register {target.name}")

        if self.number >= 2147483647:
            if target.size() == 4:
                return hex(self.number) + "u32"
            return hex(self.number) + "u64"
        else:
            return hex(self.number)

    def size(self):
        raise ValueError("ImmediateOperand has no size")

    def __eq__(self, other):
        return self.number == other.number

    def __str__(self):
        return hex(self.number)


class MemoryOperand(Operand):
    def __init__(
            self,
            base_register: RegisterOperand | str,
            offset: int = 0,
            scale: int = 1,
            index_register: RegisterOperand | str | None = None,
            size: Literal[1, 2, 4, 8] = None
    ):
        self.base_register = base_register if isinstance(base_register, RegisterOperand) else RegisterOperand(base_register)

        self.offset = offset
        self.scale = scale

        self.index_register = index_register if isinstance(index_register, RegisterOperand) else RegisterOperand(index_register) if index_register is not None else None

        assert size in [1, 2, 4, 8]
        self._size = size

    def size(self):
        return self._size

    def __eq__(self, other: MemoryOperand) -> bool:
        return self.base_register == other.base_register and \
            self.offset == other.offset and \
            self.scale == other.scale and \
            self.index_register == other.index_register and \
            self._size == other._size

    def __str__(self) -> str:
        size_prefix = ""
        if self._size == 1:
            size_prefix = "byte ptr "
        elif self._size == 2:
            size_prefix = "word ptr "
        elif self._size == 4:
            size_prefix = "dword ptr "
        elif self._size == 8:
            size_prefix = "qword ptr "

        if self.index_register is None:
            return f"{size_prefix}[{self.base_register.name}]" if self.offset == 0 else \
                    f"{size_prefix}[{self.base_register.name}+{self.offset}]"

        if self.offset == 0:
            return f"{size_prefix}[{self.base_register.name}+{self.scale}*{self.index_register.name}]" \
                if self.scale != 1 \
                else f"{size_prefix}[{self.base_register.name}+{self.index_register.name}]"
        raise ValueError("cannot have offset and index register")

    @staticmethod
    def parse(argument: str, other_operand: Operand | None) -> MemoryOperand:
        original_argument = argument
        argument = argument.lower()

        # parse GNU Assembler syntax memory operands
        if argument.startswith("byte ptr"):
            size = 1
            argument = argument[8:]
        elif argument.startswith("word ptr"):
            size = 2
            argument = argument[8:]
        elif argument.startswith("dword ptr"):
            size = 4
            argument = argument[9:]
        elif argument.startswith("qword ptr"):
            size = 8
            argument = argument[9:]
        elif argument.startswith("ptr"):
            size = other_operand.size()
            argument = argument[3:]
        else:
            size = other_operand.size()

        argument = argument.strip()

        if argument.startswith("[") and argument.endswith("]"):
            argument = argument[1:-1]

        argument = argument.strip()

        # parse base register
        for register in registers:
            if argument.startswith(register):
                base_register = register
                argument = argument[len(register):]
                break
        else:
            base_register = None

        argument = argument.strip()

        # parse offset
        if argument.startswith("+") or argument.startswith("-"):
            # parse however digits are there
            offset = str(argument[0])
            argument = argument[1:]

            argument = argument.strip()

            for char in argument:
                if char.isdigit():
                    offset += char
                elif isspace(char):
                    continue
                else:
                    break
            argument = argument[len(offset) - 1:]
            offset = int(offset, base=0)
        else:
            offset = 0

        argument = argument.strip()

        # parse scale
        if argument.startswith("*"):
            scale = offset
            offset = None
            argument = argument[1:]
        else:
            scale = None

        argument = argument.strip()

        # parse index register
        for register in registers:
            if argument.startswith(register):
                index_register = register
                argument = argument[len(register):]
                break
        else:
            index_register = None

        if len(argument) > 0:
            raise ValueError(f"Could not parse memory argument: {original_argument}(rest is {argument})\n"
                             "Note that the parser is very basic and cares about order")

        return MemoryOperand(base_register, offset, scale, index_register, size)


class Instruction:
    def __init__(
            self,
            mnemonic: str,
            arguments: list[Operand],
            additional_imm: ImmediateOperand | None,
            implicit: list[Operand] | None = None
    ):
        if implicit is None:
            implicit = []
        self.mnemonic = mnemonic.lower()
        self.arguments = arguments
        self.implicit_arguments = implicit
        # currently only 0-2 operands are supported
        assert len(self.arguments) + len(self.implicit_arguments) <= 2
        self.additional_imm = additional_imm

    def set_implicit(self, implicit: list[Operand]):
        self.implicit_arguments = implicit
        assert len(self.arguments) + len(self.implicit_arguments) <= 2

    @staticmethod
    def parse_operand(argument: str, other_operand: Operand | None):
        # try to parse as register
        try:
            return RegisterOperand(argument)
        except:
            pass

        # try to parse as immediate
        try:
            return ImmediateOperand(argument)
        except:
            pass

        # try to parse as memory
        try:
            return MemoryOperand.parse(argument, other_operand)
        except:
            pass

        raise ValueError("Could not parse operand: " + argument)

    @staticmethod
    def parse(argument: str) -> Instruction:
        # parse GNU Assembler syntax instructions, e.g.
        # mov rax, 0x5
        # mov rax, [rsp+8]
        # mov [rsp+4*rcx], rax
        # push rax
        # pop al
        # ret

        # split into mnemonic and arguments
        parts = argument.split(maxsplit=1)
        mnemonic = parts[0].lower()

        if len(parts) == 1:
            return Instruction(mnemonic, [], None)

        # now split at the comma, but ignore commas in brackets
        operands = []
        current_argument = ""
        bracket_level = 0
        for char in parts[1]:
            if char == "[":
                bracket_level += 1
            elif char == "]":
                bracket_level -= 1
            elif char == "," and bracket_level == 0:
                operands.append(current_argument.strip())
                current_argument = ""
                continue
            current_argument += char

        operands.append(current_argument.strip())

        parsed_operands = []
        additional_immediate = None
        if len(operands) == 1:
            parsed_operands.append(
                Instruction.parse_operand(operands[0], None))
        elif len(operands) == 2:
            try:
                first_operand = Instruction.parse_operand(operands[0], None)
                second_operand = Instruction.parse_operand(
                    operands[1], first_operand)
            except:
                second_operand = Instruction.parse_operand(operands[1], None)
                first_operand = Instruction.parse_operand(
                    operands[0], second_operand)

            parsed_operands.append(first_operand)
            parsed_operands.append(second_operand)
        elif len(operands) == 3:
            try:
                first_operand = Instruction.parse_operand(operands[0], None)
                second_operand = Instruction.parse_operand(
                    operands[1], first_operand)
            except:
                second_operand = Instruction.parse_operand(operands[1], None)
                first_operand = Instruction.parse_operand(
                    operands[0], second_operand)

            additional_immediate = ImmediateOperand(operands[2])

            parsed_operands.append(first_operand)
            parsed_operands.append(second_operand)
        else:
            raise ValueError("Too many operands")

        return Instruction(mnemonic, parsed_operands, additional_immediate)

    def __eq__(self, other):
        return self.mnemonic == other.mnemonic and self.arguments == other.arguments

    def __str__(self) -> str:
        if len(self.arguments) == 0:
            assert self.additional_imm is None
            return self.mnemonic
        elif len(self.arguments) == 1:
            assert self.additional_imm is None
            return f"{self.mnemonic} {self.arguments[0]}"
        elif len(self.arguments) == 2:
            if self.additional_imm is None:
                return f"{self.mnemonic} {self.arguments[0]}, {self.arguments[1]}"
            else:
                return f"{self.mnemonic} {self.arguments[0]}, {self.arguments[1]}, {self.additional_imm}"
        else:
            raise ValueError("str not implement for Instruction with more than 2 operands")


class Tests(unittest.TestCase):
    def test_parse_memory_operand(self):
        self.assertEqual(MemoryOperand.parse(
            "[rsp+8* Rcx]", RegisterOperand("al")),
            MemoryOperand("rsp", 0, 8, "rcx", 1),
        )
        self.assertEqual(MemoryOperand.parse(
            "byte ptr [rax]", RegisterOperand("rax")),
            MemoryOperand("rax", 0, 1, None, 1),
            "byte ptr [rax]"
        )
        self.assertEqual(MemoryOperand.parse(
            "qword ptr [rsp+8]", RegisterOperand("rcx")),
            MemoryOperand("rsp", 8, 1, None, 8),
            "qword ptr [rsp+8]")

        self.assertEqual(MemoryOperand.parse(
            "qword ptr [rsp+ 8]", RegisterOperand("rcx")),
            MemoryOperand("rsp", 8, 1, None, 8),
            "qword ptr [rsp+ 8]")

        self.assertEqual(MemoryOperand.parse(
            "qword ptr [rsp + 8]", RegisterOperand("rcx")),
            MemoryOperand("rsp", 8, 1, None, 8),
            "qword ptr [rsp + 8]")

        self.assertEqual(MemoryOperand.parse(
            "[rsp+4*rcx]", RegisterOperand("al")),
            MemoryOperand("rsp", 0, 4, "rcx", 1),
            "[rsp+4*rcx]"
        )

    def test_parse_register_operand(self):
        op = RegisterOperand("rax")

        self.assertEqual(op.name, "rax")
        self.assertEqual(op.size(), 8)

        op = RegisterOperand("r11B")

        self.assertEqual(op.name, "r11b")
        self.assertEqual(op.size(), 1)

    def test_immediate_operand(self):
        op = ImmediateOperand(0x5)
        self.assertEqual(op.number, 0x5)
        self.assertEqual(op.hexify(RegisterOperand("rax")), "0x5")

        op = ImmediateOperand(0x80000000)
        self.assertEqual(op.number, 0x80000000)

        self.assertEqual(op.hexify(RegisterOperand("rax")), "0x80000000u64")
        self.assertEqual(op.hexify(RegisterOperand("eax")), "0x80000000u32")

        with self.assertRaises(Exception):
            op.hexify(RegisterOperand("ax"))

    def test_parse_instruction(self):
        instr = Instruction.parse("mov rax, 0x5")
        self.assertEqual(instr.mnemonic, "mov")
        self.assertEqual(instr.arguments[0].name, "rax")
        self.assertEqual(instr.arguments[1].number, 0x5)

        instr = Instruction.parse("mov rax, [rsp+8]")
        self.assertEqual(instr.mnemonic, "mov")
        self.assertEqual(instr.arguments[0].name, "rax")
        self.assertEqual(instr.arguments[1].base_register, RegisterOperand("rsp"))
        self.assertEqual(instr.arguments[1].offset, 8)

        instr = Instruction.parse("mov [rsp+4*rcx], rax")
        self.assertEqual(instr.mnemonic, "mov")
        self.assertEqual(instr.arguments[0].base_register, RegisterOperand("rsp"))
        self.assertEqual(instr.arguments[0].offset, 0)
        self.assertEqual(instr.arguments[0].scale, 4)
        self.assertEqual(instr.arguments[0].index_register, RegisterOperand("rcx"))
        self.assertEqual(instr.arguments[1].name, "rax")

        instr = Instruction.parse("push rax")
        self.assertEqual(instr.mnemonic, "push")
        self.assertEqual(instr.arguments[0].name, "rax")

        instr = Instruction.parse("pop al")
        self.assertEqual(instr.mnemonic, "pop")
        self.assertEqual(instr.arguments[0].name, "al")

        instr = Instruction.parse("ret")
        self.assertEqual(instr.mnemonic, "ret")
        self.assertEqual(instr.arguments, [])


def assemble(instruction: Instruction | str) -> list[str]:
    # create temporary directory
    with tempfile.TemporaryDirectory(prefix="ax_assemble", dir="/dev/shm") as tmpdir:
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
            {instruction}
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

        return hex_arr


def test_id(instruction: Instruction | str, flags_set, inputs=None):
    def map_flags(f):
        # remove the FLAG_ prefix from each flag
        return [x[5:] for x in f]

    # generate name from instruction string and flags set, but replaces spaces and commas with _
    test_name = f"{instruction}_{'_'.join(map_flags(flags_set))}"

    if isinstance(instruction, Instruction) and len(instruction.implicit_arguments) > 0 and inputs is not None:
        test_name += f"_{'_'.join([f'{op}_{inputs[i + len(instruction.arguments)]}' for i, op in enumerate(instruction.implicit_arguments)])}"

    # only keep alphanumerial characters
    test_name = re.sub(r'\W+', '_', test_name)

    # replace consecutive _ with only one
    test_name = re.sub(r'_+', '_', test_name)

    return test_name.strip("_").lower()


def flag_to_literal(flag: str | int):
    # if string return as is
    if isinstance(flag, str):
        return flag

    lit = ""
    for (v, k) in FLAGS:
        if flag & v:
            lit += ("" if lit == "" else " | ") + f"FLAG_{k}"

    return lit


def joinflags(flags: list[str | int] | int, separator: str = " | ") -> str:
    if isinstance(flags, int):
        return flag_to_literal(flags).replace(" | ", separator)
    return separator.join(list(map(flag_to_literal, flags))) if len(flags) > 0 else "0"


def flags_to_str(set: list[str], notset: list[str]):
    return f"{joinflags(set)}; {joinflags(notset)}"


class Input:
    def __init__(self, values: list[int], flags: list[int] | int):
        self.values = values
        self.flags = flags


class TestCase:
    def __init__(
            self,
            assembled: list[str],
            instruction: Instruction,
            set_flags: list[str],
            flags_not_set: list[str],
            args: Input,
            expected_values: list[int]
    ):
        self.instruction = instruction if isinstance(instruction, Instruction) else Instruction.parse(instruction)
        assert isinstance(self.instruction, Instruction)

        assert len(args.values) == len(expected_values)

        self.assembled_bytes = assembled

        self.flags_set = set_flags
        self.flags_not_set = flags_not_set
        self.args = args
        self.expected_values = expected_values

    GOOD_TEST_VALUES = list(dict.fromkeys(
        [
            0x0,
            0x1,
            7,
            8,
            15,
            16,
            17,
            31,
            32,
            33,
            63,
            64,
            65,
            0x7f,
            0x80,
            0xff,
            0x100,
            0x100,
            0x7fff,
            0x8000,
            0x10000,
            0x7fffffff,
            0x80000000,
            0x100000000,
            0x7fffffffffffffff,
            0x8000000000000000,
        ]
        +  # powers of 2
        [2 ** i for i in range(64)]
    ))

    @staticmethod
    def dynamic_operands(i: Instruction) -> list[Operand]:
        return list(filter(lambda o: not isinstance(o, ImmediateOperand), i.arguments + i.implicit_arguments))

    @staticmethod
    def permutate_with_flags(inputs: list[list[int]], flags_to_permutate: list[int]) -> list[Input]:
        if len(flags_to_permutate) == 0:
            return [Input(i, 0) for i in inputs]

        permut = [0]
        for f in flags_to_permutate:
            permut += [x | f for x in permut]

        return [Input(i, f) for i in inputs for f in permut]

    @staticmethod
    def generate_inputs(dynamic_operands: list[Operand]) -> list[list[int]]:
        dynamic_operands = len(dynamic_operands)

        if dynamic_operands == 0:
            return [[]]
        elif dynamic_operands == 1:
            # TODO: changed to [i] but was i
            return [[v] for v in TestCase.GOOD_TEST_VALUES] + \
                [[i] for i in range(0, 1024)] + \
                [[random.randint(0, 2 ** 64)] for _ in range(50)]
        elif dynamic_operands == 2:
            return ([[v1, v2]
                     for v1 in TestCase.GOOD_TEST_VALUES for v2 in TestCase.GOOD_TEST_VALUES]
                    # random values and good test values
                    + [[random.randint(0, 2 ** 64), v]
                       for v in TestCase.GOOD_TEST_VALUES]
                    + [[v, random.randint(0, 2 ** 64)]
                       for v in TestCase.GOOD_TEST_VALUES]
                    # 50 random combinations
                    + [[random.randint(0, 2 ** 64), random.randint(0, 2 ** 64)]
                       for _ in range(50)])
        else:
            raise NotImplementedError("Too many dynamic operands")

    @staticmethod
    def auto_learn_flags(i: Instruction, result_only: bool, flags_to_permutate: list[int]) -> list:
        dynamic_operands = TestCase.dynamic_operands(i)

        inputs = TestCase.generate_inputs(dynamic_operands)

        with_flags = TestCase.permutate_with_flags(inputs, flags_to_permutate)

        return TestCase.learn_flags(i, with_flags, result_only)

    NEWLINE = "\n"

    last_exception = None

    @staticmethod
    def learn_single_flags(
            i: int,
            assembled: list[str],
            instruction: Instruction,
            args: Input,
            tmpdir: str
    ) -> TestCase | None:
        try:
            setup_code = []

            dynamic_operands = TestCase.dynamic_operands(instruction)

            idx = 0
            for arg in dynamic_operands:
                if isinstance(arg, RegisterOperand):
                    setup_code.append(f"mov {arg}, {args.values[idx]}")
                    idx += 1
                elif isinstance(arg, MemoryOperand):
                    # write memory operands to the stack
                    if arg.base_register != RegisterOperand("rsp"):
                        setup_code.append(f"mov {arg.base_register}, rsp")
                    if arg.index_register is not None:
                        setup_code.append(f"mov {arg.index_register}, 0")
                    setup_code.append(
                        f"mov {arg}, {args.values[idx]}")
                    idx += 1
                else:
                    raise ValueError("invalid dynamic operand" + str(arg))

            assert idx == len(dynamic_operands), "Not all dynamic operands were used in setup"

            assert len(dynamic_operands) <= 2, "Too many dynamic operands"

            assembly_path = os.path.join(tmpdir, f"{i}.asm")

            def get_rax(op):
                return {1: 'al', 2: 'ax', 4: 'eax', 8: 'rax'}[op.size()]

            generated_code = f""".intel_syntax noprefix
                .data
                rflags_dest: .space 8
                output_val: .space 8
                output_val2: .space 8
                .text
                .global _start
                _start:
                # Setup
                {TestCase.NEWLINE.join(setup_code)}

                push rax
                # Reset flags
                mov rax, {hex(args.flags) if args.flags else 0}
                push rax
                POPFQ
                pop rax # We can do this because push/pop doesn't affect flags

                # Run the actual instruction we care about
                {instruction}

                push rax
                # Save flag state
                PUSHFQ
                pop rax # load flags into rax
                mov [rflags_dest], rax

                pop rax

                # Now read the output values

                push rax
                {f'mov {get_rax(dynamic_operands[0])}, {dynamic_operands[0]}; mov [rip+output_val], {get_rax(dynamic_operands[0])}' if len(
                    dynamic_operands) > 0 else ''}
                pop rax

                push rax
                {f'mov {get_rax(dynamic_operands[1])}, {dynamic_operands[1]}; mov [rip+output_val2], {get_rax(dynamic_operands[1])}' if len(
                    dynamic_operands) > 1 else ''}
                pop rax


                mov rax, 1
                mov rdi, 1
                lea rsi, [rip+rflags_dest]
                mov rdx, 24
                syscall

                mov rax, 60
                mov rdi, 0

                syscall
                """

            with open(assembly_path, "w", encoding='utf8') as f:
                f.write(generated_code)

            # assemble with as
            object_path = os.path.join(tmpdir, f"{i}.o")
            subprocess.run(["as", "-moperand-check=error", "-o", object_path, assembly_path],
                           stderr=subprocess.DEVNULL, stdout=subprocess.DEVNULL, check=True)

            # We instantly remove non-needed things as in the devcontainer /dev/shm is very limited
            os.remove(assembly_path)

            # turn into executable with gcc, symbol _start
            executable_path = os.path.join(tmpdir, f"{i}")
            subprocess.run(["gcc", "-m64", "-nostdlib", "-static",
                            "-o", executable_path, object_path], stderr=subprocess.DEVNULL, stdout=subprocess.DEVNULL, check=True)

            os.remove(object_path)

            # run executable and capture 24 bytes of output
            output = subprocess.run([executable_path], stdout=subprocess.PIPE).stdout

            os.remove(executable_path)

            assert len(output) == 24, "Output is not 24 bytes long"

            rflags = int.from_bytes(output[:8], byteorder="little", signed=False)

            # find out which flags were set
            set_flags, flags_not_set = [], []
            for flag, flag_name in OUTPUT_FLAGS_TO_ANALYZE:
                if rflags & flag:
                    set_flags.append("FLAG_" + flag_name)
                else:
                    flags_not_set.append("FLAG_" + flag_name)

            if len(dynamic_operands) == 0:
                return TestCase(
                    assembled,
                    instruction,
                    set_flags,
                    flags_not_set,
                    Input([], []),
                    []
                )
            elif len(dynamic_operands) == 1:
                output_op_val1 = int.from_bytes(
                    output[8:8+dynamic_operands[0].size()], byteorder="little", signed=False)

                return TestCase(
                    assembled,
                    instruction,
                    set_flags,
                    flags_not_set,
                    args,
                    [output_op_val1],
                )
            elif len(dynamic_operands) == 2:
                output_op_val1 = int.from_bytes(
                    output[8:8+dynamic_operands[0].size()], byteorder="little", signed=False)
                output_op_val2 = int.from_bytes(
                    output[16:16+dynamic_operands[1].size()], byteorder="little", signed=False)

                return TestCase(
                    assembled,
                    instruction,
                    set_flags,
                    flags_not_set,
                    args,
                    [output_op_val1, output_op_val2],
                )
            else:
                raise ValueError("invalid number of dynamic operands")
        except Exception:
            # include stack trace
            TestCase.last_exception = traceback.format_exc()
            return None

    @staticmethod
    def learn_flags(instruction: Instruction, input_args: list[Input], result_only: bool) -> list[TestCase]:
        results: list[TestCase] = []

        assembled = assemble(instruction)

        def is_new(flags_set, flags_not_set, input_flags):
            if result_only:
                return True

            for ts in results:
                if ts.flags_set == flags_set and ts.flags_not_set == flags_not_set and ts.args.flags == input_flags:
                    return False
            return True

        with tempfile.TemporaryDirectory(prefix="ax_flag_learner", dir="/dev/shm") as tmpdir:
            def imap_func(input: tuple[int, Input]):
                return TestCase.learn_single_flags(input[0], assembled, instruction, input[1], tmpdir)

            with Pool(os.cpu_count() * 4) as p:
                temp_results = list(
                    tqdm(
                        p.imap(imap_func, enumerate(input_args)),
                        total=len(input_args)
                    ))

            for r in temp_results:
                if r is not None and is_new(r.flags_set, r.flags_not_set, r.args.flags):
                    # Only keep tests we know will work later (sometimes immediate values are too large, but this is ignored by as)
                    try:
                        str(r)
                    except Exception:
                        continue

                    results.append(r)

            if len(results) == 0:
                raise ValueError(
                    f"""Could not learn any flags for instruction {instruction}, likely due to some bug with with the flag learner
                    Here's the last exception:
                    {TestCase.last_exception}""")
        return results

    def test_id(self):
        flags = "_" + joinflags(self.args.flags, separator="_").replace("FLAG_", "") if self.args.flags else ""
        return test_id(self.instruction, self.flags_set, self.args.values) + flags.lower()

    def __str__(self):
        dynamic_operands = self.dynamic_operands(self.instruction)

        mem_start = 0x1000

        # generate syntax for writing the given value to the operand (register or memory)
        def operand_write(operand, value):
            if isinstance(operand, RegisterOperand):
                return f"write_reg_value!({operand.size_letter()}; a; {operand.name.upper()}; {ImmediateOperand(value).hexify(operand)});"
            elif isinstance(operand, MemoryOperand):
                # Set up base and index registers, as well as memory
                return f"""write_reg_value!({operand.base_register.size_letter()}; a; {operand.base_register.name.upper()}; {hex(mem_start + operand.offset)});{
                f'{TestCase.NEWLINE}        write_reg_value!({operand.index_register.size_letter()}; a; {operand.index_register.name.upper()}; 0);' if operand.index_register is not None else ''}
        a.mem_init_zero({hex(mem_start + operand.offset)}, {operand.size()}).unwrap();
        a.mem_write_{operand.size() * 8}({hex(mem_start + operand.offset)}, {ImmediateOperand(value).hexify(operand)}).unwrap();"""
            else:
                raise ValueError("invalid operand type")

        # generate syntax for asserting the given register/memory value
        def assert_operand(operand, value):
            if isinstance(operand, RegisterOperand):
                return f"assert_reg_value!({operand.size_letter()}; a; {operand.name.upper()}; {ImmediateOperand(value).hexify(operand)});"
            elif isinstance(operand, MemoryOperand):
                # assert registers unchanged and memory changed
                return f"""assert_reg_value!({operand.base_register.size_letter()}; a; {operand.base_register.name.upper()}; {hex(mem_start + operand.offset)});
        assert_mem_value!({operand.size_letter()}; a; {hex(mem_start + operand.offset)}; {ImmediateOperand(value).hexify(operand)});"""
            else:
                raise ValueError("invalid operand type")

        # Now just generate cases depending on the number of dynamic operands
        if len(dynamic_operands) == 0:
            return f"""// {self.instruction}
ax_test![{self.test_id()}; {", ".join(self.assembled_bytes)}; |a: Axecutor| {{
        todo!("Asset state of registers and/or memory");{
            f'{TestCase.NEWLINE}        write_flags!(a; {joinflags(self.args.flags)});' if self.args.flags else ''}
    }};
    ({flags_to_str(self.flags_set, self.flags_not_set)})
];"""
        elif len(dynamic_operands) == 1:
            return f"""// {self.instruction}
ax_test![{self.test_id()}; {", ".join(self.assembled_bytes)};
    |a: &mut Axecutor| {{
        {operand_write(dynamic_operands[0], self.args.values[0])}{
            f'{TestCase.NEWLINE}        write_flags!(a; {joinflags(self.args.flags)});' if self.args.flags else ''}
    }};
    |a: Axecutor| {{
        {assert_operand(dynamic_operands[0], self.expected_values[0])}
    }};
    ({flags_to_str(self.flags_set, self.flags_not_set)})
];"""
        elif len(dynamic_operands) == 2:
            return f"""// {self.instruction}
ax_test![{self.test_id()}; {", ".join(self.assembled_bytes)};
    |a: &mut Axecutor| {{
        {operand_write(dynamic_operands[0], self.args.values[0])}
        {operand_write(dynamic_operands[1], self.args.values[1])}{
            f'{TestCase.NEWLINE}        write_flags!(a; {joinflags(self.args.flags)});' if self.args.flags else ''}
    }};
    |a: Axecutor| {{
        {assert_operand(dynamic_operands[0], self.expected_values[0])}
        {assert_operand(dynamic_operands[1], self.expected_values[1])}
    }};
    ({flags_to_str(self.flags_set, self.flags_not_set)})
];"""
        raise ValueError("invalid number of dynamic operands")


def parse_flags(text: str) -> list[int]:
    flags = [s.strip().upper() for s in text.split(',')]
    valid = all(map(lambda f: any(map(lambda t: t[1] == f, OUTPUT_FLAGS_TO_ANALYZE)), flags))

    if not valid:
        raise ValueError(
            f"Invalid flags: {text}, valid flags are {FLAGS}")

    flags = list(map(lambda t: t[0], filter(lambda t: t[1] in flags, FLAGS)))

    return flags


def main():
    import argparse

    parser = argparse.ArgumentParser(description='Generate tests for axecutor')
    parser.add_argument('-t', '--test', help='Run tests for this script', dest='test', action='store_true')
    parser.add_argument('-f', '--flags', help='Select flags to test for', action='store', dest='flags')
    parser.add_argument('-s', '--set', help='Flags that should be set or not set before the instruction is executed',
                        action='store', dest='flags_set')
    parser.add_argument('-e', '--extreme', help='Run more tests (default for < 2 dynamic arguments)',
                        action='store_true', dest='extreme')
    parser.add_argument("-i", "--implicit-operands", help="Test implicit operands", action="store",
                        dest="implicit_operands")
    parser.add_argument("-r", "--result", help="Deduplicate by result of test, not by flags set", action="store_true",
                        dest="result")

    parser.add_argument('rest', nargs=argparse.REMAINDER, action='store')

    args = parser.parse_args()

    # Arguments of this script are joined together
    assembly_code = "imul ax,bx,0x5" if len(
        args.rest) == 0 else " ".join(args.rest)
    instruction = Instruction.parse(assembly_code)

    if args.test:
        sys.argv = [sys.argv[0]]
        unittest.main()
        exit(0)

    if args.flags:
        flags = [s.strip().upper() for s in args.flags.split(',')]
        valid = all(map(lambda f: any(map(lambda t: t[1] == f, FLAGS)), flags))

        if not valid:
            raise ValueError(
                f"Invalid flags: {args.flags}, valid flags are {FLAGS}")
        global OUTPUT_FLAGS_TO_ANALYZE
        OUTPUT_FLAGS_TO_ANALYZE = list(filter(lambda t: t[1] in flags, FLAGS))

        print(f"Testing flags: {OUTPUT_FLAGS_TO_ANALYZE}")

    permut_flags = []
    if args.flags_set:
        permut_flags = parse_flags(args.flags_set)
        print(f"Permuting the following flags: {permut_flags}")

    # Implicit operands, such as RAX:RDX in CQO
    if args.implicit_operands:
        parsed = [Instruction.parse_operand(
            s.strip(), None) for s in args.implicit_operands.split(',')]
        instruction.set_implicit(list(parsed))

    if args.extreme or len(TestCase.dynamic_operands(instruction)) < 2:
        TestCase.GOOD_TEST_VALUES += [i for i in range(0, 256)]

    print(
        f"Testing instruction {instruction} with more than {len(TestCase.GOOD_TEST_VALUES)} values (all combinations)")

    test_cases = TestCase.auto_learn_flags(instruction, args.result, permut_flags)

    test_cases.sort(key=lambda t: t.test_id())

    test_cases_str = []
    tids = []
    for test_case in test_cases:
        try:
            prev_tid = tid = test_case.test_id()
            ts = str(test_case)
            ctr = 0
            while tid in tids:
                tid = tid + "_" + str(random.randint(0, 100))
                ctr += 1
                if ctr > 100:
                    continue

            test_cases_str.append(ts.replace(prev_tid, tid))
            tids.append(tid)
        except Exception as e:
            print(f"Failed to generate test case {test_case}: {e}")
            pass

    print(f"Found {len(test_cases_str)} test cases for {assembly_code}")

    too_many = False
    TOO_MANY_TRESHOLD = 50
    if len(test_cases_str) > TOO_MANY_TRESHOLD:
        print(f"Too many test cases, only a sample of {TOO_MANY_TRESHOLD} will be returned")
        test_cases_str = random.sample(test_cases_str, TOO_MANY_TRESHOLD)
        too_many = True

    tests = "\n\n".join(test_cases_str)
    clipboard = False
    try:
        pyperclip.copy(tests)
        clipboard = True
    except:
        pass

    print(tests)

    print(f"Generated {len(test_cases_str)} tests" + " and copied them to the clipboard" if clipboard else "")

    if too_many:
        print(f"Note that too many test cases were generated, so only a sample of {TOO_MANY_TRESHOLD} was returned")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        raise e
    finally:
        if delete_at_exit:
            shutil.rmtree(temp_dir_filesystem)
