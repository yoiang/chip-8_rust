use std::{cell::RefCell, rc::Rc};

extern crate yacurses;
use yacurses::Curses;

pub struct Keypad {
    curses: Rc<RefCell<Curses>>
}

impl Keypad {
    pub fn new(curses: &Rc<RefCell<Curses>>) -> Keypad {
        curses.borrow_mut().set_timeout(0);
        Keypad {
            curses: curses.to_owned()
        }
    }
}

// TODO:
/*
1	2	3	C
4	5	6	D
7	8	9	E
A	0	B	F

1	2	3	4
Q	W	E	R
A	S	D	F
Z	X	C	V
*/

impl crate::traits::Keypad for Keypad {
    fn state(&mut self) -> (u8, u8) {
        let events = self.curses.borrow_mut().poll_events();
        match events {
            None => return (0, 0),
            Some(value) => {
                match value {
                    // CursesKey::Ascii(ascii_value) => {
                        
                    // }
                    _ => return (0, 0)
                }
            }
        }
    }
}