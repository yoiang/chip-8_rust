use crate::{DelayTimer, Memory, ScreenMemory, SoundTimer, Stack, VariableRegisters, count16, count8, instruction::{InstructionError}};

pub struct ExecutionState {
    pub instruction_disassembly: String
}

type ExecuteResult<Instruction> = std::result::Result<ExecutionState, InstructionError<Instruction>>;

/// Execute an instruction, or interpret the instruction if apply_instruction == false
pub fn execute<
    Instruction: chip8_traits::Instruction, 
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>,
    Keypad: chip8_traits::Keypad, 
    Random: chip8_traits::Random
> (
    apply_instruction: bool,
    instruction: Instruction, 
    program_counter: &mut ProgramCounter,
    stack: &mut Stack,
    memory: &mut Memory,
    screen_memory: &mut ScreenMemory,
    variable_registers: &mut VariableRegisters,
    keypad: &Keypad,
    index_register: &mut usize,
    delay_timer: &mut DelayTimer,
    sound_timer: &mut SoundTimer,
    random: &mut Random,
    font_start: usize
) -> ExecuteResult<Instruction> {
    let count = count8(instruction.w().to_vec());

    match count {
        0x00 => {
            let nn = count8(instruction.nn().to_vec());
            match nn {
                0xe0 => return clear_screen(apply_instruction, screen_memory),
                0xee => return pop_stack(apply_instruction, instruction, stack, program_counter),
                _ => return Err(InstructionError::UnsupportedInstructionError(instruction)) // TODO: 0x0nnn
            }
        },
        0x01 => return jump(apply_instruction, instruction, program_counter),
        0x02 => return push_stack(apply_instruction, instruction, stack, program_counter),
        0x03 => return skip_if_equal_value(apply_instruction, instruction, variable_registers, program_counter),
        0x04 => return skip_if_not_equal_to_value(apply_instruction, instruction, variable_registers, program_counter),
        0x05 => return skip_if_equal(apply_instruction, instruction, variable_registers, program_counter),
        0x09 => return skip_if_not_equal(apply_instruction, instruction, variable_registers, program_counter),
        0x06 => return set_register(apply_instruction, instruction, variable_registers),
        0x07 => return add_to_register(apply_instruction, instruction, variable_registers),
        0x08 => {
            let n = count8(instruction.n().to_vec());
            match n {
                0x00 => return set_x_value_of_y(apply_instruction, instruction, variable_registers),
                0x01 => return or_x_value_of_y(apply_instruction, instruction, variable_registers),
                0x02 => return and_x_value_of_y(apply_instruction, instruction, variable_registers),
                0x03 => return xor_x_value_of_y(apply_instruction, instruction, variable_registers),
                0x04 => return add_to_x_value_of_y(apply_instruction, instruction, variable_registers),
                0x05 => return subtract_to_x_value_of_y(apply_instruction, instruction, variable_registers),
                0x06 => return set_x_right_shifted_y(apply_instruction, instruction, variable_registers),
                0x07 => return subtract_to_x_value_of_y_reversed(apply_instruction, instruction, variable_registers),
                0x0e => return set_x_left_shifted_y(apply_instruction, instruction, variable_registers),

                _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
            }
        },
        0x0a => return set_index_register(apply_instruction, instruction, index_register),
        0x0b => return jump_v0(apply_instruction, instruction, program_counter, variable_registers), // TODO: optional BXNN
        0x0c => return set_register_random(apply_instruction, instruction, variable_registers, random),
        0x0d => return display(apply_instruction, instruction, index_register, variable_registers, memory, screen_memory),
        0x0e => {
            let nn = count8(instruction.nn().to_vec());
            match nn {
                0x9e => return skip_if_pressed(apply_instruction, instruction, keypad, variable_registers, program_counter),
                0xa1 => return skip_if_not_pressed(apply_instruction, instruction, keypad, variable_registers, program_counter),
                _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
            }
        },
        0x0f => {
            let nn = count8(instruction.nn().to_vec());
            match nn {
                0x07 | 0x15 | 0x18 => return set_timer(apply_instruction, instruction, variable_registers, delay_timer, sound_timer),
                0x1e => return add_to_index(apply_instruction, instruction, variable_registers, index_register),
                0x0a => return wait_for_key(apply_instruction, instruction, keypad, variable_registers, program_counter),
                0x29 => return font_character(apply_instruction, instruction, variable_registers, index_register, font_start),
                0x33 => return binary_to_decimal(apply_instruction, instruction, variable_registers, memory, index_register),
                0x55 => return register_to_memory(apply_instruction, instruction, variable_registers, memory, *index_register),
                0x65 => return memory_to_register(apply_instruction, instruction, variable_registers, memory, *index_register),
                _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
            }
        },
        _ => return Err(InstructionError::UnsupportedInstructionError(instruction))
    }
}

