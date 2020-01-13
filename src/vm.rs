pub type Word = u64;

use crate::instruction::Instruction;

pub struct Clockwork {
    registers: [i64; Self::NUM_REGS],
    //memory: [u64; MEMORY_SIZE],
    program: Vec<Word>,
    running: bool
}

impl Clockwork {
    const NUM_REGS: usize = 7;
    //const MEMORY_SIZE: usize = 1024;

    const REG_D0: usize = 0;
    const REG_D1: usize = 1;
    const REG_D2: usize = 2;
    const REG_D3: usize = 3;
    const REG_IP: usize = 4;
    const REG_F0: usize = 5;
    const REG_F1: usize = 6;

    const INITIAL_IP: usize = 0;

    pub fn new(program: Vec<Word>) -> Self {
        Clockwork {
            registers: [0, 0, 0, 0, Self::INITIAL_IP as i64, 0, 0],
            //memory: [0; MEMORY_SIZE],
            program,
            running: false
        }
    }

    fn fetch_next_instr(&mut self) -> u64 {
        let instruction = self.program[self.registers[Self::REG_IP] as usize];
        self.registers[Self::REG_IP] += 1;
        instruction
    }

    pub fn step(&mut self) -> bool {
        let instruction = &self.fetch_next_instr();

        match Instruction::from(*instruction) {
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
            Instruction::Jeq { src }                              => self.perform_jeq(src),
            Instruction::Jneq { src }                             => self.perform_jneq(src),
            Instruction::Jgt { src }                              => self.perform_jgt(src),
            Instruction::Jlt { src }                              => self.perform_jlt(src),
        }
    }

    fn is_reg_writable(index: usize) -> bool {
        index != Self::REG_F0 && index != Self::REG_F1
    }

    fn perform_load(&mut self, value: u64, dest_reg: u8) -> bool {
        if Self::is_reg_writable(dest_reg as usize) {
            self.registers[dest_reg as usize] = value as i64;
            self.registers[Self::REG_F0] = 0;
        } else {
            self.registers[Self::REG_F0] = 1;
        }
        true
    }

