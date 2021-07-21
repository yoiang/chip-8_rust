use std::{cell::RefCell, fs, rc::Rc, usize};

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
            0x00 => self.screen_memory.clear(),
            0x01 => self.jump(instruction),
            0x06 => self.set_register(instruction),
            0x07 => self.add_to_register(instruction),
            0x0A => self.set_index_register(instruction),
            0x0D => self.display(instruction),
            _ => return, // TODO: log unsupported
        }
    }
}

impl Interpreter {
    fn jump(&mut self, instruction: Instruction) {
        let count = Interpreter::count16(instruction.nnn().to_vec());
        self.program_counter.set_position(count as usize);
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