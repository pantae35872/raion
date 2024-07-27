use crate::memory::Memory;

pub struct Executor<'a> {
    memory: &'a mut Memory,
}

impl<'a> Executor<'a> {
    pub fn new(memory: &'a mut Memory) -> Self {
        Self { memory }
    }

    pub fn tick(&mut self) {
        let a = self.memory.mem_get(0);
    }
}
