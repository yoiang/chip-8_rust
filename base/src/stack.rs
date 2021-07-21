pub struct Stack {
    contents: Vec<usize>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            contents: vec![]
        } 
    }

    // TODO: limit to 12-bit (or memory length) values
    // TODO: optionally limit size
    pub fn push(&mut self, value: usize) {
        self.contents.push(value);
    }

    // TODO: stack underflow
    pub fn pop(&mut self) -> Option<usize> {
        self.contents.pop()
    }

    pub fn clear(&mut self) {
        self.push(0);
        self.pop();
        self.contents.clear();
    }
}