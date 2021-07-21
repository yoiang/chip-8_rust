use std::{fs, usize};
use nanorand::{Rng};

use super::{DelayTimer, Instruction, Memory, ProgramCounter, ScreenMemory, SoundTimer, Stack};

extern crate yacurses;

pub struct Interpreter {
    memory: Box<dyn crate::traits::Memory>,

    screen_memory: super::ScreenMemory,

    renderer: Box<dyn crate::traits::Renderer>,

    stack: super::Stack,

    delay_timer: Box<dyn crate::traits::Timer>,
    sound_timer: Box<dyn crate::traits::Timer>,

    keypad: Box<dyn crate::traits::Keypad>,

    program_counter: ProgramCounter,

    index_register: usize,

    variable_registers: [u8; 16],

    font_start: usize,

    random: nanorand::WyRand,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut result = Interpreter {
            memory: Box::new(Memory::new(4096)),

            screen_memory: ScreenMemory::new(64, 32),

            renderer: Box::new(super::console::Renderer::new()),

            stack: Stack::new(),

            delay_timer: Box::new(DelayTimer::new()),

            sound_timer: Box::new(SoundTimer::new()),

            keypad: Box::new(super::console::Keypad::new()),

            program_counter: ProgramCounter::new(),

            index_register: 0,

            variable_registers: [0; 16],

            font_start: 0x050,

            random: nanorand::WyRand::new(),
        };

        let font = super::Font::new();
        font.apply(result.memory.as_mut(), result.font_start);

        result.memory.set(result.font_start, 0);
        result.keypad.state();
        result.stack.clear();

        result
    }
}

impl Interpreter {
    fn fetch(&mut self) -> Instruction {
        self.program_counter.read(self.memory.as_ref())
    }

    fn count8(bits: Vec<bool>) -> u8 {
        let mut result: u8 = 0;

        let mut place = 1;
        for value in bits.iter().rev() {
            if *value {
                result += place;
            }
            
            if place == 128 { // TODO: come up with better solution
                break;
            }
            place *= 2;
        }

        result
    }

    fn count16(bits: Vec<bool>) -> u16 {
        let mut result: u16 = 0;

        let mut place = 1;
        for value in bits.iter().rev() {
            if *value {
                result += place;
            }
            
            if place == 32768 { // TODO: come up with better solution
                break;
            }
            place *= 2;
        }

        result
    }

