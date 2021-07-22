use chip8_base::{DelayTimer, Memory, ProgramCounter, ScreenMemory, SoundTimer, Stack};

pub fn new(renderer: crate::renderer::Renderer) -> chip8_base::Interpreter<crate::renderer::Renderer, crate::keypad::Keypad, crate::random::Random> {
    chip8_base::Interpreter::new(
        Box::new(Memory::new(4096)),

        Box::new(ScreenMemory::new(64, 32)),

        Box::new(renderer),

        Box::new(Stack::new()),

        Box::new(DelayTimer::new()),

        Box::new(SoundTimer::new()),

        Box::new(crate::keypad::Keypad::new()),

        ProgramCounter::new(),

        Box::new(crate::random::Random::new()),
    )
}