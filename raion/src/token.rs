use std::fmt::{Debug, Display};

pub mod asm_token;

pub trait Token: Display + Debug + Clone + PartialEq {
    fn is_newline(&self) -> bool;
    fn from_string(string: String) -> Self;
    fn from_u64(num: u64) -> Self;
}
