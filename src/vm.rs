pub type Word = i64;

use crate::instruction::Instruction;

pub struct VM {
    registers: [Word; NUM_REGS],
    memory: Vec<Word>,
    running: bool
}

const NUM_REGS: usize = 9; // TODO: get this automatically by the number of variants on the "Reg" enum
enum Reg {
    Data0        = 0x0000,
    Data1        = 0x0001,
    Data2        = 0x0002,
    Data3        = 0x0003,
    InstPointer  = 0x0004,
    Flags        = 0x0005,
    CodeSegment  = 0x0006,
    DataSegment  = 0x0007,
    StackSegment = 0x0008,
}

enum Flag {
    Zero  = 0x0001,
    Carry = 0x0002,
}

fn default_reg_values(program_size: Word) -> [Word; NUM_REGS] {
    [0, 0, 0, 0, VM::INITIAL_IP, 0, VM::DEFAULT_CS, program_size, 0]
}

impl VM {
    const DEFAULT_MEMORY_SIZE_BYTES: usize = 2097152;
    const DEFAULT_CS: Word = 0;
    const INITIAL_IP: Word = 0;

    fn is_reg_writable(index: usize) -> bool {
        index != Reg::Flags as usize
    }

    fn init_memory(program: Vec<Word>, memory_vec_size: usize) -> Vec<Word> {
        let mut memory = vec![0; memory_vec_size];
        for (index, inst) in program.iter().enumerate() {
            memory[Self::DEFAULT_CS as usize + index] = *inst;
        }
        memory
    }

    pub fn new_with_memory_size(program: Vec<Word>, memory_size: usize) -> Self {
        let program_size = program.len() as Word;
        let mem_vec_size = memory_size / std::mem::size_of::<Word>();
        VM {
            registers: default_reg_values(program_size),
            memory: Self::init_memory(program, mem_vec_size),
            running: false
        }
    }

    pub fn new(program: Vec<Word>) -> Self {
        Self::new_with_memory_size(program, Self::DEFAULT_MEMORY_SIZE_BYTES)
    }

    fn read_next_inst(&self) -> Word {
        let current_cs = self.registers[Reg::CodeSegment as usize] as usize;
        let current_ip = self.registers[Reg::InstPointer as usize] as usize;
        let position = current_cs + current_ip;
        self.memory[position]
    }

    fn fetch_next_instr(&mut self) -> Word {
        let instruction = self.read_next_inst();
        self.registers[Reg::InstPointer as usize] += 1;
        instruction
    }

