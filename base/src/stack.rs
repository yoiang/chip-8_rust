pub struct Stack {
    contents: Vec<usize>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            contents: vec![]
        } 
    }
}

impl chip8_traits::Stack for Stack {
    // TODO: limit to 12-bit (or memory length) values
    // TODO: optionally limit size
    fn push(&mut self, value: usize) {
        self.contents.push(value);
    }

    // TODO: stack underflow
    fn pop(&mut self) -> Option<usize> {
        self.contents.pop()
    }

    fn clear(&mut self) {
        self.push(0);
        self.pop();
        self.contents.clear();
    }
}