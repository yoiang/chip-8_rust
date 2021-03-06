use std::{convert::TryInto, fmt};

pub struct Instruction {
    first: u8,
    second: u8
}

// TODO: remove panics
// TODO: double check trading in [bool; x]s vs one or two bytes from a design perspective
impl Instruction {

    pub fn new(first: u8, second: u8) -> Instruction {
        Instruction {
            first,
            second
        }
    }

    fn get_bits(from: u8) -> [bool; 8] {
        let mut result = [false; 8];
        
        result[7] = from & (1 << 0) != 0;
        result[6] = from & (1 << 1) != 0;
        result[5] = from & (1 << 2) != 0;
        result[4] = from & (1 << 3) != 0;
        result[3] = from & (1 << 4) != 0;
        result[2] = from & (1 << 5) != 0;
        result[1] = from & (1 << 6) != 0;
        result[0] = from & (1 << 7) != 0;

        result
    }
}

impl Copy for Instruction {}

impl Clone for Instruction {
    fn clone(&self) -> Instruction {
        Instruction::new(self.first, self.second)
    }
}

impl chip8_traits::Instruction for Instruction {
    fn w(&self) -> [bool; 4] {
        Instruction::get_bits(self.first)[0..=3].try_into().expect("Slice with incorrect length")
    }

    fn x(&self) -> [bool; 4] {
        Instruction::get_bits(self.first)[4..=7].try_into().expect("Slice with incorrect length")        
    }

    fn y(&self) -> [bool; 4] {
        Instruction::get_bits(self.second)[0..=3].try_into().expect("Slice with incorrect length")
        
    }

    fn n(&self) -> [bool; 4] {
        Instruction::get_bits(self.second)[4..=7].try_into().expect("Slice with incorrect length")        
        
    }

    fn nn(&self) -> [bool; 8] {
        Instruction::get_bits(self.second)
    }

    fn nnn(&self) -> [bool; 12] {
        let mut result = [false; 12];

        let mut index = 0;
        for value in self.x().iter() {
            result[index] = *value;
            index += 1;
        }

        for value in self.nn().iter() {
            result[index] = *value;
            index += 1;
        }

        result
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}{:02X}", self.first, self.second)
    }
}

// TODO: add optional context String
#[derive(Debug, Clone)]
pub enum InstructionError<Instruction: chip8_traits::Instruction> {
    UnsupportedInstructionError(Instruction),
    InstructionExecuteError(Instruction),
}

impl<Instruction: chip8_traits::Instruction> fmt::Display for InstructionError<Instruction> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionError::InstructionExecuteError(instruction) => write!(f, "Issue when executing instruction '{}'", instruction),
            InstructionError::UnsupportedInstructionError(instruction) => write!(f, "Unsupported instruction '{}'", instruction)
        }
    }
}
