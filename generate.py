# Run cargo metadata
# Parse the output
import json
import os
import subprocess
import sys

# Run cargo metadata
output = subprocess.run(
    ["cargo", "metadata", "--format-version", "1"],
    stdout=subprocess.PIPE,
    stderr=None,
    encoding="utf-8",
)

if output.returncode != 0:
	print("Failed to run cargo metadata")
	exit(output.returncode);

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
    for i,v in enumerate(lst):
        if pred(v):
            return i
    return None


# read_mnemonics reads all mnemonic names from the iced-x86 crate
def read_mnemonics():
	mnemonic_rs_path = os.path.join(iced_package_dir, "mnemonic.rs")
	with open(mnemonic_rs_path, "r", encoding='utf8') as f:
		mnemonic_rs = f.readlines()

	start_idx = index_of_first(mnemonic_rs, lambda line: line.startswith("pub enum Mnemonic"))
	mnemonic_rs = mnemonic_rs[start_idx+1:]

	end_idx = index_of_first(mnemonic_rs, lambda line: line.startswith("}"))
	mnemonic_rs = mnemonic_rs[:end_idx]

	mnemonics = map(lambda line: line.strip().split(",")[0].split("=")[0].strip(), mnemonic_rs)

	mnemonics = filter(lambda mnemonic: not mnemonic.startswith("//"), mnemonics)

	return list(mnemonics)

# Data class with Instruction description, encoding example, isa extension name
class Instruction:
	def __init__(self, description, code, isa):
		self.description = description
		self.code = code
		self.isa = isa

# read_code reads all instruction codes from the iced-x86 crate
def read_code():
	code_rs_path = os.path.join(iced_package_dir, "code.rs")
	with open(code_rs_path, "r", encoding='utf8') as f:
		code_rs = f.readlines()

	start_idx = index_of_first(code_rs, lambda line: line.startswith("pub enum Code"))
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
				instructions.append(Instruction(current_description, current_code, current_isa))
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


print(len(read_code()))
print(len(read_mnemonics()))
