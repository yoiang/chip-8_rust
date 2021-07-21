use std::slice::Iter;

pub trait Memory {
    fn set_size(&mut self, size: usize);

    fn set(&mut self, location: usize, value: u8);
    fn get(&self, location: usize) -> u8;
    fn get_iter(&self, location: usize) -> Iter<u8>;

    fn dump(&self) -> Vec<u8>;
}