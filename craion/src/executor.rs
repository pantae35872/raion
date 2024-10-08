use std::collections::HashMap;

use common::sin::sections::SinSection;

use crate::{
    decoder::decode,
    memory::{argument_memory::ArgumentMemory, Memory},
    ret_stack::RetStack,
    section_manager::SectionManager,
};

use self::registers::RegisterFile;

pub mod registers;

#[derive(Debug)]
pub struct ExecutorState {
    stack_saved_size: Vec<u64>,
    procedure_arguments: HashMap<u32, u64>,
    exit_code: u64,
}

pub struct Executor {
    memory: Memory,
    register: RegisterFile,
    argument_memory: ArgumentMemory,
    ret_stack: RetStack,
    section_manager: SectionManager,
    state: ExecutorState,
}

impl ExecutorState {
    pub fn new() -> Self {
        Self {
            stack_saved_size: Vec::new(),
            procedure_arguments: HashMap::new(),
            exit_code: 0,
        }
    }

    pub fn save_stack_size(&mut self, size: u64) {
        self.stack_saved_size.push(size);
    }

    pub fn consume_stack_size(&mut self) -> u64 {
        return self.stack_saved_size.pop().unwrap_or(0);
    }

    pub fn load_argument(&mut self, index: u32, value: u64) {
        self.procedure_arguments.insert(index, value);
    }

    pub fn get_argument(&self, index: u32) -> u64 {
        return *self.procedure_arguments.get(&index).unwrap_or(&0);
    }

    pub fn set_exit_code(&mut self, value: u64) {
        self.exit_code = value;
    }
}

impl Executor {
    pub fn new(mem_size: usize) -> Self {
        Self {
            memory: Memory::new(mem_size),
            register: RegisterFile::new(),
            argument_memory: ArgumentMemory::new(),
            ret_stack: RetStack::new(),
            section_manager: SectionManager::new(),
            state: ExecutorState::new(),
        }
    }

    pub fn section_manager(&mut self) -> &mut SectionManager {
        return &mut self.section_manager;
    }

    pub fn registers(&mut self) -> &mut RegisterFile {
        return &mut self.register;
    }

    pub fn registers_ref(&self) -> &RegisterFile {
        return &self.register;
    }

    pub fn load_section(&mut self, section: &SinSection, data: &[u8]) {
        self.section_manager
            .load_section(section, data, &mut self.memory);
    }

    pub fn memory(&mut self) -> &mut Memory {
        return &mut self.memory;
    }

    pub fn memory_ref(&self) -> &Memory {
        return &self.memory;
    }

    pub fn execute(&mut self) {
        while !self.register.get_halt() {
            {
                let mut instruction = match decode(
                    &mut self.memory,
                    &mut self.register,
                    &mut self.argument_memory,
                    &mut self.ret_stack,
                    &mut self.section_manager,
                    &mut self.state,
                ) {
                    Ok(result) => result,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };

                match instruction.execute() {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "Error occur while executing instruction: '{}', opcode: {}, instruction pointer: {}",
                            e,
                            instruction.op_code(),
                            self.register.get_ip()
                        );
                        return;
                    }
                };
            }
        }
        println!("Program exit with exit code {}", self.state.exit_code);
    }

    pub fn debug_register(&self) {
        println!("{:?}", self.register);
    }
}
