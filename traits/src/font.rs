pub trait Font {
    fn apply(&self, memory: &mut dyn crate::Memory, start: usize);
}