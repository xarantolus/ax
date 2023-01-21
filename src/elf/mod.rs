extern crate elf_rs;
use elf_rs::*;
use std::string::FromUtf8Error;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::debug::debug_log;
use crate::{axecutor::Axecutor, helpers::errors::AxError};

impl From<Error> for AxError {
    fn from(err: Error) -> Self {
        match err {
            Error::BufferTooShort => AxError::from("ELF: Buffer too short"),
            Error::InvalidMagic => AxError::from("ELF: Invalid magic"),
            Error::InvalidClass => AxError::from("ELF: Invalid class"),
        }
    }
}

impl From<FromUtf8Error> for AxError {
    fn from(err: FromUtf8Error) -> Self {
        AxError::from(format!("ELF: Invalid UTF-8 in section name: {}", err))
    }
}

#[wasm_bindgen]
impl Axecutor {
    pub fn from_binary(binary: &[u8]) -> Result<Axecutor, AxError> {
        debug_log!("Calling Axecutor::from_binary");

        let obj_file = Elf::from_bytes(binary)?;

        let text_section = match obj_file.lookup_section(b".text") {
            Some(section) => section,
            None => return Err(AxError::from("ELF: No .text section")),
        };
        let text_section_content = match text_section.content() {
            Some(content) => content,
            None => return Err(AxError::from("ELF: .text section has no content")),
        };

        let mut axecutor = Axecutor::new(
            text_section_content,
            text_section.addr(),
            match obj_file.entry_point() {
                0 => text_section.addr(),
                n => n,
            },
        )?;

        // See https://docs.oracle.com/cd/E19683-01/816-1386/chapter6-83432/index.html
        for (i, header) in obj_file.program_header_iter().enumerate() {
            let content = match header.content() {
                Some(c) => c,
                None => continue,
            };
            if header.vaddr() == text_section.addr() {
                // skip .text section -- we already loaded it
                // Aspirationally this should go away once the memory implementation also holds the code
                continue;
            }

            match header.ph_type() {
                ProgramType::LOAD => {
                    if header.memsz() > header.filesz() {
                        // Make sure we create the memory at full size and then write the first bytes, rest should be zeroed
                        axecutor.mem_init_zero_named(
                            header.vaddr(),
                            header.memsz(),
                            format!("elf_zeroed_header{}", i),
                        )?;

                        if content.len() > header.memsz() as usize {
                            return Err(AxError::from(
                                "ELF: content of program header is longer than headers' memsz"
                                    .to_string(),
                            ));
                        }

                        axecutor.mem_write_bytes(header.vaddr(), content)?;
                    } else {
                        axecutor.mem_init_area_named(
                            header.vaddr(),
                            content.to_vec(),
                            Some(format!("elf_header{}", i)),
                        )?;
                    }
                }
                ProgramType::DYNAMIC => {
                    return Err(AxError::from("ELF: DYNAMIC program header not supported"))
                }
                _ => continue, // ignored
            }
        }

        Ok(axecutor)
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::test_async;

    // This macro is very limited as it only allows checking the first write call and exit code
    macro_rules! test_binary {
        [$name:ident; $binary_path:expr; $expected_output:expr; $expected_exit_code:expr] => {
            test_async![$name; async {
                use crate::instructions::generated::SupportedMnemonic;
                use crate::state::registers::SupportedRegister;

                let binary = include_bytes!($binary_path);

                let mut ax = Axecutor::from_binary(binary).expect("Failed to parse binary");

                ax.hook_before_mnemonic(SupportedMnemonic::Syscall, move |ax, _| {
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

                            assert_eq!(output_text, $expected_output, "Output of first write call does not match");

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

                    Ok(())
                }).expect("Failed add hook before Syscall");

                ax.execute().await.expect("Failed to execute");


                let exit_code = ax.reg_read_64(SupportedRegister::RDI).expect("Failed to read exit code from RDI");
                assert_eq!(exit_code, $expected_exit_code, "Exit code does not match");
            }];
        };
    }

    use super::*;

    test_binary![test_hello_world; "../../testdata/hello_world.bin"; "Hello, World!\n"; 0];
    test_binary![test_alphabet; "../../testdata/alphabet.bin"; "abcdefghijklmnopqrstuvwxyz\n"; 0];
}
