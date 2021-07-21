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

impl chip8_traits::ProgramCounter<super::Instruction> for ProgramCounter  {
    fn read(&mut self, memory: &dyn chip8_traits::Memory) -> Box<super::Instruction> {
        let first = memory.get(self.position);
        let second = memory.get(self.position + 1);
        self.position += 2;

        Box::new(super::Instruction::new(first, second))
    }
    
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