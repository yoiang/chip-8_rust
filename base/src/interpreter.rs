use std::{fs, usize};

use crate::{count16, count8};

pub struct Interpreter<Memory, Instruction> where Memory: chip8_traits::Memory, Instruction: chip8_traits::Instruction {
    memory: Box<Memory>,

    screen_memory: super::ScreenMemory,

    renderer: Box<dyn chip8_traits::Renderer>,

    stack: super::Stack,

    delay_timer: Box<dyn chip8_traits::Timer>,
    sound_timer: Box<dyn chip8_traits::Timer>,

    keypad: Box<dyn chip8_traits::Keypad>,

    program_counter: Box<dyn chip8_traits::ProgramCounter<Instruction>>,

    index_register: usize,

    variable_registers: [u8; 16],

    font_start: usize,

    random: Box<dyn chip8_traits::Random>,
}

impl<Memory, Instruction> Interpreter<Memory, Instruction> where Memory:chip8_traits::Memory, Instruction: chip8_traits::Instruction {
    pub fn new(
        memory: Box<Memory>,
        screen_memory: super::ScreenMemory,
        renderer: Box<dyn chip8_traits::Renderer>,
        stack: super::Stack,

        delay_timer: Box<dyn chip8_traits::Timer>,
        sound_timer: Box<dyn chip8_traits::Timer>,

        keypad: Box<dyn chip8_traits::Keypad>,

        program_counter: Box<dyn chip8_traits::ProgramCounter<Instruction>>,

        random: Box<dyn chip8_traits::Random>,
    ) -> Interpreter<Memory, Instruction> {
        Interpreter {
            memory,
    
            screen_memory,
    
            renderer,
    
            stack,
    
            delay_timer,
    
            sound_timer,
    
            keypad,
    
            program_counter,
    
            index_register: 0,
    
            variable_registers: [0; 16],
    
            font_start: 0x050,
    
            random,
        }
    }

    pub fn apply_font(&mut self, font: impl chip8_traits::Font) {
        font.apply(self.memory.as_mut(), self.font_start);
    }

    fn fetch(&mut self) -> Box<Instruction> {
        self.program_counter.read(self.memory.as_ref())
    }

    fn execute(&mut self, instruction: Box<Instruction>) {
        let instruction = *instruction.as_ref();
        let count = count8(instruction.w().to_vec());
        match count {
            0x00 => {
                let nn = count8(instruction.nn().to_vec());
                match nn {
                    0xe0 => self.screen_memory.clear(),
                    0xee => self.pop_stack(),
                    _ => return, // TODO: log unsupported
                }
            },
            0x01 => self.jump(instruction),
            0x02 => self.push_stack(instruction),
            0x03 | 0x04 | 0x05 | 0x09 => self.skip(instruction),
            0x06 => self.set_register(instruction),
            0x07 => self.add_to_register(instruction),
            0x08 => {
                let n = count8(instruction.n().to_vec());
                match n {
                    0x00 => self.set_x_value_of_y(instruction),
                    0x01 => self.or_x_value_of_y(instruction),
                    0x02 => self.and_x_value_of_y(instruction),
                    0x03 => self.xor_x_value_of_y(instruction),
                    0x04 => self.add_to_x_value_of_y(instruction),
                    0x05 => self.subtract_to_x_value_of_y(instruction),
                    0x07 => self.subtract_to_x_value_of_y_reversed(instruction),

                    0x06 => {}, // TODO:
                    0x0E => {}, 
                    _ => return, // TODO: log unsupported
                }
            },
            0x0A => self.set_index_register(instruction),
            0x0B => {}, // TODO:
            0x0C => self.set_register_random(instruction),
            0x0D => self.display(instruction),
            0x0E => {}, // TODO:
            0x0F => {
                let nn = count8(instruction.nn().to_vec());
                match nn {
                    0x07 | 0x15 | 0x18 => self.set_timer(instruction),
                    0x1e => self.add_to_index(instruction),
                    0x0a => self.get_key(instruction),
                    0x29 => self.font_character(instruction),
                    0x33 => self.binary_to_decimal(instruction),
                    0x55 => self.register_to_memory(instruction),
                    0x65 => self.memory_to_register(instruction),
                    _ => {} // TODO:
                }
            },
            _ => return, // TODO: log unsupported
        }
    }
}

