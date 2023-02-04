use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::helpers::debug::debug_log;
use crate::{helpers::macros::assert_fatal, state::registers::SupportedRegister};

use crate::{axecutor::Axecutor, helpers::errors::AxError};

#[cfg(all(target_arch = "wasm32", not(test)))]
use wasm_bindgen::JsValue;

use std::cmp::min;
use std::convert::TryInto;

/// The area must not be accessed
pub const PROT_NONE: u32 = 0x0;
/// The area can be read from
pub const PROT_READ: u32 = 0x1;
/// The area can be written to
pub const PROT_WRITE: u32 = 0x2;
/// The area can be executed
pub const PROT_EXEC: u32 = 0x4;

fn access_to_string(prot: u32) -> String {
    if prot == PROT_NONE {
        return "PROT_NONE".to_string();
    }

    let mut s = String::new();

    if prot & PROT_READ != 0 {
        s.push_str("READ");
    }

    if prot & PROT_WRITE != 0 {
        if !s.is_empty() {
            s.push_str(" | ");
        }

        s.push_str("WRITE");
    }

    if prot & PROT_EXEC != 0 {
        if !s.is_empty() {
            s.push_str(" | ");
        }

        s.push_str("EXEC");
    }

    s
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MemoryArea {
    name: Option<String>,
    start: u64,
    length: u64,
    data: Vec<u8>,
    access: u32,
}

impl MemoryArea {
    pub fn len(&self) -> u64 {
        self.length
    }

    pub fn to_string_ident(&self, i: usize) -> String {
        let mut s = String::new();

        s.push_str("MemoryArea {\n");
        s.push_str(&format!(
            "{}    name: {:?},\n",
            " ".repeat(i * 4),
            match self.name {
                Some(ref s) => s,
                None => "<unnamed>",
            }
        ));
        s.push_str(&format!(
            "{}    start: {:#x},\n",
            " ".repeat(i * 4),
            self.start
        ));
        s.push_str(&format!(
            "{}    length: {},\n",
            " ".repeat(i * 4),
            self.length
        ));

        s.push_str(&format!(
            "{}    access: {},\n",
            " ".repeat(i * 4),
            access_to_string(self.access)
        ));

        s.push_str(&format!("{}    data: [", " ".repeat(i * 4)));

        const MAX_LEN: usize = 255;

        if self.data.len() < MAX_LEN {
            for (i, byte) in self.data.iter().enumerate() {
                s.push_str(&format!("0x{byte:02x}"));

                if i != self.data.len() - 1 {
                    s.push_str(", ");
                }
            }
        } else {
            s.push_str("<too long to display>");
        }

        s.push_str("],\n");

        // Add a string-representation if all bytes are ascii bytes and the last byte is zero
        if !self.data.is_empty()
            && self.data.len() < MAX_LEN
            && self.data[self.data.len() - 1] == 0
            && self.data.iter().all(|b| b.is_ascii())
        {
            s.push_str(&format!(
                "{}    string: {:?},\n",
                " ".repeat(i * 4),
                String::from_utf8_lossy(&self.data)
            ));
        }

        s.push_str(&format!("{}}}", " ".repeat(i * 4)));

        s
    }
}

#[wasm_bindgen]
impl Axecutor {
    // TODO: Currently cannot read consecutive sections of memory
    /// Reads `length` bytes from memory at `address`
    pub fn mem_read_bytes(&self, address: u64, length: u64) -> Result<Vec<u8>, AxError> {
        debug_log!(
            "Calling Axecutor::mem_read_bytes, address={:#x}, length={}",
            address,
            length
        );

        let area = self
            .state
            .memory
            .iter()
            .find(|area| area.start <= address && address < area.start + area.length)
            .ok_or_else(|| self.collect_mem_error_hints(address, length, "Read".to_string()))?;

        if area.access & PROT_READ == 0 {
            return Err(AxError::from(format!(
                "Cannot read {} bytes from memory area{} @ {:#x}, access is {}",
                length,
                match &area.name {
                    Some(name) => format!(" {name}"),
                    None => String::new(),
                },
                address,
                access_to_string(area.access)
            )));
        }

        let offset = (address - area.start) as usize;
        let slice = &area.data[offset..offset + length as usize];

        let result = slice.to_vec();

        #[cfg(debug_assertions)]
        if result.len() <= 100 {
            debug_log!(
                "Read from memory area{}, start={:#x}, area_length={}, read={:?}{}",
                match &area.name {
                    Some(name) => format!(" {name}"),
                    None => String::new(),
                },
                area.start,
                area.length,
                result,
                match result.len() {
                    1 => format!(", formatted=0x{:02x}", result[0]),
                    2 => format!(
                        ", formatted=0x{:04x}",
                        u16::from_le_bytes(slice.try_into().unwrap())
                    ),
                    4 => format!(
                        ", formatted=0x{:08x}",
                        u32::from_le_bytes(slice.try_into().unwrap())
                    ),
                    8 => format!(
                        ", formatted=0x{:016x}",
                        u64::from_le_bytes(slice.try_into().unwrap())
                    ),
                    _ => "".to_string(),
                }
            );
        } else {
            // Only log the first 50 and last 50 bytes of the memory area, with "<too much data to display>" in the middle
            debug_log!(
                        "Read from memory area{}, start={:#x}, length={}, read=[{:?}, <too much data to display>, {:?}]",
                        match &area.name {
                            Some(name) => format!(" {name}"),
                            None => String::new(),
                        },
                        area.start,
                        area.length,
                        &result[0..50],
                        &result[result.len() - 50..]
                    );
        }

        Ok(result)
    }

    pub(crate) fn mem_read_executable_bytes(&self, address: u64) -> Result<Vec<u8>, AxError> {
        // TODO: Optimize these reads by caching a reference to the last section we used?
        let area = self
            .state
            .memory
            .iter()
            .find(|area| area.start <= address && address < area.start + area.length)
            .ok_or_else(|| {
                self.collect_mem_error_hints(address, 15, "Read executable".to_string())
            })?;

        if area.access & PROT_EXEC == 0 {
            return Err(AxError::from(format!(
                "Cannot read executable bytes from memory area{} @ {:#x}, access is {} (does not include PROT_EXEC)",
                match &area.name {
                    Some(name) => format!(" {name}"),
                    None => String::new(),
                },
                address,
                access_to_string(area.access)
            )));
        }

        // Read up to 15 bytes, but only as many as are available in the memory area

        // &self.code[code_offset..min(code_offset + 15, self.code.len())];

        let offset = (address - area.start) as usize;
        let slice = &area.data[offset..min(offset + 15, area.data.len())];

        let result = slice.to_vec();

        Ok(result)
    }

    fn collect_mem_error_hints(&self, address: u64, length: u64, operation: String) -> AxError {
        // check if start or end address is within any of the memory areas
        for area in &self.state.memory {
            if address >= area.start && address < area.start + area.length {
                return AxError::from(format!(
                    "Memory {} of length {} at address {:#x} over end of memory area {} (start {:#x}, length {})",
                    operation,
                    length,
                    address,
                    match &area.name {
                        Some(name) => name,
                        None => "<unnamed>",
                    },
                    area.start,
                    area.length,
                ));
            }
        }

        for area in &self.state.memory {
            if address + length > area.start && address + length <= area.start + area.length {
                return AxError::from(format!(
                    "Memory {} of length {} at address {:#x} before start of memory area {} (start {:#x}, length {})",
                    operation,
                    length,
                    address,
                    match &area.name {
                        Some(name) => name,
                        None => "<unnamed>",
                    },
                    area.start,
                    area.length,
                ));
            }
        }

        // Check if an area with name "Stack" exists -- this is something one can easily forget
        let have_stack = self
            .state
            .memory
            .iter()
            .any(|area| area.name == Some("Stack".to_string()));

        AxError::from(
            format!(
                "Memory {} of length {} at address {:#x}: this address is not contained in any memory area{}",
                operation.to_lowercase(),
                length,
                address,
                if have_stack {
                    ""
                } else {
                    "\n\nHint: it appears you have not set up any memory areas for the stack"
                }
            ),
        )
    }

    /// Reads a 64-bit value from memory at `address`
    pub fn mem_read_64(&self, address: u64) -> Result<u64, AxError> {
        let bytes = self.mem_read_bytes(address, 8)?;

        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    /// Reads a 32-bit value from memory at `address`
    pub fn mem_read_32(&self, address: u64) -> Result<u64, AxError> {
        let bytes = self.mem_read_bytes(address, 4)?;

        Ok(u32::from_le_bytes(bytes.try_into().unwrap()) as u64)
    }

    /// Reads a 16-bit value from memory at `address`
    pub fn mem_read_16(&self, address: u64) -> Result<u64, AxError> {
        let bytes = self.mem_read_bytes(address, 2)?;

        Ok(u16::from_le_bytes(bytes.try_into().unwrap()) as u64)
    }

    /// Reads an 8-bit value from memory at `address`
    pub fn mem_read_8(&self, address: u64) -> Result<u64, AxError> {
        let bytes = self.mem_read_bytes(address, 1)?;

        Ok(bytes[0] as u64)
    }

    pub(crate) fn mem_get_area(&self, start_addr: u64) -> Option<MemoryArea> {
        for area in &self.state.memory {
            if start_addr == area.start {
                return Some(area.clone());
            }
        }

        None
    }

    // TODO: Currently cannot write consecutive sections of memory
    // It would also make sense to give better error messages, e.g. if the write start address is within an area, but the data is too long
    /// Writes bytes of `data` to memory at `address`
    pub fn mem_write_bytes(&mut self, address: u64, data: &[u8]) -> Result<(), AxError> {
        debug_log!(
            "Calling Axecutor::mem_write_bytes, address={:#x}, data_len={:?}",
            address,
            data.len()
        );

        let area = match self
            .state
            .memory
            .iter_mut()
            .find(|area| area.start <= address && address < area.start + area.length)
        {
            Some(area) => area,
            None => {
                return Err(self.collect_mem_error_hints(
                    address,
                    data.len() as u64,
                    "Write".to_string(),
                ))
            }
        };

        if area.access & PROT_WRITE == 0 {
            return Err(AxError::from(format!(
                "Cannot write {} bytes to memory area{} @ {:#x}, access is {}",
                data.len(),
                match &area.name {
                    Some(name) => format!(" {name}"),
                    None => String::new(),
                },
                address,
                access_to_string(area.access)
            )));
        }

        let offset = (address - area.start) as usize;
        area.data[offset..offset + data.len()].copy_from_slice(data);

        #[cfg(debug_assertions)]
        if data.len() <= 100 {
            debug_log!(
                "Wrote to memory area, start={:#x}, length={}, wrote={:?}{}",
                area.start,
                area.length,
                data,
                match data.len() {
                    1 => format!(", formatted=0x{:02x}", data[0]),
                    2 => format!(
                        ", formatted=0x{:04x}",
                        u16::from_le_bytes(data.try_into().unwrap())
                    ),
                    4 => format!(
                        ", formatted=0x{:08x}",
                        u32::from_le_bytes(data.try_into().unwrap())
                    ),
                    8 => format!(
                        ", formatted=0x{:016x}",
                        u64::from_le_bytes(data.try_into().unwrap())
                    ),
                    _ => "".to_string(),
                }
            );
        } else {
            // Only log the first 50 and last 50 bytes of data, with "<too much data to display>" in the middle
            debug_log!(
                        "Wrote to memory area, start={:#x}, length={}, wrote=[{:?}, <too much data to display>, {:?}]",
                        area.start,
                        area.length,
                        &data[0..50],
                        &data[data.len() - 50..]
                    );
        }

        Ok(())
    }

    /// Writes a 64-bit value to memory at `address`
    pub fn mem_write_64(&mut self, address: u64, data: u64) -> Result<(), AxError> {
        self.mem_write_bytes(address, &data.to_le_bytes())
    }

    /// Writes a 32-bit value to memory at `address`
    pub fn mem_write_32(&mut self, address: u64, data: u64) -> Result<(), AxError> {
        assert_fatal!(
            data <= u32::MAX as u64,
            "Could not write {:#x} to 4 bytes of memory, value is too large",
            data
        );

        self.mem_write_bytes(address, &(data as u32).to_le_bytes())
    }

    /// Writes a 16-bit value to memory at `address`
    pub fn mem_write_16(&mut self, address: u64, data: u64) -> Result<(), AxError> {
        assert_fatal!(
            data <= u16::MAX as u64,
            "Could not write {:#x} to 2 bytes of memory, value is too large",
            data
        );
        self.mem_write_bytes(address, &(data as u16).to_le_bytes())
    }

    /// Writes an 8-bit value to memory at `address`
    pub fn mem_write_8(&mut self, address: u64, data: u64) -> Result<(), AxError> {
        assert_fatal!(
            data <= u8::MAX as u64,
            "Could not write {:#x} to 1 byte of memory, value is too large",
            data
        );

        self.mem_write_bytes(address, &[data as u8])
    }

    /// Resize the already existing section of memory with start address `start_addr` to `new_size`
    /// The code section cannot be resized.
    pub fn mem_resize_section(&mut self, start_addr: u64, new_size: u64) -> Result<(), AxError> {
        debug_log!(
            "Calling Axecutor::mem_resize_section, start_addr={:#x}, new_size={}",
            start_addr,
            new_size
        );

        // Iterate all areas once and save the index of the area to resize
        let mut area_to_resize = None;

        // Also make sure there's no overlapping area already defined, including code region
        for (i, area) in self.state.memory.iter().enumerate() {
            if start_addr == area.start {
                area_to_resize = Some(i);
            }

            // Make sure the new length doesn't overlap with any other area after it
            if start_addr + new_size > area.start {
                return Err(AxError::from(format!(
                    "Cannot resize section at address {:#x} to length {}, as it overlaps with another section starting at {:#x} (len={})",
                    start_addr, new_size, area.start, area.length
                )));
            }
        }

        if let Some(i) = area_to_resize {
            // Resize the area -- this works for both shrinking and growing
            let mut new_data = vec![0; new_size as usize];
            let old_data = &self.state.memory[i].data;

            // Copy the old data into the new data
            let copy_len = std::cmp::min(old_data.len(), new_data.len());
            new_data[..copy_len].copy_from_slice(&old_data[..copy_len]);

            // Update the area
            self.state.memory[i].data = new_data;
            self.state.memory[i].length = new_size;

            return Ok(());
        }

        Err(AxError::from(format!(
            "No section has start address {start_addr:#x}"
        )))
    }

    /// Initialize a memory area with the given data and name.
    /// The name is used for logging and debugging purposes.
    pub fn mem_init_area_named(
        &mut self,
        start: u64,
        data: Vec<u8>,
        name: Option<String>,
    ) -> Result<(), AxError> {
        for area in &self.state.memory {
            if start >= area.start && start < area.start + area.length {
                let overlap_name = area
                    .name
                    .to_owned()
                    .unwrap_or_else(|| "<unnamed>".to_string());
                return Err(AxError::from(format!(
                    "cannot create memory area {} with start={:#x}, length={:#x}: overlaps with area {} with start={:#x}, length={:#x}",
                    name.unwrap_or_else(||"<unnamed>".to_string()), start, data.len(), overlap_name, area.start, area.length
                )));
            }
        }

        #[allow(unused_variables)]
        #[cfg(debug_assertions)]
        let display_name = match &name {
            Some(n) => format!(" {n}"),
            None => "".to_string(),
        };

        let len = data.len() as u64;
        self.state.memory.push(MemoryArea {
            start,
            length: len,
            data,
            name,
            access: PROT_READ | PROT_WRITE,
        });

        debug_log!(
            "Initialized memory area{}, start={:#x}, length={:#x}, access={}",
            display_name,
            start,
            len,
            access_to_string(PROT_READ | PROT_WRITE)
        );

        Ok(())
    }

    /// Set the access permissions of the memory area with the given start address.
    /// The access permissions are a bitmask of `PROT_READ` (1), `PROT_WRITE` (2), and `PROT_EXEC` (4).
    /// A value of 0 means no access.
    /// By default, all memory areas are initialized with `PROT_READ | PROT_WRITE`.
    pub fn mem_prot(&mut self, section_start: u64, prot: u32) -> Result<(), AxError> {
        assert_fatal!(
            prot <= 7,
            "Invalid access permissions {:#x} for memory area, must be a bitmask of PROT_READ (1), PROT_WRITE (2), and PROT_EXEC (4)",
            prot
        );

        debug_log!(
            "Calling Axecutor::mem_prot, section_start={:#x}, prot={}",
            section_start,
            access_to_string(prot)
        );

        for area in &mut self.state.memory {
            if section_start == area.start {
                area.access = prot;
                debug_log!(
                    "Set access rights of memory area{}, start={:#x}, rights={}",
                    match &area.name {
                        Some(name) => format!(" {name}"),
                        None => String::new(),
                    },
                    area.start,
                    access_to_string(area.access)
                );

                return Ok(());
            }
        }

        Err(AxError::from(format!(
            "No section has start address {section_start:#x}"
        )))
    }

    /// Initialize a memory area with the given data.
    pub fn mem_init_area(&mut self, start: u64, data: Vec<u8>) -> Result<(), AxError> {
        self.mem_init_area_named(start, data, None)
    }
    /// Initialize a memory area with the given length.
    pub fn mem_init_zero(&mut self, start: u64, length: u64) -> Result<(), AxError> {
        self.mem_init_area_named(start, vec![0; length as usize], None)
    }

    /// Initialize a memory area with the given length and name.
    pub fn mem_init_zero_named(
        &mut self,
        start: u64,
        length: u64,
        name: String,
    ) -> Result<(), AxError> {
        self.mem_init_area_named(start, vec![0; length as usize], Some(name))
    }

    /// Initialize a memory area of the given length at a random address.
    /// The start address is returned.
    pub fn mem_init_zero_anywhere(&mut self, length: u64) -> Result<u64, AxError> {
        let mut start: u64 = 0x1000;

        loop {
            if start >= 0x7fff_ffff_ffff_ffff {
                return Err(AxError::from(
                    "Could not find a suitable memory start address",
                ));
            }

            if self.mem_init_zero(start, length).is_ok() {
                break;
            }
            start += length;
        }

        Ok(start)
    }

    /// Initialize a memory area with the given data at a random address.
    /// The start address is returned.
    pub fn mem_init_anywhere(
        &mut self,
        data: Vec<u8>,
        name: Option<String>,
    ) -> Result<u64, AxError> {
        let mut start: u64 = 0x1000;

        loop {
            if start >= 0x7fff_ffff_ffff_ffff {
                return Err(AxError::from(
                    "Could not find a suitable memory start address",
                ));
            }

            let res = match &name {
                Some(n) => self.mem_init_area_named(start, data.clone(), Some(n.clone())),
                None => self.mem_init_area(start, data.clone()),
            };
            if res.is_ok() {
                break;
            }
            start += data.len() as u64;
        }

        Ok(start)
    }

    /// Initializes the stack at a random location with the given length.
    pub fn init_stack(&mut self, length: u64) -> Result<u64, AxError> {
        let mut stack_start: u64 = 0x1000;

        loop {
            if stack_start >= 0x7fff_ffff_ffff_ffff {
                return Err(AxError::from(
                    "Could not find a suitable stack start address",
                ));
            }

            if self
                .mem_init_zero_named(stack_start, length, "Stack".to_string())
                .is_ok()
            {
                break;
            }
            stack_start <<= 1;
        }

        // Align the stack pointer to 16 bytes -- System V ABI requires this on program entry
        let initial_rsp = (stack_start + length - 8) & !0xf;
        self.reg_write_64(SupportedRegister::RSP, initial_rsp)?;
        self.stack_top = initial_rsp + 8;

        Ok(stack_start)
    }

    /// Initializes the stack with the given length, command-line arguments and environment variables according to the System V ABI.
    /// This is useful for emulating ELF binaries.
    #[cfg(all(target_arch = "wasm32", not(test)))]
    pub fn init_stack_program_start(
        &mut self,
        length: u64,
        argv: Vec<JsValue>, // Vec<String>
        envp: Vec<JsValue>, // Vec<String>
    ) -> Result<u64, AxError> {
        self.init_stack_program_start_impl(length, from_js_vec(argv)?, from_js_vec(envp)?)
    }
}

#[cfg(all(target_arch = "wasm32", not(test)))]
fn from_js_vec(vec: Vec<JsValue>) -> Result<Vec<String>, AxError> {
    let mut result = Vec::new();
    for s in vec {
        result.push(s.as_string().ok_or_else(|| {
            AxError::from(
                "Invalid argument in init_stack_program_start: argv contains non-string value",
            )
        })?);
    }
    Ok(result)
}

impl Axecutor {
    /// Initializes the stack with the given length, command-line arguments and environment variables according to the System V ABI.
    /// This is useful for emulating ELF binaries.
    #[cfg(not(all(target_arch = "wasm32", not(test))))]
    pub fn init_stack_program_start(
        &mut self,
        length: u64,
        argv: Vec<String>,
        envp: Vec<String>,
    ) -> Result<u64, AxError> {
        self.init_stack_program_start_impl(length, argv, envp)
    }

    fn init_stack_program_start_impl(
        &mut self,
        length: u64,
        argv: Vec<String>, // Vec<String>
        envp: Vec<String>, // Vec<String>
    ) -> Result<u64, AxError> {
        debug_log!(
            "Initializing stack with length {}, argv: {:?}, envp: {:?}",
            length,
            argv,
            envp
        );

        let stack_layout = &mut Vec::new();

        // First comes argc -- if the first instruction of the program is
        // pop rdi, then rdi should contain the argc value
        stack_layout.push(argv.len() as u64);

        // argv
        for (i, arg) in argv.iter().enumerate() {
            let mut arg_bytes = Vec::from(arg.as_bytes());
            arg_bytes.push(0);

            // Allocate space for the string
            let str_addr = self.mem_init_anywhere(arg_bytes, Some(format!("arg{i}")))?;
            stack_layout.push(str_addr);
        }
        // argv[argc] = NULL
        stack_layout.push(0);

        // envp
        for (i, env) in envp.iter().enumerate() {
            let mut env_bytes = Vec::from(env.as_bytes());
            env_bytes.push(0);

            // Allocate space for the string
            let str_addr = self.mem_init_anywhere(env_bytes, Some(format!("env{i}")))?;
            stack_layout.push(str_addr);
        }

        // envp[0] = NULL
        stack_layout.push(0);

        let mut stack_start: u64 = 0x1000;
        loop {
            if stack_start >= 0x7fff_ffff_ffff_ffff {
                return Err(AxError::from(
                    "Could not find a suitable stack start address",
                ));
            }

            if self
                .mem_init_zero_named(
                    stack_start,
                    length + (stack_layout.len() as u64) * 8,
                    "Stack".to_string(),
                )
                .is_ok()
            {
                break;
            }
            stack_start <<= 1;
        }

        // Make sure the stack is aligned to 16 bytes
        let mut stack_top = (stack_start + length - 16) & !0xf;
        if stack_layout.len() % 2 == 1 {
            // However, if we push an uneven amount of 64 bit values, we need to adjust
            stack_top -= 8;
        }

        for val in stack_layout.iter().rev() {
            self.mem_write_64(stack_top, *val)?;
            stack_top -= 8;
        }

        // Alignment should still be OK
        assert!(
            stack_top & 0xf == 0,
            "Stack top is not aligned to 16 bytes: {stack_top:#x}"
        );

        self.reg_write_64(SupportedRegister::RSP, stack_top)?;

        debug_log!(
            "Initialized stack, stack_top={:#x}, self.stack_top={:#x}",
            stack_top,
            self.stack_top
        );
        self.stack_top = stack_top;

        Ok(stack_start)
    }
}

// 128 bit reads/writes
#[wasm_bindgen]
#[cfg(all(target_arch = "wasm32", not(test)))]
impl Axecutor {
    /// Writes an 128-bit value to memory at `address`
    pub fn mem_write_128(&mut self, address: u64, data: js_sys::BigInt) -> Result<(), AxError> {
        let value: u128 = data
            .try_into()
            .map_err(|e| AxError::from(format!("Could not convert value to u128: {}", e)))?;

        self.internal_mem_write_128(address, value)
    }

    /// Reads an 128-bit value from memory at `address`
    pub fn mem_read_128(&self, address: u64) -> Result<js_sys::BigInt, AxError> {
        let value = self.internal_mem_read_128(address)?;

        Ok(js_sys::BigInt::from(value))
    }
}

impl Axecutor {
    /// Writes an 128-bit value to memory at `address`
    pub(crate) fn internal_mem_write_128(
        &mut self,
        address: u64,
        data: u128,
    ) -> Result<(), AxError> {
        self.mem_write_bytes(address, &data.to_le_bytes())
    }

    /// Reads an 128-bit value from memory at `address`
    pub(crate) fn internal_mem_read_128(&self, address: u64) -> Result<u128, AxError> {
        let bytes = self.mem_read_bytes(address, 16)?;

        Ok(u128::from_le_bytes(bytes.try_into().unwrap()))
    }
}

#[cfg(not(all(target_arch = "wasm32", not(test))))]
impl Axecutor {
    /// Writes an 128-bit value to memory at `address`
    pub fn mem_write_128(&mut self, address: u64, data: u128) -> Result<(), AxError> {
        self.internal_mem_write_128(address, data)
    }

    /// Reads an 128-bit value from memory at `address`
    pub fn mem_read_128(&self, address: u64) -> Result<u128, AxError> {
        self.internal_mem_read_128(address)
    }
}
