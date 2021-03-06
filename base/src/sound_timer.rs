pub struct SoundTimer {
    value: u8
}

impl SoundTimer {
    pub fn new() -> SoundTimer {
        SoundTimer {
            value: 0
        }
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}

impl chip8_traits::Timer for SoundTimer {
    fn get(&self) -> u8 {
        self.value
    }

    fn set(&mut self, value: u8) {
        self.value = value;
    }

    fn update(&mut self) -> Result<(), String> {
        // TODO: only call beep if changed
        if self.value > 0 {
            self.value -= 1;
            print!("\x07\r");
            // let result = beep(440);
            // if let Err(error) = result {
            //     return Err(format!("{}", error));
            // }
        // } else {
        //     let result = beep(0);
        //     if let Err(error) = result {
        //         return Err(format!("{}", error));
        //     }
        }
        Ok(())
    }
}