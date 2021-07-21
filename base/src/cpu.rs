use crate::{Instruction, Memory, ProgramCounter, ScreenMemory, Stack, VariableRegisters, instruction::{InstructionError}};

type InstructionResult<T, Instruction: chip8_traits::Instruction> = std::result::Result<T, InstructionError<Instruction>>;
type ExecuteResult = InstructionResult<(), Instruction>;

pub fn execute<Random: chip8_traits::Random> (
    instruction: Instruction, 
    program_counter: &mut ProgramCounter,
    stack: &mut Stack,
    memory: &mut Memory,
    screen_memory: &mut ScreenMemory,
    variable_registers: &mut VariableRegisters,
    index_register: &mut usize,
    random: &mut Random,
    font_start: usize
) -> ExecuteResult {

    let count = chip8_traits::count8(chip8_traits::Instruction::w(&instruction).to_vec());

    match count {
        0x00 => {
            let nn = chip8_traits::count8(chip8_traits::Instruction::nn(&instruction).to_vec());
            match nn {
                0xe0 => clear_screen(screen_memory),
                0xee => pop_stack(stack, program_counter),
                _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
            }
        },
        0x01 => jump(instruction, program_counter),
        0x02 => push_stack(instruction, stack, program_counter),
        0x03 | 0x04 | 0x05 | 0x09 => return skip(instruction, variable_registers, program_counter),
        0x06 => return set_register(instruction, variable_registers),
        0x07 => return add_to_register(instruction, variable_registers),
        0x08 => {
            let n = chip8_traits::count8(chip8_traits::Instruction::n(&instruction).to_vec());
            match n {
                0x00 => return set_x_value_of_y(instruction, variable_registers),
                0x01 => return or_x_value_of_y(instruction, variable_registers),
                0x02 => return and_x_value_of_y(instruction, variable_registers),
                0x03 => return xor_x_value_of_y(instruction, variable_registers),
                0x04 => return add_to_x_value_of_y(instruction, variable_registers),
                0x05 => return subtract_to_x_value_of_y(instruction, variable_registers),
                0x07 => return subtract_to_x_value_of_y_reversed(instruction, variable_registers),

                // 0x06 => {}, // TODO:
                // 0x0E => {}, 
                _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
            }
        },
        0x0A => set_index_register(instruction, index_register),
        // 0x0B => {}, // TODO:
        0x0C => return set_register_random(instruction, variable_registers, random),
        0x0D => return display(instruction, index_register, variable_registers, memory, screen_memory),
        // 0x0E => {}, // TODO:
        0x0F => {
            let nn = chip8_traits::count8(chip8_traits::Instruction::nn(&instruction).to_vec());
            match nn {
                // 0x07 | 0x15 | 0x18 => self.set_timer(instruction),
                // 0x1e => self.add_to_index(instruction),
                // 0x0a => self.get_key(instruction),
                0x29 => return font_character(instruction, variable_registers, index_register, font_start),
                // 0x33 => self.binary_to_decimal(instruction),
                // 0x55 => self.register_to_memory(instruction),
                // 0x65 => self.memory_to_register(instruction),
                _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
            }
        },
        _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
    }
    Ok(())
}

