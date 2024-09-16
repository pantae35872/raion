use std::fmt::{Debug, Display};

pub mod asm_token;

pub trait Token: Display + Debug + Clone + PartialEq {
    fn is_newline(&self) -> bool;
}
