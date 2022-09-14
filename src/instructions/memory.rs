use super::{axecutor::Axecutor, errors::AxError};

use std::convert::TryInto;

#[derive(Debug, Clone)]
pub(crate) struct MemoryArea {
    start: u64,
    length: u64,
    data: Vec<u8>,
}

impl Axecutor {
    // TODO: Currently cannot read consecutive sections of memory
    pub fn mem_read_bytes(&self, address: u64, length: u64) -> Result<Vec<u8>, AxError> {
        let mut result = Vec::new();

        for area in &self.state.memory {
            if address >= area.start && address + length <= area.start + area.length {
                let offset = (address - area.start) as usize;
                result.extend_from_slice(&area.data[offset..offset + length as usize]);

                return Ok(result);
            }
        }

        Err(AxError::from(format!(
            "Could not read memory at address {:#x}",
            address
        )))
    }

    pub fn mem_read_64(&self, address: u64) -> Result<u64, AxError> {
        let bytes = self.mem_read_bytes(address, 8)?;

        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn mem_read_32(&self, address: u64) -> Result<u32, AxError> {
        let bytes = self.mem_read_bytes(address, 4)?;

        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn mem_read_16(&self, address: u64) -> Result<u16, AxError> {
        let bytes = self.mem_read_bytes(address, 2)?;

        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn mem_read_8(&self, address: u64) -> Result<u8, AxError> {
        let bytes = self.mem_read_bytes(address, 1)?;

        Ok(bytes[0])
    }

    // TODO: Currently cannot write consecutive sections of memory
    // It would also make sense to give better error messages, e.g. if the write start address is within an area, but the data is too long
    pub fn mem_write_bytes(&mut self, address: u64, data: &[u8]) -> Result<(), AxError> {
        for area in &mut self.state.memory {
            if address >= area.start && address + data.len() as u64 <= area.start + area.length {
                let offset = (address - area.start) as usize;
                area.data[offset..offset + data.len()].copy_from_slice(data);

                return Ok(());
            }
        }

        Err(AxError::from(format!(
            "Could not write memory at address {:#x}",
            address
        )))
    }

    pub fn mem_write_64(&mut self, address: u64, data: u64) -> Result<(), AxError> {
        self.mem_write_bytes(address, &data.to_le_bytes())
    }

    pub fn mem_write_32(&mut self, address: u64, data: u32) -> Result<(), AxError> {
        self.mem_write_bytes(address, &data.to_le_bytes())
    }

    pub fn mem_write_16(&mut self, address: u64, data: u16) -> Result<(), AxError> {
        self.mem_write_bytes(address, &data.to_le_bytes())
    }

    pub fn mem_write_8(&mut self, address: u64, data: u8) -> Result<(), AxError> {
        self.mem_write_bytes(address, &[data])
    }

    pub fn mem_add_area(&mut self, start: u64, length: u64, data: Vec<u8>) -> Result<(), AxError> {
        // Make sure there's no overlapping area already defined
        for area in &self.state.memory {
            if start >= area.start && start < area.start + area.length {
                return Err(AxError::from(format!(
                    "cannot create memory area with start={:#x}, length={:#x}: overlaps with area with start={:#x}, length={:#x}",
                    start, length, area.start, area.length
                )));
            }
        }

        self.state.memory.push(MemoryArea {
            start,
            length,
            data,
        });

        Ok(())
    }
}
