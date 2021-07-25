pub trait Timer {
    fn get(&mut self) -> u8;
    fn set(&mut self, value: u8);
}

