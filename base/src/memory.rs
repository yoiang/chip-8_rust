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

    pub fn new_chip8() -> Self {
        Memory::new(4096)
    }

    pub fn clear(&mut self) {
        self.contents = vec![0; self.contents.len()];
    }
}

impl chip8_traits::Memory for Memory {
    fn set_size(&mut self, size: usize) {
        self.contents = vec![0; size];
    }

    fn set(&mut self, location: usize, value: u8) {
        // TODO: Result
        self.contents[location] = value;
    }

    fn get(&self, location: usize) -> u8 {
        // TODO: Result
        self.contents[location]
    }

    fn get_iter(&self, location: usize) -> Iter<u8> {
        return self.contents.split_at(location).1.iter();
    }

    fn dump(&self) -> Vec<u8> {
        self.contents.clone()
    }
}

pub struct PartialSnapshot {
    pub location: usize,
    pub value: u8,
}

impl Memory {
    pub fn snapshot(&self, start_location: usize, end_location: usize) -> Vec<PartialSnapshot> {
        let mut result: Vec<PartialSnapshot> = vec![];

        for location in start_location..=end_location {
            result.push(PartialSnapshot {
                location: location,
                value: chip8_traits::Memory::get(self, location)
            })
        }

        result
    }
}