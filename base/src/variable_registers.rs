use crate::OutOfBoundsError;

pub struct VariableRegisters {
    value: [u8; 16]
}

impl VariableRegisters {
    pub fn new() -> VariableRegisters {
        VariableRegisters {
            value: [0; 16]
        }
    }

    pub fn reset(&mut self) {
        self.value = [0; 16];
    }

    pub fn get(&self, index: u8) -> Option<u8> {
        self.value.get(index as usize).cloned()
    }

    pub fn set(&mut self, index: u8, value: u8) -> Result<(), OutOfBoundsError> {
        if index as usize >=  self.value.len() {
            Err(OutOfBoundsError::new(index))
        } else {
            self.value[index as usize] = value;
            Ok(())
        }
    } 
}