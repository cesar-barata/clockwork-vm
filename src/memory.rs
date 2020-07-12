use crate::vm::Word;

pub enum Error {
    InvalidAddress { requested_address: Word, upper_bound: usize },
    OutOfMemory,
}

type Result<T> = std::result::Result<T, Error>;

pub struct Memory {
    buffer: Vec<Word>,
}

impl Memory {
    const DEFAULT_MEMORY_SIZE_BYTES: usize = 2097152;

    fn new_with_size(size_bytes: usize) -> Self {
        let mem_vec_size = size_bytes / std::mem::size_of::<Word>();
        Memory { buffer: vec![0; mem_vec_size] }
    }

    fn write(&mut self, address: Word, data: Word) -> Result<()> {
        if address < 0 || address as usize >= self.buffer.len() {
            Err(Error::InvalidAddress { requested_address: address, upper_bound: self.buffer.len() })
        } else {
            self.buffer[address as usize] = data;
            Ok(())
        }
    }

    fn read(&self, address: Word) -> Result<Word> {
        if address < 0 || address as usize >= self.buffer.len() {
            Err(Error::InvalidAddress { requested_address: address, upper_bound: self.buffer.len() })
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