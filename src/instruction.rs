use crate::Word;

use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Instruction {
	Illegal,
	Halt,
	Load { value: Word, dest_reg: u8 },
    Add { src1: u8, src2: u8, dest: u8  }
}

/*
 * For each instruction there is a corresponding parsing function to be used on the
 * implementation for the "From" trait.	Each function has a comment describing the
 * binary layout of the instruction.
 */
impl Instruction {
	const OPCODE_OFFSET: usize = 10;
	const OPCODE_MASK: u64 = 0b000000_1111111111;

	const LOAD_RANDS_MASK: u64 = 0b00000000_1111111111111111111111111111111111111111111111;
	const LOAD_DEST_OFFSET: usize = 46;

	const ADD_RAND2_OFFSET: usize = 18;
	const ADD_DEST_OFFSET: usize = 36;

	/*
	 * LOAD
	 *
	 *    DEST                         VALUE                         OPCODE
     * 0b00000000_00000000000000000000000000000000000000000000000(_0000000000)
     * 
     */
    fn parse_load(operands: u64) -> Self {
        let value = (operands & Self::LOAD_RANDS_MASK) as u64;
        let dest_reg = (operands >> Self::LOAD_DEST_OFFSET) as u8;
        Instruction::Load { value, dest_reg }
    }

    /*
     * ADD
     *
	 *          DEST               SRC2               SRC1           OPCODE
     * 0b000000000000000000_000000000000000000_000000000000000000(_0000000000)
     */
    fn parse_add(operands: u64) -> Self {
        let src1 = operands as u8;
        let src2 = (operands >> Self::ADD_RAND2_OFFSET) as u8;
        let dest = (operands >> Self::ADD_DEST_OFFSET) as u8;
        Instruction::Add { src1, src2, dest }
    }
}

impl From<Word> for Instruction {
	fn from(instruction: Word) -> Self {
        let opcode = (instruction & Self::OPCODE_MASK) as u16;
        let operands = (instruction >> Self::OPCODE_OFFSET) as u64;
		match opcode {
			0 => Instruction::Halt,
            1 => Self::parse_load(operands),
            2 => Self::parse_add(operands),
			x if x > 1024 => Instruction::Illegal, // we have only 2.pow(10) = 1024 opcode slots
			_ => Instruction::Illegal              // for still unimplemented instructions
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_from_word() {
        let instruction: Word = 0b000000000000000000000000000000000000000000000000000000_0000000000;
        let expected = Instruction::Halt;
        let actual = Instruction::from(instruction);
        assert_eq!(expected, actual);

        let instruction: Word = 0b000000000000000000000000000000000000000000000000000000_1000000000;
        let expected = Instruction::Illegal;
        let actual = Instruction::from(instruction);
        assert_eq!(expected, actual);

        let instruction: Word = 0b00001010_0000000000000000000000000000000000001111101000_0000000001;
        let expected = Instruction::Load { dest_reg: 10, value: 1000 };
        let actual = Instruction::from(instruction);
        assert_eq!(expected, actual);
    }
}