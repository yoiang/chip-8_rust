pub struct DelayTimer {
    value: u8
}

impl DelayTimer {
    pub fn new() -> DelayTimer {
        DelayTimer {
            value: 0
        }
    }
}

impl crate::traits::Timer for DelayTimer {
    fn value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    fn update(&mut self) -> Result<(), String> {
        if self.value > 0 {
            self.value -= 1;
        }
        Ok(())
    }
}