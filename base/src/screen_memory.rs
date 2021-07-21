use std::slice::{self, Iter};

pub struct ScreenMemory {
    contents: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl ScreenMemory {
    pub fn new(width: usize, height: usize) -> ScreenMemory {
        let mut result = ScreenMemory {
            contents: vec![],
            width: 0,
            height: 0,
        };
        result.set_dimensions(width, height);

        result
    }

    pub fn set_dimensions(&mut self, width: usize, height: usize) {
        self.contents = vec![vec![false; width]; height];
        self.width = width;
        self.height = height;
    }

    pub fn iter(&self) -> slice::Iter<Vec<bool>> {
        return self.contents.iter();
    }
}

impl chip8_traits::ScreenMemory for ScreenMemory {
    fn clear(&mut self) {
        self.set_dimensions(self.width, self.height);
    }

    fn display(&mut self, x: u8, y: u8, memory: Iter<u8>, count: u8) -> bool {
        let x = (x as usize) % self.width; 
        let y = (y as usize) % self.height;

        let mut memory = memory;

        let mut cleared = false;

        for index in 0..count {
            let row: &mut Vec<bool> = &mut self.contents[y + index as usize];

            let memory_value = memory.as_ref()[0];

            for bit in 0..=7 {
                if memory_value & (1 << bit) != 0 {
                    let bit_offset = 7 - bit;
                    row[x + bit_offset] = !row[x + bit_offset];
                    if row[x + bit_offset] == false {
                        cleared = true;
                    }
                }
            }

            memory.next();
        }

        cleared
    }
}