use chip8_base::Font;
use chip8_traits::Interpreter;
use wasm_bindgen::prelude::*;
use std::{borrow::Borrow, cell::RefCell, fmt, panic, rc::Rc};

use crate::{fetch::fetch_body, renderer::fmt_rendered_memory, utils};

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

const DEFAULT_PROGRAM_START: usize = 0x200;
// const MAIN_LOOP_FREQUENCY: Duration = Duration::from_millis(1);

#[wasm_bindgen]
impl Index {
    pub fn new() -> Index {
        utils::set_panic_hook();
        
        let rendered_memory = Rc::new(RefCell::new(vec![]));
        let renderer = crate::renderer::Renderer::new(Rc::clone(&rendered_memory));
        let mut interpreter = crate::interpreter::new(renderer);

        let font = Font::new();
        interpreter.apply_font(font);
        
        // let load_result = chip8_traits::Interpreter::load_file(&mut interpreter, "../../../programs/Puzzle.cb8", 500);
        // if let Err(error) = load_result {
        //     unsafe {
        //         console_log!("Error: while loading: {}", error);
        //     }
        // }

        // let load_result = fetch_body("http://localhost:8080/Puzzle.ch8".to_string()).await;
        // match load_result {
        //     Ok(program) => {
        //         if program.len() > 0 {
        //             unsafe {
        //                 console_log!("Loaded program {} bytes", program.len());
        //             }
        //             chip8_traits::Interpreter::load(&mut interpreter, program, DEFAULT_PROGRAM_START);
        //         } else {
        //             unsafe {
        //                 console_log!("Unknown error loading program");
        //             }
        //         }
        //     },
        //     Err(error) => {
        //         unsafe {
        //             console_log!("Error: while loading: {:?}", error);
        //         }
        //     }
        // }

        Index {
            rendered_memory,
            interpreter,
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        let program_length = program.len();
        chip8_traits::Interpreter::load(&mut self.interpreter, program, DEFAULT_PROGRAM_START);
        unsafe {
            console_log!("Loaded program {} bytes", program_length);
        }
        chip8_traits::Interpreter::clear_screen(&mut self.interpreter);
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