fn skip_if_equal_value<
    Instruction: chip8_traits::Instruction,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
>(
    apply_instruction: bool,
    instruction: Instruction,
    variable_registers: &mut VariableRegisters,
    program_counter: &mut ProgramCounter
) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let value = count8(instruction.nn().to_vec());

    if apply_instruction {
        match variable_registers.get(x) {
            Some(register_value) => {
                if value == register_value {
                    program_counter.skip();
                }
            },
            None => return Err(InstructionError::InstructionExecuteError(instruction)),
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("skip if {:#04x} == V{}", value, x)
    })
}

fn skip_if_not_equal_to_value<
    Instruction: chip8_traits::Instruction,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
>(
    apply_instruction: bool,
    instruction: Instruction,
    variable_registers: &mut VariableRegisters,
    program_counter: &mut ProgramCounter
) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let value = count8(instruction.nn().to_vec());

    if apply_instruction {
        match variable_registers.get(x) {
            Some(register_value) => {
                if value != register_value {
                    program_counter.skip();
                }
            },
            None => return Err(InstructionError::InstructionExecuteError(instruction)),
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("skip if {:#04x} != V{}", value, x)
    })
}

fn skip_if_equal<
    Instruction: chip8_traits::Instruction,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
>(
    apply_instruction: bool,
    instruction: Instruction,
    variable_registers: &mut VariableRegisters,
    program_counter: &mut ProgramCounter
) -> ExecuteResult<Instruction> {
    let count = count8(instruction.w().to_vec());
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
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
    }

    return Ok(ExecutionState {
        instruction_disassembly: format!("skip if V{} == V{}", x, y)
    })
}

fn skip_if_not_equal<
    Instruction: chip8_traits::Instruction,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
>(
    apply_instruction: bool,
    instruction: Instruction,
    variable_registers: &mut VariableRegisters,
    program_counter: &mut ProgramCounter
) -> ExecuteResult<Instruction> {
    let count = count8(instruction.w().to_vec());
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
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
    }

    return Ok(ExecutionState {
        instruction_disassembly: format!("skip if V{} == V{}", x, y)
    })
}

/// Sets entire screen memory to 0x00
fn clear_screen<
    Instruction: chip8_traits::Instruction,
    ScreenMemory: chip8_traits::ScreenMemory
> (apply_instruction: bool, screen_memory: &mut ScreenMemory) -> ExecuteResult<Instruction> {
    if apply_instruction {
        screen_memory.clear();
    }

    Ok(ExecutionState {
        instruction_disassembly: "clear screen".to_string()
    })
}

/// Pushes the location after the "Push Stack" instruction into the stack and sets the program counter to the location NNN 
fn push_stack<
    Instruction: chip8_traits::Instruction,
    Stack: chip8_traits::Stack,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
> (apply_instruction: bool, instruction: Instruction, stack: &mut Stack, program_counter: &mut ProgramCounter) -> ExecuteResult<Instruction> {
    let new_position = count16(instruction.nnn().to_vec());

    if apply_instruction {
        stack.push(program_counter.get_position());
        program_counter.set_position(new_position as usize);
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("push stack and jump to {:#06x}", new_position) 
    })
}

