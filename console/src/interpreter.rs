use chip8_base::{DelayTimer, Font, Memory, ProgramCounter, ScreenMemory, SoundTimer, Stack};

pub fn new() -> chip8_base::Interpreter<crate::renderer::Renderer, crate::keypad::Keypad, crate::random::Random> {
    chip8_base::Interpreter::new(
        Box::new(Memory::new(4096)),

        Box::new(ScreenMemory::new(64, 32)),

        Box::new(crate::renderer::Renderer::new()),

        Box::new(Stack::new()),

        Box::new(DelayTimer::new()),

        Box::new(SoundTimer::new()),

        Box::new(crate::keypad::Keypad::new()),

        ProgramCounter::new(),

        Box::new(crate::random::Random::new()),

        Font::new(),
    )
}