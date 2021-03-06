pub type Word = i64;

use crate::util::pair_result;
use crate::instruction::Instruction;
use crate::error::{ Error, Result };
use crate::memory::Memory;
use crate::registers::Registers;

pub struct RuntimeBuilder {
    pub registers: Registers,
    pub memory: Memory,
}

impl RuntimeBuilder {
    pub fn new() -> Self {
        Self {
            registers: Registers::default(),
            memory: Memory::default(),
        }
    }

    pub fn with_registers(mut self, registers: Registers) -> Self {
        self.registers = registers;
        self
    }

    pub fn with_memory(mut self, memory: Memory) -> Self {
        self.memory = memory;
        self
    }

    pub fn with_program(mut self, program: Vec<Word>) -> Self {
        for (index, inst) in program.iter().enumerate() {
            self.memory.write(index as usize, *inst).expect("Error loading program");
        }
        self
    }

    pub fn build(self) -> Runtime {
        Runtime {
            registers: self.registers,
            memory: self.memory,
            flag_zero: false,
            flag_carry: false,
            running: false,
        }
    }
}

pub struct Runtime {
    registers: Registers,
    flag_zero: bool,
    flag_carry: bool,
    memory: Memory,
    running: bool,
}

impl Runtime {
    fn read_next_inst(&self) -> Word {
        let current_ip = self.registers.instr_pointer as usize;
        self.memory.read(current_ip).unwrap()
    }

    fn consume_next_instr(&mut self) -> Word {
        let instruction = self.read_next_inst();
        self.registers.instr_pointer += 1;
        instruction
    }

    fn perform_next_instr(&mut self) -> bool {
        let instruction = self.consume_next_instr();

        match Instruction::from(instruction) {
            Instruction::Illegal                                  => self.handle_illegal_opcode().is_ok(),
            Instruction::Halt                                     => false,
            Instruction::Load { value, dest_reg }                 => self.perform_load(value, dest_reg).is_ok(),
            Instruction::Copy { src, dest }                       => self.perform_copy(src, dest).is_ok(),
            Instruction::Add { src1, src2, dest }                 => self.perform_add(src1, src2, dest).is_ok(),
            Instruction::Sub { src1, src2, dest }                 => self.perform_sub(src1, src2, dest).is_ok(),
            Instruction::Mult { src1, src2, dest }                => self.perform_mult(src1, src2, dest).is_ok(),
            Instruction::Div { src1, src2, quot_dest, rem_dest }  => self.perform_div(src1, src2, quot_dest, rem_dest).is_ok(),
            Instruction::Cmp { src1, src2 }                       => self.perform_cmp(src1, src2).is_ok(),
            Instruction::Jmp { src }                              => self.perform_jmp(src).is_ok(),
            Instruction::Jz { src }                               => self.perform_jz(src).is_ok(),
            Instruction::Jnz { src }                              => self.perform_jnz(src).is_ok(),
            Instruction::Jgt { src }                              => self.perform_jgt(src).is_ok(),
            Instruction::Jlt { src }                              => self.perform_jlt(src).is_ok(),
            Instruction::Inc { dest }                             => self.perform_inc(dest).is_ok(),
            Instruction::Dec { dest }                             => self.perform_dec(dest).is_ok(),
            Instruction::LoadMem { src_addr, dest_reg }           => self.perform_load_mem(src_addr, dest_reg).is_ok(),
            Instruction::StoreMem { src_reg, dest_addr }          => self.perform_store_mem(src_reg, dest_addr).is_ok(),
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            self.running = self.perform_next_instr();
        }
    }

    fn handle_illegal_opcode(&self) -> Result<()> {
        Err(Error::IllegalOpcode { instruction: self.memory.read(self.registers.instr_pointer as usize).unwrap(), instr_pointer: self.registers.instr_pointer })
    }

    fn perform_load(&mut self, value: Word, dest_reg: u8) -> Result<()> {
        self.registers
            .write(dest_reg as usize, value)
    }

    fn perform_copy(&mut self, src: u8, dest: u8) -> Result<()> {
        self.registers
            .read(src as usize)
            .and_then(|value| self.registers.write(dest as usize, value))
    }

    fn perform_add(&mut self, src1: u8, src2: u8, dest: u8) -> Result<()> {
        let res1 = self.registers.read(src1 as usize);
        let res2 = self.registers.read(src2 as usize);
        pair_result(res1, res2).and_then(|(v1, v2)| self.registers.write(dest as usize, v1 + v2))
    }

