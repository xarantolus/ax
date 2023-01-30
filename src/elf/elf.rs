extern crate elfloader;

use elfloader::{ElfBinary, ElfLoaderErr, RelocationEntry};
use std::string::FromUtf8Error;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::debug::debug_log;

use crate::helpers::trace::{TraceEntry, TraceVariant};
use crate::state::memory::{PROT_EXEC, PROT_READ, PROT_WRITE};
use crate::state::registers::SupportedRegister;
use crate::{axecutor::Axecutor, helpers::errors::AxError};

impl From<ElfLoaderErr> for AxError {
    fn from(err: ElfLoaderErr) -> Self {
        AxError::from(format!("ELF: {}", err))
    }
}

impl From<AxError> for ElfLoaderErr {
    fn from(_err: AxError) -> Self {
        ElfLoaderErr::from("Internal Axecutor error that can't be converted during ELF loading")
    }
}

impl From<FromUtf8Error> for AxError {
    fn from(err: FromUtf8Error) -> Self {
        AxError::from(format!("ELF: Invalid UTF-8 in section name: {}", err))
    }
}

fn flags_mask(flags: elfloader::Flags) -> u32 {
    let mut access = 0;
    if flags.is_read() {
        access |= PROT_READ;
    }
    if flags.is_write() {
        access |= PROT_WRITE;
    }
    if flags.is_execute() {
        access |= PROT_EXEC;
    }

    access
}

// TODO: System V ABI mentions %rdx should have "a function pointer that the application should register with atexit" at process entry

struct AxecutorElfLoader {
    ax: Axecutor,
}

impl elfloader::ElfLoader for AxecutorElfLoader {
    fn allocate(&mut self, load_headers: elfloader::LoadableHeaders) -> Result<(), ElfLoaderErr> {
        debug_log!("Calling AxecutorElfLoader::allocate");

        for header in load_headers {
            let start = header.virtual_addr();
            let length = header.mem_size();
            let flags = header.flags();

            debug_log!(
                "Allocating memory for ELF section at {:#x} with length {:#x} and flags {:#?}",
                start,
                length,
                flags
            );

            self.ax
                .mem_init_zero_named(start, length, format!("elf_header_{:#x}", start))?;

            // self.ax.mem_prot(start, flags_mask(flags) | PROT_WRITE)?;
            self.ax
                .mem_prot(start, PROT_READ | PROT_WRITE | PROT_EXEC)?;
        }

        Ok(())
    }

    fn load(
        &mut self,
        _flags: elfloader::Flags,
        section_start: elfloader::VAddr,
        data: &[u8],
    ) -> Result<(), ElfLoaderErr> {
        debug_log!("Calling AxecutorElfLoader::load");

        // Make sure we can write to the section
        // self.ax.mem_prot(section_start, PROT_WRITE)?;

        self.ax.mem_write_bytes(section_start, data)?;

        self.ax
            .mem_prot(section_start, PROT_READ | PROT_WRITE | PROT_EXEC)?;

        Ok(())
    }

    fn relocate(&mut self, entry: elfloader::RelocationEntry) -> Result<(), ElfLoaderErr> {
        debug_log!("Calling AxecutorElfLoader::relocate");

        let RelocationEntry {
            rtype: _,
            offset: _,
            index: _,
            addend: _,
        } = entry;

        Ok(())
    }

    fn make_readonly(&mut self, _base: elfloader::VAddr, _size: usize) -> Result<(), ElfLoaderErr> {
        // debug_log!("Calling AxecutorElfLoader::make_readonly");
        // let initial_access = self.ax.read_mem_prot(base)?;
        // self.ax.mem_prot(base, initial_access & !PROT_WRITE)?;

        Ok(())
    }

    fn tls(
        &mut self,
        tdata_start: elfloader::VAddr,
        _tdata_length: u64,
        _total_size: u64,
        _align: u64,
    ) -> Result<(), ElfLoaderErr> {
        debug_log!("Calling AxecutorElfLoader::tls");

        self.ax.write_fs(tdata_start);

        Ok(())
    }
}

