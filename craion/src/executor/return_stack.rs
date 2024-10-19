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
        state.ip = match self.data.pop() {
            Some(ip) => ip,
            None => {
                // No more to return means the main proc returns
                state.halt = true;
                return;
            }
        };
        state.local.restore_local();
    }
}
