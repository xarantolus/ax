set pagination off

set output-radix 16

python

# Define all registers I currently care about
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

gdb.execute('set disassembly-flavor intel')

# Always try to set breakpoint at appropriate entry point
try:
    gdb.execute('b main', to_string=True)
    gdb.execute('b _start', to_string=True)
except:
    try:
        gdb.execute('b _start', to_string=True)
    except:
        pass

# All registers we see should be kept to keep track of their values
REGISTERS_SEEN = dict()

LAST_FLAGS = None

def clear_state():
    global REGISTERS_SEEN
    REGISTERS_SEEN = dict()
    global LAST_FLAGS
    LAST_FLAGS = None

    gdb.write('Cleared saved state\n')

def print_register_info(instr: str):
    def format_register_change(reg: str, old_value: str, new_value: str):
        if old_value == new_value:
            return ""

        if old_value is None:
            return " \033[37m<= \033[32mnew register\033[37m"

        return f" \033[37m<= previous: \033[91m{old_value}\033[37m"


    gdb.write('\n\033[37m')
    global REGISTERS_SEEN
    first_output = True
    for reg in registers:
        if (" " + reg in instr or "," + reg in instr) and "[" + reg not in instr or reg in REGISTERS_SEEN.keys():
            reg_value = gdb.execute('p $' + reg, to_string=True).split(" = ")[1].strip()
            if first_output:
                first_output = False
                gdb.write("Registers:\n")

            gdb.write(
                f'   \033[33m{reg}\033[30m = \033[34m{reg_value}{format_register_change(reg, REGISTERS_SEEN.get(reg), reg_value)}\n')

            REGISTERS_SEEN[reg] = reg_value

    return


def print_flags_info():
    gdb.write('\n\033[37m')
    current_flags = gdb.execute('p $eflags', to_string=True).split(" = ")[1]
    current_flags = current_flags.strip("[] \n").split()

    flags_first_output = True
    global LAST_FLAGS
    if LAST_FLAGS is not None:
        new_set_flags = [flag for flag in current_flags if flag not in LAST_FLAGS]
        new_cleared_flags = [flag for flag in LAST_FLAGS if flag not in current_flags]


        if len(new_set_flags) > 0:
            flags_first_output = False
            gdb.write(f"Flags: \033[36m{' '.join(current_flags)}\n")
            for flag in new_set_flags:
                gdb.write(f"   \033[92m+{flag}\033[30m\n")

        if len(new_cleared_flags) > 0:
            if flags_first_output:
                flags_first_output = False
                gdb.write(f"Flags: \033[36m{' '.join(current_flags)}\n")
            for flag in new_cleared_flags:
                gdb.write(f"   \033[91m-{flag}\033[30m\n")

    if flags_first_output:
        gdb.write(f"Flags: \033[36m{' '.join(current_flags)} (no change)\n")

    LAST_FLAGS = current_flags


# When breakpoints are hit
def py_stop_hook():
    gdb.write('\n\033[92m')
    try:
        try:
            gdb.write("\033[37mInstructions:\n")
            gdb.execute('x/4i $pc')
        except Exception:
            gdb.execute('printf "Invalid instructions at %#lx", $pc\n')

        current_instr = gdb.execute('x/4i $pc', to_string=True)
        print_register_info(current_instr)

        try:
            print_flags_info()
        except Exception as ex:
            gdb.write(f"Error printing flags: {ex}\n")
            pass

    except Exception as e:
        gdb.write(f'Error: {e}\n')
        pass

    gdb.write('\033[0m\n')

end

define hook-stop
    python py_stop_hook()
end

define cr
    python clear_state()
end
