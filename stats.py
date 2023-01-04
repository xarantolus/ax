import os
import re

def is_instruction_file(content):
    return "impl Axecutor" in content and \
           "pub fn mnemonic" in content and \
           "match i.code" in content

def extract_instruction_functions(content):
    # find all lines starting with "fn instr_" and ending with "{"
    regex = re.compile(r"fn instr_.*?\{", re.DOTALL)

    functions_text = []

    for match in regex.finditer(content):
        lvl = 0

        for i in range(match.start(), len(content)):
            if content[i] == "{":
                lvl += 1
            elif content[i] == "}":
                lvl -= 1
                if lvl <= 0:
                    functions_text += [content[match.start():i+1]]
                    break



    return functions_text

def info_mnemonic(content):
    functions = extract_instruction_functions(content)

    unimplemented = 0
    for function in functions:
        if "opcode_unimplemented!" in function:
            unimplemented += 1

    return (
        len(functions),
        len(functions) - unimplemented,
        unimplemented,
    )


fmt = "{: <15} | {: <15} | {: <15} | {: <15} | {: <15}"

print(fmt.format("File", "Available", "Implemented", "Unimplemented", "Ratio Implemented"))
print(fmt.format("-" * 15, "-" * 15, "-" * 15, "-" * 15, "-" * 15))

files = os.listdir("src/instructions")

total_opcodes = 0
total_implemented = 0
total_mnemonics = 0

for file in files:
    with open("src/instructions/" + file, "r") as f:
        content = f.read()

        if not is_instruction_file(content):
            continue

        opcodes, implemented, unimplemented = info_mnemonic(content)

        print(fmt.format(file, opcodes, implemented, unimplemented, "{:.2f}%".format(implemented / opcodes * 100)))

        total_opcodes += opcodes
        total_implemented += implemented

        if implemented > 0:
            total_mnemonics += 1


print(fmt.format("-" * 15, "-" * 15, "-" * 15, "-" * 15, "-" * 15))

print(fmt.format("Total", total_opcodes, total_implemented, total_opcodes - total_implemented, "{:.2f}%".format(total_implemented / total_opcodes * 100)))


print("Number of at least partially implemented mnemonics:", total_mnemonics)
