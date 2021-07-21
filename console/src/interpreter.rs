use chip8_base::{DelayTimer, Memory, ProgramCounter, ScreenMemory, SoundTimer, Stack};

pub fn new() -> chip8_base::Interpreter<chip8_base::Memory, chip8_base::Instruction> {
    chip8_base::Interpreter::new(
        Box::new(Memory::new(4096)),

        ScreenMemory::new(64, 32),

        Box::new(crate::renderer::Renderer::new()),

        Stack::new(),

        Box::new(DelayTimer::new()),

        Box::new(SoundTimer::new()),

        Box::new(crate::keypad::Keypad::new()),

        Box::new(ProgramCounter::new()),

        Box::new(crate::random::Random::new()),
    )
}