use crate::memory::{address::Address, Memory};

pub mod registers;

pub struct Executor<'a> {
    memory: &'a mut Memory,
}

impl<'a> Executor<'a> {
    pub fn new(memory: &'a mut Memory) -> Self {
        Self { memory }
    }

    pub fn tick(&mut self) {
        let a = self.memory.mem_get(Address::new(0));
    }
}
