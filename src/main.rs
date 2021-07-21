use std::{time::Duration};

use crate::traits::Interpreter;

mod chip_8;
mod traits;

const DEFAULT_PROGRAM_START: usize = 0x200;
const MAIN_LOOP_FREQUENCY: Duration = Duration::from_millis(1);

fn main() {
    let mut interpreter = chip_8::Interpreter::new();

    let load_file_name = "examples/IBM Logo.ch8";
    let result = interpreter.load(load_file_name, DEFAULT_PROGRAM_START);
    match result {
        Ok(_) => {
            println!("{:?}", interpreter.dump_memory());
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