fn skip<ProgramCounter: chip8_traits::ProgramCounter<Instruction>>(
    instruction: Instruction,
    variable_registers: &mut VariableRegisters,
    program_counter: &mut ProgramCounter
) -> ExecuteResult {

    let count = chip8_traits::count8(chip8_traits::Instruction::w(&instruction).to_vec());
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    match count {
        0x3 => {
            let value = chip8_traits::count8(chip8_traits::Instruction::nn(&instruction).to_vec());
            match variable_registers.get(x) {
                Some(register_value) => {
                    if value == register_value {
                        program_counter.skip();
                    }
                },
                None => return Err(InstructionError::InstructionExecuteError(instruction)),
            }
        },
        0x4 => {
            let value = chip8_traits::count8(chip8_traits::Instruction::nn(&instruction).to_vec());
            match variable_registers.get(x) {
                Some(register_value) => {
                    if value != register_value {
                        program_counter.skip();
                    }
                },
                None => return Err(InstructionError::InstructionExecuteError(instruction)),
            }
        },
        0x5 => {
            let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());
            match variable_registers.get(x) {
                Some(x_value) => {
                    match variable_registers.get(y) {
                        Some(y_value) => {
                            if x_value == y_value {
                                program_counter.skip();
                            }
                        },
                        None => return Err(InstructionError::InstructionExecuteError(instruction)),
                    }
                },
                None => return Err(InstructionError::InstructionExecuteError(instruction)),
            }
        },
        0x9 => {
            let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());
            match variable_registers.get(x) {
                Some(x_value) => {
                    match variable_registers.get(y) {
                        Some(y_value) => {
                            if x_value != y_value {
                                program_counter.skip();
                            }
                        },
                        None => return Err(InstructionError::InstructionExecuteError(instruction)),
                    }
                },
                None => return Err(InstructionError::InstructionExecuteError(instruction)),
            }
        },
        _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
    }
    Ok(())
}

fn clear_screen<ScreenMemory>(screen_memory: &mut ScreenMemory) where ScreenMemory: chip8_traits::ScreenMemory {
    screen_memory.clear();
}

fn push_stack<Instruction, Stack, ProgramCounter>(instruction: Instruction, stack: &mut Stack, program_counter: &mut ProgramCounter)
where Instruction: chip8_traits::Instruction,
    Stack: chip8_traits::Stack,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
        {
    stack.push(program_counter.get_position());
    let count = chip8_traits::count16(instruction.nnn().to_vec());
    program_counter.set_position(count as usize);
}

fn pop_stack<Instruction, Stack, ProgramCounter>(stack: &mut Stack, program_counter: &mut ProgramCounter) 
where Instruction: chip8_traits::Instruction,
    Stack: chip8_traits::Stack,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
        {
    let result = stack.pop();
    match result {
        Some(value) =>  program_counter.set_position(value),
        None => {
            // TODO: return ERR
            println!("Stack underflow");
        }
    }
}

fn jump<Instruction, ProgramCounter>(instruction: Instruction, program_counter: &mut ProgramCounter) 
where Instruction: chip8_traits::Instruction,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
{
    let count = chip8_traits::count16(instruction.nnn().to_vec());
    program_counter.set_position(count as usize);
} 

