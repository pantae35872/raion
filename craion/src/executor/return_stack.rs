use crate::memory::address::Address;

use super::ProgramState;

#[derive(Debug)]
pub struct ReturnStack {
    data: Vec<Address>,
}

impl ReturnStack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, address: Address) {
        self.data.push(address);
    }

    pub fn ret(&mut self, state: &mut ProgramState) {
        state.local.restore_local();
        state.ip = self.data.pop().expect("Return Stack underflow");
    }
}
