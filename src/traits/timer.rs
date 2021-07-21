pub trait Timer {
    fn value(&self) -> u8;
    fn set_value(&mut self, value: u8);

    fn update(&mut self) -> Result<(), String>;
}