    fn perform_sub(&mut self, src1: u8, src2: u8, dest: u8) -> Result<()> {
        let res1 = self.registers.read(src1 as usize);
        let res2 = self.registers.read(src2 as usize);
        pair_result(res1, res2).and_then(|(v1, v2)| self.registers.write(dest as usize, v1 - v2))
    }

    fn perform_mult(&mut self, src1: u8, src2: u8, dest: u8) -> Result<()> {
        let res1 = self.registers.read(src1 as usize);
        let res2 = self.registers.read(src2 as usize);
        pair_result(res1, res2).and_then(|(v1, v2)| self.registers.write(dest as usize, v1 * v2))
    }

    fn perform_div(&mut self, src1: u8, src2: u8, quot_dest: u8, rem_dest: u8) -> Result<()> {
        let res1 = self.registers.read(src1 as usize);
        let res2 = self.registers.read(src2 as usize);
        pair_result(res1, res2).and_then(|(v1, v2)| {
            if v2 == 0 {
                return Err(Error::DivisionByZero { instr_pointer: self.registers.instr_pointer });
            }
            self.registers
                .write(quot_dest as usize, v1 / v2)
                .and_then(|()| self.registers.write(rem_dest as usize, v1 % v2))
        })
    }

    fn perform_cmp(&mut self, src1: u8, src2: u8) -> Result<()> {
        let res1 = self.registers.read(src1 as usize);
        let res2 = self.registers.read(src2 as usize);
        pair_result(res1, res2).map(|(v1, v2)| {
            self.flag_zero = v1 == v2;
            self.flag_carry = v1 < v2;
        })
    }

    fn perform_jmp(&mut self, src: u8) -> Result<()> {
        self.registers
            .read(src as usize)
            .map(|v| self.registers.instr_pointer = v)
    }

    fn perform_jz(&mut self, src: u8) -> Result<()> {
        if self.flag_zero {
            self.registers
                .read(src as usize)
                .map(|v| self.registers.instr_pointer = v)
        } else {
            Ok(())
        }
    }

    fn perform_jnz(&mut self, src: u8) -> Result<()> {
        if !self.flag_zero {
            self.registers
                .read(src as usize)
                .map(|v| self.registers.instr_pointer = v)
        } else {
            Ok(())
        }
    }

    fn perform_jgt(&mut self, src: u8) -> Result<()> {
        if !self.flag_carry {
            self.registers
                .read(src as usize)
                .map(|v| self.registers.instr_pointer = v)
        } else {
            Ok(())
        }
    }

    fn perform_jlt(&mut self, src: u8) -> Result<()> {
        if self.flag_carry {
            self.registers
                .read(src as usize)
                .map(|v| self.registers.instr_pointer = v)
        } else {
            Ok(())
        }
    }

    fn perform_inc(&mut self, dest: u8) -> Result<()> {
        self.registers
            .read(dest as usize)
            .and_then(|current_value| self.registers.write(dest as usize, current_value + 1))
    }

    fn perform_dec(&mut self, dest: u8) -> Result<()> {
        self.registers
            .read(dest as usize)
            .and_then(|current_value| self.registers.write(dest as usize, current_value - 1))
    }

    fn perform_load_mem(&mut self, src_addr: Word, dest_reg: u8) -> Result<()> {
        self.registers.write(dest_reg as usize, self.memory.read(src_addr as usize).unwrap())
    }