/// Sets the program counter to the location on the top of the stack and removes it from the stack
fn pop_stack<
    Instruction: chip8_traits::Instruction,
    Stack: chip8_traits::Stack,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
> (apply_instruction: bool, instruction: Instruction, stack: &mut Stack, program_counter: &mut ProgramCounter) -> ExecuteResult<Instruction> {
    if apply_instruction {
        let result = stack.pop();
        match result {
            Some(value) => {
                program_counter.set_position(value);
            }
            None => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("pop stack") 
    })
}

/// Set program counter to NNN
fn jump<
    Instruction: chip8_traits::Instruction, 
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
>(apply_instruction: bool, instruction: Instruction, program_counter: &mut ProgramCounter) -> ExecuteResult<Instruction>  
{
    let nnn = count16(instruction.nnn().to_vec());
    if apply_instruction {
        program_counter.set_position(nnn as usize);
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("jump to {:#06x}", nnn) 
    })
} 

/// Set to VX value NN
fn set_register<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let index = count8(instruction.x().to_vec());
    let value = count8(instruction.nn().to_vec());

    if apply_instruction {
        match variable_registers.set(index, value) {
            Ok(_) => {},
            Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} = {:#04x} / {}", index, value, value) 
    })
}

/// Add to VX value NN
fn add_to_register<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let value = count8(instruction.nn().to_vec());

    if apply_instruction {
        match variable_registers.get(x) {
            Some(x_value) => {
                let new_value = x_value.wrapping_add(value);
                match variable_registers.set(x, new_value) {
                    Ok(_) => {},
                    Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                }
            },
            None => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }
    Ok(ExecutionState {
        instruction_disassembly: format!("V{} += {:#04x} / {} wrapped", x, value, value) 
    })
}

fn set_x_value_of_y<
    Instruction: chip8_traits::Instruction
> (apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {

        match variable_registers.get(y) {
            Some(y_value) => {
                match variable_registers.set(x, y_value) {
                    Ok(_) => {},
                    Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                }
            },
            None => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} = V{}", x, y) 
    })
}

fn or_x_value_of_y<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
        guard!(let Some(y_value) = variable_registers.get(y) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        guard!(let Some(x_value) = variable_registers.get(x) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
        let new_value = x_value | y_value;
    
        match variable_registers.set(x, new_value) {
            Ok(_) => {},
            Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }
    
    Ok(ExecutionState {
        instruction_disassembly: format!("V{} = V{} OR V{}", x, x, y) 
    })
}


fn and_x_value_of_y<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
        match variable_registers.get(y) {
            Some(y_value) => {
                match variable_registers.get(x) {
                    Some(x_value) => {
                        let new_value = x_value & y_value;
            
                        match variable_registers.set(x, new_value) {
                            Ok(_) => {},
                            Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                        }
                    },
                    None => return Err(InstructionError::InstructionExecuteError(instruction))
                }
            },
            None => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} = V{} AND V{}", x, x, y) 
    })
}

fn xor_x_value_of_y<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
        guard!(let Some(x_value) = variable_registers.get(x) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        guard!(let Some(y_value) = variable_registers.get(y) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        let new_value = x_value ^ y_value;
            
        match variable_registers.set(x, new_value) {
            Ok(_) => {},
            Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} = V{} XOR V{}", x, x, y) 
    })
}

fn add_to_x_value_of_y<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
        match variable_registers.get(y) {
            Some(y_value) => {
                match variable_registers.get(x) {
                    Some(x_value) => {
                        let result = x_value.overflowing_add(y_value);
                        if result.1 {
                            match variable_registers.set(x, result.0) {
                                Ok(_) => {
                                    match variable_registers.set(0x0f, 1) {
                                        Ok(_) => {},
                                        Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                                    }
                                },
                                Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                            }
                        } else {
                            match variable_registers.set(0x0f, 0) {
                                Ok(_) => {},
                                Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                            }
                        }
                    },
                    None => return Err(InstructionError::InstructionExecuteError(instruction))
                }
            },
            None => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} += V{}, overflow in VF", x, y) 
    })
}

