use argument_stack::ArgumentStack;
use common::sin::sections::SinSection;
use local_variables::LocalVariables;
use objects::{type_heap::TYPE_HEAP, Object};
use operand_stack::OperandStack;
use return_stack::ReturnStack;

use crate::{
    decoder::decode,
    memory::{address::Address, argument_memory::ArgumentMemory, Memory},
    section_manager::{LoadedType, SectionManager},
};

pub mod argument_stack;
pub mod local_variables;
pub mod objects;
pub mod operand_stack;
pub mod return_stack;

#[derive(Debug)]
pub struct ProgramState {
    pub ip: Address,
    pub halt: bool,
    pub local: LocalVariables,
    pub return_value: Object,
}

#[derive(Debug)]
pub struct ExecutorState {
    pub return_stack: ReturnStack,
    pub argument_stack: ArgumentStack,
    pub operand_stack: OperandStack,
    pub program_state: ProgramState,
    pub exit_code: u64,
}

pub struct Executor {
    program_memory: Memory,
    section_manager: SectionManager,
    argument_memory: ArgumentMemory,
    state: ExecutorState,
}

impl ProgramState {
    pub fn new() -> Self {
        Self {
            ip: Address::new(0),
            halt: false,
            local: LocalVariables::new(),
            return_value: Object::new(LoadedType::Void),
        }
    }

    pub fn inc_ip(&mut self, amount: usize) {
        self.ip += amount
    }
}

impl ExecutorState {
    pub fn new() -> Self {
        Self {
            return_stack: ReturnStack::new(),
            argument_stack: ArgumentStack::new(),
            operand_stack: OperandStack::new(),
            program_state: ProgramState::new(),
            exit_code: 0,
        }
    }
}

impl Executor {
    pub fn new(mem_size: usize) -> Self {
        Self {
            program_memory: Memory::new(mem_size),
            section_manager: SectionManager::new(),
            argument_memory: ArgumentMemory::new(),
            state: ExecutorState::new(),
        }
    }

    pub fn section_manager(&mut self) -> &mut SectionManager {
        return &mut self.section_manager;
    }

    pub fn load_section(&mut self, section: &SinSection, data: &[u8]) {
        self.section_manager
            .load_section(section, data, &mut self.program_memory);
    }

    pub fn init(&mut self) {
        TYPE_HEAP.write().unwrap().init(&self.section_manager);
    }

    pub fn execute(&mut self) {
        while !self.state.program_state.halt {
            {
                let mut instruction = match decode(
                    &mut self.state,
                    &mut self.argument_memory,
                    &self.program_memory,
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
                            self.state.program_state.ip
                        );
                        return;
                    }
                };
            }
        }
        println!("Program exit with exit code {}", self.state.exit_code);
    }
}
