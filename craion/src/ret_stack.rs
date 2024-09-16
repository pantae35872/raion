use crate::memory::address::Address;

#[derive(Debug)]
pub struct RetStack {
    data: Vec<Address>,
}

impl RetStack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: Address) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<Address> {
        self.data.pop()
    }
}