    fn perform_store_mem(&mut self, src_reg: u8, dest_addr: Word) -> Result<()> {
        self.registers
            .read(src_reg as usize)
            .map(|value| self.memory.write(dest_addr as usize, value).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_new_runtime_has_default_values() {
        let vm = RuntimeBuilder::new()
            .build();
        
        assert_eq!(vm.flag_zero, false);
        assert_eq!(vm.flag_carry, false);
        assert_eq!(vm.running, false);
    }

    #[test]
    fn fetching_next_instruction_consumes_previous_ones() {
        let program = vec![7, 8, 9];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();

        let instruction = vm.consume_next_instr();
        let expected = 7;
        assert_eq!(expected, instruction);

        let instruction = vm.consume_next_instr();
        let expected = 8;
        assert_eq!(expected, instruction);

        let instruction = vm.consume_next_instr();
        let expected = 9;
        assert_eq!(expected, instruction);
    }

    #[test]
    fn load_affects_specific_registers() {
        let expected_d0 = 0b1101;
        let expected_d1 = 0b0110_0100;
        let expected_d2 = 0b0110_0001;
        let expected_d3 = 0b0011_0010_1001_0100;

        /*
         * Writes to d0, d1, d2, d3 at each step
         */
        let program = vec![
            0b00000000_0000000000000000000000000000000000000000001101_0000000001i64, // load $13, d0
            0b00000001_0000000000000000000000000000000000000001100100_0000000001i64, // load $100, d1
            0b00000010_0000000000000000000000000000000000000001100001_0000000001i64, // load $99, d2
            0b00000011_0000000000000000000000000000000011001010010100_0000000001i64, // load $12948, d3
        ];

        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();

        vm.perform_next_instr();
        assert_eq!(expected_d0, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);
        assert_eq!(0, vm.registers.data2);
        assert_eq!(0, vm.registers.data3);
        assert_eq!(1, vm.registers.instr_pointer);

        vm.perform_next_instr();
        assert_eq!(expected_d0, vm.registers.data0);
        assert_eq!(expected_d1, vm.registers.data1);
        assert_eq!(0, vm.registers.data2);
        assert_eq!(0, vm.registers.data3);
        assert_eq!(2, vm.registers.instr_pointer);

        vm.perform_next_instr();
        assert_eq!(expected_d0, vm.registers.data0);
        assert_eq!(expected_d1, vm.registers.data1);
        assert_eq!(expected_d2, vm.registers.data2);
        assert_eq!(0, vm.registers.data3);
        assert_eq!(3, vm.registers.instr_pointer);

        vm.perform_next_instr();
        assert_eq!(expected_d0, vm.registers.data0);
        assert_eq!(expected_d1, vm.registers.data1);
        assert_eq!(expected_d2, vm.registers.data2);
        assert_eq!(expected_d3, vm.registers.data3);
        assert_eq!(4, vm.registers.instr_pointer);
    }

    #[test]
    fn copy_should_make_src_value_equal_to_dest_value() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000000000010001_0000000001i64,   // load $17, d0
            0b000000000000000000000000001_000000000000000000000000000_0000001100i64,   // copy d0, d1
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();

        assert_eq!(0, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);

        vm.perform_next_instr();  // load $17, d0
        assert_eq!(17, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);

        vm.perform_next_instr();  // copy d0, d1
        assert_eq!(17, vm.registers.data0);
        assert_eq!(17, vm.registers.data1);
    }

    #[test]
    fn addition_should_preserve_operand_regs_and_update_dest_reg() {
        let expected_result = 5000;

        /*
         * Loads 2000 and 3000 to d0 and d1 respectively, then performs addition with destination d3
         */
        let program = vec![
            0b00000000_0000000000000000000000000000000000011111010000_0000000001i64,
            0b00000001_0000000000000000000000000000000000101110111000_0000000001i64,
            0b000000000000000011_000000000000000001_000000000000000000_0000000010i64
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();

        vm.perform_next_instr();
        assert_eq!(0b11111010000, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);
        assert_eq!(0, vm.registers.data3);

        vm.perform_next_instr();
        assert_eq!(0b11111010000, vm.registers.data0);
        assert_eq!(0b101110111000, vm.registers.data1);
        assert_eq!(0, vm.registers.data3);

        vm.perform_next_instr();
        assert_eq!(0b11111010000, vm.registers.data0);
        assert_eq!(0b101110111000, vm.registers.data1);
        assert_eq!(expected_result, vm.registers.data3);
    }

    #[test]
    fn subtraction_should_preserve_operand_regs_and_update_dest_reg() {
        let expected_result = -1000;

        /*
         * Loads 2000 and 3000 to d0 and d1 respectively, then performs subtraction with destination d3
         */
        let program = vec![
            0b00000000_0000000000000000000000000000000000011111010000_0000000001i64,
            0b00000001_0000000000000000000000000000000000101110111000_0000000001i64,
            0b000000000000000011_000000000000000001_000000000000000000_0000000011i64
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();

        vm.perform_next_instr();
        assert_eq!(0b11111010000, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);
        assert_eq!(0, vm.registers.data3);

        vm.perform_next_instr();
        assert_eq!(0b11111010000, vm.registers.data0);
        assert_eq!(0b101110111000, vm.registers.data1);
        assert_eq!(0, vm.registers.data3);

        vm.perform_next_instr();
        assert_eq!(0b11111010000, vm.registers.data0);
        assert_eq!(0b101110111000, vm.registers.data1);
        assert_eq!(expected_result, vm.registers.data3);
    }

    #[test]
    fn multiplication_should_preserve_operand_regs_and_update_dest_reg() {
        let expected_result = 6_000_000;

        /*
         * Loads 2000 and 3000 to d0 and d1 respectively, then performs multiplication with destination d3
         */
        let program = vec![
            0b00000000_0000000000000000000000000000000000011111010000_0000000001i64,
            0b00000001_0000000000000000000000000000000000101110111000_0000000001i64,
            0b000000000000000011_000000000000000001_000000000000000000_0000000100i64
        ];

        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();

        vm.perform_next_instr();
        assert_eq!(0b11111010000, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);
        assert_eq!(0, vm.registers.data3);

        vm.perform_next_instr();
        assert_eq!(0b11111010000, vm.registers.data0);
        assert_eq!(0b101110111000, vm.registers.data1);
        assert_eq!(0, vm.registers.data3);

        vm.perform_next_instr();
        assert_eq!(0b11111010000, vm.registers.data0);
        assert_eq!(0b101110111000, vm.registers.data1);
        assert_eq!(expected_result, vm.registers.data3);
    }

    #[test]
    fn division_should_affect_both_quot_dest_and_rem_dest() {
        let expected_quotient = 3;
        let expected_remainder = 619;

        let program = vec![
            0b00000000_0000000000000000000000000000000001000011100001_0000000001i64,    // load $4321, d0
            0b00000001_0000000000000000000000000000000000010011010010_0000000001i64,    // load $1234, d1
            0b000000000000011_0000000000010_0000000000001_0000000000000_0000001011i64,  // div d0 d1 d2 d3
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();

        vm.perform_next_instr();  // load $4321, d0
        vm.perform_next_instr();  // load $1234, d1
        vm.perform_next_instr();  // div d0 d1 d2 d3

        assert_eq!(expected_quotient, vm.registers.data2);
        assert_eq!(expected_remainder, vm.registers.data3);
    }

    #[test]
    fn cmp_should_affect_zero_flag() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000011111010000_0000000001i64,    // load $2000, d0
            0b00000001_0000000000000000000000000000000000101110111000_0000000001i64,    // load $3000, d1
            0b00000010_0000000000000000000000000000000000011111010000_0000000001i64,    // load $2000, d2
            0b000000000000000000000000001_000000000000000000000000000_0000000101i64,    // cmp d0, d1
            0b000000000000000000000000010_000000000000000000000000000_0000000101i64,    // cmp d0, d2
            0b000000000000000000000000000_000000000000000000000000001_0000000101i64,    // cmp d1, d0
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();

        vm.perform_next_instr();  // load $2000, d0
        vm.perform_next_instr();  // load $3000, d1
        vm.perform_next_instr();  // load $2000, d2

        vm.perform_next_instr();  // cmp d0, d1
        assert!(!vm.flag_zero);

        vm.perform_next_instr();  // cmp d0, d2
        assert!(vm.flag_zero);

        vm.perform_next_instr();  // cmp d1, d0
        assert!(!vm.flag_zero);
    }

    #[test]
    fn jmp_should_affect_ip_reg() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000000000000100_0000000001i64,    // load $4, d0
            0b00000000_0000000000000000000000000000000000000000000011_0000000001i64,    // load $3, d0
            0b00000000_0000000000000000000000000000000000000000000010_0000000001i64,    // load $2, d0
            0b00000001_0000000000000000000000000000000000000000000001_0000000001i64,    // load $1, d1
            0b000000000000000000000000000000000000000000000000000001_0000000110i64,     // jmp d1
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();

        assert_eq!(0, vm.registers.instr_pointer);
        assert_eq!(0, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);

        vm.perform_next_instr();  // load $4, d0

        assert_eq!(1, vm.registers.instr_pointer);
        assert_eq!(4, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);

        vm.perform_next_instr();  // load $3, d0

        assert_eq!(2, vm.registers.instr_pointer);
        assert_eq!(3, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);

        vm.perform_next_instr();  // load $2, d0

        assert_eq!(3, vm.registers.instr_pointer);
        assert_eq!(2, vm.registers.data0);
        assert_eq!(0, vm.registers.data1);

        vm.perform_next_instr();  // load $1, d1

        assert_eq!(4, vm.registers.instr_pointer);
        assert_eq!(2, vm.registers.data0);
        assert_eq!(1, vm.registers.data1);

        vm.perform_next_instr();  // jmp d1

        assert_eq!(1, vm.registers.instr_pointer);
        assert_eq!(2, vm.registers.data0);
        assert_eq!(1, vm.registers.data1);

        vm.perform_next_instr();  // load $3, d0

        assert_eq!(2, vm.registers.instr_pointer);
        assert_eq!(3, vm.registers.data0);
        assert_eq!(1, vm.registers.data1);
    }

    #[test]
    fn euclidean_algorithm_gcd_of_230_449() {
        let program = vec![
            0b00000001_0000000000000000000000000000000000000011100110_0000000001i64,    // load $230, d1     ; divisor
            0b00000000_0000000000000000000000000000000000000111000001_0000000001i64,    // load $449, d0     ; dividend
            0b00000010_0000000000000000000000000000000000000000000000_0000000001i64,    // load $0, d2       ; clear remainder location
            0b00000011_0000000000000000000000000000000000000000000000_0000000001i64,    // load $0, d3       ; for zero comparison
            0b000000000000010_0000000000000_0000000000001_0000000000000_0000001011i64,  // div  d0 d1 d0 d2  ; perform division
            0b000000000000000000000000000_000000000000000000000000001_0000001100i64,    // copy d1, d0       ; divisor is the new dividend
            0b000000000000000000000000001_000000000000000000000000010_0000001100i64,    // copy d2, d1       ; remainder is the new divisor
            0b000000000000000000000000011_000000000000000000000000010_0000000101i64,    // cmp  d2, d3       ; check if remainder is zero
            0b00000011_0000000000000000000000000000000000000000000010_0000000001i64,    // load $2, d3       ; load wanted ip value
            0b000000000000000000000000000000000000000000000000000011_0000001000i64,     // jnz d3            ; jump back to step 2 (0-based)
            0b0000000000000000000000000000000000000000000000000000000000000000i64,      // halt              ; stop (result is in d0)
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();
        vm.run();
    
        assert_eq!(1, vm.registers.data0);
    }

    #[test]
    fn inc_should_increment_a_reg_by_one() {
        let expected_value = 231;

        let program = vec![
            0b00000000_0000000000000000000000000000000000000011100110_0000000001i64,    // load $230, d0
            0b000000000000000000000000000000000000000000000000000000_0000001101i64,     // inc d0
            0b0000000000000000000000000000000000000000000000000000000000000000i64,      // halt
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();
        vm.run();

        assert_eq!(expected_value, vm.registers.data0);
    }

    #[test]
    fn dec_should_decrement_a_reg_by_one() {
        let expected_value = 448;

        let program = vec![
            0b00000000_0000000000000000000000000000000000000111000001_0000000001i64,    // load $449, d0
            0b000000000000000000000000000000000000000000000000000000_0000001110i64,     // dec d0
            0b0000000000000000000000000000000000000000000000000000000000000000i64,      // halt
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();
        vm.run();

        assert_eq!(expected_value, vm.registers.data0);
    }

    #[test]
    fn storing_on_mem_affects_mem() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000000111000001_0000000001i64,    // load $449, d0
            0b000000000000000000000000000_000000000000000000000000000_0000010000i64,    // strm d0, @0
            0b0000000000000000000000000000000000000000000000000000000000000000i64,      // halt
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();
        vm.run();

        assert_eq!(449, vm.memory.read(0).unwrap());
    }

    #[test]
    fn loading_from_mem_affects_reg() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000000111000001_0000000001i64,    // load $449, d0
            0b000000000000000000000000000_000000000000000000000000000_0000010000i64,    // strm d0, @0
            0b000000000000000000000000001_000000000000000000000000000_0000001111i64,    // ldm @0, d1
            0b000000000000000000000000000000000000000000000000000000_0000000000i64,      // halt
        ];
        let mut vm = RuntimeBuilder::new()
            .with_program(program)
            .build();
        vm.run();

        assert_eq!(449, vm.registers.data1);
    }
}