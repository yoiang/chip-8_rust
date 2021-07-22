use crate::{DelayTimer, Instruction, Memory, ProgramCounter, ScreenMemory, SoundTimer, Stack, VariableRegisters, instruction::{InstructionError}};

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
    delay_timer: &mut DelayTimer,
    sound_timer: &mut SoundTimer,
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
                _ => return Err(InstructionError::UnsupportedInstructionError(instruction)) // TODO: 0x0nnn
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
                0x06 => return set_x_right_shifted_y(instruction, variable_registers),
                0x07 => return subtract_to_x_value_of_y_reversed(instruction, variable_registers),
                0x0e => return set_x_left_shifted_y(instruction, variable_registers),

                _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
            }
        },
        0x0a => set_index_register(instruction, index_register),
        0x0b => return jump_v0(instruction, program_counter, variable_registers), // TODO: optional BXNN
        0x0c => return set_register_random(instruction, variable_registers, random),
        0x0d => return display(instruction, index_register, variable_registers, memory, screen_memory),
        // 0x0e => {}, // TODO: skip if key
        0x0f => {
            let nn = chip8_traits::count8(chip8_traits::Instruction::nn(&instruction).to_vec());
            match nn {
                0x07 | 0x15 | 0x18 => return set_timer(instruction, variable_registers, delay_timer, sound_timer),
                0x1e => return add_to_index(instruction, variable_registers, index_register),
                // 0x0a => get_key(instruction),
                0x29 => return font_character(instruction, variable_registers, index_register, font_start),
                0x33 => return binary_to_decimal(instruction, variable_registers, memory, index_register),
                0x55 => return register_to_memory(instruction, variable_registers, memory, *index_register),
                0x65 => return memory_to_register(instruction, variable_registers, memory, *index_register),
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

    guard!(let Some(x_value) = variable_registers.get(x) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    guard!(let Some(y_value) = variable_registers.get(y) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    let new_value = x_value ^ y_value;
        
    match variable_registers.set(x, new_value) {
        Ok(_) => Ok(()),
        Err(_) => Err(InstructionError::InstructionExecuteError(instruction))
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

fn set_x_right_shifted_y(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());

    guard!(let Some(y_value) = variable_registers.get(y) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    let flag = {
        if y_value & 1 > 0 {
            1
        } else {
            0
        }
    };

    let y_value = y_value >> 1;
    // TODO: clarify or make an option to assign back shifted to original
    // if let Err(_) = variable_registers.set(y, y_value) {
    //     return Err(InstructionError::InstructionExecuteError(instruction));
    // }
    if let Err(_) = variable_registers.set(x, y_value) {
        return Err(InstructionError::InstructionExecuteError(instruction));
    }
    if let Err(_) = variable_registers.set(0x0f, flag) {
        return Err(InstructionError::InstructionExecuteError(instruction));
    }

    Ok(())
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

fn set_x_left_shifted_y(instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    let y = chip8_traits::count8(chip8_traits::Instruction::y(&instruction).to_vec());

    guard!(let Some(y_value) = variable_registers.get(y) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    let flag = {
        if y_value & 128 > 0 {
            1
        } else {
            0
        }
    };

    let y_value = y_value << 1;
    // TODO: clarify or make an option to assign back shifted to original
    // if let Err(_) = variable_registers.set(y, y_value) {
    //     return Err(InstructionError::InstructionExecuteError(instruction));
    // }
    if let Err(_) = variable_registers.set(x, y_value) {
        return Err(InstructionError::InstructionExecuteError(instruction));
    }
    if let Err(_) = variable_registers.set(0x0f, flag) {
        return Err(InstructionError::InstructionExecuteError(instruction));
    }

    Ok(())
}

fn set_index_register(instruction: Instruction, index_register: &mut usize) {
    let value = chip8_traits::count16(chip8_traits::Instruction::nnn(&instruction).to_vec());
    (*index_register) = value as usize;
}

fn jump_v0(instruction: Instruction, program_counter: &mut ProgramCounter, variable_registers: &VariableRegisters) -> ExecuteResult {
    guard!(let Some(x_value) = variable_registers.get(0) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    let value = chip8_traits::count16(chip8_traits::Instruction::nnn(&instruction).to_vec());
    (program_counter as &mut dyn chip8_traits::ProgramCounter<Instruction>).set_position(value as usize + x_value as usize);

    Ok(())
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

fn set_timer(instruction: crate::Instruction, variable_registers: &mut VariableRegisters, delay_timer: &mut DelayTimer, sound_timer: &mut SoundTimer) -> ExecuteResult {
    let x_value = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());

    match chip8_traits::count8(chip8_traits::Instruction::nn(&instruction).to_vec()) {
        7 => {
            let value = (delay_timer as &mut dyn chip8_traits::Timer).value();

            if let Err(_) = variable_registers.set(x_value, value) {
                return Err(InstructionError::InstructionExecuteError(instruction));
            }
        },
        15 => {
            guard!(let Some(value) = variable_registers.get(x_value) else {
                return Err(InstructionError::InstructionExecuteError(instruction));
            });

            (delay_timer as &mut dyn chip8_traits::Timer).set_value(value);
        },
        18 => {
            guard!(let Some(value) = variable_registers.get(x_value) else {
                return Err(InstructionError::InstructionExecuteError(instruction));
            });

            (sound_timer as &mut dyn chip8_traits::Timer).set_value(value);
        }
        _ => return Err(InstructionError::InstructionExecuteError(instruction))
    }

    Ok(())
}

fn add_to_index(instruction: crate::Instruction, variable_registers: &VariableRegisters, index_register: &mut usize) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    guard!(let Some(x_value) = variable_registers.get(x) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });
    // TODO: Amiga intepreter marks overflow option
    (*index_register) += x_value as usize;

    Ok(())
}

fn font_character(instruction: Instruction, variable_registers: &VariableRegisters, index_register: &mut usize, font_start: usize) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    guard!(let Some(x_value) = variable_registers.get(x) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    // TODO: find better place to keep this '5' size
    (*index_register) = font_start + (x_value * 5) as usize;

    Ok(())
}

fn binary_to_decimal(instruction: Instruction, variable_registers: &VariableRegisters, memory: &mut Memory, index_register: &usize) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    guard!(let Some(x_value) = variable_registers.get(x) else {
        return Err(InstructionError::InstructionExecuteError(instruction));
    });

    (memory as &mut dyn chip8_traits::Memory).set(*index_register, x_value / 100);
    (memory as &mut dyn chip8_traits::Memory).set(*index_register + 1, x_value % 100 / 10);
    (memory as &mut dyn chip8_traits::Memory).set(*index_register + 2, x_value % 10);

    Ok(())
}

fn register_to_memory(instruction: Instruction, variable_registers: &VariableRegisters, memory: &mut Memory, index_register: usize) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    // TODO: option to incriment I while working

    for offset in 0..=x {
        guard!(let Some(offset_value) = variable_registers.get(offset) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
        // if let Err(error) = 
        (memory as &mut dyn chip8_traits::Memory).set(index_register + offset as usize, offset_value);
        // {
        //     return Err(InstructionError::InstructionExecuteError(instruction));
        // }
    }
    Ok(())
}

fn memory_to_register(instruction: Instruction, variable_registers: &mut VariableRegisters, memory: &Memory, index_register: usize) -> ExecuteResult {
    let x = chip8_traits::count8(chip8_traits::Instruction::x(&instruction).to_vec());
    // TODO: option to incriment I while working

    for offset in 0..=x {
        // guard!(let Some(offset_value) = variable_registers.get(offset) else {
        //     return Err(InstructionError::InstructionExecuteError(instruction));
        // });

        // TODO: WTFFFFFFF WITH NEEDINGTHIS CAST
        let offset_value = (memory as &dyn chip8_traits::Memory).get(index_register + offset as usize);

        guard!(let Ok(_) = variable_registers.set(offset, offset_value) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    }

    Ok(())
}