fn subtract_to_x_value_of_y<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
        match variable_registers.get(y) {
            Some(y_value) => {
                match variable_registers.get(x) {
                    Some(x_value) => {
                        let result = x_value.overflowing_sub(y_value);
                        if result.1 {
                            match variable_registers.set(x, result.0) {
                                Ok(_) => {
                                    match variable_registers.set(0x0f, 0) {
                                        Ok(_) => {},
                                        Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                                    }
                                },
                                Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                            }
                        } else {
                            match variable_registers.set(0x0f, 1) {
                                Ok(_) => {},
                                Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                            }
                        }
                    },
                    None => return Err(InstructionError::InstructionExecuteError(instruction))
                }
            },
            None => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} -= V{}, underflow in VF", x, y) 
    })
}

fn set_x_right_shifted_y<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
        guard!(let Some(y_value) = variable_registers.get(y) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        let flag = {
            if y_value & 0x01 > 0 {
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
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} = V{} >> 1, overflow in VF", x, y) 
    })
}

fn subtract_to_x_value_of_y_reversed<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
        match variable_registers.get(y) {
            Some(y_value) => {
                match variable_registers.get(x) {
                    Some(x_value) => {
                        let result = y_value.overflowing_sub(x_value);
                        if result.1 {
                            match variable_registers.set(x, result.0) {
                                Ok(_) => {
                                    match variable_registers.set(0x0f, 0) {
                                        Ok(_) => {},
                                        Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                                    }
                                },
                                Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                            }
                        } else {
                            match variable_registers.set(x, y_value) {
                                Ok(_) => {
                                    match variable_registers.set(0x0f, 1) {
                                        Ok(_) => {},
                                        Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                                    }
                                },
                                Err(_) => return Err(InstructionError::InstructionExecuteError(instruction))
                            }
                        }
                    },
                    None => return Err(InstructionError::InstructionExecuteError(instruction))
                }
            },
            None => return Err(InstructionError::InstructionExecuteError(instruction))
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} = V{} - V{}, underflow in VF", x, y, x) 
    })
}

fn set_x_left_shifted_y<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let y = count8(instruction.y().to_vec());

    if apply_instruction {
        guard!(let Some(y_value) = variable_registers.get(y) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        let flag = {
            if y_value & 0x0f > 0 {
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
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} = V{} << 1, overflow in VF", x, y) 
    })
}

fn set_index_register<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, index_register: &mut usize) -> ExecuteResult<Instruction> {
    let value = count16(chip8_traits::Instruction::nnn(&instruction).to_vec());
    if apply_instruction {
        (*index_register) = value as usize;
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("I = {:#06x}", value) 
    })
}

fn jump_v0<
    Instruction: chip8_traits::Instruction,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
>(apply_instruction: bool, instruction: Instruction, program_counter: &mut ProgramCounter, variable_registers: &VariableRegisters) -> ExecuteResult<Instruction> {
    let value = count16(chip8_traits::Instruction::nnn(&instruction).to_vec());

    if apply_instruction {
        guard!(let Some(x_value) = variable_registers.get(0) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        (program_counter as &mut dyn chip8_traits::ProgramCounter<Instruction>).set_position(value as usize + x_value as usize);
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("jump V0 + {:#06x}", {value}) 
    })
}

fn set_register_random<
    Instruction: chip8_traits::Instruction,
    Random: chip8_traits::Random
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters, random: &mut Random)-> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    let value = count8(instruction.nn().to_vec());

    if apply_instruction {
        let random_value = random.value();
        if let Err(_) = variable_registers.set(x, random_value & value) {
            return Err(InstructionError::InstructionExecuteError(instruction));
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V{} = random AND {:#04x}", x, value) 
    })
}

