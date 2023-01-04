# Run cargo metadata
# Parse the output
import json
import os
import subprocess
import sys
from typing import List

# Run cargo metadata
output = subprocess.run(
    ["cargo", "metadata", "--format-version", "1"],
    stdout=subprocess.PIPE,
    stderr=None,
    encoding="utf-8",
)

if output.returncode != 0:
    print("Failed to run cargo metadata")
    exit(output.returncode)

# Parse the output
data = json.loads(output.stdout)


def get_package_dir(name):
    for package in data["packages"]:
        if package["name"] != name:
            continue
        if len(package["targets"]) != 1:
            print("Unexpected number of targets")
            exit(1)

        target = package["targets"][0]
        if "lib" not in target["kind"]:
            print("Unexpected target kind")
            exit(1)
        return os.path.dirname(target["src_path"])


iced_package_dir = get_package_dir("iced-x86")


def index_of_first(lst, pred):
    for i, v in enumerate(lst):
        if pred(v):
            return i
    return None


# read_mnemonics reads all mnemonic names from the iced-x86 crate
def read_mnemonics(with_codes: bool) -> List[str] | List:
    mnemonic_rs_path = os.path.join(iced_package_dir, "mnemonic.rs")
    with open(mnemonic_rs_path, "r", encoding='utf8') as f:
        mnemonic_rs = f.readlines()

    start_idx = index_of_first(
        mnemonic_rs, lambda line: line.startswith("pub enum Mnemonic"))
    mnemonic_rs = mnemonic_rs[start_idx+1:]

    end_idx = index_of_first(mnemonic_rs, lambda line: line.startswith("}"))
    mnemonic_rs = mnemonic_rs[:end_idx]

    if with_codes is True:
        mnemonics = map(lambda line: line.strip().split(",")[0].split("="), mnemonic_rs)
        mnemonics = filter(lambda mnemonic: not (mnemonic)[0].startswith("//"), mnemonics)
        mnemonics = filter(lambda line: len(line) == 2, mnemonics)
        mnemonics = list(map(lambda line: (line[0].strip(), int(line[1].strip())), mnemonics))
    else:
        mnemonics = map(lambda line: line.strip().split(
            ",")[0].split("=")[0].strip(), mnemonic_rs)

        mnemonics = filter(
            lambda mnemonic: not mnemonic.startswith("//"), mnemonics)

    return list(mnemonics)

# Data class with Instruction description, encoding example, isa extension name


class Instruction:
    def __init__(self, description, code, isa, enum_name):
        self.description = description
        self.code = code
        self.isa = isa
        self.enum_name = enum_name

# read_code reads all instruction codes from the iced-x86 crate


def read_code() -> List[Instruction]:
    code_rs_path = os.path.join(iced_package_dir, "code.rs")
    with open(code_rs_path, "r", encoding='utf8') as f:
        code_rs = f.readlines()

    start_idx = index_of_first(
        code_rs, lambda line: line.startswith("pub enum Code"))
    code_rs = code_rs[start_idx+1:]

    end_idx = index_of_first(code_rs, lambda line: line.startswith("}"))
    code_rs = code_rs[:end_idx]

    # Now parse something like the following into instruction class; first line is description
    # /// `VCVTTPS2DQ zmm1 {k1}{z}, zmm2/m512/m32bcst{sae}`
    # ///
    # /// `EVEX.512.F3.0F.W0 5B /r`
    # ///
    # /// `AVX512F`
    # ///
    # /// `16/32/64-bit`
    # EVEX_Vcvttps2dq_zmm_k1z_zmmm512b32_sae = 1410,
    instructions = []

    current_description = current_code = current_isa = ""

    for line in code_rs:
        line = line.strip()
        if not line.startswith("///"):
            if current_description != "" and current_code != "" and current_isa != "":
                enum_name = line.split("=")[0].strip()
                instructions.append(Instruction(
                    current_description, current_code, current_isa, enum_name))
                current_description = current_code = current_isa = ""
            continue

        stripped = line[3:].strip()
        if stripped.startswith("`") and stripped.endswith("`"):
            if current_description == "":
                current_description = stripped[1:-1]
            elif current_code == "":
                current_code = stripped[1:-1]
            elif current_isa == "":
                current_isa = stripped[1:-1]
            elif stripped.endswith("-bit`"):
                # ignore these lines
                pass
            else:
                print("Unexpected line: " + line)
                exit(1)

    return instructions


available_codes = read_code()
mnemonics = read_mnemonics(False)


def generate_mnemonic_text(instruction_codes: List[Instruction], mnemonic: str):
    code = f"""use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::{mnemonic};
use iced_x86::OpKind;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;
use crate::instructions::registers::SupportedRegister::*;
use crate::{{calculate_r_rm, calculate_rm_imm, calculate_rm_r, calculate_rm, opcode_unimplemented, fatal_error}};


impl Axecutor {{
    pub fn mnemonic_{mnemonic.lower()}(&mut self, i: Instruction) -> Result<(), AxError> {{
        debug_assert_eq!(i.mnemonic(), {mnemonic});

        match i.code() {{""" + "\n"

    for instr in instruction_codes:
        # Push_imm16 => self.instr_push_imm16(i),
        code += f"""            {instr.enum_name} => self.instr_{instr.enum_name.lower()}(i),
"""

    code += f"""            _ => fatal_error!("Invalid instruction code {{:?}} for mnemonic {mnemonic}", i.code()),
        }}
    }}""" + "\n\n"

    for instr in instruction_codes:
        # Push_imm16 => self.instr_push_imm16(i),
        code += f"""
    /// {instr.description}
    ///
    /// {instr.code}
    fn instr_{instr.enum_name.lower()}(&mut self, i: Instruction) -> Result<(), AxError> {{
        debug_assert_eq!(i.code(), {instr.enum_name});

        opcode_unimplemented!("instr_{instr.enum_name.lower()} for {mnemonic}")
    }}
"""

    code += """}

    #[cfg(test)]
mod tests {
    use iced_x86::Register::*;
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, jmp_test, ax_test, instructions::registers::SupportedRegister, write_reg_value,
    };

}
"""

    return code

