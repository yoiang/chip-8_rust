use std::slice::Iter;

pub trait Renderer {
    // TODO: think up way to do without mutable
    fn render(&mut self, memory: Iter<Vec<bool>>) -> Result<(), &'static str> ;
}