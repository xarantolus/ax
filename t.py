from multiprocessing.dummy import Pool
import re
import shutil
import pyperclip
import abc
from curses.ascii import isspace
import os
import random
import subprocess
from tqdm import tqdm
import sys
import tempfile
from typing import List, Literal, Union
import unittest

qword_registers = ["rip", "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp",
                   "rsp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"]
dword_registers = ["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp",
                   "esp", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d"]
word_registers = ["ax", "bx", "cx", "dx", "si", "di", "bp", "sp",
                  "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w"]
byte_registers = ["al", "ah", "bl", "bh", "cl", "ch", "dl", "dh", "sil", "dil",
                  "bpl", "spl", "r8b", "r9b", "r10b", "r11b", "r12b", "r13b", "r14b", "r15b"]

registers = (byte_registers + word_registers +
             dword_registers + qword_registers)
# Sorting makes sure we find e.g. "rax" first instead of "ax"
registers.sort(key=len, reverse=True)

FLAG_CF: int = 0x0001
FLAG_PF: int = 0x0004
FLAG_ZF: int = 0x0040
FLAG_SF: int = 0x0080
FLAG_OF: int = 0x0800
FLAGS = [
    (FLAG_CF, "CF"),
    (FLAG_PF, "PF"),
    (FLAG_ZF, "ZF"),
    (FLAG_SF, "SF"),
    (FLAG_OF, "OF"),
]

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


class Operand(abc.ABC):
    @abc.abstractclassmethod
    def size(self) -> int:
        pass

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
        self.size()

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
    def __init__(self, number):
        if isinstance(number, str):
            number = int(number, base=0)
        self.number = number

    def hexify(self, target: Operand):
        if pow(2, target.size() * 8) <= self.number:
            raise ValueError(
                f"Number {hex(self.number)} too big for target register {target.name}")

        if self.number >= (2147483647):
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
    def __init__(self, base_register, offset=None, scale=1, index_register=None, size=None):
        self.base_register = base_register if isinstance(
            base_register, RegisterOperand) else RegisterOperand(base_register)

        self.offset = 0 if offset is None else offset
        self.scale = 1 if scale is None else scale

        self.index_register = index_register if isinstance(
            index_register, RegisterOperand) else (
                RegisterOperand(index_register) if index_register != None else None)

        assert size in [1, 2, 4, 8]
        self._size = size

    def size(self) -> Literal[1, 2, 4, 8]:
        return self._size

    def __eq__(self, other):
        return self.base_register == other.base_register and self.offset == other.offset and self.scale == other.scale and self.index_register == other.index_register and self._size == other._size

    def __str__(self):
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
            if self.offset == 0:
                return f"{size_prefix}[{self.base_register.name}]"
            else:
                return f"{size_prefix}[{self.base_register.name}+{self.offset}]"
        else:
            if self.offset == 0:
                if self.scale != 1:
                    return f"{size_prefix}[{self.base_register.name}+{self.scale}*{self.index_register.name}]"
                else:
                    return f"{size_prefix}[{self.base_register.name}+{self.index_register.name}]"
            else:
                raise ValueError("cannot have offset and index register")

    @staticmethod
    def parse(argument: str, other_operand: Union[Operand, None]):
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
            argument = argument[len(offset)-1:]
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
            raise ValueError("Could not parse memory argument: " + original_argument + "(rest is " + argument + ")"
                             "\nNote that the parser is very basic and cares about order")

        return MemoryOperand(base_register, offset, scale, index_register, size)


class Instruction:
    def __init__(self, mnemonic: str, arguments: List[Operand], additional_imm: Union[ImmediateOperand, None], implicit: List[Operand] = []):
        self.mnemonic = mnemonic.lower()
        self.arguments = arguments
        self.implicit_arguments = implicit
        # currently only 0-2 operands are supported
        assert len(self.arguments) + len(self.implicit_arguments) <= 2
        self.additional_imm = additional_imm

    def set_implicit(self, implicit: List[Operand]):
        self.implicit_arguments = implicit
        assert len(self.arguments) + len(self.implicit_arguments) <= 2

    @staticmethod
    def parse_operand(argument: str, other_operand: Union[Operand, None]):
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
    def parse(argument: str):
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

    def __str__(self):
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
            raise ValueError(
                "str not implement for Instruction with more than 2 operands")


class Tests(unittest.TestCase):
    def test_parse_memory_operand(self):
        self.assertEqual(MemoryOperand.parse(
            "[rsp+8* Rcx]", RegisterOperand("al")),
            MemoryOperand("rsp", 0, 8, "rcx", 1),
        )
        self.assertEqual(
            MemoryOperand.parse(
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
        self.assertEqual(
            instr.arguments[1].base_register, RegisterOperand("rsp"))
        self.assertEqual(instr.arguments[1].offset, 8)

        instr = Instruction.parse("mov [rsp+4*rcx], rax")
        self.assertEqual(instr.mnemonic, "mov")
        self.assertEqual(
            instr.arguments[0].base_register, RegisterOperand("rsp"))
        self.assertEqual(instr.arguments[0].offset, 0)
        self.assertEqual(instr.arguments[0].scale, 4)
        self.assertEqual(
            instr.arguments[0].index_register, RegisterOperand("rcx"))
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


def assemble(instruction: Union[Instruction, str]):
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


def test_id(instruction: Union[Instruction, str], flags_set, inputs=None):
    def map_flags(f):
        # remove the FLAG_ prefix from each flag
        return [x[5:] for x in f]

    # generate name from instruction string and flags set, but replaces spaces and commas with _

    test_name = f"{instruction}_{'_'.join(map_flags(flags_set))}"

    if isinstance(instruction, Instruction) and len(instruction.implicit_arguments) > 0 and inputs is not None:
        test_name += f"_{'_'.join([f'{op}_{inputs[i+len(instruction.arguments)]}' for i, op in enumerate(instruction.implicit_arguments)])}"

    # only keep alphanumerial characters
    test_name = re.sub(r'\W+', '_', test_name)

    # replace consecutive _ with only one
    test_name = re.sub(r'_+', '_', test_name)

    return test_name.strip("_").lower()


def flags_to_str(set, notset):
    def joinflags(flags):
        return " | ".join(flags) if len(flags) > 0 else "0"

    return f"{joinflags(set)}; {joinflags(notset)}"


class TestCase:
    def __init__(self, assembled, instruction: Instruction, set_flags: List[str], flags_not_set: List[str], operand_values: List[int], expected_values: List[int]):
        self.instruction = instruction if isinstance(
            instruction, Instruction) else Instruction.parse(instruction)
        assert isinstance(instruction, Instruction)

        assert len(operand_values) == len(expected_values)

        self.assembled_bytes = assembled

        self.flags_set = set_flags
        self.flags_not_set = flags_not_set
        self.operand_values = operand_values
        self.expected_values = expected_values

    GOOD_TEST_VALUES = list(dict.fromkeys([
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
        [2**i for i in range(64)]
    ))

    @staticmethod
    def dynamic_operands(i: Instruction) -> List[Operand]:
        return list(filter(lambda o: not isinstance(o, ImmediateOperand), i.arguments + i.implicit_arguments))

    @staticmethod
    def auto_learn_flags(i: Instruction, result_only: bool) -> List:
        dynamic_operands = len(TestCase.dynamic_operands(i))
        if dynamic_operands == 0:
            return TestCase.learn_flags(i, [[]], result_only)
        elif dynamic_operands == 1:
            return TestCase.learn_flags(i,
                                        [[v]
                                            for v in TestCase.GOOD_TEST_VALUES]
                                        + [i for i in range(0, 1024)]
                                        + [[random.randint(0, 2**64)]
                                           for _ in range(50)],
                                        result_only
                                        )
        elif dynamic_operands == 2:
            return TestCase.learn_flags(i,
                                        [[v1, v2]
                                            for v1 in TestCase.GOOD_TEST_VALUES for v2 in TestCase.GOOD_TEST_VALUES]
                                        # random values and good test values
                                        + [[random.randint(0, 2**64), v]
                                            for v in TestCase.GOOD_TEST_VALUES]
                                        + [[v, random.randint(0, 2**64)]
                                            for v in TestCase.GOOD_TEST_VALUES]
                                        # 50 random combinations
                                        + [[random.randint(0, 2**64), random.randint(0, 2**64)]
                                            for _ in range(50)],
                                        result_only
                                        )
        else:
            raise NotImplementedError()

    NEWLINE = "\n"

    last_exception = None

    @staticmethod
    def learn_single_flags(i: int, assembled, instruction: Instruction, operand_values: List[int], tmpdir: str):
        try:
            setup_code = []

            dynamic_operands = TestCase.dynamic_operands(instruction)

            idx = 0
            for arg in dynamic_operands:
                if isinstance(arg, RegisterOperand):
                    setup_code.append(
                        f"mov {arg}, {operand_values[idx]}")
                    idx += 1
                elif isinstance(arg, MemoryOperand):
                    # move rsp to base register to make sure memory operands
                    if arg.base_register != RegisterOperand("rsp"):
                        setup_code.append(f"mov {arg.base_register}, rsp")
                    # set index register to 1
                    if arg.index_register is not None:
                        setup_code.append(f"mov {arg.index_register}, 0")
                    # write to memory
                    setup_code.append(
                        f"mov {arg}, {operand_values[idx]}")
                    idx += 1
                else:
                    raise ValueError("invalid operand" + arg)

            assert idx == len(
                dynamic_operands), "Not all setup operand values were used"

            assert len(
                dynamic_operands) <= 2, "Too many dynamic operands"

            # write assembly code to file
            assembly_path = os.path.join(tmpdir, f"{i}.asm")

            def get_rax(op):
                return {1: 'al', 2: 'ax', 4: 'eax', 8: 'rax'}[op.size()]

            with open(assembly_path, "w", encoding='utf8') as f:
                f.write(
                    f""".intel_syntax noprefix
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
                mov rax, 0x0000000000000000
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
                mov rdx, 8
                syscall


                mov rax, 1
                mov rdi, 1
                lea rsi, [rip+output_val]
                mov rdx, 16
                syscall

                mov rax, 60
                mov rdi, 0

                syscall
                """)

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
            output = subprocess.run(
                [executable_path], stdout=subprocess.PIPE).stdout

            os.remove(executable_path)

            assert len(output) == 24, "Output is not 24 bytes long"

            rflags = int.from_bytes(
                output[:8], byteorder="little", signed=False)

            # find out which flags were set
            set_flags, flags_not_set = [], []
            for flag, flag_name in FLAGS:
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
                    [],
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
                    operand_values,
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
                    operand_values,
                    [output_op_val1, output_op_val2],
                )
            else:
                raise ValueError(
                    "invalid number of dynamic operands")
        except Exception as e:
            TestCase.last_exception = e
            return None

    @staticmethod
    def learn_flags(instruction: Instruction, operand_values_arg: List[List[int]], result_only: bool):
        results: List[TestCase] = []

        assembled = assemble(instruction)

        def is_new(flags_set, flags_not_set):
            if result_only:
                return True

            for ts in results:
                if ts.flags_set == flags_set and ts.flags_not_set == flags_not_set:
                    return False
            return True

        with tempfile.TemporaryDirectory(prefix="ax_flag_learner", dir="/dev/shm") as tmpdir:
            with Pool(os.cpu_count() * 4) as p:
                temp_results = list(
                    tqdm(
                        p.imap(lambda tpl: TestCase.learn_single_flags(
                            tpl[0],  assembled, instruction, tpl[1], tmpdir
                        ), enumerate(operand_values_arg)),
                        total=len(operand_values_arg)
                    ))

            for r in temp_results:
                if r is not None and is_new(r.flags_set, r.flags_not_set):
                    # Only keep tests we know will work later (sometimes immediate values are too large, but this is ignored by as)
                    try:
                        str(r)
                    except Exception as e:
                        continue

                    results.append(r)

            if len(results) == 0:
                raise ValueError(
                    f"""Could not learn any flags for instruction {instruction}, likely due to some bug with with the flag learner
                    Here's the last exception:
                    {TestCase.last_exception}""")
        return results

    def test_id(self):
        return test_id(self.instruction, self.flags_set, self.operand_values)

    def __str__(self):
        dynamic_operands = self.dynamic_operands(self.instruction)

        if len(dynamic_operands) == 0:
            return f"""// {self.instruction}
ax_test![{self.test_id()}; {", ".join(self.assembled_bytes)}; |a: Axecutor| {{
        todo!("Asset state of registers");
    }};
    ({flags_to_str(self.flags_set, self.flags_not_set)})
];"""
        elif len(dynamic_operands) == 1:
            if isinstance(dynamic_operands[0], RegisterOperand):
                return f"""// {self.instruction}
ax_test![{self.test_id()}; {", ".join(self.assembled_bytes)};
    |a: &mut Axecutor| {{
        write_reg_value!({dynamic_operands[0].size_letter()}; a; {dynamic_operands[0].name.upper()}; {ImmediateOperand(self.operand_values[0]).hexify(dynamic_operands[0])});
    }};
    |a: Axecutor| {{
        assert_reg_value!({dynamic_operands[0].size_letter()}; a; {dynamic_operands[0].name.upper()}; {ImmediateOperand(self.expected_values[0]).hexify(dynamic_operands[0])});
    }};
    ({flags_to_str(self.flags_set, self.flags_not_set)})
];"""
            elif isinstance(dynamic_operands[0], MemoryOperand):
                mem_start = 0x1000
                return f"""// {self.instruction}
ax_test![{self.test_id()}; {", ".join(self.assembled_bytes)};
    |a: &mut Axecutor| {{
        write_reg_value!({dynamic_operands[0].base_register.size_letter()}; a; {dynamic_operands[0].base_register.name.upper()}; {hex(mem_start)});{
        f'{TestCase.NEWLINE}        write_reg_value!({dynamic_operands[0].index_register.size_letter()}; a; {dynamic_operands[0].index_register.name.upper()}; 0);' if dynamic_operands[0].index_register is not None else ''}
        a.mem_init_zero({hex(mem_start +  + dynamic_operands[0].offset)}, {dynamic_operands[0].size()}).unwrap();
        a.mem_write_{dynamic_operands[0].size() * 8}({hex(mem_start + dynamic_operands[0].offset)}, {ImmediateOperand(self.operand_values[0]).hexify(dynamic_operands[0])}).unwrap();
    }};
    |a: Axecutor| {{
        assert_reg_value!({dynamic_operands[0].base_register.size_letter()}; a; {dynamic_operands[0].base_register.name.upper()}; {hex(mem_start)});
        assert_mem_value!({dynamic_operands[0].size_letter()}; a; {hex(mem_start + dynamic_operands[0].offset)}; {ImmediateOperand(self.expected_values[0]).hexify(dynamic_operands[0])});
    }};
    ({flags_to_str(self.flags_set, self.flags_not_set)})
];"""
            else:
                raise ValueError("invalid dynamic operand")

        elif len(dynamic_operands) == 2:
            if isinstance(dynamic_operands[0], RegisterOperand) and isinstance(dynamic_operands[1], RegisterOperand):
                return f"""// {self.instruction}
ax_test![{self.test_id()}; {", ".join(self.assembled_bytes)}; |a: &mut Axecutor| {{
        write_reg_value!({dynamic_operands[0].size_letter()}; a; {dynamic_operands[0].name.upper()}; {ImmediateOperand(self.operand_values[0]).hexify(dynamic_operands[0])});
        write_reg_value!({dynamic_operands[1].size_letter()}; a; {dynamic_operands[1].name.upper()}; {ImmediateOperand(self.operand_values[1]).hexify(dynamic_operands[1])});
    }};
    |a: Axecutor| {{
        assert_reg_value!({dynamic_operands[0].size_letter()}; a; {dynamic_operands[0].name.upper()}; {ImmediateOperand(self.expected_values[0]).hexify(dynamic_operands[0])});
        assert_reg_value!({dynamic_operands[1].size_letter()}; a; {dynamic_operands[1].name.upper()}; {ImmediateOperand(self.expected_values[1]).hexify(dynamic_operands[1])});
    }};
    ({flags_to_str(self.flags_set, self.flags_not_set)})
];"""
            elif isinstance(dynamic_operands[0], RegisterOperand) and isinstance(dynamic_operands[1], MemoryOperand):
                mem_start = 0x1000
                return f"""// {self.instruction}
ax_test![{self.test_id()}; {", ".join(self.assembled_bytes)};
    |a: &mut Axecutor| {{
        write_reg_value!({dynamic_operands[0].size_letter()}; a; {dynamic_operands[0].name.upper()}; {ImmediateOperand(self.operand_values[0]).hexify(dynamic_operands[0])});
        write_reg_value!({dynamic_operands[1].base_register.size_letter()}; a; {dynamic_operands[1].base_register.name.upper()}; {hex(mem_start)});{
        f'{TestCase.NEWLINE}        write_reg_value!({dynamic_operands[1].index_register.size_letter()}; a; {dynamic_operands[1].index_register.name.upper()}; 0);' if dynamic_operands[1].index_register is not None else ''}
        a.mem_init_zero({hex(mem_start + dynamic_operands[1].offset)}, {dynamic_operands[1].size()}).unwrap();
        a.mem_write_{dynamic_operands[1].size() * 8}({hex(mem_start + dynamic_operands[1].offset)}, {ImmediateOperand(self.operand_values[1]).hexify(dynamic_operands[1])}).unwrap();
    }};
    |a: Axecutor| {{
        assert_reg_value!({dynamic_operands[0].size_letter()}; a; {dynamic_operands[0].name.upper()}; {ImmediateOperand(self.expected_values[0]).hexify(dynamic_operands[0])});
        assert_mem_value!({dynamic_operands[1].size_letter()}; a; {hex(mem_start + dynamic_operands[1].offset)}; {ImmediateOperand(self.expected_values[1]).hexify(dynamic_operands[1])});
    }};
    ({flags_to_str(self.flags_set, self.flags_not_set)})
];"""
            elif isinstance(dynamic_operands[0], MemoryOperand) and isinstance(dynamic_operands[1], RegisterOperand):
                mem_start = 0x1000
                return f"""// {self.instruction}
ax_test![{self.test_id()}; {", ".join(self.assembled_bytes)};
    |a: &mut Axecutor| {{
        write_reg_value!({dynamic_operands[1].size_letter()}; a; {dynamic_operands[1].name.upper()}; {ImmediateOperand(self.operand_values[1]).hexify(dynamic_operands[1])});
        write_reg_value!({dynamic_operands[0].base_register.size_letter()}; a; {dynamic_operands[0].base_register.name.upper()}; {hex(mem_start)});{
        f'{TestCase.NEWLINE}        write_reg_value!({dynamic_operands[0].index_register.size_letter()}; a; {dynamic_operands[0].index_register.name.upper()}; 0);' if dynamic_operands[0].index_register is not None else ''}
        a.mem_init_zero({hex(mem_start +  + dynamic_operands[0].offset)}, {dynamic_operands[0].size()}).unwrap();
        a.mem_write_{dynamic_operands[0].size() * 8}({hex(mem_start + dynamic_operands[0].offset)}, {ImmediateOperand(self.operand_values[0]).hexify(dynamic_operands[0])}).unwrap();
    }};
    |a: Axecutor| {{
        assert_reg_value!({dynamic_operands[1].size_letter()}; a; {dynamic_operands[1].name.upper()}; {ImmediateOperand(self.expected_values[1]).hexify(dynamic_operands[1])});
        assert_mem_value!({dynamic_operands[0].size_letter()}; a; {hex(mem_start + dynamic_operands[0].offset)}; {ImmediateOperand(self.expected_values[0]).hexify(dynamic_operands[0])});
    }};
    ({flags_to_str(self.flags_set, self.flags_not_set)})
];"""
        else:
            raise ValueError("invalid number of dynamic operands")


def main():
    import argparse

    parser = argparse.ArgumentParser(description='Generate tests for axecutor')
    parser.add_argument(
        '-t', '--test', help='Run tests for this script', dest='test', action='store_true')
    parser.add_argument(
        '-f', '--flags', help='Select flags to test for', action='store', dest='flags',)
    parser.add_argument('-e',                       '--extreme',
                        help='Run more tests (default for < 2 dynamic arguments)', action='store_true', dest='extreme',)
    parser.add_argument("-i", "--implicit-operands",
                        help="Test implicit operands", action="store", dest="implicit_operands")
    parser.add_argument("-r", "--result", help="Deduplicate by result of test, not by flags set",
                        action="store_true", dest="result")

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
        FLAGS = list(filter(lambda t: t[1] in flags, FLAGS))

        print(f"Testing flags: {FLAGS}")

    # Implicit operands, such as RAX:RDX in CQO
    if args.implicit_operands:
        parsed = [Instruction.parse_operand(
            s.strip(), None) for s in args.implicit_operands.split(',')]
        instruction.set_implicit(list(parsed))

    if args.extreme or len(TestCase.dynamic_operands(instruction)) < 2:
        TestCase.GOOD_TEST_VALUES += [i for i in range(0, 256)]

    print(
        f"Testing instruction {instruction} with more than {len(TestCase.GOOD_TEST_VALUES)} values (all combinations)")

    test_cases = TestCase.auto_learn_flags(instruction, args.result)

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
    if len(test_cases_str) > 25:
        print("Too many test cases, only a sample of 25 will be returned")
        test_cases_str = random.sample(test_cases_str, 25)
        too_many = True

    tests = "\n\n".join(test_cases_str)
    try:
        pyperclip.copy(tests)
        print(f"Copied {len(test_cases_str)} tests to clipboard")
    except:
        pass

    print(tests)

    if too_many:
        print("Note that too many test cases were generated, so only a sample of 25 was returned")


if __name__ == "__main__":
    try:
        main()
    finally:
        if delete_at_exit:
            shutil.rmtree(temp_dir_filesystem)
