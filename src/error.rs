use crate::vm::Word;

pub fn pair_result<T1, T2, E>(
    res1: std::result::Result<T1, E>,
    res2: std::result::Result<T2, E>
) -> std::result::Result<(T1, T2), E> {
    res1.and_then(|v1| res2.and_then(|v2| Ok((v1, v2))))
}

pub enum Error {
    IllegalOpcode { instruction: Word, instr_pointer: Word },
    InvalidRegister { number: usize, instr_pointer: Word },
    DivisionByZero { instr_pointer: Word },
    HaltSignal { instr_pointer: Word },
}

pub type Result<T> = std::result::Result<T, Error>;