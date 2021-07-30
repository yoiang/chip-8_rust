pub trait ProgramCounter {
    // fn read<Instruction: crate::Instruction, Memory: crate::Memory>(&mut self, memory: &Memory) -> Box<Instruction>;
    
    fn get_position(&self) -> usize;
    fn set_position(&mut self, new_position: usize);

    fn skip(&mut self);
    fn go_back(&mut self);
}