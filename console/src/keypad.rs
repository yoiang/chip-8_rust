pub struct Keypad {
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
        }
    }
}

// TODO:
/*
1	2	3	C
4	5	6	D
7	8	9	E
A	0	B	F

1	2	3	4
Q	W	E	R
A	S	D	F
Z	X	C	V
*/

impl chip8_traits::Keypad for Keypad {
    fn state(&mut self) -> (u8, u8) {
        (0, 0)

        /*
                // TODO: move into keypad
        // TODO: option to register on press and release onlys
        if let Some(Ok(key)) = input {
            match key {
        // for event in self.stdin.as_mut().events() {
            // match event.unwrap() {
                termion::event::Key::Char('1') => {
                    self.variable_registers[x as usize] = 0x01;
                },
                termion::event::Key::Char('2') => {
                    self.variable_registers[x as usize] = 0x02;
                },
                termion::event::Key::Char('3') => {
                    self.variable_registers[x as usize] = 0x3;
                },
                termion::event::Key::Char('4') => {
                    self.variable_registers[x as usize] = 0x0c;
                },

                termion::event::Key::Char('q') => {
                    self.variable_registers[x as usize] = 0x04;
                },
                termion::event::Key::Char('w') => {
                    self.variable_registers[x as usize] = 0x05;
                },
                termion::event::Key::Char('e') => {
                    self.variable_registers[x as usize] = 0x06;
                },
                termion::event::Key::Char('r') => {
                    self.variable_registers[x as usize] = 0x0d;
                },

                termion::event::Key::Char('a') => {
                    self.variable_registers[x as usize] = 0x07;
                },
                termion::event::Key::Char('s') => {
                    self.variable_registers[x as usize] = 0x08;
                },
                termion::event::Key::Char('d') => {
                    self.variable_registers[x as usize] = 0x09;
                },
                termion::event::Key::Char('f') => {
                    self.variable_registers[x as usize] = 0x0e;
                },

                termion::event::Key::Char('z') => {
                    self.variable_registers[x as usize] = 0x0a;
                },
                termion::event::Key::Char('x') => {
                    self.variable_registers[x as usize] = 0x00;
                },
                termion::event::Key::Char('c') => {
                    self.variable_registers[x as usize] = 0x0b;
                },
                termion::event::Key::Char('v') => {
                    self.variable_registers[x as usize] = 0x0f;
                },

                _ => {
                    self.program_counter.go_back();
                },
            }
        }
        */
    }
}