mod state_machine {
    pub enum State<Ph> {
        FetchInstruction { next: Ph }
    }
}

mod kinematic {
	use std::convert::From;
	
    type Word = u64;

	//const MEMORY_SIZE: usize = 1024;
	const INITIAL_IP: usize = 0;
	const OPCODE_OFFSET: usize = 10;

	#[derive(Debug, PartialEq)]
	enum Instruction {
		Illegal,
		Halt,
		Load { value: Word, dest_reg: u8 },
        Add { src1: u8, src2: u8, dest: u8  }
	}

    impl Instruction {
        fn parse_load(operands: u64) -> Self {
            // load layout: 0b00000000_00000000000000000000000000000000000000000000000(_0000000000)
            //                 DEST                         VALUE                         OPCODE
            let value = (operands & 0b00000000_1111111111111111111111111111111111111111111111) as u64;
            let dest_reg = (operands >> 46) as u8;
            Instruction::Load { value, dest_reg }
        }

        fn parse_add(operands: u64) -> Self {
            // add layout: 0b000000000000000000_000000000000000000_000000000000000000(_0000000000)
            //                     DEST               SRC2               SRC1           OPCODE
            let src1 = operands as u8;
            let src2 = (operands >> 18) as u8;
            let dest = (operands >> 36) as u8;
            Instruction::Add { src1, src2, dest }
        }
    }

	impl From<Word> for Instruction {
		fn from(instruction: Word) -> Self {
            let opcode = (instruction & 0b000000_1111111111) as u16;
            let operands = (instruction >> OPCODE_OFFSET) as u64;
			match opcode {
				0 => Instruction::Halt,
                1 => Self::parse_load(operands),
                2 => Self::parse_add(operands),
				x if x > 1024 => Instruction::Illegal, // we have only 2.pow(10) = 1024 opcode slots
				_ => Instruction::Illegal              // for still unimplemented instructions
			}
		}
    }

	pub struct Kinematic {
        // TODO represent writable registers as array
		da: i64,
		db: i64,
		dc: i64,
		dd: i64,
		ip: usize,
		f0: i64,
		//memory: [u64; MEMORY_SIZE],
		program: Vec<Word>,
		running: bool
	}

	// instruction
	// 2^8
	// MOV $A, $B, $C

	impl Kinematic {
		pub fn new(program: Vec<Word>) -> Self {
			Kinematic {
				da: 0, db: 0, dc: 0, dd: 0,
				ip: INITIAL_IP,
				f0: 0,
				//memory: [0; MEMORY_SIZE],
				program,
				running: false
			}
		}

		fn fetch_next_instr(&mut self) -> u64 {
			let instruction = self.program[self.ip];
			self.ip += 1;
			instruction
		}

		fn step(&mut self) -> bool {
			let instruction = &self.fetch_next_instr();

			match Instruction::from(*instruction) {
				Instruction::Illegal => panic!("Illegal opcode"),
				Instruction::Halt => false,
                Instruction::Load { value, dest_reg } => self.perform_load(value, dest_reg),
                Instruction::Add { src1, src2, dest } => self.perform_add(src1, src2, dest),
            }
        }

		fn perform_load(&mut self, value: u64, dest_reg: u8) -> bool {
            match dest_reg {
                0 => {
                    self.da = value as i64;
                    true
                },
                1 => {
                    self.db = value as i64;
                    true
                },
                2 => {
                    self.dc = value as i64;
                    true
                },
                3 => {
                    self.dd = value as i64;
                    true
                },
                _ => panic!("Invalid register")
            }
		}

        fn perform_add(&mut self, src1: u8, src2: u8, dest: u8) -> bool {
            unimplemented!()
        }
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn default_vm() {
			let vm = Kinematic::new(vec![0; 0]);
			assert_eq!(vm.da, 0);
			assert_eq!(vm.db, 0);
			assert_eq!(vm.dc, 0);
			assert_eq!(vm.dd, 0);
			assert_eq!(vm.ip, INITIAL_IP);
			assert_eq!(vm.f0, 0);
			assert_eq!(vm.running, false);
		}

		#[test]
		fn fetch_next_instr() {
			let program = vec![7, 8, 9];
			let mut vm = Kinematic::new(program);

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

        #[test]
        fn load_affects_registers() {
            let instruction: Word = 0b0000000000000000000000000000000000000000001101_00000000_0000000001;
            let expected_da = 0b1101;
            let expected_db = 0b0110_0100;
            let expected_dc = 0b0110_0001;
            let expected_dd = 0b0011_0010_1001_0100;
            let program = vec![
                0b00000000_0000000000000000000000000000000000000000001101_0000000001u64, // load $13, da
                0b00000001_0000000000000000000000000000000000000001100100_0000000001u64, // load $100, db
                0b00000010_0000000000000000000000000000000000000001100001_0000000001u64, // load $99, dc
                0b00000011_0000000000000000000000000000000011001010010100_0000000001u64, // load $12948, db
            ];
            let mut vm = Kinematic::new(program);
            vm.step();
            assert_eq!(expected_da, vm.da);
            assert_eq!(0, vm.db);
            assert_eq!(0, vm.dc);
            assert_eq!(0, vm.dd);
            assert_eq!(1, vm.ip);
            vm.step();
            assert_eq!(expected_da, vm.da);
            assert_eq!(expected_db, vm.db);
            assert_eq!(0, vm.dc);
            assert_eq!(0, vm.dd);
            assert_eq!(2, vm.ip);
            vm.step();
            assert_eq!(expected_da, vm.da);
            assert_eq!(expected_db, vm.db);
            assert_eq!(expected_dc, vm.dc);
            assert_eq!(0, vm.dd);
            assert_eq!(3, vm.ip);
            vm.step();
            assert_eq!(expected_da, vm.da);
            assert_eq!(expected_db, vm.db);
            assert_eq!(expected_dc, vm.dc);
            assert_eq!(expected_dd, vm.dd);
            assert_eq!(4, vm.ip);
	    }       
    }
}

fn main() {
	println!("works");
}
