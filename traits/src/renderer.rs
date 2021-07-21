use std::slice::Iter;

pub trait Renderer {
    fn render(&self, memory: Iter<Vec<bool>>) -> Result<(), &'static str> ;
}