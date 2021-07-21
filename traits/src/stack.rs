pub trait Stack {
    fn push(&mut self, value: usize);

    // TODO: stack underflow
    fn pop(&mut self) -> Option<usize>;

    fn clear(&mut self);
}