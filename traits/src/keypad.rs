pub trait Keypad {
    fn state(&mut self) -> (u8, u8);
}