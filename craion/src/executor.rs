use crate::{
    decoder::decode,
    memory::{argument_memory::ArgumentMemory, Memory},
};

use self::registers::RegisterFile;

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
                let mut instruction = match decode(self.memory, self.register, self.argument_memory)
                {
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
    }

    pub fn debug_register(&self) {
        println!("{:?}", self.register);
    }
}
