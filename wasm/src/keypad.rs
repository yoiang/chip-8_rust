use std::{cell::RefCell, rc::Rc};

pub struct Keypad {
    key_pressed: Rc<RefCell<[bool; 16]>>
}

impl Keypad {
    pub fn new(key_pressed: Rc<RefCell<[bool; 16]>>) -> Keypad {
        Keypad {
            key_pressed
        }
    }
}

impl chip8_traits::Keypad for Keypad {
    fn state(&self) -> [bool; 16] {
        return (self.key_pressed.borrow()).clone();
    }

    fn key_state(&self, key_index: usize) -> bool {
        let key_pressed = *(self.key_pressed.borrow());
        if key_index < key_pressed.len() {
            return key_pressed[key_index];
        }
        
        return false;
    }
}