impl<Memory, Instruction> Interpreter<Memory, Instruction> where Memory: chip8_traits::Memory, Instruction: chip8_traits::Instruction {
    fn jump(&mut self, instruction: Instruction) {
        let count = count16(instruction.nnn().to_vec());
        self.program_counter.set_position(count as usize);
    }

    fn push_stack(&mut self, instruction: Instruction) {
        self.stack.push(self.program_counter.get_position());
        let count = count16(instruction.nnn().to_vec());
        self.program_counter.set_position(count as usize);
    }

    fn pop_stack(&mut self) {
        let result = self.stack.pop();
        match result {
            Some(value) =>  self.program_counter.set_position(value),
            None => {
                // TODO: log
                println!("Stack underflow");
            }
        }
    }

    fn set_register(&mut self, instruction: Instruction) {
        let index = count8(instruction.x().to_vec());
        let value = count8(instruction.nn().to_vec());
        self.variable_registers[index as usize] = value;
    }

    fn add_to_register(&mut self, instruction: Instruction) {
        let index = count8(instruction.x().to_vec());
        let value = count8(instruction.nn().to_vec());
        let new_value = self.variable_registers[index as usize].wrapping_add(value);
        self.variable_registers[index as usize] = new_value;
    }

    fn set_index_register(&mut self, instruction: Instruction) {
        let value = count16(instruction.nnn().to_vec());
        self.index_register = value as usize;
    }

    fn get_register(&mut self, index: usize) -> u8 {
        self.variable_registers[index]
    }

    fn display(&mut self, instruction: Instruction) {
        let vx = count8(instruction.x().to_vec());
        let vy = count8(instruction.y().to_vec());
        let n = count8(instruction.n().to_vec());
        
        self.variable_registers[15] = 0;

        let x_value = self.get_register(vx as usize);
        let y_value = self.get_register(vy as usize);

        if self.screen_memory.display(
            x_value, 
            y_value, 
            self.memory.get_iter(self.index_register), 
            n) {
                self.variable_registers[15] = 1;
        }
    }

    fn set_timer(&mut self, instruction: Instruction) {
        let x = instruction.x();
        match count8(instruction.nn().to_vec()) {
            7 => {
                self.variable_registers[count8(x.to_vec()) as usize] = self.delay_timer.value();
            },
            15 => {
                self.delay_timer.set_value(
                    self.variable_registers[count8(x.to_vec()) as usize]
                );
            },
            18 => {
                self.sound_timer.set_value(
                    self.variable_registers[count8(x.to_vec()) as usize]
                );
            }
            _ => {
                // TODO: log
                println!("Unexpected timer");
            }
        }
    }

    fn skip(&mut self, instruction: Instruction) {
        let count = count8(instruction.w().to_vec());
        let x = count8(instruction.x().to_vec());
        match count {
            0x3 => {
                let value = count8(instruction.nn().to_vec());
                if value == self.variable_registers[x as usize] {
                    self.program_counter.skip();
                }
            },
            0x4 => {
                let value = count8(instruction.nn().to_vec());
                if value != self.variable_registers[x as usize] {
                    self.program_counter.skip();
                }
            },
            0x5 => {
                let y = count8(instruction.y().to_vec());
                if self.variable_registers[x as usize] == self.variable_registers[y as usize] {
                    self.program_counter.skip();
                }
            },
            0x9 => {
                let y = count8(instruction.y().to_vec());
                if self.variable_registers[x as usize] != self.variable_registers[y as usize] {
                    self.program_counter.skip();
                }
            },
            _ => {
                // TODO: log
                println!("Unexpected timer");
            }
        }
    }

    fn set_x_value_of_y(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        let y = count8(instruction.y().to_vec());

        self.variable_registers[x as usize] = self.variable_registers[y as usize];
    }

