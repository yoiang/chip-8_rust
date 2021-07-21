use super::Instruction;

pub struct ProgramCounter {
    position: usize
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            position: 0
        }
    }

    pub fn read(&mut self, memory: &dyn crate::traits::Memory) -> Instruction {
        let first = memory.get(self.position);
        let second = memory.get(self.position + 1);
        self.position += 2;

        Instruction::new(first, second)
    }

    pub fn set_position(&mut self, new_position: usize) {
        self.position = new_position;
    }
}