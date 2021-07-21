use std::slice;

pub struct Renderer {
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
        }
    }
}

impl chip8_traits::Renderer for Renderer {
    fn render(&self, memory: slice::Iter<Vec<bool>>) -> Result<(), &'static str> {
        // print!("{}[2J", 27 as char);
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // TODO: change to just move, we're overwriting everyone anyway
        print!("\x33{esc}[1;1H", esc = 27 as char);
        for row in memory {
            for value in row.iter() {
                if *value {
                    print!("🁢");
                } else {
                    print!(" ");
                }
            }
            print!("\n\r");
        }

        Ok(())
    }
}