#[wasm_bindgen]
impl Axecutor {
    /// Create a new Axecutor from the bytes of an ELF binary.
    /// This will load the `.text` section into memory and set the program counter to the entry point.
    /// One thing to note is that you might want to set up the stack via `init_stack_program_start` before running the binary.
    pub fn from_binary(binary: &[u8]) -> Result<Axecutor, AxError> {
        debug_log!("Calling Axecutor::from_binary");

        // Following reference contains a lot of info about what all these ELF fields mean:
        // https://man7.org/linux/man-pages/man5/elf.5.html

        let binary = ElfBinary::new(binary)?;

        let mut loader = AxecutorElfLoader {
            ax: Axecutor::empty(),
        };

        binary.load(&mut loader)?;

        let mut ax = loader.ax;

        let initial_rip = binary.entry_point();
        ax.state
            .registers
            .insert(SupportedRegister::RIP, initial_rip);

        // Pretend to call _start
        ax.state.call_stack.push(initial_rip);
        ax.symbol_table.insert(initial_rip, "_start".to_string());
        ax.state.trace.push(TraceEntry {
            instr_ip: 0,
            target: initial_rip,
            variant: TraceVariant::Call,
            level: 0,
        });

        Ok(ax)
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::test_async;
    use crate::state::hooks::HookResult;

    // This macro runs an executable and checks that the combined output of stdout and stderr as well as the exit code is as expected.
    macro_rules! test_binary {
        [$name:ident; $binary_path:expr; $expected_output:expr; $expected_exit_code:expr] => {
            test_async![$name; async {
                use crate::instructions::generated::SupportedMnemonic;
                use crate::state::registers::SupportedRegister;
                use crate::helpers::syscalls::Syscall;

                let binary = include_bytes!($binary_path);

                let mut ax = Axecutor::from_binary(binary).expect("Failed to parse binary");

                ax.init_stack_program_start(
                    0x1000,
                    vec!["/bin/my_binary".to_string(), "arg1".to_string()],
                    vec!["env1=val1".to_string(), "env2=val2".to_string()],
                ).expect("Failed to init stack");

                #[allow(non_upper_case_globals)]
                static mut output: String = String::new();

                let cb = &move |ax: &mut Axecutor, _: SupportedMnemonic| {
                    let syscall_num = ax.reg_read_64(SupportedRegister::RAX)?;
                    let rdi = ax.reg_read_64(SupportedRegister::RDI)?;
                    let rsi = ax.reg_read_64(SupportedRegister::RSI)?;
                    let rdx = ax.reg_read_64(SupportedRegister::RDX)?;

                    match syscall_num {
                        // Write
                        1 => {
                            // rdi must be 0-2 (stdin, stdout, stderr) -- yes, we allow writing to stdin
                            if rdi > 2 {
                                return Err(AxError::from("write: invalid file descriptor").into());
                            }

                            let result_buf = ax.mem_read_bytes(rsi, rdx)?;
                            let output_text = String::from_utf8(result_buf)?;

                            unsafe {
                                output.push_str(&output_text);
                            }

                            // Return number of bytes written
                            ax.reg_write_64(SupportedRegister::RAX, rdx)?;
                        }
                        102 | 104 | 107 | 108 => {
                            // getuid, getgid, geteuid, getegid
                            ax.reg_write_64(SupportedRegister::RAX, 0)?;
                        }
                        _ => {
                            return Err(AxError::from(format!("Unsupported syscall: {}", syscall_num)).into());
                        }
                    }

                    Ok(HookResult::Handled)
                };

                ax.handle_syscalls(vec![Syscall::Exit, Syscall::Brk, Syscall::Pipe, Syscall::ArchPrctl]).expect("Failed to add syscall handlers");

                ax.hook_before_mnemonic_native(SupportedMnemonic::Syscall, cb).expect("Failed add hook before Syscall");

                ax.execute().await.expect("Failed to execute");

                assert_eq!(unsafe { output.clone() }, $expected_output, "Output does not match");

                let exit_code = ax.reg_read_64(SupportedRegister::RDI).expect("Failed to read exit code from RDI");
                assert_eq!(exit_code, $expected_exit_code, "Exit code does not match");
            }];
        };
    }

    use super::*;

    // Aspirationally, compatibility of all the programs on the demo site should be tested here

    test_binary![test_hello_world; "../../testdata/hello_world.bin"; "Hello, World!\n"; 0];
    test_binary![test_alphabet; "../../testdata/alphabet.bin"; "abcdefghijklmnopqrstuvwxyz\n"; 0];
    test_binary![test_args; "../../testdata/args.bin"; "--------------------------------------------------\n\
                                                        argv values:\n\
                                                        --------------------------------------------------\n\
                                                        /bin/my_binary\n\
                                                        arg1\n\
                                                        --------------------------------------------------\n\
                                                        envp values:\n\
                                                        --------------------------------------------------\n\
                                                        env1=val1\n\
                                                        env2=val2\n"; 2];

    // test_binary![exit_c; "../../testdata/exit_c.bin"; ""; 5];

    test_async![binary_without_symbols; async {
        let bin = Axecutor::from_binary(include_bytes!("../../testdata/exit_c_no_symbols.bin")).expect("Failed to parse binary");
        // Should only include the _start symbol
        assert_eq!(bin.symbol_table.len(), 1);
    }];
}
