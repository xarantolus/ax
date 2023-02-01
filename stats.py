from __future__ import annotations
import os
import re


def is_instruction_file(content: str) -> bool:
    return "impl Axecutor" in content and \
        "pub(crate) fn mnemonic" in content and \
        "match i.code" in content


def count_test_cases(content: str) -> int:
    return content.count("ax_test!") + content.count("test_async!") + content.count("jmp_test!")


def extract_instruction_functions(content: str) -> list[str]:
    # Basically we find the start of all opcode implementations and then find the end/matching brace.
    regex = re.compile(r"fn instr_.*?\{", re.DOTALL)

    functions_text = []

    for match in regex.finditer(content):
        lvl = 0

        # This matching is really simple (and could be broken by braces in comments etc.), but it's good enough
        for i in range(match.start(), len(content)):
            if content[i] == "{":
                lvl += 1
            elif content[i] == "}":
                lvl -= 1
                if lvl <= 0:
                    functions_text += [content[match.start():i + 1]]
                    break

    return functions_text


def mnemonic_info(content: str) -> tuple[int, int, int, int]:
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


def replace_in_readme(total_mnemonic_count: int, full_count: int, partial_count: int, opcode_count: int) -> None:
    with open("README.md", "r") as f:
        content = f.read()

    start = content.find("<!-- stats-count-marker -->")
    end = content.find("<!-- stats-count-marker -->", start + 1)

    new_content = content[:start + len("<!-- stats-count-marker -->")]
    new_content += f"{opcode_count} opcodes for {total_mnemonic_count} mnemonics "
    new_content += f"({full_count} complete, {partial_count} partial)"
    new_content += content[end:]

    with open("README.md", "w") as f:
        f.write(new_content)


def main():
    fmt = "{: <15} | {: >9} | {: >11} | {: >13} | {: >13} | {: >10}"

    print(fmt.format("File", "Available", "Implemented", "Unimplemented", "% Implemented", "Test Cases"))
    print(fmt.format("-" * 15, "-" * 9, "-" * 11, "-" * 13, "-" * 13, "-" * 10))

    files = os.listdir("src/instructions")

    total_opcodes = 0
    opcodes_implemented = 0
    total_partial = 0
    total_full = 0
    total_test_cases = 0

    for file in files:
        with open("src/instructions/" + file, "r") as f:
            content = f.read()

            if not is_instruction_file(content):
                continue

            opcodes, implemented, unimplemented, tests = mnemonic_info(content)

            print(fmt.format(file, opcodes, implemented, unimplemented, "{:.2f}%".format(implemented / opcodes * 100),
                             tests))

            total_opcodes += opcodes
            opcodes_implemented += implemented
            total_test_cases += tests

            if implemented == opcodes:
                total_full += 1
            elif implemented > 0:
                total_partial += 1

    print(fmt.format("-" * 15, "-" * 9, "-" * 11, "-" * 13, "-" * 13, "-" * 10))

    print(fmt.format("Total", total_opcodes, opcodes_implemented, total_opcodes - opcodes_implemented,
                     "{:.2f}%".format(opcodes_implemented / total_opcodes * 100), total_test_cases))

    print(f"{total_full} fully implemented, {total_partial} partially implemented mnemonics ->",
          f"{total_full + total_partial} total")

    replace_in_readme(total_full + total_partial, total_full, total_partial, opcodes_implemented)


if __name__ == '__main__':
    main()
