use std::fmt;

#[derive(Debug, Clone)]
pub struct OutOfBoundsError {
    index: u8
}

impl OutOfBoundsError {
    pub fn new(index: u8) -> OutOfBoundsError {
        OutOfBoundsError {
            index
        }
    }
}

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of Bounds index {}", self.index)
    }
}