use crate::{
    decoder::Decoder,
    memory::{address::Address, Memory},
};

use self::registers::RegisterFile;
use crate::decoder::instruction::Instruction;

pub mod registers;

pub struct Executor<'a> {
    memory: &'a mut Memory,
    register: &'a mut RegisterFile,
}

impl<'a> Executor<'a> {
    pub fn new(memory: &'a mut Memory, register: &'a mut RegisterFile) -> Self {
        Self { memory, register }
    }

    pub fn execute(&mut self) {
        while !self.register.get_halt() {
            {
                let mut decoder = Decoder::new(self.memory, self.register);
                let mut instruction = match decoder.decode() {
                    Ok(result) => result,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };

                instruction.execute().unwrap();
            }
            self.debug_register();
        }
    }

    pub fn debug_register(&self) {
        println!("{:?}", self.register);
    }
}
