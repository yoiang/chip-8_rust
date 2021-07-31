use std::{borrow::{BorrowMut}, fs, usize};

use chip8_traits::{ProgramCounter, Timer};

use crate::{DelayTimer, Instruction, SoundTimer, bus::Bus, cpu::execute};


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

    font: crate::Font,
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

        font: crate::Font,
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
    
            font,
            font_start: 0x050,
    
            random,

        }
    }

    pub fn new_crate_defaults(renderer: Box<Renderer>, keypad: Box<Keypad>, random: Box<Random>) -> Interpreter<Renderer, Keypad, Random> {
        Interpreter::new(
             Box::new(crate::Memory::new(4096)),
             Box::new(crate::ScreenMemory::new(64, 32)),
            renderer,
            Box::new(crate::Stack::new()),
            Box::new(crate::DelayTimer::new()),
            Box::new(crate::SoundTimer::new()),
            keypad,
            crate::ProgramCounter::new(),
            random,
            crate::Font::new()
        )
    }

    pub fn create_bus<'bus>(&'bus mut self) -> Bus<'bus, Keypad, Random> {
        Bus {
            program_counter: &mut self.program_counter,
            stack: self.stack.borrow_mut(),
            memory: self.memory.borrow_mut(),
            screen_memory: self.screen_memory.borrow_mut(),
            variable_registers: self.variable_registers.borrow_mut(),
            keypad: self.keypad.borrow_mut(),
            index_register: self.index_register.borrow_mut(),
            delay_timer: self.delay_timer.borrow_mut(),
            sound_timer: self.sound_timer.borrow_mut(),
            random: self.random.borrow_mut()
        }
    }

    pub fn apply_font(&mut self, font: impl chip8_traits::Font) {
        font.apply(self.memory.as_mut(), self.font_start);
    }

    fn fetch(&mut self) -> Box<crate::Instruction> {
        // chip8_traits::ProgramCounter::read(&mut self.program_counter, self.memory.as_ref())

        let position = self.program_counter.get_position();
        let first = chip8_traits::Memory::get(self.memory.as_ref(), position);
        let second = chip8_traits::Memory::get(self.memory.as_ref(), position + 1);
        self.program_counter.skip();

        Box::new(super::Instruction::new(first, second))
    }

    fn reset(&mut self) {
        self.memory.clear();
        self.apply_font(self.font.clone());
        self.variable_registers.reset();
        self.index_register = 0;
        self.sound_timer.reset();
        self.delay_timer.reset();
    }
}

impl<Renderer, Keypad, Random> chip8_traits::Interpreter<crate::cpu::ExecutionState> for Interpreter<Renderer, Keypad, Random> 
where Renderer: chip8_traits::Renderer, 
    Keypad: chip8_traits::Keypad,
    Random: chip8_traits::Random {
    fn load(&mut self, program: Vec<u8>, start_position: usize) {
        self.reset();
        
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

    fn update(&mut self) -> Result<crate::cpu::ExecutionState, String> {
        let instruction = self.fetch();
        
        let execution_state: crate::cpu::ExecutionState;

        let result = execute(
            true,
            *instruction.as_ref(), 
            &mut self.program_counter, 
            self.stack.as_mut(), 
            self.memory.as_mut(),
            self.screen_memory.as_mut(),
            &mut self.variable_registers,
            self.keypad.as_mut(),
            &mut self.index_register,
            self.delay_timer.as_mut(),
            self.sound_timer.as_mut(),
            self.random.as_mut(),
            self.font_start
        );
        match result {
            Ok(value) => { execution_state = value; },
            Err(error) => {
                return Err(format!("While executing instruction: {}", error))
            }
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

        Ok(execution_state)
    }

    fn clear_screen(&mut self) {
        (self.screen_memory.as_mut() as &mut dyn chip8_traits::ScreenMemory).clear();
    }

    fn dump_memory(&self) -> Vec<u8> {
        chip8_traits::Memory::dump(self.memory.as_ref())
    }
}

pub struct InterpreterSnapshot {
    pub program_counter_position: usize,

    pub index_register_value: usize,
    pub variable_register_values: [u8; 16],

    pub delay_timer_value: u8,
    pub sound_timer_value: u8,

    pub partial_disassemble: Vec<PartialDisassembleSnapshot>
}

pub struct PartialDisassembleOptions {
    pub count_before: usize, 
    pub count_after: usize,
    pub fix_misalignment: bool,
    pub maintain_length: bool
}

pub struct PartialDisassembleSnapshot {
    pub location: usize,
    pub value: (u8, u8),
    pub disassembly: String
}

impl<Renderer, Keypad, Random> Interpreter<Renderer, Keypad, Random> 
where Renderer: chip8_traits::Renderer, 
    Keypad: chip8_traits::Keypad,
    Random: chip8_traits::Random {
    pub fn dump_program_counter(&self) -> usize {
        chip8_traits::ProgramCounter::get_position(&self.program_counter)
    }

    // TODO: figure out a way to separate from mutating execute
    pub fn create_partial_disassemble_snapshot(&mut self, disassemble_options: PartialDisassembleOptions) -> Vec<PartialDisassembleSnapshot> {
        // TODO: fix_misalignment
        // TODO: maintain_length
        let start_location = self.program_counter.get_position() - disassemble_options.count_before * 2;
        let end_location = self.program_counter.get_position() + disassemble_options.count_after * 2 + 1;
        let memory_snapshot = self.memory.snapshot(start_location, end_location);

        let mut result: Vec<PartialDisassembleSnapshot> = vec![];

        let mut is_first = true;
        let mut first_value: u8 = 0;
        let mut first_location: usize = 0;
        for index in 0..memory_snapshot.len() {
            if is_first {
                first_value = memory_snapshot[index].value;
                first_location = memory_snapshot[index].location;
            } else {
                let second_value = memory_snapshot[index].value;
                let instruction = Instruction::new(first_value, second_value);
                let disassembly: String;
                
                match execute(
                    false,
                    instruction, 
                    &mut self.program_counter, 
                    self.stack.as_mut(), 
                    self.memory.as_mut(),
                    self.screen_memory.as_mut(),
                    &mut self.variable_registers,
                    self.keypad.as_mut(),
                    &mut self.index_register,
                    self.delay_timer.as_mut(),
                    self.sound_timer.as_mut(),
                    self.random.as_mut(),
                    self.font_start
                ) {
                    Ok(result) => { disassembly = result.instruction_disassembly; },
                    Err(_) => { disassembly = "".to_string(); }
                }

                result.push(PartialDisassembleSnapshot {
                    location: first_location,
                    value: (first_value, second_value),
                    disassembly 
                });
            }

            is_first = !is_first;
        }

        result
    }

    pub fn create_snapshot(&mut self, disassemble_options: PartialDisassembleOptions) -> InterpreterSnapshot {
        InterpreterSnapshot {
            program_counter_position: self.program_counter.get_position(),
            
            index_register_value: self.index_register,
            variable_register_values: self.variable_registers.get_all(),

            delay_timer_value: self.delay_timer.get(),
            sound_timer_value: self.sound_timer.get(),

            partial_disassemble: self.create_partial_disassemble_snapshot(disassemble_options)
        }
    }
}