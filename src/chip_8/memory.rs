use std::{slice::Iter, usize};

pub struct Memory {
    contents: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            contents: vec![0; size]
        }
    }
}

impl crate::traits::Memory for Memory {
    fn set_size(&mut self, size: usize) {
        self.contents = vec![0; size];
    }

    fn set(&mut self, location: usize, value: u8) {
        self.contents[location] = value;
    }

    fn get(&self, location: usize) -> u8 {
        self.contents[location]
    }

    fn dump(&self) -> Vec<u8> {
        self.contents.clone()
    }

    fn get_iter(&self, location: usize) -> Iter<u8> {
        return self.contents.split_at(location).1.iter();
    }
}