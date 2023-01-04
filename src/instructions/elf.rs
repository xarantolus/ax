extern crate elf_rs;
use elf_rs::*;
use std::string::FromUtf8Error;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    debug_log,
    instructions::{axecutor::Axecutor, errors::AxError},
};

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

        let mut axecutor = Axecutor::new(
            text_section.content(),
            text_section.addr(),
            if obj_file.entry_point() == 0 {
                text_section.addr()
            } else {
                obj_file.entry_point()
            },
        )?;

        // See https://docs.oracle.com/cd/E19683-01/816-1386/chapter6-83432/index.html
        for header in obj_file.program_header_iter() {
            match header.ph_type() {
                ProgramType::LOAD => {
                    if header.memsz() > header.filesz() {
                        // Make sure we create the memory at full size and then write the first bytes, rest should be zeroed
                        axecutor.mem_init_zero(header.vaddr(), header.memsz())?;

                        if header.content().len() > header.memsz() as usize {
                            return Err(AxError::from(format!(
                                "ELF: content of program header is longer than headers' memsz"
                            )));
                        }

                        axecutor.mem_write_bytes(header.vaddr(), &header.content())?;
                    } else {
                        axecutor.mem_init_area(header.vaddr(), header.content().to_vec())?;
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
