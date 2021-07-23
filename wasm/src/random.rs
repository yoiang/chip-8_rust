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
        #[allow(unused_unsafe)] // Currently unsafe not properly recognized by analyzer
        unsafe {
            random().to_be_bytes()[3]
        }
    }
}