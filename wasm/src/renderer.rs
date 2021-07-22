use std::{cell::RefCell, fmt, rc::Rc, slice};

pub struct Renderer {
    rendered_memory: Rc<RefCell<Vec<Vec<bool>>>>
}

impl Renderer {
    pub fn new(rendered_memory: Rc<RefCell<Vec<Vec<bool>>>>) -> Renderer {
        Renderer {
            rendered_memory
        }
    }
}

impl chip8_traits::Renderer for Renderer {
    fn render(&mut self, memory: slice::Iter<Vec<bool>>) -> Result<(), &'static str> {
        let mut rendered_contents = self.rendered_memory.borrow_mut();
        for (row_index, row) in memory.enumerate() {
            if row_index == rendered_contents.len() {
                rendered_contents.push(vec![]);
            }

            for (column_index, value) in row.iter().enumerate() {
                if column_index == rendered_contents[row_index].len() {
                    rendered_contents[row_index].push(false);
                }
                if *value {
                    rendered_contents[row_index][column_index] = true;
                } else {
                    rendered_contents[row_index][column_index] = false;
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_rendered_memory(self.rendered_memory.borrow().as_ref(), f)
    }
}

pub fn fmt_rendered_memory(rendered_memory: &Vec<Vec<bool>>, f: &mut fmt::Formatter) -> fmt::Result {
    for row in rendered_memory.iter() {
        for &value in row {
            let character = {
                if value {
                    '◼'
                } else {
                    ' '
                }
            };
            write!(f, "{}", character)?;
        }
        write!(f, "\n")?;
    }

    Ok(())
}