pub struct Keypad {
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
        }
    }
}

impl chip8_traits::Keypad for Keypad {
    fn state(&self) -> [bool; 16] {
        [false; 16]
    }

    fn key_state(&self, _key_index: usize) -> bool {
        return false;
    }
}