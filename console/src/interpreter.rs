use std::time::Duration;

use chip8_base::{DelayTimer, Font, Memory, ProgramCounter, ScreenMemory, SoundTimer, Stack};

pub fn new() -> chip8_base::Interpreter<crate::renderer::Renderer, crate::keypad::Keypad, crate::random::Random> {
    let timer_wait_between_cycles = Duration::from_secs_f32(1.0 / 60.0);
    chip8_base::Interpreter::new(
        Box::new(Memory::new(4096)),

        Box::new(ScreenMemory::new(64, 32)),

        Box::new(crate::renderer::Renderer::new()),

        Box::new(Stack::new()),

        Box::new(DelayTimer::new(timer_wait_between_cycles)),

        Box::new(SoundTimer::new()),

        Box::new(crate::keypad::Keypad::new()),

        ProgramCounter::new(),

        Box::new(crate::random::Random::new()),

        Font::new(),
    )
}