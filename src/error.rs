use crate::runtime::Word;

#[derive(Debug)]
pub enum Error {
    IllegalOpcode { instruction: Word, instr_pointer: Word },
    InvalidRegister { number: usize, instr_pointer: Word },
    DivisionByZero { instr_pointer: Word },
    InvalidMemoryAddress { requested_address: usize, upper_bound: usize },
}

pub type Result<T> = std::result::Result<T, Error>;