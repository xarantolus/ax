import os
import re

def is_instruction_file(content):
    return "impl Axecutor" in content and \
           "pub fn mnemonic" in content and \
           "match i.code" in content

def count_test_cases(content):
    return content.count("ax_test!") + content.count("test_async!") + content.count("jmp_test!")

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
        count_test_cases(content)
    )


def replace_in_readme(mnemonic_count, opcode_count):
    # Replace the text between the two "<!-- stats-count-marker -->"
    # with "x mnemonics/y opcodes"
    with open("README.md", "r") as f:
        content = f.read()

    start = content.find("<!-- stats-count-marker -->")
    end = content.find("<!-- stats-count-marker -->", start + 1)

    new_content = content[:start + len("<!-- stats-count-marker -->")] + \
                  f"{mnemonic_count} mnemonics/{opcode_count} opcodes" + \
                  content[end:]

    with open("README.md", "w") as f:
        f.write(new_content)



fmt = "{: <15} | {: <15} | {: <15} | {: <15} | {: <15} | {: <15}"

print(fmt.format("File", "Available", "Implemented", "Unimplemented", "% Implemented", "Test Cases"))
print(fmt.format("-" * 15, "-" * 15, "-" * 15, "-" * 15, "-" * 15, "-" * 15))

files = os.listdir("src/instructions")

total_opcodes = 0
total_implemented = 0
total_partial = 0
total_full = 0
total_test_cases = 0

for file in files:
    with open("src/instructions/" + file, "r") as f:
        content = f.read()

        if not is_instruction_file(content):
            continue

        opcodes, implemented, unimplemented, tests = info_mnemonic(content)

        print(fmt.format(file, opcodes, implemented, unimplemented, "{:.2f}%".format(implemented / opcodes * 100), tests))

        total_opcodes += opcodes
        total_implemented += implemented
        total_test_cases += tests

        if implemented == opcodes:
            total_full += 1
        elif implemented > 0:
            total_partial += 1

print(fmt.format("-" * 15, "-" * 15, "-" * 15, "-" * 15, "-" * 15, "-" * 15))

print(fmt.format("Total", total_opcodes, total_implemented, total_opcodes - total_implemented, "{:.2f}%".format(total_implemented / total_opcodes * 100), total_test_cases))


print(f"{total_full} implemented mnemonics, {total_partial} partially implemented mnemonics -> {total_full + total_partial} implemented mnemonics")

replace_in_readme(total_full + total_partial, total_implemented)
