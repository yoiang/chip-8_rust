use nanorand::Rng;

pub struct Random {
    internal: nanorand::WyRand
}

impl Random {
    pub fn new() -> Random {
        Random {
            internal: nanorand::WyRand::new()
        }
    }
}

impl chip8_traits::Random for Random {
    fn value(&mut self) -> u8 {
        self.internal.generate::<u8>()
    }
}