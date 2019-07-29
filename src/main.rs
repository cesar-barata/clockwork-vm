//#[derive(Debug)]
pub struct Kinematic {
	da: u64,
	db: u64,
	dc: u64,
	dd: u64,
	ip: u64,
	f0: u64,
	memory: [u64; 256]
}

// instruction
// 2^8
// MOV $A, $B, $C

impl Kinematic {
	pub fn new(memory: [u64; 256]) -> Kinematic {
		Kinematic {
			da: 0, db: 0, dc: 0, dd: 0,
			ip: 0,
			f0: 0,
			memory
		}
	}

	fn read_byte(instruction: u64, offset: u8) -> u8 {
		//let eqv_offset = offset % 64;
		(instruction >> (offset * 8)) as u8
	}

	pub fn parse_instruction(instruction: u64) {
		unimplemented!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn read_instruction_from_first_bit() {
		let expected = 6u8;
		let actual = Kinematic::read_byte(0x0D0C0B0A09080706u64, 0);
		assert_eq!(expected, actual);
	}

	#[test]
	fn read_instruction_from_second_bit() {
		let expected = 7u8;
		let actual = Kinematic::read_byte(0x0D0C0B0A09080706u64, 1);
		assert_eq!(expected, actual);
	}

	#[test]
	fn read_instruction_from_eighth_bit() {
		let expected = 13u8;
		let actual = Kinematic::read_byte(0x0D0C0B0A09080706u64, 7);
		assert_eq!(expected, actual);
	}

	#[test]
	fn default_Kinematic() {
		let Kinematic = Kinematic::new([0; 256]);
		assert_eq!(Kinematic.da, 0);
		assert_eq!(Kinematic.db, 0);
		assert_eq!(Kinematic.dc, 0);
		assert_eq!(Kinematic.dd, 0);
		assert_eq!(Kinematic.ip, 0);
		assert_eq!(Kinematic.f0, 0);
	}
}

fn main() {
	println!("works");
}