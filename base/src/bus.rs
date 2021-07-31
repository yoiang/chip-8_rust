use crate::{DelayTimer, Memory, ProgramCounter, ScreenMemory, SoundTimer, Stack, VariableRegisters, cpu::ExecuteResult};

// trait Bus {
//     // fn program_counter(&self) -> &mut ProgramCounter;
//     fn stack(&mut self) -> &mut Stack;
//     fn memory(&mut self) -> &mut Memory;
//     fn screen_memory(&mut self) -> &mut ScreenMemory;
//     fn variable_registers(&mut self) -> &mut VariableRegisters;
//     // fn keypad(&self) -> &mut dyn Keypad;
//     fn index_register(&mut self) -> &mut usize;
//     fn delay_timer(&mut self) -> &mut DelayTimer;
//     fn sound_timer(&mut self) -> &mut SoundTimer;
//     // Random
// }

pub struct Bus<'a,
    Keypad: chip8_traits::Keypad, 
    Random: chip8_traits::Random
    > {
    pub program_counter: &'a mut ProgramCounter,
    pub stack: &'a mut Stack,
    pub memory: &'a mut Memory,
    pub screen_memory: &'a mut ScreenMemory,
    pub variable_registers: &'a mut VariableRegisters,
    pub keypad: &'a Keypad,
    pub index_register: &'a mut usize,
    pub delay_timer: &'a mut DelayTimer,
    pub sound_timer: &'a mut SoundTimer,
    pub random: &'a mut Random,
}

impl<'a,
    Keypad: chip8_traits::Keypad, 
    Random: chip8_traits::Random
> Bus<'a, Keypad, Random> {
    pub fn execute<'b, Instruction: chip8_traits::Instruction>(
        &'b mut self,
        apply_instruction: bool,
        instruction: Instruction, 
        font_start: usize
    ) -> ExecuteResult<Instruction> {
        return crate::cpu::execute(
            apply_instruction,
            instruction,
            self.program_counter,
            self.stack,
            self.memory,
            self.screen_memory,
            self.variable_registers,
            self.keypad,
            self.index_register,
            self.delay_timer,
            self.sound_timer,
            self.random,
            font_start
        );
    }
}