fn display<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, index_register: &usize, variable_registers: &mut VariableRegisters, memory: &Memory, screen_memory: &mut ScreenMemory) -> ExecuteResult<Instruction> {
    let vx = count8(instruction.x().to_vec());
    let vy = count8(instruction.y().to_vec());
    let n = count8(chip8_traits::Instruction::n(&instruction).to_vec());
    
    if apply_instruction {
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
        } else {
            guard!(let Ok(_) = variable_registers.set(0x0f, 0) else {
                return Err(InstructionError::InstructionExecuteError(instruction));
            });
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("display (V{}, V{}) -> 5x{:#04x}, flip in VF", vx, vy, n) 
    })
}

fn skip_if_pressed<
    Instruction: chip8_traits::Instruction,
    Keypad: chip8_traits::Keypad, 
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
>(
    apply_instruction: bool, 
    instruction: Instruction, 
    keypad: &Keypad, 
    variable_registers: &VariableRegisters, 
    program_counter: &mut ProgramCounter
) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    
    if apply_instruction {
        guard!(let Some(x_value) = variable_registers.get(x) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        if keypad.key_state(x_value as usize) {
            program_counter.skip();
        }
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("skip if key V{} down", x) 
    })
}

fn skip_if_not_pressed<
    Instruction: chip8_traits::Instruction,
    Keypad: chip8_traits::Keypad, 
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
>(
    apply_instruction: bool, 
    instruction: Instruction, 
    keypad: &Keypad, 
    variable_registers: &VariableRegisters, 
    program_counter: &mut ProgramCounter
) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    
    if apply_instruction {
        guard!(let Some(x_value) = variable_registers.get(x) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        if keypad.key_state(x_value as usize) == false {
            program_counter.skip();
        }    
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("skip if key V{} up", x) 
    })
}

fn set_timer<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters, delay_timer: &mut DelayTimer, sound_timer: &mut SoundTimer) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());

    match count8(instruction.nn().to_vec()) {
        0x07 => {
            if apply_instruction {
                let value = (delay_timer as &mut dyn chip8_traits::Timer).get();

                if let Err(_) = variable_registers.set(x, value) {
                    return Err(InstructionError::InstructionExecuteError(instruction));
                }
            }

            Ok(ExecutionState {
                instruction_disassembly: format!("V{} = delay timer", x) 
            })
        },
        0x15 => {
            if apply_instruction {
                guard!(let Some(value) = variable_registers.get(x) else {
                    return Err(InstructionError::InstructionExecuteError(instruction));
                });

                (delay_timer as &mut dyn chip8_traits::Timer).set(value);
            }

            Ok(ExecutionState {
                instruction_disassembly: format!("set delay timer V{}", x) 
            })
        },
        0x18 => {
            if apply_instruction {
                guard!(let Some(value) = variable_registers.get(x) else {
                    return Err(InstructionError::InstructionExecuteError(instruction));
                });

                (sound_timer as &mut dyn chip8_traits::Timer).set(value);
            }

            Ok(ExecutionState {
                instruction_disassembly: format!("set sound timer V{}", x) 
            })
        }
        _ => return Err(InstructionError::InstructionExecuteError(instruction))
    }
}

fn add_to_index<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &VariableRegisters, index_register: &mut usize) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());

    if apply_instruction {
        guard!(let Some(x_value) = variable_registers.get(x) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
        // TODO: Amiga intepreter marks overflow option
        (*index_register) += x_value as usize;
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("I += V{}", x) 
    })
}

fn wait_for_key<
    Instruction: chip8_traits::Instruction,
    ProgramCounter: chip8_traits::ProgramCounter<Instruction>
