use std::{fs, usize};

use crate::{DelayTimer, SoundTimer, cpu::execute};

pub struct Interpreter<Renderer, Keypad, Random> 
where Renderer: chip8_traits::Renderer, 
    Keypad: chip8_traits::Keypad,
    Random: chip8_traits::Random {
    memory: Box<crate::Memory>,

    screen_memory: Box<crate::ScreenMemory>,

    renderer: Box<Renderer>,

    stack: Box<crate::Stack>,

    delay_timer: Box<DelayTimer>,
    sound_timer: Box<SoundTimer>,

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

        delay_timer: Box<DelayTimer>,
        sound_timer: Box<SoundTimer>,

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


    // fn get_key(&mut self, instruction: crate::Instruction) {
    //     let x = count8(instruction.x().to_vec());
    // }

    // fn binary_to_decimal(&mut self, instruction: crate::Instruction) {
    // }

    pub fn dump_program_counter(&self) -> usize {
        chip8_traits::ProgramCounter::<crate::Instruction>::get_position(&self.program_counter)
    }
}

impl<Renderer, Keypad, Random> chip8_traits::Interpreter for Interpreter<Renderer, Keypad, Random> 
where Renderer: chip8_traits::Renderer, 
    Keypad: chip8_traits::Keypad,
    Random: chip8_traits::Random {
    fn load(&mut self, program: Vec<u8>, start_position: usize) {
        for (index, value) in program.iter().enumerate() {
            chip8_traits::Memory::set(self.memory.as_mut(), start_position + index, *value);
        }
        chip8_traits::ProgramCounter::set_position(&mut self.program_counter, start_position);
    }

    fn load_file(&mut self, file_name: &str, start_position: usize) -> Result<(), std::io::Error> {
        let result = fs::read(file_name);
        match result {
            Ok(contents) => {
                self.load(contents, start_position);
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
            self.delay_timer.as_mut(),
            self.sound_timer.as_mut(),
            self.random.as_mut(),
            self.font_start
        );
        if let Err(error) = result {
            // TODO: handle for real
            println!("While executing instruction: {}", error);
        }

        // TODO: something seems broken that I can't do "use" and have to fully qualify or when that fails, cast
        let result = (self.sound_timer.as_mut() as &mut dyn chip8_traits::Timer).update();
        if let Err(error) = result {
            return Err(error);
        }

        // TODO: something seems broken that I can't do "use" and have to fully qualify or when that fails, cast
        let result = (self.delay_timer.as_mut() as &mut dyn chip8_traits::Timer).update();
        if let Err(error) = result {
            return Err(error);
        }

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