def generate_mnemonic_file(mnemonic: str):
    has_underscore = mnemonic.endswith("_")
    mnemonic = mnemonic.lower().strip("_")
    # find mnemonic from mnemonics string list
    normalized_mnemonic_idx = index_of_first(mnemonics, lambda m: m.lower() == mnemonic)

    codes = list(filter(lambda instr: instr.enum_name.lower().startswith(mnemonic + ("_" if has_underscore else '')), available_codes))
    if len(codes) == 0:
        print(f"Warning: no instructions for mnemonic {mnemonic}")
        return

    normalized = mnemonics[normalized_mnemonic_idx]
    text = generate_mnemonic_text(codes, normalized)

    # write to file at src/instructions/{mnemonic.lower()}.rs if not exists
    mnemonic_path = os.path.join("src", "instructions", mnemonic.lower() + ".rs")
    if os.path.exists(mnemonic_path):
        print(f"Warning: file {mnemonic_path} already exists, not overwriting")
        return

    with open(mnemonic_path, "w", encoding='utf8') as f:
        f.write(text)

    # append pub mod mnemonic; to src/instructions/mod.rs if not already present
    mod_path = os.path.join("src", "instructions", "mod.rs")
    with open(mod_path, "r", encoding='utf8') as f:
        mod_rs = f.readlines()

    if f"pub mod {normalized.lower()};" not in mod_rs:
        mod_rs.append(f"pub mod {normalized.lower()};\r")
        with open(mod_path, "w", encoding='utf8') as f:
            f.writelines(mod_rs)

    # run rustfmt on files
    subprocess.run(["rustfmt", "--edition=2021", mnemonic_path, mod_path])

def generate_all_switch():
    mnemonics = read_mnemonics(True)

    # Read list of files ending with ".rs" in src/instructions
    instructions_dir = os.path.join("src", "instructions")
    files = os.listdir(instructions_dir)
    files = list(filter(lambda f: f.endswith(".rs"), files))

    mnems = list(filter(lambda m: m[0].lower()+".rs" in files, mnemonics))

    # Generate match statement
    code = f"""// THIS FILE IS AUTOGENERATED, DO NOT EDIT
// You can regenerate it using `make switch` after creating a new instruction file with `python3 generate.py <mneumonic>`
use std::convert::TryFrom;
use crate::fatal_error;

use super::{{axecutor::Axecutor, errors::AxError}};
use iced_x86::{{
    Instruction,
    Mnemonic::{{self, *}},
}};
use wasm_bindgen::{{prelude::wasm_bindgen}};
use serde::{{Serialize, Deserialize}};

impl Axecutor {{
    pub fn switch_instruction_mnemonic(&mut self, i: Instruction) -> Result<(), AxError> {{
        match i.mnemonic() {{"""

    for (mnemonic, num) in mnems:
        code += f"""            {mnemonic} => self.mnemonic_{mnemonic.lower()}(i),
"""

    code += f"""            _ => Err(AxError::from(format!(
                "cannot execute unimplemented mnemonic {{:?}}",
                i.mnemonic()
            ))),
        }}
    }}
}}
"""

    code += """
#[wasm_bindgen(js_name = Mnemonic)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportedMnemonic {
"""
    for (mnemonic, num) in mnems:
        code += f"    {mnemonic} = {num},\n"

    code += """}

impl SupportedMnemonic {
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

impl TryFrom<Mnemonic> for SupportedMnemonic {
    type Error = AxError;

    fn try_from(mnemonic: Mnemonic) -> Result<Self, Self::Error> {
        Ok(match mnemonic {
"""

    for (mnemonic, num) in mnems:
        code += f"            {mnemonic} => SupportedMnemonic::{mnemonic},\n"

    code += """            _ => {
                fatal_error!(
                    "Conversion from Mnemonic to SupportedMnemonic: mnemonic {:?} is not supported",
                    mnemonic
                );
            }
        })
    }
}
"""


    # Write to file generated.rs
    with open("src/instructions/generated.rs", "w", encoding='utf8') as f:
        f.write(code)
    subprocess.run(["rustfmt", "--edition=2021", "src/instructions/generated.rs"])


if __name__ == '__main__':
    mnemonics_to_generate = None
    # set to first argument and handle invalid inputs
    if len(sys.argv) > 1:
        mnemonics_to_generate = sys.argv[1]

    if mnemonics_to_generate == "switch":
        generate_all_switch()
        exit(0)

    if mnemonics_to_generate == "all":
        for mnemonic in filter(lambda m: m != "INVALID", mnemonics):
            generate_mnemonic_file(mnemonic)
    elif mnemonics_to_generate is not None:
        generate_mnemonic_file(mnemonics_to_generate)
        print(f"Generated file for new mnemonic. To integrate this mnemonic into the project, run `make switch`")
    else:
        print("Usage: python3 generate.py <mnemonic>|switch|all")
        exit(1)