>(apply_instruction: bool, instruction: Instruction, keypad: &dyn chip8_traits::Keypad, variable_registers: &mut VariableRegisters, program_counter: &mut ProgramCounter) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());

    if apply_instruction {
        let keypad_state = keypad.state();
        
        let mut key_down = false;
        for index in 0..keypad_state.len() {
            if keypad_state[index] {
                let result = variable_registers.set(x, index as u8);
                if let Err(_) = result {
                    return Err(InstructionError::InstructionExecuteError(instruction));
                }

                key_down = true;
                break;
            }
        }

        if !key_down {
            (program_counter as &mut dyn chip8_traits::ProgramCounter<Instruction>).go_back();
        }
    }


    Ok(ExecutionState {
        instruction_disassembly: format!("wait for any key down, set key to V{}", x) 
    })
}

fn font_character<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &VariableRegisters, index_register: &mut usize, font_start: usize) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());

    if apply_instruction {
        guard!(let Some(x_value) = variable_registers.get(x) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        // TODO: find better place to keep this '5' size
        (*index_register) = font_start + (x_value * 5) as usize;    
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("I = font_start + V{}", x) 
    })
}

fn binary_to_decimal<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &VariableRegisters, memory: &mut Memory, index_register: &usize) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());

    if apply_instruction {
        guard!(let Some(x_value) = variable_registers.get(x) else {
            return Err(InstructionError::InstructionExecuteError(instruction));
        });
    
        (memory as &mut dyn chip8_traits::Memory).set(*index_register, x_value / 100);
        (memory as &mut dyn chip8_traits::Memory).set(*index_register + 1, x_value % 100 / 10);
        (memory as &mut dyn chip8_traits::Memory).set(*index_register + 2, x_value % 10);
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("Memory[I..I+2] = V{}", x) 
    })
}

fn register_to_memory<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &VariableRegisters, memory: &mut Memory, index_register: usize) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    // TODO: option to incriment I while working

    if apply_instruction {
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
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("Memory[I..I + {}] = V0..V{}", x, x) 
    })
}

fn memory_to_register<
    Instruction: chip8_traits::Instruction
>(apply_instruction: bool, instruction: Instruction, variable_registers: &mut VariableRegisters, memory: &Memory, index_register: usize) -> ExecuteResult<Instruction> {
    let x = count8(instruction.x().to_vec());
    // TODO: option to incriment I while working

    if apply_instruction {
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
    }

    Ok(ExecutionState {
        instruction_disassembly: format!("V0..V{} = Memory[I..I + {}]", x, x) 
    })
}

#[cfg(test)]
mod cpu_tests {
    use crate::{Instruction, ProgramCounter, ScreenMemory, Stack, VariableRegisters, cpu::add_to_register};

    use super::{clear_screen, jump, pop_stack, push_stack, set_register};


    // TODO: test execute

    // TODO: test skip
    // #[test]
    // fn skip_test() {
    //     let instruction: Instruction = Instruction::new(first, second)
    // } 

    fn is_empty(screen_memory: &ScreenMemory) -> bool {
        for value in screen_memory.iter() {
            if value[0] {
                return false;
            } 
        }
        return true;
    }

    #[test]
    fn clear_screen_test() {
        let mut screen_memory = ScreenMemory::new(10, 10);
        chip8_traits::ScreenMemory::display(&mut screen_memory, 0, 0, [0xff, 0x13, 0x53, 0x5a].iter(), 4);

        assert_eq!(is_empty(&screen_memory), false);
        
        clear_screen::<crate::Instruction, crate::ScreenMemory>(false, &mut screen_memory);

        assert_eq!(is_empty(&screen_memory), false);

        clear_screen::<crate::Instruction, crate::ScreenMemory>(true, &mut screen_memory);
        
        assert_eq!(is_empty(&screen_memory), true);
    }

    #[test]
    fn push_stack_test() {
        let mut stack = Stack::new();
        let mut program_counter = ProgramCounter::new();

        assert_eq!(stack.is_empty(), true);
        assert_eq!(chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter), 0);

        push_stack(false, Instruction::new(0x01, 0x23), &mut stack, &mut program_counter);

