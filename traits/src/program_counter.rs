pub trait ProgramCounter<Instruction> where Instruction: crate::Instruction {
    fn read(&mut self, memory: &dyn crate::Memory) -> Box<Instruction>;
    
    fn get_position(&self) -> usize;
    fn set_position(&mut self, new_position: usize);

    fn skip(&mut self);
    fn go_back(&mut self);
}