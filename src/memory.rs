use crate::runtime::Word;
use crate::error::{ Error, Result };

pub struct Memory {
    buffer: Vec<Word>,
}

impl Memory {
    const DEFAULT_MEMORY_SIZE_BYTES: usize = 2097152;

    pub fn new_with_size(size_bytes: usize) -> Self {
        let mem_vec_size = size_bytes / std::mem::size_of::<Word>();
        Memory { buffer: vec![0; mem_vec_size] }
    }

    pub fn write(&mut self, address: usize, data: Word) -> Result<()> {
        if address as usize >= self.buffer.len() {
            Err(Error::InvalidMemoryAddress { requested_address: address, upper_bound: self.buffer.len() })
        } else {
            self.buffer[address as usize] = data;
            Ok(())
        }
    }

    pub fn read(&self, address: usize) -> Result<Word> {
        if address as usize >= self.buffer.len() {
            Err(Error::InvalidMemoryAddress { requested_address: address, upper_bound: self.buffer.len() })
        } else {
            Ok(self.buffer[address as usize])
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new_with_size(Self::DEFAULT_MEMORY_SIZE_BYTES)
    }
}