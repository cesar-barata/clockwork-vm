pub type Word = i64;

use crate::instruction::Instruction;

#[derive(Default)]
struct Registers {
    data0: Word,
    data1: Word,
    data2: Word,
    data3: Word,
    instr_pointer: Word,
}

impl Registers {
    fn write(&mut self, index: usize, data: Word) {
        match index {
            0 => self.data0 = data,
            1 => self.data1 = data,
            2 => self.data2 = data,
            3 => self.data3 = data,
            4 => self.instr_pointer = data,
            _ => panic!("invalid register"),
        }
    }

    fn read(&self, index: usize) -> Word {
        match index {
            0 => self.data0,
            1 => self.data1,
            2 => self.data2,
            3 => self.data3,
            4 => self.instr_pointer,
            _ => panic!("invalid register"),
        }
    }
}

pub struct VM {
    registers: Registers,
    flag_zero: bool,
    flag_carry: bool,
    memory: Vec<Word>,
    running: bool
}

impl VM {
    const DEFAULT_MEMORY_SIZE_BYTES: usize = 2097152;

    fn init_memory(program: Vec<Word>, memory_vec_size: usize) -> Vec<Word> {
        let mut memory = vec![0; memory_vec_size];
        for (index, inst) in program.iter().enumerate() {
            memory[index] = *inst;
        }
        memory
    }

    pub fn new_with_memory_size(program: Vec<Word>, memory_size: usize) -> Self {
        let mem_vec_size = memory_size / std::mem::size_of::<Word>();
        VM {
            registers: Registers::default(),
            flag_zero: false,
            flag_carry: false,
            memory: Self::init_memory(program, mem_vec_size),
            running: false
        }
    }

    pub fn new(program: Vec<Word>) -> Self {
        Self::new_with_memory_size(program, Self::DEFAULT_MEMORY_SIZE_BYTES)
    }

    fn read_next_inst(&self) -> Word {
        let current_ip = self.registers.instr_pointer as usize;
        self.memory[current_ip]
    }

    fn consume_next_instr(&mut self) -> Word {
        let instruction = self.read_next_inst();
        self.registers.instr_pointer += 1;
        instruction
    }

    fn perform_next_instr(&mut self) -> bool {
        let instruction = self.consume_next_instr();

        match Instruction::from(instruction) {
            Instruction::Illegal                                  => panic!("Illegal opcode"),
            Instruction::Halt                                     => false,
            Instruction::Load { value, dest_reg }                 => self.perform_load(value, dest_reg),
            Instruction::Copy { src, dest }                       => self.perform_copy(src, dest),
            Instruction::Add { src1, src2, dest }                 => self.perform_add(src1, src2, dest),
            Instruction::Sub { src1, src2, dest }                 => self.perform_sub(src1, src2, dest),
            Instruction::Mult { src1, src2, dest }                => self.perform_mult(src1, src2, dest),
            Instruction::Div { src1, src2, quot_dest, rem_dest }  => self.perform_div(src1, src2, quot_dest, rem_dest),
            Instruction::Cmp { src1, src2 }                       => self.perform_cmp(src1, src2),
            Instruction::Jmp { src }                              => self.perform_jmp(src),
            Instruction::Jz { src }                               => self.perform_jz(src),
            Instruction::Jnz { src }                              => self.perform_jnz(src),
            Instruction::Jgt { src }                              => self.perform_jgt(src),
            Instruction::Jlt { src }                              => self.perform_jlt(src),
            Instruction::Inc { dest }                             => self.perform_inc(dest),
            Instruction::Dec { dest }                             => self.perform_dec(dest),
            Instruction::LoadMem { src_addr, dest_reg }           => self.perform_load_mem(src_addr, dest_reg),
            Instruction::StoreMem { src_reg, dest_addr }          => self.perform_store_mem(src_reg, dest_addr),
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            println!("mem: {:?}", self.memory);
            self.running = self.perform_next_instr();
        }
    }

    fn perform_load(&mut self, value: Word, dest_reg: u8) -> bool {
        self.registers.write(dest_reg as usize, value);
        true
    }

    fn perform_copy(&mut self, src: u8, dest: u8) -> bool {
        self.registers.write(dest as usize,self.registers.read(src as usize));
        true
    }

    fn perform_add(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
        let v1 = self.registers.read(src1 as usize);
        let v2 = self.registers.read(src2 as usize);
        self.registers.write(dest as usize, v1 + v2);
        true
    }