    fn execute(&mut self, instruction: Instruction) {
        let count = Interpreter::count8(instruction.w().to_vec());
        match count {
            0x00 => {
                let nn = Interpreter::count8(instruction.nn().to_vec());
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
                let n = Interpreter::count8(instruction.n().to_vec());
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
                let nn = Interpreter::count8(instruction.nn().to_vec());
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

impl Interpreter {
    fn jump(&mut self, instruction: Instruction) {
        let count = Interpreter::count16(instruction.nnn().to_vec());
        self.program_counter.set_position(count as usize);
    }

    fn push_stack(&mut self, instruction: Instruction) {
        self.stack.push(self.program_counter.get_position());
        let count = Interpreter::count16(instruction.nnn().to_vec());
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
        let index = Interpreter::count8(instruction.x().to_vec());
        let value = Interpreter::count8(instruction.nn().to_vec());
        self.variable_registers[index as usize] = value;
    }

    fn add_to_register(&mut self, instruction: Instruction) {
        let index = Interpreter::count8(instruction.x().to_vec());
        let value = Interpreter::count8(instruction.nn().to_vec());
        let new_value = self.variable_registers[index as usize].wrapping_add(value);
        self.variable_registers[index as usize] = new_value;
    }

    fn set_index_register(&mut self, instruction: Instruction) {
        let value = Interpreter::count16(instruction.nnn().to_vec());
        self.index_register = value as usize;
    }

    fn get_register(&mut self, index: usize) -> u8 {
        self.variable_registers[index]
    }

    fn display(&mut self, instruction: Instruction) {
        let vx = Interpreter::count8(instruction.x().to_vec());
        let vy = Interpreter::count8(instruction.y().to_vec());
        let n = Interpreter::count8(instruction.n().to_vec());
        
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
        match Interpreter::count8(instruction.nn().to_vec()) {
            7 => {
                self.variable_registers[Interpreter::count8(x.to_vec()) as usize] = self.delay_timer.value();
            },
            15 => {
                self.delay_timer.set_value(
                    self.variable_registers[Interpreter::count8(x.to_vec()) as usize]
                );
            },
            18 => {
                self.sound_timer.set_value(
                    self.variable_registers[Interpreter::count8(x.to_vec()) as usize]
                );
            }
            _ => {
                // TODO: log
                println!("Unexpected timer");
            }
        }
    }

    fn skip(&mut self, instruction: Instruction) {
        let count = Interpreter::count8(instruction.w().to_vec());
        let x = Interpreter::count8(instruction.x().to_vec());
        match count {
            0x3 => {
                let value = Interpreter::count8(instruction.nn().to_vec());
                if value == self.variable_registers[x as usize] {
                    self.program_counter.skip();
                }
            },
            0x4 => {
                let value = Interpreter::count8(instruction.nn().to_vec());
                if value != self.variable_registers[x as usize] {
                    self.program_counter.skip();
                }
            },
            0x5 => {
                let y = Interpreter::count8(instruction.y().to_vec());
                if self.variable_registers[x as usize] == self.variable_registers[y as usize] {
                    self.program_counter.skip();
                }
            },
            0x9 => {
                let y = Interpreter::count8(instruction.y().to_vec());
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
        let x = Interpreter::count8(instruction.x().to_vec());
        let y = Interpreter::count8(instruction.y().to_vec());

        self.variable_registers[x as usize] = self.variable_registers[y as usize];
    }

    fn or_x_value_of_y(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        let y = Interpreter::count8(instruction.y().to_vec());

        self.variable_registers[x as usize] |= self.variable_registers[y as usize];
    }

    fn and_x_value_of_y(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        let y = Interpreter::count8(instruction.y().to_vec());

        self.variable_registers[x as usize] &= self.variable_registers[y as usize];
    }

    fn xor_x_value_of_y(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        let y = Interpreter::count8(instruction.y().to_vec());

        self.variable_registers[x as usize] ^= self.variable_registers[y as usize];
    }

    fn add_to_x_value_of_y(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        let y = Interpreter::count8(instruction.y().to_vec());

        let result = self.variable_registers[x as usize].overflowing_add(self.variable_registers[y as usize]);
        if result.1 {
            self.variable_registers[x as usize] = result.0;
            self.variable_registers[0x0f] = 1;
        } else {
            self.variable_registers[0x0f] = 0;
        }
    }

    fn subtract_to_x_value_of_y(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        let y = Interpreter::count8(instruction.y().to_vec());

        let result = self.variable_registers[x as usize].overflowing_sub(self.variable_registers[y as usize]);
        if result.1 {
            self.variable_registers[x as usize] = result.0;
            self.variable_registers[0x0f] = 0;
        } else {
            self.variable_registers[0x0f] = 1;
        }
    }

    fn subtract_to_x_value_of_y_reversed(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        let y = Interpreter::count8(instruction.y().to_vec());

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
        let x = Interpreter::count8(instruction.x().to_vec());
        let value = Interpreter::count8(instruction.nn().to_vec());
        let random_value = self.random.generate::<u8>();
        self.variable_registers[x as usize] = random_value & value;
    }

    fn add_to_index(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        // TODO: Amiga intepreter marks overflow option
        self.index_register += self.variable_registers[x as usize] as usize;
    }

    fn get_key(&mut self, instruction: Instruction) {
    }

    fn font_character(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        self.index_register = self.font_start + self.variable_registers[x as usize] as usize;
    }

    fn binary_to_decimal(&mut self, instruction: Instruction) {
    }

    fn register_to_memory(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        // TODO: option to incriment I while working

        for offset in 0..=x {
            self.memory.set(
                self.index_register + offset as usize, 
                self.variable_registers[offset as usize]
            );
        }
    }

    fn memory_to_register(&mut self, instruction: Instruction) {
        let x = Interpreter::count8(instruction.x().to_vec());
        // TODO: option to incriment I while working

        for offset in 0..=x {
            self.variable_registers[offset as usize] = self.memory.get(self.index_register + offset as usize);
        }
    }
}

impl super::super::traits::Interpreter for Interpreter {
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