    fn or_x_value_of_y(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        let y = count8(instruction.y().to_vec());

        self.variable_registers[x as usize] |= self.variable_registers[y as usize];
    }

    fn and_x_value_of_y(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        let y = count8(instruction.y().to_vec());

        self.variable_registers[x as usize] &= self.variable_registers[y as usize];
    }

    fn xor_x_value_of_y(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        let y = count8(instruction.y().to_vec());

        self.variable_registers[x as usize] ^= self.variable_registers[y as usize];
    }

    fn add_to_x_value_of_y(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        let y = count8(instruction.y().to_vec());

        let result = self.variable_registers[x as usize].overflowing_add(self.variable_registers[y as usize]);
        if result.1 {
            self.variable_registers[x as usize] = result.0;
            self.variable_registers[0x0f] = 1;
        } else {
            self.variable_registers[0x0f] = 0;
        }
    }

    fn subtract_to_x_value_of_y(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        let y = count8(instruction.y().to_vec());

        let result = self.variable_registers[x as usize].overflowing_sub(self.variable_registers[y as usize]);
        if result.1 {
            self.variable_registers[x as usize] = result.0;
            self.variable_registers[0x0f] = 0;
        } else {
            self.variable_registers[0x0f] = 1;
        }
    }

    fn subtract_to_x_value_of_y_reversed(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        let y = count8(instruction.y().to_vec());

        let y_value = self.variable_registers[y as usize];
        let result = y_value.overflowing_sub(self.variable_registers[x as usize]);
        if result.1 {
            self.variable_registers[x as usize] = result.0;
            self.variable_registers[0x0f] = 0;
        } else {
            self.variable_registers[x as usize] = y_value;
            self.variable_registers[0x0f] = 1;
        }
    }

    fn set_register_random(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        let value = count8(instruction.nn().to_vec());
        let random_value = self.random.value();
        self.variable_registers[x as usize] = random_value & value;
    }

    fn add_to_index(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        // TODO: Amiga intepreter marks overflow option
        self.index_register += self.variable_registers[x as usize] as usize;
    }

    fn get_key(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
    }

    fn font_character(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        self.index_register = self.font_start + (self.variable_registers[x as usize] * 5) as usize;
    }

    fn binary_to_decimal(&mut self, instruction: Instruction) {
    }

    fn register_to_memory(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        // TODO: option to incriment I while working

        for offset in 0..=x {
            self.memory.set(
                self.index_register + offset as usize, 
                self.variable_registers[offset as usize]
            );
        }
    }

    fn memory_to_register(&mut self, instruction: Instruction) {
        let x = count8(instruction.x().to_vec());
        // TODO: option to incriment I while working

        for offset in 0..=x {
            self.variable_registers[offset as usize] = self.memory.get(self.index_register + offset as usize);
        }
    }
}

impl<Memory, Instruction> chip8_traits::Interpreter for Interpreter<Memory, Instruction> where Memory: chip8_traits::Memory, Instruction: chip8_traits::Instruction {
    fn load(&mut self, file_name: &str, start_position: usize) -> Result<(), std::io::Error> {
        let result = fs::read(file_name);
        match result {
            Ok(contents) => {
                for (index, value) in contents.iter().enumerate() {
                    self.memory.set(start_position + index, *value);
                }
                self.program_counter.set_position(start_position);
                return Ok(());
            },
            Err(error) => {
                return Err(error);
            }
        }
    }

    fn update(&mut self) -> Result<(), String> {
        let instruction = self.fetch();
        self.execute(instruction);

        let result = self.sound_timer.update();
        if let Err(error) = result {
            return Err(error);
        }

        let result = self.delay_timer.update();
        if let Err(error) = result {
            return Err(error);
        }

        // self.keypad.state();
        let result = self.renderer.render(self.screen_memory.iter());
        if let Err(error) = result {
            return Err(format!("{}", error));
        }

        Ok(())
    }

    fn dump_memory(&self) -> Vec<u8> {
        self.memory.dump()
    }
}