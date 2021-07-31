mod cpu_tests {
    use std::borrow::BorrowMut;
    use std::slice::Iter;

    use chip8_base::cpu::ExecutionState;
    use chip8_base::instruction::InstructionError;
    use chip8_base::{Instruction, Interpreter};

    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        Renderer {}
        impl chip8_traits::Renderer for Renderer {
            fn render<'a>(&mut self, memory: Iter<'a, Vec<bool>>) -> Result<(), &'static str> ;
        }
    }

    mock! {
        Keypad {}
        impl chip8_traits::Keypad for Keypad {
            fn state(&self) -> [bool; 16];
            fn key_state(&self, key_index: usize) -> bool;
        }
    }

    mock! {
        Random {}
        impl chip8_traits::Random for Random {
            fn value(&mut self) -> u8;
        }
    }

    fn assert_execution_result(result: Result<ExecutionState, InstructionError<Instruction>>) {
        match result {
            Ok(value) => {
                assert_ne!(value.instruction_disassembly.len(), 0);
            },
            Err(error) => {
                panic!("{}", error);
            }
        }
    }

    #[test]
    fn clear_screen_test() {
        let instruction = Instruction::new(0x00, 0xe0);

        let mut interpreter = Interpreter::new_crate_defaults(MockRenderer::new(), MockKeypad::new(), MockRandom::new());
        let mut bus = interpreter.create_bus();
        let font_start: usize = 200;

        (bus.screen_memory.borrow_mut() as &mut dyn chip8_traits::ScreenMemory).display(0, 0, [0xff, 0x13, 0x53, 0x5a].iter(), 4);

        assert_eq!(bus.screen_memory.is_empty(), false);

        let result = bus.execute(false, instruction, font_start);
        assert_execution_result(result);

        assert_eq!(bus.screen_memory.is_empty(), false);
        
        let result = bus.execute(true, instruction, font_start);
        assert_execution_result(result);

        assert_eq!(bus.screen_memory.is_empty(), true);
    }

    #[test]
    fn pop_stack_test() {
        let instruction = Instruction::new(0x00, 0xee);

        let mut interpreter = Interpreter::new_crate_defaults(MockRenderer::new(), MockKeypad::new(), MockRandom::new());
        let mut bus = interpreter.create_bus();
        let font_start: usize = 200;

        assert_eq!(bus.stack.is_empty(), true);
        assert_eq!(chip8_traits::ProgramCounter::get_position(bus.program_counter), 0);

        let first_program_counter_position = chip8_traits::ProgramCounter::get_position(bus.program_counter);

        let _ = bus.execute(true, Instruction::new(0x21, 0x23), font_start);
        let _ = bus.execute(true, Instruction::new(0x24, 0x56), font_start);

        let pre_pop_program_counter_position = chip8_traits::ProgramCounter::get_position(bus.program_counter);

        assert_eq!(bus.stack.is_empty(), false);

        let result = bus.execute(false, instruction, font_start);
        assert_execution_result(result);

        assert_eq!(pre_pop_program_counter_position, chip8_traits::ProgramCounter::get_position(bus.program_counter));

        let result = bus.execute(false, instruction, font_start);
        assert_execution_result(result);

        assert_eq!(bus.stack.is_empty(), false);

        let result = bus.execute(true, instruction, font_start);
        assert_execution_result(result);

        assert_eq!(0x0123, chip8_traits::ProgramCounter::get_position(bus.program_counter));

        assert_eq!(bus.stack.is_empty(), false);

        let result = bus.execute(true, instruction, font_start);
        assert_execution_result(result);

        assert_eq!(first_program_counter_position, chip8_traits::ProgramCounter::get_position(bus.program_counter));

        assert_eq!(bus.stack.is_empty(), true);
    }

/*
    #[test]
    fn jump_test() {
        let mut program_counter = ProgramCounter::new();

        let original_program_counter_position = chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter);

        jump(false, Instruction::new(0x01, 0x23), &mut program_counter);

        assert_eq!(original_program_counter_position, chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter));

        jump(true, Instruction::new(0x01, 0x23), &mut program_counter);

        assert_eq!(0x0123, chip8_traits::ProgramCounter::<Instruction>::get_position(&program_counter));
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
    fn skip_if_equal_value_test() {

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
     */
}