    fn perform_sub(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
        let v1 = self.registers.read(src1 as usize);
        let v2 = self.registers.read(src2 as usize);
        self.registers.write(dest as usize, v1 - v2);
        true
    }

    fn perform_mult(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
        let v1 = self.registers.read(src1 as usize);
        let v2 = self.registers.read(src2 as usize);
        self.registers.write(dest as usize, v1 * v2);
        true
    }

    fn perform_div(&mut self, src1: u8, src2: u8, quot_dest: u8, rem_dest: u8) -> bool {
        let v1 = self.registers.read(src1 as usize);
        let v2 = self.registers.read(src2 as usize);
        if v2 == 0 {
            todo!("division by zero");
        }
        self.registers.write(quot_dest as usize, v1 / v2);
        self.registers.write(rem_dest as usize, v1 % v2);
        true
    }

    fn perform_cmp(&mut self, src1: u8, src2: u8) ->  bool {
        let v1 =  self.registers.read(src1 as usize);
        let v2 =  self.registers.read(src2 as usize);

        if v1 == v2 {
            self.flag_zero = true;
            self.flag_carry = false;
        } else {
            self.flag_zero = false;
        }

        if v1 < v2 {
            self.flag_carry = true;
        }

        true
    }

    fn perform_jmp(&mut self, src: u8) -> bool {
        let v = self.registers.read(src as usize);
        self.registers.instr_pointer = v;
        true
    }

    fn perform_jz(&mut self, src: u8) -> bool {
        if self.flag_zero {
            let v = self.registers.read(src as usize);
            self.registers.instr_pointer = v;
        }
        true
    }

    fn perform_jnz(&mut self, src: u8) -> bool {
        if !self.flag_zero {
            let v = self.registers.read(src as usize);
            self.registers.instr_pointer = v;
        }
        true
    }

    fn perform_jgt(&mut self, src: u8) -> bool {
        if !self.flag_carry {
            let v = self.registers.read(src as usize);
            self.registers.instr_pointer = v;
        }
        true
    }

    fn perform_jlt(&mut self, src: u8) -> bool {
        if self.flag_carry {
            let v = self.registers.read(src as usize);
            self.registers.instr_pointer = v;
        }
        true
    }

    fn perform_inc(&mut self, dest: u8) -> bool {
        let current_value = self.registers.read(dest as usize);
        self.registers.write(dest as usize, current_value + 1);
        true
    }

    fn perform_dec(&mut self, dest: u8) -> bool {
        let current_value = self.registers.read(dest as usize);
        self.registers.write(dest as usize, current_value - 1);
        true
    }

    fn perform_load_mem(&mut self, src_addr: Word, dest_reg: u8) -> bool {
        self.registers.write(dest_reg as usize, self.memory[src_addr as usize]);
        true
    }

    fn perform_store_mem(&mut self, src_reg: u8, dest_addr: Word) -> bool {
        self.memory[dest_addr as usize] = self.registers.read(src_reg as usize);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_new_vm_has_default_values() {
        let program = vec![0; 0];
        let vm = VM::new(program);
        // assert_eq!(vm.old_registers, default_reg_values());
        assert_eq!(vm.running, false);
    }

    #[test]
    fn fetching_next_instruction_consumes_previous_ones() {
        let program = vec![7, 8, 9];
        let mut vm = VM::new(program);

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

        let mut vm = VM::new(program);

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
        let mut vm = VM::new(program);

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
        let mut vm = VM::new(program);

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
        let mut vm = VM::new(program);

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

        let mut vm = VM::new(program);

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
        let mut vm = VM::new(program);

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
        let mut vm = VM::new(program);

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
        let mut vm = VM::new(program);

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
        let mut vm = VM::new(program);
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
        let mut vm = VM::new(program);
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
        let mut vm = VM::new(program);
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
        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(449, vm.memory[0]);
    }

    #[test]
    fn loading_from_mem_affects_reg() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000000111000001_0000000001i64,    // load $449, d0
            0b000000000000000000000000000_000000000000000000000000000_0000010000i64,    // strm d0, @0
            0b000000000000000000000000001_000000000000000000000000000_0000001111i64,    // ldm @0, d1
            0b000000000000000000000000000000000000000000000000000000_0000000000i64,      // halt
        ];
        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(449, vm.registers.data1);
    }
}