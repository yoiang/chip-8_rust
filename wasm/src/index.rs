use wasm_bindgen::prelude::*;
use std::{cell::RefCell, fmt, panic, rc::Rc};

use crate::{renderer::fmt_rendered_memory, utils};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Index {
    rendered_memory: Rc<RefCell<Vec<Vec<bool>>>>,
    interpreter: chip8_base::Interpreter<crate::renderer::Renderer, crate::keypad::Keypad, crate::random::Random>,
}

#[wasm_bindgen]
impl Index {
    pub fn new() -> Index {
        utils::set_panic_hook();
        
        let rendered_memory = Rc::new(RefCell::new(vec![]));
        let renderer = crate::renderer::Renderer::new(Rc::clone(&rendered_memory));
        let mut interpreter = crate::interpreter::new(renderer);
        let load_result = chip8_traits::Interpreter::load(&mut interpreter, "../../../examples/Puzzle.cb8", 500);
        if let Err(error) = load_result {
            unsafe {
                console_log!("Error: while loading: {}", error);
            }
        }
        Index {
            rendered_memory,
            interpreter,
        }
    }

    pub fn update(&mut self) {
        if let Err(error) = chip8_traits::Interpreter::update(&mut self.interpreter) {
            unsafe {
                console_log!("Error: while updating: {}", error);
            }
        }
    }

    pub fn render_text(&self) -> String {
        self.to_string()
    }
}


impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_rendered_memory(self.rendered_memory.borrow().as_ref(), f)
    }
} 