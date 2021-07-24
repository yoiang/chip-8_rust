pub struct DelayTimer {
    value: u8
}

impl DelayTimer {
    pub fn new() -> DelayTimer {
        DelayTimer {
            value: 0
        }
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}

impl chip8_traits::Timer for DelayTimer {
    fn get(&self) -> u8 {
        self.value
    }

    fn set(&mut self, value: u8) {
        self.value = value;
    }

    fn update(&mut self) -> Result<(), String> {
        if self.value > 0 {
            self.value -= 1;
        }
        Ok(())
    }
}