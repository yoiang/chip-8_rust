use std::{fs, usize};

use crate::cpu::execute;

pub struct Interpreter<Renderer, Keypad, Random> 
where Renderer: chip8_traits::Renderer, 
    Keypad: chip8_traits::Keypad,
    Random: chip8_traits::Random {
    memory: Box<crate::Memory>,

    screen_memory: Box<crate::ScreenMemory>,

    renderer: Box<Renderer>,

    stack: Box<crate::Stack>,

    delay_timer: Box<dyn chip8_traits::Timer>,
    sound_timer: Box<dyn chip8_traits::Timer>,

    keypad: Box<Keypad>,

    program_counter: crate::ProgramCounter,

    index_register: usize,

    variable_registers: crate::VariableRegisters,

    font_start: usize,

    random: Box<Random>,
}

impl<Renderer, Keypad, Random> Interpreter<Renderer, Keypad, Random> 
where Renderer: chip8_traits::Renderer, 
    Keypad: chip8_traits::Keypad,
    Random: chip8_traits::Random  {
    pub fn new(
        memory: Box<crate::Memory>,
        screen_memory: Box<crate::ScreenMemory>,
        renderer: Box<Renderer>,
        stack: Box<crate::Stack>,

        delay_timer: Box<crate::DelayTimer>,
        sound_timer: Box<crate::SoundTimer>,

        keypad: Box<Keypad>,

        program_counter: crate::ProgramCounter,

        random: Box<Random>,
    ) -> Interpreter<Renderer, Keypad, Random> {
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
    
            variable_registers: crate::VariableRegisters::new(),
    
            font_start: 0x050,
    
            random,
        }
    }

    pub fn apply_font(&mut self, font: impl chip8_traits::Font) {
        font.apply(self.memory.as_mut(), self.font_start);
    }

    fn fetch(&mut self) -> Box<crate::Instruction> {
        chip8_traits::ProgramCounter::read(&mut self.program_counter, self.memory.as_ref())
    }
}

impl<Renderer, Keypad, Random> Interpreter<Renderer, Keypad, Random> 
where Renderer: chip8_traits::Renderer, 
    Keypad: chip8_traits::Keypad,
    Random: chip8_traits::Random {



    // fn get_register(&mut self, index: usize) -> u8 {
    //     self.variable_registers[index]
    // }

    // fn set_timer(&mut self, instruction: crate::Instruction) {
    //     let x = instruction.x();
    //     match count8(instruction.nn().to_vec()) {
    //         7 => {
    //             self.variable_registers[count8(x.to_vec()) as usize] = self.delay_timer.value();
    //         },
    //         15 => {
    //             self.delay_timer.set_value(
    //                 self.variable_registers[count8(x.to_vec()) as usize]
    //             );
    //         },
    //         18 => {
    //             self.sound_timer.set_value(
    //                 self.variable_registers[count8(x.to_vec()) as usize]
    //             );
    //         }
    //         _ => {
    //             // TODO: log
    //             println!("Unexpected timer");
    //         }
    //     }
    // }

    // fn add_to_index(&mut self, instruction: crate::Instruction) {
    //     let x = count8(instruction.x().to_vec());
    //     // TODO: Amiga intepreter marks overflow option
    //     self.index_register += self.variable_registers[x as usize] as usize;
    // }

    // fn get_key(&mut self, instruction: crate::Instruction) {
    //     let x = count8(instruction.x().to_vec());
    // }

    // fn binary_to_decimal(&mut self, instruction: crate::Instruction) {
    // }

    // fn register_to_memory(&mut self, instruction: crate::Instruction) {
    //     let x = count8(instruction.x().to_vec());
    //     // TODO: option to incriment I while working

    //     for offset in 0..=x {
    //         self.memory.set(
    //             self.index_register + offset as usize, 
    //             self.variable_registers[offset as usize]
    //         );
    //     }
    // }

    // fn memory_to_register(&mut self, instruction: crate::Instruction) {
    //     let x = count8(instruction.x().to_vec());
    //     // TODO: option to incriment I while working

    //     for offset in 0..=x {
    //         self.variable_registers[offset as usize] = self.memory.get(self.index_register + offset as usize);
    //     }
    // }
}

impl<Renderer, Keypad, Random> chip8_traits::Interpreter for Interpreter<Renderer, Keypad, Random> 
where Renderer: chip8_traits::Renderer, 
    Keypad: chip8_traits::Keypad,
    Random: chip8_traits::Random {
    fn load(&mut self, file_name: &str, start_position: usize) -> Result<(), std::io::Error> {
        let result = fs::read(file_name);
        match result {
            Ok(contents) => {
                for (index, value) in contents.iter().enumerate() {
                    chip8_traits::Memory::set(self.memory.as_mut(), start_position + index, *value);
                }
                chip8_traits::ProgramCounter::set_position(&mut self.program_counter, start_position);
                return Ok(());
            },
            Err(error) => {
                return Err(error);
            }
        }
    }

    fn update(&mut self) -> Result<(), String> {
        let instruction = self.fetch();
        let result = execute(
            *instruction.as_ref(), 
            &mut self.program_counter, 
            self.stack.as_mut(), 
            self.memory.as_mut(),
            self.screen_memory.as_mut(),
            &mut self.variable_registers,
            &mut self.index_register,
            self.random.as_mut(),
            self.font_start
        );
        if let Err(error) = result {
            // TODO: handle for real
            println!("While executing instruction: {}", error);
        }

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
        chip8_traits::Memory::dump(self.memory.as_ref())
    }
}