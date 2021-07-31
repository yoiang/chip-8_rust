use chip8_base::{DelayTimer, Font, Memory, ProgramCounter, ScreenMemory, SoundTimer, Stack};

pub fn new(renderer: crate::renderer::Renderer, keypad: crate::keypad::Keypad) -> chip8_base::Interpreter<crate::renderer::Renderer, crate::keypad::Keypad, crate::random::Random> {
    chip8_base::Interpreter::new(
        Memory::new_chip8(),

        ScreenMemory::new_chip8(),

        renderer,

        Stack::new(),

        DelayTimer::new(),

        SoundTimer::new(),

        keypad,

        ProgramCounter::new(),

        crate::random::Random::new(),

        Font::new(),
    )
}