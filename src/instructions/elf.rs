extern crate elf_rs;
use elf_rs::*;
use std::str;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::instructions::{axecutor::Axecutor, errors::AxError};

impl From<Error> for AxError {
    fn from(err: Error) -> Self {
        match err {
            Error::BufferTooShort => AxError::from("ELF: Buffer too short"),
            Error::InvalidMagic => AxError::from("ELF: Invalid magic"),
            Error::InvalidClass => AxError::from("ELF: Invalid class"),
        }
    }
}

#[wasm_bindgen]
impl Axecutor {
    pub fn from_binary(binary: &[u8]) -> Result<Axecutor, AxError> {
        let obj_file = Elf::from_bytes(&*binary)?;

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

        for section in obj_file.section_header_iter() {
            if section.section_name() != b".text" && section.sh_type() == SectionType::SHT_PROGBITS
            {
                axecutor
                    .mem_init_area(section.addr(), section.content().to_vec())
                    .map_err(|err| {
                        AxError::from(format!(
                            "ELF: initializing {} section: {}",
                            match str::from_utf8(section.section_name()) {
                                Ok(name) => name.to_string(),
                                Err(e) => e.to_string(),
                            },
                            err
                        ))
                    })?;
            }
        }

        Ok(axecutor)
    }
}
