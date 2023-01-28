extern crate elf;
use elf::endian::AnyEndian;
use elf::{ElfBytes, ParseError};

use std::string::FromUtf8Error;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::debug::debug_log;
use crate::helpers::macros::assert_fatal;
use crate::{axecutor::Axecutor, helpers::errors::AxError};

impl From<ParseError> for AxError {
    fn from(err: ParseError) -> Self {
        AxError::from(format!("ELF: Parse error: {}", err))
    }
}

impl From<FromUtf8Error> for AxError {
    fn from(err: FromUtf8Error) -> Self {
        AxError::from(format!("ELF: Invalid UTF-8 in section name: {}", err))
    }
}

#[wasm_bindgen]
impl Axecutor {
    /// Create a new Axecutor from the bytes of an ELF binary.
    /// This will load the `.text` section into memory and set the program counter to the entry point.
    /// One thing to note is that you might want to set up the stack via `init_stack_program_start` before running the binary.
    pub fn from_binary(binary: &[u8]) -> Result<Axecutor, AxError> {
        debug_log!("Calling Axecutor::from_binary");

        let file = ElfBytes::<AnyEndian>::minimal_parse(binary)?;

        let text_section = match file.section_header_by_name(".text")? {
            Some(section) => section,
            None => return Err(AxError::from("ELF: No .text section")),
        };

        let mut initial_addr = text_section.sh_addr;
        let mut code = Vec::from(match file.section_data(&text_section)? {
            (data, None) => data,
            (_, Some(_)) => {
                return Err(AxError::from("ELF: Compressed .text section not supported"))
            }
        });

        // Usually in binaries, we have a `.init` section, then the `.plt` section and then `.text`
        // This order is also recommended in the System V ABI, see "4.2.3 Special Sections"
        // We try to load the `.init` section into memory and then set the code section initial address
        if let Some(plt_section) = file.section_header_by_name(".plt")? {
            // Now if the .plt section is exactly before the .text section, let the overlap work out
            if plt_section.sh_addr + plt_section.sh_size == initial_addr {
                // We have a .plt section, let's load it into memory
                let plt_data = match file.section_data(&plt_section)? {
                    (data, None) => data,
                    (_, Some(_)) => {
                        return Err(AxError::from("ELF: Compressed .plt section not supported"))
                    }
                };

                initial_addr = plt_section.sh_addr;

                // Prepend to code
                code = [plt_data, &code].concat();

                debug_log!("ELF: Found .plt section and prepended it before .text");
            }
        }
        // Now the same for .init section
        if let Some(init_section) = file.section_header_by_name(".init")? {
            if init_section.sh_addr + init_section.sh_size == initial_addr {
                let init_data = match file.section_data(&init_section)? {
                    (data, None) => data,
                    (_, Some(_)) => {
                        return Err(AxError::from("ELF: Compressed .init section not supported"))
                    }
                };

                initial_addr = init_section.sh_addr;

                // Prepend to code
                code = [init_data, &code].concat();

                debug_log!("ELF: Found .init section and prepended it before .text");
            }
        }

        let entrypoint = file.ehdr.e_entry;
        let mut axecutor = Axecutor::new(
            &code,
            initial_addr,
            if entrypoint == 0 {
                text_section.sh_addr
            } else {
                entrypoint
            },
        )?;

        let headers = match file.segments() {
            Some(headers) => headers,
            None => return Err(AxError::from("ELF: No program headers")),
        };

        for header in headers {
            if header.p_vaddr >= initial_addr && header.p_vaddr < initial_addr + code.len() as u64 {
                // skip .text, .init and .plt section -- we already them
                // Aspirationally this should go away once the memory implementation also handles the code section
                continue;
            }

            let content = file.segment_data(&header)?;

            match header.p_type {
                1 => {
                    // LOAD
                    debug_log!(
                        "ELF: Loading header at {:#x} with size {:#x} and offset {:#x}",
                        header.p_vaddr,
                        header.p_memsz,
                        header.p_offset,
                    );

                    if header.p_memsz == header.p_filesz {
                        axecutor.mem_init_area_named(
                            header.p_vaddr,
                            content.to_vec(),
                            Some(format!("elf_header_{:#x}", header.p_vaddr)),
                        )?;
                    } else {
                        // Make sure we create the memory at full size and then write the first bytes, rest should be zeroed
                        axecutor.mem_init_zero_named(
                            header.p_vaddr,
                            header.p_memsz,
                            format!("elf_zeroed_header_{:#x}", header.p_vaddr),
                        )?;

                        if content.len() > header.p_filesz as usize {
                            return Err(AxError::from(
                                "ELF: Content is larger than specified in header".to_string(),
                            ));
                        }

                        axecutor.mem_write_bytes(
                            header.p_vaddr,
                            &content[..header.p_filesz as usize],
                        )?;
                    }
                }
                2 => {
                    return Err(AxError::from("ELF: Dynamic linking not supported"));
                }
                7 => {
                    // Thread-local storage
                    debug_log!(
                        "ELF: Loading TLS header at {:#x} with size {:#x} and offset {:#x}",
                        header.p_vaddr,
                        header.p_memsz,
                        header.p_offset
                    );

                    assert_fatal!(axecutor.read_fs() == 0, "ELF: TLS already initialized");

                    // See if we already have a memory area with that address
                    let end_addr = match axecutor.mem_get_area(header.p_vaddr) {
                        Some(a) => {
                            // We already have an area, let's make sure it's big enough
                            assert_fatal!(
                                a.len() >= header.p_memsz,
                                "ELF: preexisting TLS area is too small"
                            );
                            debug_log!("ELF: TLS area already exists, reusing it");
                            header.p_vaddr + a.len() as u64
                        }
                        None => {
                            // We don't have an area, let's create one
                            debug_log!("ELF: TLS area doesn't exist, creating it");

                            if header.p_memsz == header.p_filesz {
                                let addr = axecutor.mem_init_anywhere(
                                    content.to_vec(),
                                    Some(format!("elf_tls_header_{:#x}", header.p_vaddr)),
                                )?;
                                addr + content.len() as u64
                            } else {
                                // Make sure we create the memory at full size and then write the first bytes, rest should be zeroed
                                let addr = axecutor.mem_init_anywhere(
                                    vec![0; header.p_memsz as usize],
                                    Some(format!("elf_tls_zeroed_header_{:#x}", header.p_vaddr)),
                                )?;

                                axecutor.mem_write_bytes(
                                    header.p_vaddr,
                                    &content[..header.p_filesz as usize],
                                )?;

                                addr + header.p_memsz as u64
                            }
                        }
                    };

                    axecutor.write_fs(end_addr);
                }
                _v => {
                    debug_log!(
                        "ELF: Ignoring section of type {:#x} at address {:#x}, len={}",
                        _v,
                        header.p_vaddr,
                        header.p_memsz
                    );
                }
            }
        }

        match file.symbol_table() {
            Ok(Some((symbol_table, str_table))) => {
                for symbol in symbol_table.iter() {
                    if symbol.is_undefined() {
                        debug_log!("ELF: Ignoring undefined symbol in strtab");
                        continue;
                    }

                    let name = match str_table.get(symbol.st_name as usize) {
                        Ok(name) => name,
                        Err(_) => {
                            debug_log!("ELF: Invalid symbol name (wrong string table index)");
                            continue;
                        }
                    };

                    debug_log!("ELF: Found symbol {} at {:#x}", name, symbol.st_value);

                    axecutor
                        .symbol_table
                        .insert(symbol.st_value, name.to_string());
                }
            }
            Ok(None) => {
                debug_log!("ELF: No symbol table");
            }
            Err(_) => {
                debug_log!("ELF: No symbol table");
            }
        }

        Ok(axecutor)
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
                        // Exit
                        60 => {
                            ax.stop();
                        }
                        _ => {
                            return Err(AxError::from(format!("Unsupported syscall: {}", syscall_num)).into());
                        }
                    }

                    Ok(HookResult::Handled)
                };

                ax.hook_before_mnemonic_native(SupportedMnemonic::Syscall, cb).expect("Failed add hook before Syscall");

                ax.execute().await.expect("Failed to execute");

                assert_eq!(unsafe { output.clone() }, $expected_output, "Output does not match");

                let exit_code = ax.reg_read_64(SupportedRegister::RDI).expect("Failed to read exit code from RDI");
                assert_eq!(exit_code, $expected_exit_code, "Exit code does not match");
            }];
        };
    }

    use super::*;

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

    test_async![binary_without_symbols; async {
        let bin = Axecutor::from_binary(include_bytes!("../../testdata/exit_c_no_symbols.bin")).expect("Failed to parse binary");
        // Should only include the _start symbol
        assert_eq!(bin.symbol_table.len(), 1);
    }];
}
