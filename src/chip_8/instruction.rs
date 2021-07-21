use std::convert::TryInto;

pub struct Instruction {
    first: u8,
    second: u8
}

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

    pub fn w(&self) -> [bool; 4] {
        Instruction::get_bits(self.first)[0..=3].try_into().expect("Slice with incorrect length")
    }

    pub fn x(&self) -> [bool; 4] {
        Instruction::get_bits(self.first)[4..=7].try_into().expect("Slice with incorrect length")        
    }

    pub fn y(&self) -> [bool; 4] {
        Instruction::get_bits(self.second)[0..=3].try_into().expect("Slice with incorrect length")
        
    }

    pub fn n(&self) -> [bool; 4] {
        Instruction::get_bits(self.second)[4..=7].try_into().expect("Slice with incorrect length")        
        
    }

    pub fn nn(&self) -> [bool; 8] {
        Instruction::get_bits(self.second)
    }

    pub fn nnn(&self) -> [bool; 12] {
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

/*
X: The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
Y: The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
N: The fourth nibble. A 4-bit number.
NN: The second byte (third and fourth nibbles). An 8-bit immediate number.
NNN: The second, third and fourth nibbles. A 12-bit immediate memory address.*/