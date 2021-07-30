pub struct ProgramCounter {
    position: usize
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            position: 0
        }
    }
}

impl chip8_traits::ProgramCounter for ProgramCounter  {
    // fn read<Instruction: chip8_traits::Instruction, Memory: chip8_traits::Memory>(&mut self, memory: &Memory) -> Box<Instruction> {
    //     let first = chip8_traits::Memory::get(memory, self.position);
    //     let second = chip8_traits::Memory::get(memory, self.position + 1);
    //     self.position += 2;

    //     Box::new(super::Instruction::new(first, second) as chip8_traits::Instruction)
    // }
    
    fn get_position(&self) -> usize {
        return self.position;
    }

    fn set_position(&mut self, new_position: usize) {
        self.position = new_position;
    }

    fn skip(&mut self) {
        self.position += 2;
    }

    fn go_back(&mut self) {
        self.position -= 2;
    }
}