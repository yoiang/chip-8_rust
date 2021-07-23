pub trait Keypad {
    fn state(&self) -> [bool; 16];
    fn key_state(&self, key_index: usize) -> bool;
}