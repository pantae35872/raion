use crate::{
    decoder::Decoder,
    memory::{address::Address, argument_memory::ArgumentMemory, Memory},
};

use self::registers::RegisterFile;
use crate::decoder::instruction::Instruction;

pub mod registers;

pub struct Executor<'a> {
    memory: &'a mut Memory,
    register: &'a mut RegisterFile,
    argument_memory: &'a mut ArgumentMemory,
}

impl<'a> Executor<'a> {
    pub fn new(
        memory: &'a mut Memory,
        register: &'a mut RegisterFile,
        argument_memory: &'a mut ArgumentMemory,
    ) -> Self {
        Self {
            memory,
            register,
            argument_memory,
        }
    }

    pub fn execute(&mut self) {
        while !self.register.get_halt() {
            {
                let mut decoder = Decoder::new(self.memory, self.register, self.argument_memory);
                let mut instruction = match decoder.decode() {
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
                            "Error occur while executing instruction: {}, with opcode: {}",
                            e,
                            instruction.op_code()
                        );
                        return;
                    }
                };
            }
        }
    }

    pub fn debug_register(&self) {
        println!("{:?}", self.register);
    }
}
