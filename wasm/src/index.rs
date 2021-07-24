use chip8_base::Font;
use chip8_traits::Interpreter;
use wasm_bindgen::prelude::*;
use std::{borrow::Borrow, cell::{RefCell}, fmt, rc::Rc};

use crate::{renderer::fmt_rendered_memory, utils};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! console_log_unsafe {
    ( $( $t:tt )* ) => {
        #[allow(unused_unsafe)] // Currently unsafe not properly recognized by analyzer
        unsafe {
            console_log!($( $t )*);
        }
    }
}

#[wasm_bindgen]
pub struct Index {
    rendered_memory: Rc<RefCell<Vec<Vec<bool>>>>,
    keypad_state: Rc<RefCell<[bool; 16]>>,

    interpreter: chip8_base::Interpreter<crate::renderer::Renderer, crate::keypad::Keypad, crate::random::Random>,
}

const DEFAULT_PROGRAM_START: usize = 0x200;
// const MAIN_LOOP_FREQUENCY: Duration = Duration::from_millis(1);

#[wasm_bindgen]
impl Index {
    pub fn new() -> Index {
        utils::set_panic_hook();
        
        let rendered_memory = Rc::new(RefCell::new(vec![]));
        let renderer = crate::renderer::Renderer::new(Rc::clone(&rendered_memory));

        let keypad_state = Rc::new(RefCell::new([false; 16]));
        let keypad = crate::keypad::Keypad::new(Rc::clone(&keypad_state));
        
        let interpreter = crate::interpreter::new(renderer, keypad);

        Index {
            rendered_memory,
            keypad_state,

            interpreter,
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        let program_length = program.len();
        chip8_traits::Interpreter::load(&mut self.interpreter, program, DEFAULT_PROGRAM_START);
        console_log_unsafe!("Loaded program {} bytes", program_length);
        chip8_traits::Interpreter::clear_screen(&mut self.interpreter);
    }

    pub fn update(&mut self) {
        if let Err(error) = chip8_traits::Interpreter::update(&mut self.interpreter) {
            console_log_unsafe!("Error: while updating: {}", error);
        }
    }

    pub fn render_text(&self) -> String {
        self.to_string()
    }

    pub fn update_keypad(&mut self, update: Vec<u8>) {
        let mut update_normalized = [false; 16];

        for (index, value) in update.iter().enumerate() {
            if index >= update_normalized.len() {
                break;
            }
            update_normalized[index] = *value > 0;
        }

        (*self.keypad_state.borrow_mut()) = update_normalized;
    }

    pub fn dump_memory(&self) -> String {
        match String::from_utf8(self.interpreter.dump_memory()) {
            Ok(result) => result.to_string(),
            Err(error) => error.to_string()
        }
    }

    pub fn dump_program_counter(&self) -> String {
        self.interpreter.dump_program_counter().to_string()
    }
}


impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let memory: &RefCell<Vec<Vec<bool>>> = self.rendered_memory.borrow();
        fmt_rendered_memory(&(*memory).borrow(), f)
    }
} 