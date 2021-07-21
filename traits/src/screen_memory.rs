use std::slice::{Iter};

pub trait ScreenMemory {
    fn clear(&mut self);

    // TODO: more explicit than bool
    fn display(&mut self, x: u8, y: u8, memory: Iter<u8>, count: u8) -> bool;
}