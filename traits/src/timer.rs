pub trait Timer {
    fn get(&self) -> u8;
    fn set(&mut self, value: u8);

    fn update(&mut self) -> Result<(), String>;
}

