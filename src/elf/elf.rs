extern crate elf;
use elf::abi::*;
use elf::endian::AnyEndian;
use elf::to_str::p_type_to_str;
use elf::{ElfBytes, ParseError};

use std::string::FromUtf8Error;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::debug::debug_log;
use crate::helpers::macros::{assert_fatal, fatal_error};
use crate::helpers::trace::{TraceEntry, TraceVariant};
use crate::state::memory::{PROT_EXEC, PROT_READ, PROT_WRITE};
use crate::state::registers::SupportedRegister::RIP;
use crate::{axecutor::Axecutor, helpers::errors::AxError};

impl From<ParseError> for AxError {
    fn from(err: ParseError) -> Self {
        AxError::from(format!("ELF: Parse error: {err}"))
    }
}

impl From<FromUtf8Error> for AxError {
    fn from(err: FromUtf8Error) -> Self {
        AxError::from(format!("ELF: Invalid UTF-8 in section name: {err}"))
    }
}

fn elf_flags_to_prot(flags: u32) -> u32 {
    let mut proc_flags = 0;
    if flags & PF_R != 0 {
        proc_flags |= PROT_READ;
    }
    if flags & PF_W != 0 {
        proc_flags |= PROT_WRITE;
    }
    if flags & PF_X != 0 {
        proc_flags |= PROT_EXEC;
    }
    proc_flags
}

fn round_up_to_page_size(size: u64) -> u64 {
    (size + 0xfff) & !0xfff
}

// TODO: System V ABI mentions %rdx should have "a function pointer that the application should register with atexit" at process entry