    fn perform_copy(&mut self, src: u8, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            let value = self.registers[src as usize];
            self.registers[dest as usize] = value as i64;
            self.registers[Self::REG_F0] = 0;
        } else {
            self.registers[Self::REG_F0] = 1;
        }
        true
    }

    fn perform_add(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            let v1 = self.registers[src1 as usize];
            let v2 = self.registers[src2 as usize];
            self.registers[dest as usize] = v1 + v2;
            self.registers[Self::REG_F0] = 0;
        } else {
            self.registers[Self::REG_F0] = 1;
        }
        true
    }

    fn perform_sub(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            let v1 = self.registers[src1 as usize];
            let v2 = self.registers[src2 as usize];
            self.registers[dest as usize] = v1 - v2;
            self.registers[Self::REG_F0] = 0;
        } else {
            self.registers[Self::REG_F0] = 1;
        }
        true
    }

    fn perform_mult(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
        if Self::is_reg_writable(dest as usize) {
            let v1 = self.registers[src1 as usize];
            let v2 = self.registers[src2 as usize];
            self.registers[dest as usize] = v1 * v2;
            self.registers[Self::REG_F0] = 0;
        } else {
            self.registers[Self::REG_F0] = 1;
        }
        true
    }

    fn perform_div(&mut self, src1: u8, src2: u8, quot_dest: u8, rem_dest: u8) -> bool {
        if Self::is_reg_writable(quot_dest as usize) && Self::is_reg_writable(rem_dest as usize) {
            let v1 = self.registers[src1 as usize];
            let v2 = self.registers[src2 as usize];
            self.registers[quot_dest as usize] = v1 / v2;
            self.registers[rem_dest as usize] = v1 % v2;
            self.registers[Self::REG_F0] = 0;
        } else {
            self.registers[Self::REG_F0] = 1;
        }
        true
    }

    fn perform_cmp(&mut self, src1: u8, src2: u8) ->  bool {
        let v1 =  self.registers[src1 as usize];
        let v2 =  self.registers[src2 as usize];
        self.registers[Self::REG_F1] = v1 - v2;
        true
    }

    fn perform_jmp(&mut self, src: u8) -> bool {
        let v = self.registers[src as usize];
        self.registers[Self::REG_IP] = v;
        true
    }

    fn perform_jeq(&mut self, src: u8) -> bool {
        let cmp = self.registers[Self::REG_F1];
        if cmp == 0 {
            let v = self.registers[src as usize];
            self.registers[Self::REG_IP] = v;
        }
        true
    }

    fn perform_jneq(&mut self, src: u8) -> bool {
        let cmp = self.registers[Self::REG_F1];
        if cmp != 0 {
            let v = self.registers[src as usize];
            self.registers[Self::REG_IP] = v;
        }
        true
    }

    fn perform_jgt(&mut self, src: u8) -> bool {
        let cmp = self.registers[Self::REG_F1];
        if cmp > 0 {
            let v = self.registers[src as usize];
            self.registers[Self::REG_IP] = v;
        }
        true
    }

    fn perform_jlt(&mut self, src: u8) -> bool {
        let cmp = self.registers[Self::REG_F1];
        if cmp < 0 {
            let v = self.registers[src as usize];
            self.registers[Self::REG_IP] = v;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_new_vm_has_default_values() {
        let vm = Clockwork::new(vec![0; 0]);
        assert_eq!(vm.registers, [
            0i64,
            0i64,
            0i64,
            0i64,
            Clockwork::INITIAL_IP as i64,
            0i64,
            0i64,
        ]);
        assert_eq!(vm.running, false);
    }

    #[test]
    fn fetching_next_instruction_consumes_previous_ones() {
        let program = vec![7, 8, 9];
        let mut vm = Clockwork::new(program);

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
            0b00000000_0000000000000000000000000000000000000000001101_0000000001u64, // load $13, d0
            0b00000001_0000000000000000000000000000000000000001100100_0000000001u64, // load $100, d1
            0b00000010_0000000000000000000000000000000000000001100001_0000000001u64, // load $99, d2
            0b00000011_0000000000000000000000000000000011001010010100_0000000001u64, // load $12948, d3
        ];

        let mut vm = Clockwork::new(program);
        
        vm.step();
        assert_eq!(expected_d0, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);
        assert_eq!(0, vm.registers[Clockwork::REG_D2]);
        assert_eq!(0, vm.registers[Clockwork::REG_D3]);
        assert_eq!(1, vm.registers[Clockwork::REG_IP]);
        assert_eq!(0, vm.registers[Clockwork::REG_F0]);
        
        vm.step();
        assert_eq!(expected_d0, vm.registers[Clockwork::REG_D0]);
        assert_eq!(expected_d1, vm.registers[Clockwork::REG_D1]);
        assert_eq!(0, vm.registers[Clockwork::REG_D2]);
        assert_eq!(0, vm.registers[Clockwork::REG_D3]);
        assert_eq!(2, vm.registers[Clockwork::REG_IP]);
        assert_eq!(0, vm.registers[Clockwork::REG_F0]);
        
        vm.step();
        assert_eq!(expected_d0, vm.registers[Clockwork::REG_D0]);
        assert_eq!(expected_d1, vm.registers[Clockwork::REG_D1]);
        assert_eq!(expected_d2, vm.registers[Clockwork::REG_D2]);
        assert_eq!(0, vm.registers[Clockwork::REG_D3]);
        assert_eq!(3, vm.registers[Clockwork::REG_IP]);
        assert_eq!(0, vm.registers[Clockwork::REG_F0]);
        
        vm.step();
        assert_eq!(expected_d0, vm.registers[Clockwork::REG_D0]);
        assert_eq!(expected_d1, vm.registers[Clockwork::REG_D1]);
        assert_eq!(expected_d2, vm.registers[Clockwork::REG_D2]);
        assert_eq!(expected_d3, vm.registers[Clockwork::REG_D3]);
        assert_eq!(4, vm.registers[Clockwork::REG_IP]);
        assert_eq!(0, vm.registers[Clockwork::REG_F0]);
    }

    #[test]
    fn copy_should_make_src_value_equal_to_dest_value() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000000000010001_0000000001u64,   // load $17, d0
            0b000000000000000000000000001_000000000000000000000000000_0000001100u64,   // copy d0, d1
        ];
        let mut vm = Clockwork::new(program);

        assert_eq!(0, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);

        vm.step();  // load $17, d0
        assert_eq!(17, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);

        vm.step();  // copy d0, d1
        assert_eq!(17, vm.registers[Clockwork::REG_D0]);
        assert_eq!(17, vm.registers[Clockwork::REG_D1]);
    }

    #[test]
    fn shouldnt_write_to_read_only_regs() {
        let expected_f0 = 1;

        /*
         * Tries to load the number 5 to register f0
         */
        let program = vec![
            0b00000101_0000000000000000000000000000000000000000001101_0000000001u64
        ];
        let mut vm = Clockwork::new(program);

        vm.step();
        assert_eq!(expected_f0, vm.registers[Clockwork::REG_F0]);
    }

    #[test]
    fn addition_should_preserve_operand_regs_and_update_dest_reg() {
        let expected_result = 5000;

        /*
         * Loads 2000 and 3000 to d0 and d1 respectively, then performs addition with destination d3
         */
        let program = vec![
            0b00000000_0000000000000000000000000000000000011111010000_0000000001u64,
            0b00000001_0000000000000000000000000000000000101110111000_0000000001u64,
            0b000000000000000011_000000000000000001_000000000000000000_0000000010u64
        ];
        let mut vm = Clockwork::new(program);

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);
        assert_eq!(0, vm.registers[Clockwork::REG_D3]);
        
        vm.step();
        assert_eq!(0b11111010000, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0b101110111000, vm.registers[Clockwork::REG_D1]);
        assert_eq!(0, vm.registers[Clockwork::REG_D3]);

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0b101110111000, vm.registers[Clockwork::REG_D1]);
        assert_eq!(expected_result, vm.registers[Clockwork::REG_D3]);
    }

    #[test]
    fn subtraction_should_preserve_operand_regs_and_update_dest_reg() {
        let expected_result = -1000;

        /*
         * Loads 2000 and 3000 to d0 and d1 respectively, then performs subtraction with destination d3
         */
        let program = vec![
            0b00000000_0000000000000000000000000000000000011111010000_0000000001u64,
            0b00000001_0000000000000000000000000000000000101110111000_0000000001u64,
            0b000000000000000011_000000000000000001_000000000000000000_0000000011u64
        ];
        let mut vm = Clockwork::new(program);

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);
        assert_eq!(0, vm.registers[Clockwork::REG_D3]);
        
        vm.step();
        assert_eq!(0b11111010000, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0b101110111000, vm.registers[Clockwork::REG_D1]);
        assert_eq!(0, vm.registers[Clockwork::REG_D3]);

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0b101110111000, vm.registers[Clockwork::REG_D1]);
        assert_eq!(expected_result, vm.registers[Clockwork::REG_D3]);
    }

    #[test]
    fn multiplication_should_preserve_operand_regs_and_update_dest_reg() {
        let expected_result = 6_000_000;

        /*
         * Loads 2000 and 3000 to d0 and d1 respectively, then performs multiplication with destination d3
         */
        let program = vec![
            0b00000000_0000000000000000000000000000000000011111010000_0000000001u64,
            0b00000001_0000000000000000000000000000000000101110111000_0000000001u64,
            0b000000000000000011_000000000000000001_000000000000000000_0000000100u64
        ];
        let mut vm = Clockwork::new(program);

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);
        assert_eq!(0, vm.registers[Clockwork::REG_D3]);
        
        vm.step();
        assert_eq!(0b11111010000, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0b101110111000, vm.registers[Clockwork::REG_D1]);
        assert_eq!(0, vm.registers[Clockwork::REG_D3]);

        vm.step();
        assert_eq!(0b11111010000, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0b101110111000, vm.registers[Clockwork::REG_D1]);
        assert_eq!(expected_result, vm.registers[Clockwork::REG_D3]);
    }

    #[test]
    fn division_should_affect_both_quot_dest_and_rem_dest() {
        let expected_quotient = 3;
        let expected_remainder = 619;

        let program = vec![
            0b00000000_0000000000000000000000000000000001000011100001_0000000001u64,    // load $4321, d0
            0b00000001_0000000000000000000000000000000000010011010010_0000000001u64,    // load $1234, d1
            0b000000000000011_0000000000010_0000000000001_0000000000000_0000001011u64,  // div d0 d1 d2 d3
        ];
        let mut vm = Clockwork::new(program);

        vm.step();  // load $4321, d0
        vm.step();  // load $1234, d1
        vm.step();  // div d0 d1 d2 d3

        assert_eq!(expected_quotient, vm.registers[Clockwork::REG_D2]);
        assert_eq!(expected_remainder, vm.registers[Clockwork::REG_D3]);
    }

    #[test]
    fn cmp_should_affect_f1_reg() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000011111010000_0000000001u64,    // load $2000, d0
            0b00000001_0000000000000000000000000000000000101110111000_0000000001u64,    // load $3000, d1
            0b00000010_0000000000000000000000000000000000011111010000_0000000001u64,    // load $2000, d2
            0b000000000000000000000000001_000000000000000000000000000_0000000101u64,    // cmp d0, d1
            0b000000000000000000000000010_000000000000000000000000000_0000000101u64,    // cmp d0, d2
            0b000000000000000000000000000_000000000000000000000000001_0000000101u64,    // cmp d1, d0
        ];
        let mut vm = Clockwork::new(program);

        vm.step();  // load $2000, d0
        vm.step();  // load $3000, d1
        vm.step();  // load $2000, d2

        vm.step();  // cmp d0, d1
        assert!(vm.registers[Clockwork::REG_F1] < 0);

        vm.step();  // cmp d0, d2
        assert_eq!(0, vm.registers[Clockwork::REG_F1]);

        vm.step();  // cmp d1, d0
        assert!(vm.registers[Clockwork::REG_F1] > 0);
    }

    #[test]
    fn jmp_should_affect_ip_reg() {
        let program = vec![
            0b00000000_0000000000000000000000000000000000000000000100_0000000001u64,    // load $4, d0
            0b00000000_0000000000000000000000000000000000000000000011_0000000001u64,    // load $3, d0
            0b00000000_0000000000000000000000000000000000000000000010_0000000001u64,    // load $2, d0
            0b00000001_0000000000000000000000000000000000000000000001_0000000001u64,    // load $1, d1
            0b000000000000000000000000000000000000000000000000000001_0000000110u64,     // jmp d1
        ];
        let mut vm = Clockwork::new(program);

        assert_eq!(0, vm.registers[Clockwork::REG_IP]);
        assert_eq!(0, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);
        
        vm.step();  // load $4, d0

        assert_eq!(1, vm.registers[Clockwork::REG_IP]);
        assert_eq!(4, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);
        
        vm.step();  // load $3, d0

        assert_eq!(2, vm.registers[Clockwork::REG_IP]);
        assert_eq!(3, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);
        
        vm.step();  // load $2, d0

        assert_eq!(3, vm.registers[Clockwork::REG_IP]);
        assert_eq!(2, vm.registers[Clockwork::REG_D0]);
        assert_eq!(0, vm.registers[Clockwork::REG_D1]);

        vm.step();  // load $1, d1
        
        assert_eq!(4, vm.registers[Clockwork::REG_IP]);
        assert_eq!(2, vm.registers[Clockwork::REG_D0]);
        assert_eq!(1, vm.registers[Clockwork::REG_D1]);

        vm.step();  // jmp d1

        assert_eq!(1, vm.registers[Clockwork::REG_IP]);
        assert_eq!(2, vm.registers[Clockwork::REG_D0]);
        assert_eq!(1, vm.registers[Clockwork::REG_D1]);

        vm.step();  // load $3, d0

        assert_eq!(2, vm.registers[Clockwork::REG_IP]);
        assert_eq!(3, vm.registers[Clockwork::REG_D0]);
        assert_eq!(1, vm.registers[Clockwork::REG_D1]);
    }
}