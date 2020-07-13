use crate::runtime::Word;
use crate::error::{ Error, Result };

#[derive(Default)]
pub struct Registers {
    pub data0: Word,
    pub data1: Word,
    pub data2: Word,
    pub data3: Word,
    pub instr_pointer: Word,
}

impl Registers {
    pub fn write(&mut self, index: usize, data: Word) -> Result<()> {
        match index {
            0 => {
                self.data0 = data;
                Ok(())
            },
            1 => {
                self.data1 = data;
                Ok(())
            },
            2 => {
                self.data2 = data;
                Ok(())
            },
            3 => {
                self.data3 = data;
                Ok(())
            },
            _ => Err(Error::InvalidRegister { number: index, instr_pointer: self.instr_pointer }),
        }
    }

    pub fn read(&self, index: usize) -> Result<Word> {
        match index {
            0 => Ok(self.data0),
            1 => Ok(self.data1),
            2 => Ok(self.data2),
            3 => Ok(self.data3),
            4 => Ok(self.instr_pointer),
            _ => Err(Error::InvalidRegister { number: index, instr_pointer: self.instr_pointer }),
        }
    }
}