    fn step(&mut self) -> bool {
        let instruction = self.fetch_next_instr();

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
            println!("reg: {:?}", self.registers);
            println!("mem: {:?}", self.memory);
            self.running = self.step();
        }
    }

    fn set_flag_on(&mut self, flag: Flag) {
        self.registers[Reg::Flags as usize] |= flag as Word;
    }

    fn set_flag_off(&mut self, flag: Flag) {
        self.registers[Reg::Flags as usize] &= !(flag as Word);
    }

    fn is_flag_on(&self, flag: Flag) -> bool {
        self.registers[Reg::Flags as usize] & (flag as Word) != 0
    }

    fn perform_load(&mut self, value: Word, dest_reg: u8) -> bool {
        if Self::is_reg_writable(dest_reg as usize) {
            self.registers[dest_reg as usize] = value as Word;
        } else {
            todo!("attempt to write to invalid register");
        }
        true
    }

    fn perform_copy(&mut self, src: u8, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            let value = self.registers[src as usize];
            self.registers[dest as usize] = value as Word;
        } else {
            todo!("attempt to write to invalid register");
        }
        true
    }

    fn perform_add(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            let v1 = self.registers[src1 as usize];
            let v2 = self.registers[src2 as usize];
            self.registers[dest as usize] = v1 + v2;
        } else {
            todo!("attempt to write to invalid register");
        }
        true
    }

    fn perform_sub(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            let v1 = self.registers[src1 as usize];
            let v2 = self.registers[src2 as usize];
            self.registers[dest as usize] = v1 - v2;
        } else {
            todo!("attempt to write to invalid register");
        }
        true
    }

    fn perform_mult(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            let v1 = self.registers[src1 as usize];
            let v2 = self.registers[src2 as usize];
            self.registers[dest as usize] = v1 * v2;
        } else {
            todo!("attempt to write to invalid register");
        }
        true
    }

    fn perform_div(&mut self, src1: u8, src2: u8, quot_dest: u8, rem_dest: u8) -> bool {
        if Self::is_reg_writable(quot_dest as usize) && Self::is_reg_writable(rem_dest as usize) {
            let v1 = self.registers[src1 as usize];
            let v2 = self.registers[src2 as usize];
            if v2 == 0 {
                todo!("division by zero");
            }
            self.registers[quot_dest as usize] = v1 / v2;
            self.registers[rem_dest as usize] = v1 % v2;
        } else {
            todo!("attempt to write to invalid register");
        }
        true
    }

    fn perform_cmp(&mut self, src1: u8, src2: u8) ->  bool {
        let v1 =  self.registers[src1 as usize];
        let v2 =  self.registers[src2 as usize];
        
        if v1 == v2 {
            self.set_flag_on(Flag::Zero);
            self.set_flag_off(Flag::Carry);
        } else {
            self.set_flag_off(Flag::Zero);
        }

        if v1 < v2 {
            self.set_flag_on(Flag::Carry);
        }

        true
    }

    fn perform_jmp(&mut self, src: u8) -> bool {
        let v = self.registers[src as usize];
        self.registers[Reg::InstPointer as usize] = v;
        true
    }

    fn perform_jz(&mut self, src: u8) -> bool {
        if self.is_flag_on(Flag::Zero) {
            let v = self.registers[src as usize];
            self.registers[Reg::InstPointer as usize] = v;
        }
        true
    }

    fn perform_jnz(&mut self, src: u8) -> bool {
        if !self.is_flag_on(Flag::Zero) {
            let v = self.registers[src as usize];
            self.registers[Reg::InstPointer as usize] = v;
        }
        true
    }

    fn perform_jgt(&mut self, src: u8) -> bool {
        if !self.is_flag_on(Flag::Carry) {
            let v = self.registers[src as usize];
            self.registers[Reg::InstPointer as usize] = v;
        }
        true
    }

    fn perform_jlt(&mut self, src: u8) -> bool {
        if self.is_flag_on(Flag::Carry) {
            let v = self.registers[src as usize];
            self.registers[Reg::InstPointer as usize] = v;
        }
        true
    }

    fn perform_inc(&mut self, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            self.registers[dest as usize] += 1;
        }
        true
    }

    fn perform_dec(&mut self, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            self.registers[dest as usize] -= 1;
        }
        true
    }

    fn perform_load_mem(&mut self, src_addr: Word, dest_reg: u8) -> bool {
        if Self::is_reg_writable(dest_reg as usize) {
            let ds = self.registers[Reg::DataSegment as usize] as usize;
            self.registers[dest_reg as usize] = self.memory[ds + src_addr as usize];
        }
        true
    }

    fn perform_store_mem(&mut self, src_reg: u8, dest_addr: Word) -> bool {
        let ds = self.registers[Reg::DataSegment as usize] as usize;
        self.memory[ds + dest_addr as usize] = self.registers[src_reg as usize];
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_new_vm_has_default_values() {
        let program = vec![0; 0];
        let program_len = program.len();
        let vm = VM::new(program);
        assert_eq!(vm.registers, default_reg_values(program_len as Word));
        assert_eq!(vm.running, false);
    }

    #[test]
    fn fetching_next_instruction_consumes_previous_ones() {
        let program = vec![7, 8, 9];
        let mut vm = VM::new(program);

        let instruction = vm.fetch_next_instr();
        let expected = 7;
        assert_eq!(expected, instruction);

        let instruction = vm.fetch_next_instr();
        let expected = 8;
        assert_eq!(expected, instruction);

        let instruction = vm.fetch_next_instr();
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
        
        vm.step();
        assert_eq!(expected_d0, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);
        assert_eq!(0, vm.registers[Reg::Data2 as usize]);
        assert_eq!(0, vm.registers[Reg::Data3 as usize]);
        assert_eq!(1, vm.registers[Reg::InstPointer as usize]);
        
        vm.step();
        assert_eq!(expected_d0, vm.registers[Reg::Data0 as usize]);
        assert_eq!(expected_d1, vm.registers[Reg::Data1 as usize]);
        assert_eq!(0, vm.registers[Reg::Data2 as usize]);
        assert_eq!(0, vm.registers[Reg::Data3 as usize]);
        assert_eq!(2, vm.registers[Reg::InstPointer as usize]);
        
        vm.step();
        assert_eq!(expected_d0, vm.registers[Reg::Data0 as usize]);
        assert_eq!(expected_d1, vm.registers[Reg::Data1 as usize]);
        assert_eq!(expected_d2, vm.registers[Reg::Data2 as usize]);
        assert_eq!(0, vm.registers[Reg::Data3 as usize]);
        assert_eq!(3, vm.registers[Reg::InstPointer as usize]);
        
        vm.step();
        assert_eq!(expected_d0, vm.registers[Reg::Data0 as usize]);
        assert_eq!(expected_d1, vm.registers[Reg::Data1 as usize]);
        assert_eq!(expected_d2, vm.registers[Reg::Data2 as usize]);
        assert_eq!(expected_d3, vm.registers[Reg::Data3 as usize]);
        assert_eq!(4, vm.registers[Reg::InstPointer as usize]);
    }

    #[test]
    fn copy_should_make_src_value_equal_to_dest_value() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000000000010001_0000000001i64,   // load $17, d0
            0b000000000000000000000000001_000000000000000000000000000_0000001100i64,   // copy d0, d1
        ];
        let mut vm = VM::new(program);

        assert_eq!(0, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);

        vm.step();  // load $17, d0
        assert_eq!(17, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);

        vm.step();  // copy d0, d1
        assert_eq!(17, vm.registers[Reg::Data0 as usize]);
        assert_eq!(17, vm.registers[Reg::Data1 as usize]);
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

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);
        assert_eq!(0, vm.registers[Reg::Data3 as usize]);
        
        vm.step();
        assert_eq!(0b11111010000, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0b101110111000, vm.registers[Reg::Data1 as usize]);
        assert_eq!(0, vm.registers[Reg::Data3 as usize]);

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0b101110111000, vm.registers[Reg::Data1 as usize]);
        assert_eq!(expected_result, vm.registers[Reg::Data3 as usize]);
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

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);
        assert_eq!(0, vm.registers[Reg::Data3 as usize]);
        
        vm.step();
        assert_eq!(0b11111010000, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0b101110111000, vm.registers[Reg::Data1 as usize]);
        assert_eq!(0, vm.registers[Reg::Data3 as usize]);

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0b101110111000, vm.registers[Reg::Data1 as usize]);
        assert_eq!(expected_result, vm.registers[Reg::Data3 as usize]);
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

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);
        assert_eq!(0, vm.registers[Reg::Data3 as usize]);
        
        vm.step();
        assert_eq!(0b11111010000, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0b101110111000, vm.registers[Reg::Data1 as usize]);
        assert_eq!(0, vm.registers[Reg::Data3 as usize]);

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0b101110111000, vm.registers[Reg::Data1 as usize]);
        assert_eq!(expected_result, vm.registers[Reg::Data3 as usize]);
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

        vm.step();  // load $4321, d0
        vm.step();  // load $1234, d1
        vm.step();  // div d0 d1 d2 d3

        assert_eq!(expected_quotient, vm.registers[Reg::Data2 as usize]);
        assert_eq!(expected_remainder, vm.registers[Reg::Data3 as usize]);
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

        vm.step();  // load $2000, d0
        vm.step();  // load $3000, d1
        vm.step();  // load $2000, d2

        vm.step();  // cmp d0, d1
        assert!(!vm.is_flag_on(Flag::Zero));

        vm.step();  // cmp d0, d2
        assert!(vm.is_flag_on(Flag::Zero));

        vm.step();  // cmp d1, d0
        assert!(!vm.is_flag_on(Flag::Zero));
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

        assert_eq!(0, vm.registers[Reg::InstPointer as usize]);
        assert_eq!(0, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);
        
        vm.step();  // load $4, d0

        assert_eq!(1, vm.registers[Reg::InstPointer as usize]);
        assert_eq!(4, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);
        
        vm.step();  // load $3, d0

        assert_eq!(2, vm.registers[Reg::InstPointer as usize]);
        assert_eq!(3, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);
        
        vm.step();  // load $2, d0

        assert_eq!(3, vm.registers[Reg::InstPointer as usize]);
        assert_eq!(2, vm.registers[Reg::Data0 as usize]);
        assert_eq!(0, vm.registers[Reg::Data1 as usize]);

        vm.step();  // load $1, d1
        
        assert_eq!(4, vm.registers[Reg::InstPointer as usize]);
        assert_eq!(2, vm.registers[Reg::Data0 as usize]);
        assert_eq!(1, vm.registers[Reg::Data1 as usize]);

        vm.step();  // jmp d1

        assert_eq!(1, vm.registers[Reg::InstPointer as usize]);
        assert_eq!(2, vm.registers[Reg::Data0 as usize]);
        assert_eq!(1, vm.registers[Reg::Data1 as usize]);

        vm.step();  // load $3, d0

        assert_eq!(2, vm.registers[Reg::InstPointer as usize]);
        assert_eq!(3, vm.registers[Reg::Data0 as usize]);
        assert_eq!(1, vm.registers[Reg::Data1 as usize]);
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

        assert_eq!(1, vm.registers[Reg::Data0 as usize]);
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

        assert_eq!(expected_value, vm.registers[Reg::Data0 as usize]);
    }

    #[test]
    fn dec_should_decrement_a_reg_by_one() {
        let expected_value = 448 - 5 + 5 -10 + 15 -5 + 1248 -8 -1100 -100 - 40;

        let program = vec![
            0b00000000_0000000000000000000000000000000000000111000001_0000000001i64,    // load $449, d0
            0b000000000000000000000000000000000000000000000000000000_0000001110i64,     // dec d0
            0b0000000000000000000000000000000000000000000000000000000000000000i64,      // halt
        ];
        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(expected_value, vm.registers[Reg::Data0 as usize]);
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

        let ds = vm.registers[Reg::DataSegment as usize] as usize;
        assert_eq!(449, vm.memory[ds]);
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

        assert_eq!(449, vm.registers[Reg::Data1 as usize]);
    }
}
