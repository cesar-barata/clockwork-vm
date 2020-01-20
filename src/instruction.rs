use crate::vm::Word;

use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Illegal,
    Halt,
    Load { value: Word, dest_reg: u8 },
    LoadMem { src_addr: Word, dest_reg: u8 },
    StoreMem { src_reg: u8, dest_addr: Word },
    Copy { src: u8, dest: u8 },
    Add { src1: u8, src2: u8, dest: u8 },
    Sub { src1: u8, src2: u8, dest: u8 },
    Mult { src1: u8, src2: u8, dest: u8 },
    Div { src1: u8, src2: u8, quot_dest: u8, rem_dest: u8 },
    Cmp { src1: u8, src2: u8 },
    Jmp { src: u8 },
    Jz { src: u8 },
    Jnz { src: u8 },
    Jgt { src: u8 },
    Jlt { src: u8 },
    Inc { dest: u8 },
    Dec { dest: u8 },
}

/*
 * For each instruction there is a corresponding parsing function to be used on the
 * implementation for the "From" trait. Each function has a comment describing the
 * binary layout of the instruction.
 */
impl Instruction {
    const OPCODE_OFFSET: usize = 10;
    const OPCODE_MASK: Word = 0b000000_1111111111;

    const LOAD_RANDS_MASK: Word = 0b00000000_1111111111111111111111111111111111111111111111;
    const LOAD_DEST_OFFSET: usize = 46;

    const COPY_RAND2_OFFSET: usize = 27;

    const ADD_RAND2_OFFSET: usize = 18;
    const ADD_DEST_OFFSET: usize = 36;

    const SUB_RAND2_OFFSET: usize = 18;
    const SUB_DEST_OFFSET: usize = 36;

    const MULT_RAND2_OFFSET: usize = 18;
    const MULT_DEST_OFFSET: usize = 36;

    const DIV_RAND2_OFFSET: usize = 13;
    const DIV_QUOT_OFFSET: usize = 26;
    const DIV_REM_OFFSET: usize = 39;

    const CMP_RAND2_OFFSET: usize = 27;

    const LOAD_MEM_DEST_OFFSET: usize = 27;
    const STORE_MEM_DEST_OFFSET: usize = 27;

    /*
     * LOAD
     *
     *    DEST                        VALUE                         OPCODE
     * 0b00000000_0000000000000000000000000000000000000000000000(_0000000000)
     * 0x00_00_00_00_00_00_00_00
     */
    fn parse_load(operands: Word) -> Self {
        let value = (operands & Self::LOAD_RANDS_MASK) as Word;
        let dest_reg = (operands >> Self::LOAD_DEST_OFFSET) as u8;
        Instruction::Load { value, dest_reg }
    }

    /*
     * COPY
     *
     *             DEST                          SRC                OPCODE
     * 0b000000000000000000000000000_000000000000000000000000000(_0000000000)
     */
    fn parse_copy(operands: Word) -> Self {
        let src = operands as u8;
        let dest = (operands >> Self::COPY_RAND2_OFFSET) as u8;
        Instruction::Copy { src, dest }
    }

    /*
     * ADD
     *
     *          DEST               SRC2               SRC1           OPCODE
     * 0b000000000000000000_000000000000000000_000000000000000000(_0000000000)
     */
    fn parse_add(operands: Word) -> Self {
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
    fn parse_sub(operands: Word) -> Self {
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
    fn parse_mult(operands: Word) -> Self {
        let src1 = operands as u8;
        let src2 = (operands >> Self::MULT_RAND2_OFFSET) as u8;
        let dest = (operands >> Self::MULT_DEST_OFFSET) as u8;
        Instruction::Mult { src1, src2, dest }
    }

    /*
     * DIV
     *
     *       REM              QUOT          SRC2          SRC1       OPCODE
     * 0b000000000000000_0000000000000_0000000000000_0000000000000(_0000000000)
     */
    fn parse_div(operands: Word) -> Self {
        let src1 = operands as u8;
        let src2 = (operands >> Self::DIV_RAND2_OFFSET) as u8;
        let quot_dest = (operands >> Self::DIV_QUOT_OFFSET) as u8;
        let rem_dest =  (operands >> Self::DIV_REM_OFFSET) as u8;
        Instruction::Div { src1, src2, quot_dest, rem_dest }
    }

    /*
     * CMP
     *
     *              SRC2                         SRC1                OPCODE
     * 0b000000000000000000000000000_000000000000000000000000000(_0000000000)
     */
    fn parse_cmp(operands: Word) -> Self {
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
    fn parse_jmp(operands: Word) -> Self {
        Instruction::Jmp { src: operands as u8 }
    }

    /*
     * JZ
     *
     *                            SRC                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_jz(operands: Word) -> Self {
        Instruction::Jz { src: operands as u8 }
    }

    /*
     * JNZ
     *
     *                            SRC                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_jnz(operands: Word) -> Self {
        Instruction::Jnz { src: operands as u8 }
    }

    /*
     * JGT
     *
     *                            SRC                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_jgt(operands: Word) -> Self {
        Instruction::Jgt { src: operands as u8 }
    }

    /*
     * JLT
     *
     *                            SRC                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_jlt(operands: Word) -> Self {
        Instruction::Jlt { src: operands as u8 }
    }

    /*
     * INC
     *
     *                           DEST                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_inc(operands: Word) -> Self {
        Instruction::Inc { dest: operands as u8 }
    }

    /*
     * DEC
     *
     *                           DEST                               OPCODE
     * 0b000000000000000000000000000000000000000000000000000000(_0000000000)
     */
    fn parse_dec(operands: Word) -> Self {
        Instruction::Dec { dest: operands as u8 }
    }

    /*
     * LDM
     *
     *             DEST                           SRC               OPCODE
     * 0b000000000000000000000000000_000000000000000000000000000(_0000000000)
     */
    fn parse_load_mem(operands: Word) -> Self {
        let src_addr = (operands as i16) as Word; // TODO: use bit mask to extract src_addr
        let dest_reg = (operands >> Self::LOAD_MEM_DEST_OFFSET) as u8;
        Instruction::LoadMem { src_addr, dest_reg }
    }

    /*
     * STRM
     *
     *             DEST                           SRC               OPCODE
     * 0b000000000000000000000000000_000000000000000000000000000(_0000000000)
     */
    fn parse_store_mem(operands: Word) -> Self {
        let src_reg = operands as u8;
        let dest_addr = (operands >> Self::STORE_MEM_DEST_OFFSET) as Word;
        Instruction::StoreMem { src_reg, dest_addr }
    }
}

impl From<Word> for Instruction {
    fn from(instruction: Word) -> Self {
        let opcode = instruction & Self::OPCODE_MASK;
        let operands = (instruction >> Self::OPCODE_OFFSET) as Word;
        match opcode {
            0             => Instruction::Halt,
            1             => Self::parse_load(operands),
            2             => Self::parse_add(operands),
            3             => Self::parse_sub(operands),
            4             => Self::parse_mult(operands),
            5             => Self::parse_cmp(operands),
            6             => Self::parse_jmp(operands),
            7             => Self::parse_jz(operands),
            8             => Self::parse_jnz(operands),
            9             => Self::parse_jgt(operands),
            10            => Self::parse_jlt(operands),
            11            => Self::parse_div(operands),
            12            => Self::parse_copy(operands),
            13            => Self::parse_inc(operands),
            14            => Self::parse_dec(operands),
            15            => Self::parse_load_mem(operands),
            16            => Self::parse_store_mem(operands),
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