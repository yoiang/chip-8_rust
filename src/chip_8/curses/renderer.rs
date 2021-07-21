use std::{cell::RefCell, rc::Rc, slice};

extern crate yacurses;
use yacurses::Curses;

pub struct Renderer {
    curses: Rc<RefCell<Curses>>
}

impl Renderer {
    pub fn new(curses: &Rc<RefCell<Curses>>) -> Renderer {
        Renderer {
            curses: curses.to_owned()
        }
    }
}

impl crate::traits::Renderer for Renderer {
    fn render(&self, memory: slice::Iter<Vec<bool>>) -> Result<(), &'static str> {
        let result = self.curses.borrow_mut().clear();
        if let Err(error) = result {
            return Err(error);
        }

        print!("{}[2J", 27 as char);
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for row in memory {
            for value in row.iter() {
                if *value {
                    let result = self.curses.borrow_mut().print_ch('🁢');
                    if let Err(error) = result {
                        return Err(error);
                    }
                } else {
                    let result = self.curses.borrow_mut().print_ch(' ');
                    if let Err(error) = result {
                        return Err(error);
                    }
                }
                let result = self.curses.borrow_mut().print_ch('\n');
                if let Err(error) = result {
                    return Err(error);
                }
                let result = self.curses.borrow_mut().print_ch('\r');
                if let Err(error) = result {
                    return Err(error);
                }
            }
        }

        Ok(())
    }
}