pub struct Keypad {
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
        }
    }
}

// TODO:
/*
1	2	3	C
4	5	6	D
7	8	9	E
A	0	B	F

1	2	3	4
Q	W	E	R
A	S	D	F
Z	X	C	V
*/

impl chip8_traits::Keypad for Keypad {
    fn state(&mut self) -> (u8, u8) {
        (0, 0)
    }
}