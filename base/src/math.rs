pub trait Count8 {
    fn count8(&self) -> u8;
}

impl Count8 for [bool; 4] {
    fn count8(&self) -> u8 {
        let mut result: u8 = 0;
    
        let mut place = 1;
        for value in self.iter().rev() {
            if *value {
                result += place;
            }
            
            if place == 128 { // TODO: come up with better solution
                break;
            }
            place *= 2;
        }
    
        result
    }
}

impl Count8 for [bool; 8] {
    fn count8(&self) -> u8 {
        let mut result: u8 = 0;
    
        let mut place = 1;
        for value in self.iter().rev() {
            if *value {
                result += place;
            }
            
            if place == 128 { // TODO: come up with better solution
                break;
            }
            place *= 2;
        }
    
        result
    }
}

pub trait Count16 {
    fn count16(&self) -> u16;
}

impl Count16 for [bool; 12] {
    fn count16(&self) -> u16 {
        let mut result: u16 = 0;

        let mut place = 1;
        for value in self.iter().rev() {
            if *value {
                result += place;
            }
            
            if place == 32768 { // TODO: come up with better solution
                break;
            }
            place *= 2;
        }

        result
    }
}

impl Count16 for [bool; 16] {
    fn count16(&self) -> u16 {
        let mut result: u16 = 0;

        let mut place = 1;
        for value in self.iter().rev() {
            if *value {
                result += place;
            }
            
            if place == 32768 { // TODO: come up with better solution
                break;
            }
            place *= 2;
        }

        result
    }
}

pub fn count8(bits: Vec<bool>) -> u8 {
    let mut result: u8 = 0;

    let mut place = 1;
    for value in bits.iter().rev() {
        if *value {
            result += place;
        }
        
        if place == 128 { // TODO: come up with better solution
            break;
        }
        place *= 2;
    }

    result
}

pub fn count16(bits: Vec<bool>) -> u16 {
    let mut result: u16 = 0;

    let mut place = 1;
    for value in bits.iter().rev() {
        if *value {
            result += place;
        }
        
        if place == 32768 { // TODO: come up with better solution
            break;
        }
        place *= 2;
    }

    result
}