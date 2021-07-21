use std::{thread::sleep, time::Duration};

pub trait Interpreter {
    fn load(&mut self, file_name: &str, start_position: usize) -> Result<(), std::io::Error>;
    fn update(&mut self) -> Result<(), String>;
    fn dump_memory(&self) -> Vec<u8>;

    fn run(&mut self, frequency: Duration) -> Result<(), String> {
        loop {
            let result = self.update();
            if let Err(error) = result {
                return Err(error);
            }
            sleep(frequency);
        }

        Ok(())
    }
}