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