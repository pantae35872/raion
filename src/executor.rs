use crate::{
    decoder::Decoder,
    memory::{address::Address, Memory},
};

use self::registers::RegisterFile;

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
        let mut decoder = Decoder::new(self.memory, self.register);
        decoder.decode_and_execute().unwrap();
    }

    pub fn debug_register(&self) {
        println!("{:?}", self.register);
    }
}
