mod kinematic {
	use std::convert::From;

	//const MEMORY_SIZE: usize = 1024;
	const INITIAL_IP: usize = 0;

	type Instruction = u64;

	const OPCODE_OFFSET: usize = 60;

	#[derive(Debug, PartialEq)]
	enum Opcode {
		ILG,
		HLT,
		LOAD,
	}

	impl From<Instruction> for Opcode {
		fn from(value: Instruction) -> Opcode {
			//println!("{}", value >> 60);
			match value {
				0 => Opcode::HLT,
				1 => Opcode::LOAD,
				x if x > 1024 => Opcode::ILG,
				_ => Opcode::ILG
			}
		}
	}

	pub struct Kinematic {
		da: i64,
		db: i64,
		dc: i64,
		dd: i64,
		ip: usize,
		f0: i64,
		//memory: [u64; MEMORY_SIZE],
		program: Vec<Instruction>,
		running: bool
	}

	// instruction
	// 2^8
	// MOV $A, $B, $C

	impl Kinematic {
		pub fn new(program: Vec<Instruction>) -> Self {
			Kinematic {
				da: 0, db: 0, dc: 0, dd: 0,
				ip: INITIAL_IP,
				f0: 0,
				//memory: [0; MEMORY_SIZE],
				program,
				running: true
			}
		}

		fn fetch_next_instr(&mut self) -> u64 {
			let instruction = self.program[self.ip];
			self.ip += 1;
			instruction
		}

		fn execute_next_instr(&mut self) {
			let instruction = self.fetch_next_instr();

			match Opcode::from(instruction) {
				Opcode::HLT => panic!("EXIT"),
				Opcode::LOAD => unimplemented!(),
				Opcode::ILG => panic!("Illegal opcode")
			}
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
		fn opcode_from_u64() {
			let expected = Opcode::HLT;
			let actual = Opcode::from(0);
			assert_eq!(expected, actual);

			let expected = Opcode::LOAD;
			let actual = Opcode::from(1);
			assert_eq!(expected, actual);

			let expected = Opcode::ILG;
			let actual = Opcode::from(1023);
			assert_eq!(expected, actual);
		}
	}
}

fn main() {
	println!("works");
}