use std::{env, time::Duration};

use chip8_base::Font;
use chip8_traits::Interpreter;

const DEFAULT_PROGRAM_START: usize = 0x200;
const MAIN_LOOP_FREQUENCY: Duration = Duration::from_millis(1);

mod renderer;
mod keypad;
mod interpreter;
mod random;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut interpreter = interpreter::new();
    let font = Font::new();
    interpreter.apply_font(font);

    let load_file_name = {
        if args.len() > 1 {
            args[1].as_str()
        } else {
            "programs/Puzzle.ch8"
        }
    };
    let result = interpreter.load_file(load_file_name, DEFAULT_PROGRAM_START);
    match result {
        Ok(_) => {
            let result = interpreter.run(MAIN_LOOP_FREQUENCY);
            match result {
                Ok(_) => {
                    println!("Finishing");
                }
                Err(error) => {
                    println!("Error: while running {}", error);
                }
            }
        },
        Err(error) => {
            println!("Error: while loading file {}: {}", load_file_name, error);
        }
    }
}
