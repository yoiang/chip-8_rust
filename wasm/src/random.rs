use js_sys::Math::random;

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
        unsafe {
            random().to_be_bytes()[3]
        }
    }
}