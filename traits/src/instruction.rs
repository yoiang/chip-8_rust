
/*
W: The first nibble.
X: The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
Y: The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
N: The fourth nibble. A 4-bit number.
NN: The second byte (third and fourth nibbles). An 8-bit immediate number.
NNN: The second, third and fourth nibbles. A 12-bit immediate memory address.
*/

pub trait Instruction: Copy {
    fn w(&self) -> [bool; 4];
    fn x(&self) -> [bool; 4];
    fn y(&self) -> [bool; 4];
    fn n(&self) -> [bool; 4];
    fn nn(&self) -> [bool; 8];
    fn nnn(&self) -> [bool; 12];
}