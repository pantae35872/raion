use std::{fmt::Display, ops::Add};

#[derive(PartialEq, Debug, Clone)]
pub struct Address(usize);

impl Address {
    pub fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub fn get_raw(&self) -> usize {
        self.0
    }
}

impl Add<usize> for Address {
    type Output = Address;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