#[wasm_bindgen]
impl Axecutor {
    /// Create a new Axecutor from the bytes of an ELF binary.
    /// This will load the `.text` section into memory and set the program counter to the entry point.
    /// One thing to note is that you might want to set up the stack via `init_stack_program_start` before running the binary.
    pub fn from_binary(binary: &[u8]) -> Result<Axecutor, AxError> {
        debug_log!("Calling Axecutor::from_binary");

        // Following reference contains a lot of info about what all these ELF fields mean:
        // https://man7.org/linux/man-pages/man5/elf.5.html

        let file = ElfBytes::<AnyEndian>::minimal_parse(binary)?;
        let entrypoint = file.ehdr.e_entry;

        let mut axecutor = Axecutor::empty();
        axecutor.reg_write_64(RIP, entrypoint)?;

        // Tracing: Pretend to call _start
        axecutor.state.call_stack.push(entrypoint);
        axecutor
            .symbol_table
            .insert(entrypoint, "_start".to_string());
        axecutor.state.trace.push(TraceEntry {
            instr_ip: 0,
            target: entrypoint,
            variant: TraceVariant::Call,
            level: 0,
            count: 1,
        });

        let segments = match file.segments() {
            Some(seg) => seg,
            None => return Err(AxError::from("ELF: No segments found")),
        };

        for segment in segments {
            if segment.p_vaddr == 0 {
                debug_log!(
                    "ELF: Skip loading segment with p_vaddr == 0, p_type {} ({})",
                    p_type_to_str(segment.p_type).expect("Unknown segment type"),
                    segment.p_type
                );
                continue;
            }

            let content = file.segment_data(&segment)?;

            // TODO: Scale up all allocations to next multiple of page size, also respect alignment

            match segment.p_type {
                // Skippable
                PT_NULL | PT_NOTE | PT_SHLIB | PT_PHDR => {
                    debug_log!(
                        "ELF: Skip loading segment of type {} ({:#x}) @ {:#x} with size {:#x}",
                        p_type_to_str(segment.p_type).expect("Unknown segment type"),
                        segment.p_type,
                        segment.p_vaddr,
                        segment.p_memsz
                    );
                }
                // TODO: map PT_GNU_EH_FRAME

                // Skippable, but we should warn and probably implement them in the future
                PT_GNU_EH_FRAME | PT_GNU_PROPERTY => {
                    debug_log!(
                        "ELF: Skip loading segment of type {} ({:#x}) @ {:#x} with size {:#x}",
                        p_type_to_str(segment.p_type).unwrap_or("unknown"),
                        segment.p_type,
                        segment.p_vaddr,
                        segment.p_memsz
                    );
                }
                // Unsupported segment types
                PT_DYNAMIC => {
                    return Err(AxError::from("ELF: Dynamic linking not supported"));
                }
                PT_GNU_STACK => {
                    // If the flags are not READ | WRITE, we cannot continue
                    debug_log!(
                        "ELF: Found GNU_STACK segment with flags {:#x}",
                        segment.p_flags
                    );
                    if segment.p_flags != (PF_R | PF_W) {
                        return Err(AxError::from("ELF: Non-standard stack flags"));
                    }
                }
                PT_TLS => {
                    // TODO: Make sure implementation is correct, see e.g. https://maskray.me/blog/2021-02-14-all-about-thread-local-storage for some notes

                    // Thread-local storage
                    debug_log!(
                        "ELF: Loading TLS segment at {:#x} with size {:#x} and offset {:#x}",
                        segment.p_vaddr,
                        segment.p_memsz,
                        segment.p_offset
                    );

                    assert_fatal!(axecutor.read_fs() == 0, "ELF: TLS already initialized");

                    // See if we already have a memory area with that address
                    let end_addr = match axecutor.mem_get_area(segment.p_vaddr) {
                        Some(a) => {
                            // We already have an area, let's make sure it's big enough
                            assert_fatal!(
                                a.len() >= segment.p_memsz,
                                "ELF: preexisting TLS area is too small"
                            );
                            debug_log!("ELF: TLS area already exists, reusing it");
                            segment.p_vaddr + a.len()
                        }
                        None => Err(AxError::from("ELF: TLS area does not exist, but expected it to be created by previous LOAD program header"))?,
                    };

                    // TODO: if we write the wanted flags (PROT_READ only), then libc startup will crash writing to it.
                    // Not sure what to do about this, which is why we'll keep it writable for now
                    // axecutor.mem_prot(segment.p_vaddr, PROT_READ | PROT_WRITE)?;

                    axecutor.write_fs(end_addr);
                }
                PT_GNU_RELRO => {
                    // Read-only after relocation
                    debug_log!(
                        "ELF: Loading RELRO segment at {:#x} with size {:#x} and offset {:#x}",
                        segment.p_vaddr,
                        segment.p_memsz,
                        segment.p_offset
                    );

                    // TODO: check if it matters if the relro size is smaller than the segment size
                    // Here we *should* set the flags (usually PROT_READ), but libc startup will crash writing to it.
                    // So let's keep it writable for now
                    // axecutor.mem_prot(segment.p_vaddr, elf_flags_to_prot(segment.p_flags))?;
                }
                PT_LOAD => {
                    debug_log!(
                        "ELF: Loading segment of type {} at {:#x} with size {:#x} and offset {:#x}",
                        p_type_to_str(segment.p_type).expect("Unknown segment type"),
                        segment.p_vaddr,
                        segment.p_memsz,
                        segment.p_offset,
                    );

                    let memsz = round_up_to_page_size(segment.p_memsz);

                    if memsz == segment.p_filesz {
                        axecutor.mem_init_area_named(
                            segment.p_vaddr,
                            content.to_vec(),
                            Some(format!("elf_load_header_{:#x}", segment.p_vaddr)),
                        )?;
                    } else {
                        // Make sure we create the memory at full size and then write the first bytes, rest should be zeroed
                        axecutor.mem_init_zero_named(
                            segment.p_vaddr,
                            memsz,
                            format!("elf_load_zeroed_header_{:#x}", segment.p_vaddr),
                        )?;

                        if content.len() > segment.p_filesz as usize {
                            return Err(AxError::from(
                                "ELF: Content is larger than specified in segment header"
                                    .to_string(),
                            ));
                        }

                        axecutor.mem_write_bytes(
                            segment.p_vaddr,
                            &content[..segment.p_filesz as usize],
                        )?;
                    }
                    axecutor.mem_prot(segment.p_vaddr, elf_flags_to_prot(segment.p_flags))?;
                }
                _ => {
                    fatal_error!(
                        "ELF: Unsupported segment type {} ({:#x})",
                        p_type_to_str(segment.p_type).unwrap_or("unknown"),
                        segment.p_type
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
                use crate::auto::generated::SupportedMnemonic;
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

    // test_binary![fib_c_nostdlib; "../../testdata/fib_c_nostdlib.bin"; "1\n1\n2\n3\n5\n8\nd\n15\n22\n37\n59\n90\ne9\n179\n262\n3db\n63d\na18\n1055\n1a6d\n2ac2\n452f\n6ff1\nb520\n12511"; 0];

    test_async![binary_without_symbols; async {
        let bin = Axecutor::from_binary(include_bytes!("../../testdata/exit_c_no_symbols.bin")).expect("Failed to parse binary");
        // Should only include the _start symbol
        assert_eq!(bin.symbol_table.len(), 1);
    }];
}
