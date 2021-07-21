use std::fmt;

/*
*/

pub trait Instruction: Copy + fmt::Display {
    /// The first nibble
    fn w(&self) -> [bool; 4];
    /// The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF
    fn x(&self) -> [bool; 4];
    /// The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF
    fn y(&self) -> [bool; 4];
    /// The fourth nibble
    fn n(&self) -> [bool; 4];
    // The second byte (third and fourth nibbles). An 8-bit immediate number
    fn nn(&self) -> [bool; 8];
    // The second, third and fourth nibbles. A 12-bit immediate memory address
    fn nnn(&self) -> [bool; 12];
}