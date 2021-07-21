pub struct Font {
    contents: Vec<Vec<u8>>
}

impl Font {
    pub fn new() -> Self {
        Font {
            contents: vec![
                vec![0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
                vec![0x20, 0x60, 0x20, 0x20, 0x70], // 1
                vec![0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
                vec![0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
                vec![0x90, 0x90, 0xF0, 0x10, 0x10], // 4
                vec![0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
                vec![0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
                vec![0xF0, 0x10, 0x20, 0x40, 0x40], // 7
                vec![0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
                vec![0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
                vec![0xF0, 0x90, 0xF0, 0x90, 0x90], // A
                vec![0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
                vec![0xF0, 0x80, 0x80, 0x80, 0xF0], // C
                vec![0xE0, 0x90, 0x90, 0x90, 0xE0], // D
                vec![0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
                vec![0xF0, 0x80, 0xF0, 0x80, 0x80]  // F
            ]
        }
    }

    pub fn apply(&self, memory: &mut dyn crate::traits::Memory, start: usize) {
        let mut offset = start;
        for character in self.contents.iter() {
            for value in character.iter() {
                memory.set(offset, *value);
                offset += 1;
            }
        }
    }
}