fn set_register(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let index = chip8_traits::count8(chip8_traits::Instruction::x( &instruction).to_vec());
    let value = chip8_traits::count8(chip8_traits::Instruction::nn(&instruction).to_vec());

    match variable_registers.set(index, value) {
        Ok(_) => Ok(()),
        Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn add_to_register(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let index = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let value = chip8_traits::count8(chip8_traits::Instruction::nn(&instruction).to_vec());

    match variable_registers.get(index) {
        Some(index_value) => {
            let new_value = index_value.wrapping_add(value);
            match variable_registers.set(index, new_value) {
                Ok(_) => Ok(()),
                Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
            }
        },
        None => Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn set_x_value_of_y(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());

    match variable_registers.get(y) {
        Some(y_value) => {
            match variable_registers.set(x, y_value) {
                Ok(_) => Ok(()),
                Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
            }
        },
        None => Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn or_x_value_of_y(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());

    match variable_registers.get(y) {
        Some(y_value) => {
            match variable_registers.get(x) {
                Some(x_value) => {
                    let new_value = x_value | y_value;
        
                    match variable_registers.set(x, new_value) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                    }
                },
                None => Err(InstructionError::InstructionExecuteError(instruction))
            }
        },
        None => Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn and_x_value_of_y(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());

    match variable_registers.get(y) {
        Some(y_value) => {
            match variable_registers.get(x) {
                Some(x_value) => {
                    let new_value = x_value & y_value;
        
                    match variable_registers.set(x, new_value) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                    }
                },
                None => Err(InstructionError::InstructionExecuteError(instruction))
            }
        },
        None => Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn xor_x_value_of_y(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());

    match variable_registers.get(y) {
        Some(y_value) => {
            match variable_registers.get(x) {
                Some(x_value) => {
                    let new_value = x_value ^ y_value;
        
                    match variable_registers.set(x, new_value) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                    }
                },
                None => Err(InstructionError::InstructionExecuteError(instruction))
            }
        },
        None => Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn add_to_x_value_of_y(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());

    match variable_registers.get(y) {
        Some(y_value) => {
            match variable_registers.get(x) {
                Some(x_value) => {
                    let result = x_value.overflowing_add(y_value);
                    if result.1 {
                        match variable_registers.set(x, result.0) {
                            Ok(_) => {
                                match variable_registers.set(0x0f, 1) {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                                }
                            },
                            Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                        }
                    } else {
                        match variable_registers.set(0x0f, 0) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                        }
                    }
                },
                None => Err(InstructionError::InstructionExecuteError(instruction))
            }
        },
        None => Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn subtract_to_x_value_of_y(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());

    match variable_registers.get(y) {
        Some(y_value) => {
            match variable_registers.get(x) {
                Some(x_value) => {
                    let result = x_value.overflowing_sub(y_value);
                    if result.1 {
                        match variable_registers.set(x, result.0) {
                            Ok(_) => {
                                match variable_registers.set(0x0f, 0) {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                                }
                            },
                            Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                        }
                    } else {
                        match variable_registers.set(0x0f, 1) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                        }
                    }
                },
                None => Err(InstructionError::InstructionExecuteError(instruction))
            }
        },
        None => Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn subtract_to_x_value_of_y_reversed(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());

    match variable_registers.get(y) {
        Some(y_value) => {
            match variable_registers.get(x) {
                Some(x_value) => {
                    let result = y_value.overflowing_sub(x_value);
                    if result.1 {
                        match variable_registers.set(x, result.0) {
                            Ok(_) => {
                                match variable_registers.set(0x0f, 0) {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                                }
                            },
                            Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                        }
                    } else {
                        match variable_registers.set(x, y_value) {
                            Ok(_) => {
                                match variable_registers.set(0x0f, 1) {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                                }
                            },
                            Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
                        }
                    }
                },
                None => Err(InstructionError::InstructionExecuteError(instruction))
            }
        },
        None => Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn set_index_register(instruction: Instruction, index_register: &mut usize) {
    let value = chip8_traits::count16(chip8_traits::Instruction::nnn(&instruction).to_vec());
    (*index_register) = value as usize;
}

fn set_register_random<Random: chip8_traits::Random>(instruction: Instruction, variable_registers: &mut VariableRegisters, random: &mut Random)-> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let value = chip8_traits::count8(chip8_traits::Instruction::nn(&instruction).to_vec());

    let random_value = random.value();
    if let Err(_) = variable_registers.set(x, random_value & value) {
        return Err(InstructionError::InstructionExecuteError(instruction));
    }
    Ok(())
}

fn display(instruction: Instruction, index_register: &usize, variable_registers: &mut VariableRegisters, memory: &Memory, screen_memory: &mut ScreenMemory) -> ExecuteResult {
    let vx = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let vy = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());
    let n = chip8_traits::count8(chip8_traits::Instruction::n(&instruction).to_vec());
    
    guard!(let Ok(_) = variable_registers.set(0x0f, 0) 
    else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    guard!(let Some(x_value) = variable_registers.get(vx) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    guard!(let Some(y_value) = variable_registers.get(vy) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    if chip8_traits::ScreenMemory::display(screen_memory,
        x_value, 
        y_value, 
        chip8_traits::Memory::get_iter(memory, *index_register), 
        n) {
            guard!(let Ok(_) = variable_registers.set(0x0f, 1) else {
                return Err(InstructionError::InstructionExecuteError(instruction));
            });
    }

    Ok(())
}

fn font_character(instruction: crate::Instruction, variable_registers: &VariableRegisters, index_register: &mut usize, font_start: usize) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    guard!(let Some(x_value) = variable_registers.get(x) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    // TODO: find better place to keep this '5' size
    (*index_register) = font_start + (x_value * 5) as usize;

    Ok(())
}