        assert_eq!(stack.is_empty(), true);
        assert_eq!(chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter), 0);

        push_stack(true, Instruction::new(0x01, 0x23), &mut stack, &mut program_counter);

        assert_eq!(stack.is_empty(), false);
        assert_eq!(chip8_traits::Stack::pop(&mut stack), Some(0x0));

        assert_eq!(chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter), 0x0123);

        push_stack(true, Instruction::new(0x14, 0x56), &mut stack, &mut program_counter);

        assert_eq!(stack.is_empty(), false);
        assert_eq!(chip8_traits::Stack::pop(&mut stack), Some(0x0123));

        assert_eq!(chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter), 0x0456);
    }

    #[test]
    fn pop_stack_test() {
        let mut stack = Stack::new();
        let mut program_counter = ProgramCounter::new();

        assert_eq!(stack.is_empty(), true);
        assert_eq!(chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter), 0);

        let first_program_counter_position = chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter);

        push_stack(true, Instruction::new(0x01, 0x23), &mut stack, &mut program_counter);

        push_stack(true, Instruction::new(0x14, 0x56), &mut stack, &mut program_counter);

        let pre_pop_program_counter_position = chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter);

        assert_eq!(stack.is_empty(), false);

        pop_stack(false, Instruction::new(0x00, 0x00), &mut stack, &mut program_counter);

        assert_eq!(pre_pop_program_counter_position, chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter));

        pop_stack(false, Instruction::new(0x00, 0x00), &mut stack, &mut program_counter);

        assert_eq!(stack.is_empty(), false);

        pop_stack(true, Instruction::new(0x00, 0x00), &mut stack, &mut program_counter);

        assert_eq!(0x0123, chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter));

        assert_eq!(stack.is_empty(), false);

        pop_stack(true, Instruction::new(0x00, 0x00), &mut stack, &mut program_counter);

        assert_eq!(first_program_counter_position, chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter));

        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn jump_test() {
        let mut program_counter = ProgramCounter::new();

        let original_program_counter_position = chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter);

        jump(false, Instruction::new(0x01, 0x23), &mut program_counter);

        assert_eq!(original_program_counter_position, chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter));

        jump(true, Instruction::new(0x01, 0x23), &mut program_counter);

        assert_eq!(0x0123, chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter));
    }

    fn test_set_variable_register_at_index(index: u8, variable_registers: &mut VariableRegisters) {
        let original_value = variable_registers.get(index);

        let nn: u8 = 0x12; // TODO: random value
        let instruction = Instruction::new(index, nn);

        set_register(false, instruction, variable_registers);

        assert_eq!(original_value, variable_registers.get(index));

        set_register(true, instruction, variable_registers);

        assert_eq!(Some(nn), variable_registers.get(index));
    }

    #[test]
    fn set_register_test() {
        let mut variable_registers = VariableRegisters::new();

        test_set_variable_register_at_index(0, &mut variable_registers);

        test_set_variable_register_at_index(5, &mut variable_registers); // TODO: random index
    }

    fn test_add_to_register_at_index(index: u8, variable_registers: &mut VariableRegisters) {
        let original_value = variable_registers.get(index);

        let nn: u8 = 0x12; // TODO: random value
        let instruction = Instruction::new(index, nn);

        add_to_register(false, instruction, variable_registers);

        assert_eq!(original_value, variable_registers.get(index));

        add_to_register(true, instruction, variable_registers);

        assert_eq!(Some(nn), variable_registers.get(index));

        add_to_register(true, instruction, variable_registers);

        assert_eq!(Some(nn * 2), variable_registers.get(index));

        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);
        add_to_register(true, instruction, variable_registers);

        assert_eq!(Some(nn.wrapping_mul(17)), variable_registers.get(index));
    }

    #[test]
    fn add_to_register_test() {
        let mut variable_registers = VariableRegisters::new();

        test_add_to_register_at_index(3, &mut variable_registers);

        test_add_to_register_at_index(8, &mut variable_registers); // TODO: random index
    }
}
