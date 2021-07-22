pub struct Random {
}

impl Random {
    pub fn new() -> Random {
        Random {
        }
    }
}

impl chip8_traits::Random for Random {
    fn value(&mut self) -> u8 {
        crate::extern_javascript::random().to_be_bytes()[3]
    }
}