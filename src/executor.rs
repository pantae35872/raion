use crate::{
    decoder::Decoder,
    memory::{address::Address, Memory},
};

use self::registers::Register;

pub mod registers;

pub struct Executor<'a> {
    memory: &'a mut Memory,
    register: &'a mut Register,
}

impl<'a> Executor<'a> {
    pub fn new(memory: &'a mut Memory, register: &'a mut Register) -> Self {
        Self { memory, register }
    }

    pub fn execute(&mut self) {
        let decoder = Decoder::new(self.memory, self.register);
        decoder.decode().unwrap();
    }
}
