use crate::vm::Word;

use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Illegal,
    Halt,
    Load { value: Word, dest_reg: u8 },
    Add { src1: u8, src2: u8, dest: u8 },
    Sub { src1: u8, src2: u8, dest: u8 },
    Mult { src1: u8, src2: u8, dest: u8 },
    Cmp { src1: u8, src2: u8 },
    Jmp { src: u8 },
    Jeq { src: u8 },
    Jneq { src: u8 },
    Jgt { src: u8 },
    Jlt { src: u8 },
}

/*
 * For each instruction there is a corresponding parsing function to be used on the
 * implementation for the "From" trait. Each function has a comment describing the
 * binary layout of the instruction.
 */
impl Instruction {
    const OPCODE_OFFSET: usize = 10;
    const OPCODE_MASK: u64 = 0b000000_1111111111;

    const LOAD_RANDS_MASK: u64 = 0b00000000_1111111111111111111111111111111111111111111111;
    const LOAD_DEST_OFFSET: usize = 46;

    const ADD_RAND2_OFFSET: usize = 18;
    const ADD_DEST_OFFSET: usize = 36;

    const SUB_RAND2_OFFSET: usize = 18;
    const SUB_DEST_OFFSET: usize = 36;

    const MULT_RAND2_OFFSET: usize = 18;
    const MULT_DEST_OFFSET: usize = 36;

    const CMP_RAND2_OFFSET: usize = 27;

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

    /*
     * SUB
     *
     *          DEST               SRC2               SRC1           OPCODE
     * 0b000000000000000000_000000000000000000_000000000000000000(_0000000000)
     */
    fn parse_sub(operands: u64) -> Self {
        let src1 = operands as u8;
        let src2 = (operands >> Self::SUB_RAND2_OFFSET) as u8;
        let dest = (operands >> Self::SUB_DEST_OFFSET) as u8;
        Instruction::Sub { src1, src2, dest }
    }

    /*
     * MULT
     *
     *          DEST               SRC2               SRC1           OPCODE
     * 0b000000000000000000_000000000000000000_000000000000000000(_0000000000)
     */
    fn parse_mult(operands: u64) -> Self {
        let src1 = operands as u8;
        let src2 = (operands >> Self::MULT_RAND2_OFFSET) as u8;
        let dest = (operands >> Self::MULT_DEST_OFFSET) as u8;
        Instruction::Mult { src1, src2, dest }
    }

    /*
     * CMP
     *
     *              SRC2                         SRC1                OPCODE
     * 0b000000000000000000000000000_000000000000000000000000000(_0000000000)
     */
    fn parse_cmp(operands: u64) -> Self {
        let src1 = operands as u8;
        let src2 = (operands >> Self::CMP_RAND2_OFFSET) as u8;
        Instruction::Cmp { src1, src2 }
    }

    /*
     * JMP
     *
     *                            SRC                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_jmp(operands: u64) -> Self {
        Instruction::Jmp { src: operands as u8 }
    }

    /*
     * JEQ
     *
     *                            SRC                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_jeq(operands: u64) -> Self {
        Instruction::Jeq { src: operands as u8 }
    }

    /*
     * JNEQ
     *
     *                            SRC                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_jneq(operands: u64) -> Self {
        Instruction::Jneq { src: operands as u8 }
    }

    /*
     * JGT
     *
     *                            SRC                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_jgt(operands: u64) -> Self {
        Instruction::Jgt { src: operands as u8 }
    }

    /*
     * JLT
     *
     *                            SRC                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_jlt(operands: u64) -> Self {
        Instruction::Jlt { src: operands as u8 }
    }
}

impl From<Word> for Instruction {
    fn from(instruction: Word) -> Self {
        let opcode = (instruction & Self::OPCODE_MASK) as u16;
        let operands = (instruction >> Self::OPCODE_OFFSET) as u64;
        match opcode {
            0             => Instruction::Halt,
            1             => Self::parse_load(operands),
            2             => Self::parse_add(operands),
            3             => Self::parse_sub(operands),
            4             => Self::parse_mult(operands),
            5             => Self::parse_cmp(operands),
            6             => Self::parse_jmp(operands),
            7             => Self::parse_jeq(operands),
            8             => Self::parse_jneq(operands),
            9             => Self::parse_jgt(operands),
            10            => Self::parse_jlt(operands),
            x if x > 1024 => Instruction::Illegal, // we have only 2.pow(10) = 1024 opcode slots
            _             => Instruction::Illegal              // for still unimplemented instructions
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