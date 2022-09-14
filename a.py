
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

        code = f"""// {assembly_code}
ax_test![test_{test_id}; {", ".join(hex_arr)}; |s: MachineState| {{
	assert_reg_value!(s; RBX; 0x10);
	todo!("write test cases for \\\"{assembly_code}\\\"");
}}];"""

        pyperclip.copy(code)
        print("Copied to clipboard!")
