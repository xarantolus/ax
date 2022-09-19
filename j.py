# generate jump definitions
from t import *

import pyperclip

NEWLINE = '\n'


def generate_assembly(initial: str, padding: int, final: str):
    return f"""
.intel_syntax noprefix
.data
rflags_dest: .space 8
initial_rip: .space 8
final_rip: .space 8
.text
.global _start
_start:
mov rax, 0x00000002
push rax
POPFQ
# save current address (should be equal to rip?) - "." means current address in gnu assembler
lea rax, [rip]
mov [rip+initial_rip], rax;
{initial}
.rept   {padding}
.byte 0x90 # nop
.endr
{final}
# finalize
lea rax, [rip] # 0x48, 0x8d, 0x5, 0x0, 0x0, 0x0, 0x0
mov [rip+final_rip], rax
PUSHFQ
pop rax
mov [rflags_dest], rax

# write 24 bytes to stdout
mov rax, 1
mov rdi, 1
lea rsi, [rip+rflags_dest]
mov rdx, 24
syscall

# exit
mov rax, 60
mov rdi, 0
syscall
"""


class JumpTestCase:
    # set_flags, flags_not_set, initial_rip, final_rip, initial_bytes, final_bytes, padding
    def __init__(self, set_flags: List[str], flags_not_set: List[str], initial_rip: int, final_rip: int, initial_bytes: List[str], final_bytes: List[str], padding: int, initial_code: str, final_code: str):
        self.set_flags = set_flags
        self.flags_not_set = flags_not_set
        self.initial_rip = initial_rip
        self.final_rip = final_rip
        self.initial_bytes = initial_bytes
        self.final_bytes = final_bytes
        self.padding = padding
        self.initial_code = initial_code
        self.final_code = final_code

    @staticmethod
    def _sublist_index(sublist: List[str], superlist: List[str]):
        sublist_len = len(sublist)
        for i in range(len(superlist) - sublist_len + 1):
            if superlist[i:i+sublist_len] == sublist:
                return i
        return -1

    @staticmethod
    def create(initial: str, padding: int, final: str):
        with tempfile.TemporaryDirectory(prefix="ax_jumper_", dir="/dev/shm") as tmpdir:
            # write assembly code to file
            assembly_path = os.path.join(tmpdir, "a.asm")
            with open(assembly_path, "w", encoding='utf8') as f:
                f.write(generate_assembly(initial, padding, final))

            # assemble with as
            object_path = os.path.join(tmpdir, "a.o")
            subprocess.run(["as", "-o", object_path, assembly_path])

            # turn into executable with gcc, symbol _start
            executable_path = os.path.join(tmpdir, "a")
            subprocess.run(["gcc", "-m64", "-nostdlib", "-static",
                            "-o", executable_path, object_path])

            # run executable and capture 24 bytes of output
            output = subprocess.run(
                [executable_path], stdout=subprocess.PIPE).stdout

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

            initial_rip = int.from_bytes(
                output[8:16], byteorder="little", signed=False)
            final_rip = int.from_bytes(
                output[16:24], byteorder="little", signed=False)

            final_rip -= 7  # mov [rip+initial_rip]
            final_rip -= 7  # lea rax, [rip]

            # now use the other code to get the assembled bytes, because getting it here would be a pain
            hex_arr = assemble(f"""
                {initial}
                .rept   {hex(padding)}
                .byte 0x90 # nop
                .endr
                {final}
                """)

            repeated_bytes = [hex(0x90)] * padding
            index = JumpTestCase._sublist_index(repeated_bytes, hex_arr)
            assert index != -1, "Could not find repeated bytes in hex_arr"

            initial_bytes = hex_arr[:index]
            final_bytes = hex_arr[index + padding:]

            return JumpTestCase(set_flags, flags_not_set, initial_rip, final_rip, initial_bytes, final_bytes, padding, initial, final)

    def with_setup_asserts(self):
        return f"""jmp_test![{test_id(self.initial_code + "_" + self.final_code, self.set_flags)};
    start: {hex(self.initial_rip)}; end: {hex(self.final_rip)};
    {', '.join(self.initial_bytes)}; // {self.initial_code}
    {self.padding}; // {self.padding} bytes of 0x90 (nop) as padding
    {', '.join(self.final_bytes)}; // {self.final_code}
    |a: &mut Axecutor| {{
        todo!("write setup code");
    }};
    |a: Axecutor| {{
        todo!("Write more assertions. RIP and flags are already covered");
    }};
    ({flags_to_str(self.set_flags, self.flags_not_set)})
];"""

    def no_setup_asserts(self):
        return f"""jmp_test![{test_id(self.initial_code + "_" + self.final_code, self.set_flags)};
    start: {hex(self.initial_rip)}; end: {hex(self.final_rip)};
    {', '.join(self.initial_bytes)}; // {self.initial_code}
    {self.padding}; // {self.padding} bytes of 0x90 (nop) as padding
    {', '.join(self.final_bytes)}; // {self.final_code}
    ({flags_to_str(self.set_flags, self.flags_not_set)})
];"""


if __name__ == '__main__':
    if len(sys.argv) == 1:
        sys.argv += ["mov rax, 5; JMP .label", "50", ".label: sub rax, 1", ]

    # 3 positional arguments: <initial_instructions> padding <final_instructions>
    if len(sys.argv) != 4:
        print("Invalid input, expected 3 arguments: <initial_instructions> padding <final_instructions>")
        sys.exit(1)

    # First argument is x86-64 assembly code
    code_start = sys.argv[1]
    # Second argument is padding
    padding = int(sys.argv[2], base=0)
    # Third argument is x86-64 assembly code
    code_end = sys.argv[3]

    if code_end.strip().endswith(":"):
        print("Error: Final code cannot be a label, as otherwise the test case won't work. You should insert e.g. a NOP")
        sys.exit(1)

    testcase = JumpTestCase.create(code_start, padding, code_end)

    # ask user which variant they want
    setup = input("Include setup/assert code? y/N")

    if setup.lower() == "y":
        tc_str = testcase.with_setup_asserts()
    else:
        tc_str = testcase.no_setup_asserts()

    pyperclip.copy(tc_str)
    print("Copied test case to clipboard!")
