use chip8_traits::Interpreter;
use wasm_bindgen::prelude::*;
use std::{borrow::{Borrow}, cell::{RefCell}, fmt, rc::Rc};
use serde::{Serialize, Deserialize};

use crate::{console_log_unsafe, renderer::fmt_rendered_memory, utility::{js_value_as_usize, set_panic_hook}};

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
        set_panic_hook();
        
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
        crate::console_log_unsafe!("Loaded program {} bytes", program_length);
        chip8_traits::Interpreter::clear_screen(&mut self.interpreter);
    }

    pub fn update(&mut self) {
        match chip8_traits::Interpreter::update(&mut self.interpreter) {
            Ok(result) => {
                // console_log_unsafe!("Result: {}", result.instruction_disassembly.to_string());
            },
            Err(error) => crate::console_log_unsafe!("Error: while updating: {}", error)
        }
    }

    pub fn render_text(&self) -> String {
        self.to_string()
    }

    fn set_key_state(&mut self, js_index: JsValue, state: bool) -> bool {
        match js_value_as_usize(js_index) {
            Some(index) => {
                let mut keypad_state = *(*self.keypad_state).borrow_mut();
                keypad_state[index as usize] = state;

                true
            },
            None => false
        }
    }

    pub fn keydown(&mut self, js_index: JsValue) -> bool {
        self.set_key_state(js_index, true)
    }

    pub fn keyup(&mut self, js_index: JsValue) -> bool {
        self.set_key_state(js_index, false)
    }

    pub fn key_state(&self) -> Box<[JsValue]> {
        let keypad_state = *(*self.keypad_state).borrow();
        keypad_state.iter().map(|value| JsValue::from_bool(*value)).collect()
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

#[allow(dead_code)]
#[wasm_bindgen]
pub struct InterpreterSnapshot {
    pub program_counter_position: usize,
    
    pub index_register_value: usize,
    variable_register_values: [u8; 16],
    
    pub delay_timer_value: u8,
    pub sound_timer_value: u8,

    partial_disassemble: Vec<PartialDisassembleSnapshot>
}

#[wasm_bindgen]
impl InterpreterSnapshot {
    #[wasm_bindgen(getter)]
    pub fn variable_register_values(&self) -> js_sys::Uint8Array {
        return js_sys::Uint8Array::from(&self.variable_register_values[..]);
    }

    #[wasm_bindgen(getter)]
    pub fn partial_disassemble(&self) -> JsValue {
        // TODO: remove unwrap and properly handle
        JsValue::from_serde(&self.partial_disassemble).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct PartialDisassembleSnapshot {
    pub location: usize,
    pub value: (u8, u8),
    pub disassembly: String
}

// #[wasm_bindgen]
// impl PartialDisassembleSnapshot {
//    #[wasm_bindgen(getter)]
//     pub fn value(&self) -> js_sys::Uint8Array {
//         return js_sys::Uint8Array::from(&self.value[..]);
//     }
// }

#[wasm_bindgen]
impl Index {
    pub fn create_interpreter_snapshot(&mut self, count_before: usize, count_after: usize) -> InterpreterSnapshot {
        let chip8_base::interpreter::InterpreterSnapshot { 
            program_counter_position, 
            index_register_value, 
            variable_register_values,
            delay_timer_value, 
            sound_timer_value,
            partial_disassemble
        } = self.interpreter.create_snapshot(chip8_base::interpreter::PartialDisassembleOptions{
                count_before,
                count_after,
                fix_misalignment: true,
                maintain_length: true
            });

        InterpreterSnapshot {
            program_counter_position,
            index_register_value, 
            variable_register_values,
            delay_timer_value, 
            sound_timer_value,
            partial_disassemble: partial_disassemble.iter().map(|value| { PartialDisassembleSnapshot {
                location: value.location,
                value: value.value,
                disassembly: value.disassembly.clone(),
            } }).collect()
        }
    }
}


impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let memory: &RefCell<Vec<Vec<bool>>> = self.rendered_memory.borrow();
        fmt_rendered_memory(&(*memory).borrow(), f)
    }
} 